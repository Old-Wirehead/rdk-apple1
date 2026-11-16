//! What differs between Linux and Windows: opening a page or a folder with
//! the desktop's own program, and the current date for the saved states.

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
