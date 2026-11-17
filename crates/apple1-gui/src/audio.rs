//! Cassette sound. Linux: ALSA, with libasound opened at run time (dlopen),
//! so the program has no link dependency on it and keeps starting on any
//! Linux (glibc 2.31 build). Windows: the waveOut API of winmm.dll, present
//! on every Windows. Without an audio device, the sound is simply off.

#[cfg(not(windows))]
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CString};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

pub const RATE: u32 = 22050;

#[cfg(not(windows))]
type Open = unsafe extern "C" fn(*mut *mut c_void, *const c_char, c_int, c_int) -> c_int;
#[cfg(not(windows))]
type SetParams = unsafe extern "C" fn(*mut c_void, c_int, c_int, c_uint, c_uint, c_int, c_uint) -> c_int;
#[cfg(not(windows))]
type Writei = unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_long;
#[cfg(not(windows))]
type Recover = unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int;

/// Sound output; `push` never blocks (chunks are dropped when the audio
/// is behind, e.g. when the emulation runs faster than real time).
pub struct Audio {
    tx: SyncSender<Vec<i16>>,
}

impl Audio {
    /// None when ALSA or the default device is not available.
    pub fn open() -> Option<Audio> {
        let (tx, rx) = sync_channel::<Vec<i16>>(6);
        let (ok_tx, ok_rx) = sync_channel::<bool>(1);
        std::thread::Builder::new()
            .name("son-cassette".into())
            .spawn(move || {
                // SAFETY: plain C calls into libasound with valid arguments;
                // the library stays loaded for the life of the thread.
                unsafe { play(rx, ok_tx) }
            })
            .ok()?;
        ok_rx.recv().ok()?.then_some(Audio { tx })
    }

    pub fn push(&self, samples: &[f32], volume: f32) {
        let v = volume.clamp(0.0, 1.0) * 30000.0;
        let buf: Vec<i16> = samples.iter().map(|x| (x * v) as i16).collect();
        if let Err(TrySendError::Disconnected(_)) = self.tx.try_send(buf) {
            // the audio thread stopped (device gone): nothing to do
        }
    }
}

unsafe fn play(rx: Receiver<Vec<i16>>, ok: SyncSender<bool>) {
    #[cfg(windows)]
    play_waveout(rx, ok);
    #[cfg(not(windows))]
    play_alsa(rx, ok);
}

