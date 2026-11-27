//! Windows-Aufrufe der Workbench; anderswo ohne Wirkung (Ziel ist der PC). Gleich wie im Studio, `k3c_kit` hat sie nicht.

#[cfg(windows)]
pub use self::windows::{attach_console, shell_open};

#[cfg(windows)]
mod windows {
    #![allow(unsafe_code)]

    use ::windows::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
    use ::windows::Win32::UI::Shell::ShellExecuteW;
    use ::windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
    use ::windows::core::{HSTRING, PCWSTR, w};

    /// Hängt die Release-EXE (ohne eigene Konsole) an die Konsole des Aufrufers, damit `--headless` auf Strg+C endet.
    pub fn attach_console() {
        // SAFETY: Aufruf ohne Zeiger; scheitert er (schon eine Konsole oder keine beim Aufrufer), ändert sich nichts.
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
    }

    /// Öffnet `url` mit dem Standardprogramm. Der Aufrufer prüft die Adresse vorher (`app::open_url::allowed_url`).
    pub fn shell_open(url: &str) -> Result<(), String> {
        let target = HSTRING::from(url);
        // SAFETY: `target` ist eine nullterminierte UTF-16-Zeichenkette, die den Aufruf überlebt; die übrigen Zeiger
        // sind null oder statische Zeichenketten.
        let instance =
            unsafe { ShellExecuteW(None, w!("open"), &target, PCWSTR::null(), PCWSTR::null(), SW_SHOWNORMAL) };
        // Laut Win32 bedeutet ein Wert bis 32 einen Fehler.
        let code = instance.0 as usize;
        if code > 32 {
            Ok(())
        } else {
            Err(format!("Adresse nicht geöffnet (Code {code})"))
        }
    }
}

/// Nur Windows: die Release-EXE hat keine eigene Konsole.
#[cfg(not(windows))]
pub fn attach_console() {}

/// Nur Windows.
#[cfg(not(windows))]
pub fn shell_open(_url: &str) -> Result<(), String> {
    Err("Adressen öffnen geht nur unter Windows".to_owned())
}
