//! Cassette sound through ALSA. libasound is opened at run time (dlopen),
//! so the program has no link dependency on it and keeps starting on any
//! Linux (glibc 2.31 build); without ALSA or without an audio device, the
//! sound is simply off.

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CString};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

pub const RATE: u32 = 22050;

type Open = unsafe extern "C" fn(*mut *mut c_void, *const c_char, c_int, c_int) -> c_int;
type SetParams = unsafe extern "C" fn(*mut c_void, c_int, c_int, c_uint, c_uint, c_int, c_uint) -> c_int;
type Writei = unsafe extern "C" fn(*mut c_void, *const c_void, c_ulong) -> c_long;
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
