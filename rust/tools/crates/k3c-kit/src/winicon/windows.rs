//! Windows-Fassung (`winicon_windows.go`).

#![allow(unsafe_code)]

use std::time::Duration;

use ::windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use ::windows::Win32::System::Threading::GetCurrentProcessId;
use ::windows::Win32::UI::WindowsAndMessaging::{
    CreateIconFromResourceEx, EnumWindows, GetWindowThreadProcessId, ICON_BIG, ICON_SMALL, IMAGE_FLAGS,
    IsWindowVisible, SendMessageW, WM_SETICON,
};
use ::windows::core::BOOL;

/// Version des Ressourcenformats, die `CreateIconFromResourceEx` verlangt.
const RESOURCE_VERSION: u32 = 0x0003_0000;
const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
/// Signatur und Kopfblock (IHDR): kürzer ist kein PNG.
const PNG_MIN: usize = 33;

/// Setzt das Icon aller sichtbaren Fenster dieses Prozesses aus `png`. Ein Fenster entsteht kurz nach dem Start,
/// deshalb mit wenigen Wiederholungen (bis 5 s): der Aufruf blockiert, also in einem eigenen Thread rufen. Ein Fehler
/// lässt das Standard-Icon stehen.
pub fn set(png: &[u8]) {
    let mut icons = Icons {
        // SAFETY: liest nur die eigene Prozess-Kennung, ohne Voraussetzungen.
        process: unsafe { GetCurrentProcessId() },
        big: handle(png, 32),
        small: handle(png, 16),
        set: false,
    };
    if icons.big == 0 {
        return;
    }
    for _ in 0..20 {
        let state = LPARAM(&raw mut icons as isize);
        // SAFETY: `EnumWindows` ruft `apply` nur während dieses Aufrufs; `state` zeigt solange auf `icons`, auf das
        // sonst niemand zugreift.
        let _ = unsafe { EnumWindows(Some(apply), state) };
        if icons.set {
            return;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

/// Zustand der Fenstersuche.
struct Icons {
    process: u32,
    big: isize,
    small: isize,
    set: bool,
}

/// Setzt die Icons, wenn das Fenster sichtbar ist und diesem Prozess gehört; fremde Fenster bleiben unberührt.
unsafe extern "system" fn apply(window: HWND, state: LPARAM) -> BOOL {
    // SAFETY: `state` ist der Zeiger auf `Icons` aus `set`, gültig und exklusiv für die Dauer von `EnumWindows`.
    let Some(icons) = (unsafe { (state.0 as *mut Icons).as_mut() }) else {
        return BOOL(0);
    };
    let mut owner = 0u32;
    // SAFETY: `owner` ist eine gültige Zielvariable; ein inzwischen ungültiges Fenster liefert nur 0.
    unsafe { GetWindowThreadProcessId(window, Some(&raw mut owner)) };
    // SAFETY: fragt nur den Zustand des Fensters ab.
    if owner == icons.process && unsafe { IsWindowVisible(window) }.as_bool() {
        for (kind, icon) in [(ICON_BIG, icons.big), (ICON_SMALL, icons.small)] {
            let (kind, icon) = (WPARAM(kind as usize), LPARAM(icon));
            // SAFETY: `WM_SETICON` an ein eigenes Fenster; die Parameter sind Zahlen und ein Icon-Handle, keine Zeiger.
            unsafe { SendMessageW(window, WM_SETICON, Some(kind), Some(icon)) };
        }
        icons.set = true;
    }
    BOOL(1)
}

/// Macht aus dem PNG ein Icon-Handle der Kantenlänge `px` (0 bei Fehler oder wenn es kein PNG ist). Das Handle bleibt
/// bis zum Ende des Prozesses bestehen, weil die Fenster es benutzen.
pub fn handle(png: &[u8], px: i32) -> isize {
    if png.len() < PNG_MIN || !png.starts_with(PNG_SIGNATURE) || u32::try_from(png.len()).is_err() {
        return 0;
    }
    // SAFETY: Windows liest höchstens `png.len()` Bytes aus der Scheibe und kopiert die Daten ins Icon.
    let icon = unsafe { CreateIconFromResourceEx(png, true, RESOURCE_VERSION, px, px, IMAGE_FLAGS(0)) };
    icon.map_or(0, |icon| icon.0 as isize)
}