/// Windows: waveOut, 16-bit mono PCM. Chunks are queued as wave headers;
/// at most `QUEUE` chunks wait in the device (about the latency of ALSA).
#[cfg(windows)]
unsafe fn play_waveout(rx: Receiver<Vec<i16>>, ok: SyncSender<bool>) {
    use std::collections::VecDeque;
    use windows_sys::Win32::Media::Audio::{
        waveOutClose, waveOutOpen, waveOutPrepareHeader, waveOutReset, waveOutUnprepareHeader, waveOutWrite, CALLBACK_NULL, HWAVEOUT,
        WAVEFORMATEX, WAVEHDR, WAVE_FORMAT_PCM, WAVE_MAPPER, WHDR_DONE,
    };
    use windows_sys::Win32::Media::MMSYSERR_NOERROR;
    const QUEUE: usize = 10;
    // after the device ran dry (the emulation advanced by jolts, in a slow
    // virtual machine), about 0.1 s is gathered before playing again: one
    // short gap instead of many (note 8)
    const PRIME: usize = RATE as usize / 10;
    let mut pending: Vec<i16> = Vec::new();
    let hdr_size = std::mem::size_of::<WAVEHDR>() as u32;
    let fmt = WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_PCM as u16,
        nChannels: 1,
        nSamplesPerSec: RATE,
        nAvgBytesPerSec: RATE * 2,
        nBlockAlign: 2,
        wBitsPerSample: 16,
        cbSize: 0,
    };
    let mut h: HWAVEOUT = std::ptr::null_mut();
    if waveOutOpen(&mut h, WAVE_MAPPER, &fmt, 0, 0, CALLBACK_NULL) != MMSYSERR_NOERROR {
        let _ = ok.send(false);
        return;
    }
    let _ = ok.send(true);
    // Each header lives on the heap (fixed address, raw pointer: the driver
    // writes dwFlags behind our back) with its samples, until the device is done.
    let mut queue: VecDeque<(*mut WAVEHDR, Vec<i16>)> = VecDeque::new();
    // WAVEHDR is packed: unaligned read through the raw pointer
    let done = |hdr: *mut WAVEHDR| std::ptr::addr_of!((*hdr).dwFlags).read_unaligned() & WHDR_DONE != 0;
    let release = |hdr: *mut WAVEHDR| {
        waveOutUnprepareHeader(h, hdr, hdr_size);
        drop(Box::from_raw(hdr));
    };
    while let Ok(buf) = rx.recv() {
        loop {
            while queue.front().is_some_and(|&(hdr, _)| done(hdr)) {
                let (hdr, _samples) = queue.pop_front().unwrap();
                release(hdr);
            }
            if queue.len() < QUEUE {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let mut buf = if queue.is_empty() {
            pending.extend_from_slice(&buf);
            if pending.len() < PRIME {
                continue;
            }
            std::mem::take(&mut pending)
        } else {
            buf
        };
        let mut hdr: WAVEHDR = std::mem::zeroed();
        hdr.lpData = buf.as_mut_ptr() as *mut u8;
        hdr.dwBufferLength = (buf.len() * 2) as u32;
        let hdr = Box::into_raw(Box::new(hdr));
        if waveOutPrepareHeader(h, hdr, hdr_size) != MMSYSERR_NOERROR {
            drop(Box::from_raw(hdr));
            break;
        }
        if waveOutWrite(h, hdr, hdr_size) != MMSYSERR_NOERROR {
            release(hdr);
            break;
        }
        queue.push_back((hdr, buf));
    }
    waveOutReset(h);
    for (hdr, _samples) in queue {
        release(hdr);
    }
    waveOutClose(h);
}

#[cfg(not(windows))]
unsafe fn play_alsa(rx: Receiver<Vec<i16>>, ok: SyncSender<bool>) {
    let lib = ["libasound.so.2", "libasound.so"].iter().find_map(|n| libloading::Library::new(n).ok());
    let Some(lib) = lib else {
        let _ = ok.send(false);
        return;
    };
    let f = (|| -> Result<(Open, SetParams, Writei, Recover), libloading::Error> {
        Ok((*lib.get::<Open>(b"snd_pcm_open\0")?, *lib.get::<SetParams>(b"snd_pcm_set_params\0")?, *lib.get::<Writei>(b"snd_pcm_writei\0")?, *lib.get::<Recover>(b"snd_pcm_recover\0")?))
    })();
    let Ok((open, set_params, writei, recover)) = f else {
        let _ = ok.send(false);
        return;
    };
    let mut pcm: *mut c_void = std::ptr::null_mut();
    let name = CString::new("default").unwrap();
    // 0 = playback, blocking; S16_LE = 2, RW_INTERLEAVED = 3; 80 ms latency
    if open(&mut pcm, name.as_ptr(), 0, 0) < 0 || set_params(pcm, 2, 3, 1, RATE, 1, 80_000) < 0 {
        let _ = ok.send(false);
        return;
    }
    let _ = ok.send(true);
    while let Ok(buf) = rx.recv() {
        let mut off = 0;
        while off < buf.len() {
            let n = writei(pcm, buf[off..].as_ptr() as *const c_void, (buf.len() - off) as c_ulong);
            if n < 0 {
                if recover(pcm, n as c_int, 1) < 0 {
                    return;
                }
            } else {
                off += n as usize;
            }
        }
    }
}
