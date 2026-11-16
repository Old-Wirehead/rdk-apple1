//! What differs between Linux and Windows: opening a page or a folder with
//! the desktop's own program, the current date for the saved states, the
//! Windows language, and the report of a fatal error.

use std::ffi::OsStr;

/// Opens a web address or a folder with the desktop's program
/// (`xdg-open` on Linux, the Explorer on Windows, `open` on macOS).
pub fn open(target: impl AsRef<OsStr>) -> std::io::Result<()> {
    #[cfg(windows)]
    let prog = "explorer";
    #[cfg(target_os = "macos")]
    let prog = "open";
    #[cfg(not(any(windows, target_os = "macos")))]
    let prog = "xdg-open";
    std::process::Command::new(prog).arg(target).spawn().map(|_| ())
}

/// Name of the program used by `open`, for the error messages.
pub fn opener_name() -> &'static str {
    if cfg!(windows) {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
}

/// Local date and time « dd/mm HH:MM », empty when it cannot be read.
pub fn now_short() -> String {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::SystemInformation::GetLocalTime;
        // SAFETY: GetLocalTime only fills the structure it is given.
        let t = unsafe {
            let mut t = std::mem::zeroed();
            GetLocalTime(&mut t);
            t
        };
        format!("{:02}/{:02} {:02}:{:02}", t.wDay, t.wMonth, t.wHour, t.wMinute)
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new("date")
            .arg("+%d/%m %H:%M")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    }
}

/// The Windows user language, such as `fr-FR` (None elsewhere).
pub fn windows_locale() -> Option<String> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Globalization::GetUserDefaultLocaleName;
        let mut buf = [0u16; 85];
        // SAFETY: the buffer and its length match; the function writes at most that many characters.
        let n = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
        (n > 1).then(|| String::from_utf16_lossy(&buf[..n as usize - 1]))
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// A line in `gui.log` (Windows: in the cache folder, as `fatal`; elsewhere
/// on the error output, which run.sh keeps in its own gui.log).
pub fn log(msg: &str) {
    #[cfg(windows)]
    {
        let dir = crate::config::cache_base();
        let _ = std::fs::create_dir_all(&dir);
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("gui.log")) {
            use std::io::Write;
            let _ = writeln!(f, "[{}] {msg}", now_short());
        }
    }
    #[cfg(not(windows))]
    eprintln!("apple1-emu-gui : {msg}");
}

/// Reports an error that stops the program on the error output. On Windows,
/// where there is no console, it is also written at the end of `gui.log` in
/// the cache folder and shown in a message box (on Linux, run.sh already
/// keeps the error output in `gui.log`).
pub fn fatal(msg: &str) {
    eprintln!("apple1-emu-gui : {msg}");
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};
        let dir = crate::config::cache_base();
        let log = dir.join("gui.log");
        let _ = std::fs::create_dir_all(&dir);
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&log) {
            use std::io::Write;
            let _ = writeln!(f, "[{}] {msg}", now_short());
        }
        let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
        let text = wide(&format!("{msg}\n\n{}", log.display()));
        let title = wide("Retro Dev Kit Apple 1");
        // SAFETY: both strings are NUL-terminated UTF-16 that live until the call returns.
        unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONERROR) };
    }
}
