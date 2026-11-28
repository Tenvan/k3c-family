//! Windows-Teil von `system`: TCP-Tabelle (IP Helper), Messwerte, Eltern- und Programmdatei eines Prozesses. Nur
//! lesende Zugriffe.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::time::Duration;

use ::windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, FILETIME, HANDLE, NO_ERROR};
use ::windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, MIB_TCP6ROW_OWNER_PID, MIB_TCPROW_OWNER_PID, TCP_TABLE_OWNER_PID_LISTENER,
};
use ::windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use ::windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use ::windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use ::windows::Win32::System::Threading::{
    GetProcessTimes, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};
use ::windows::core::PWSTR;
use chrono::{DateTime, Local};

use super::{Conn, Usage};

/// Roh-Tabelle der lauschenden Sockets einer Adressfamilie (`u32`-ausgerichtet): erst Anzahl, dann die Zeilen.
fn tcp_table(family: u32) -> Option<Vec<u32>> {
    let mut size = 0_u32;
    let mut table: Vec<u32> = Vec::new();
    // Die Tabelle kann zwischen zwei Aufrufen wachsen.
    for _ in 0..4 {
        let buffer = (!table.is_empty()).then(|| table.as_mut_ptr().cast());
        // SAFETY: `buffer` ist leer oder zeigt auf mindestens `size` Bytes in `table`; `size` lebt über den Aufruf.
        let code =
            unsafe { GetExtendedTcpTable(buffer, &raw mut size, false, family, TCP_TABLE_OWNER_PID_LISTENER, 0) };
        if code == NO_ERROR.0 && !table.is_empty() {
            return Some(table);
        }
        if code != ERROR_INSUFFICIENT_BUFFER.0 && code != NO_ERROR.0 {
            return None;
        }
        table = vec![0; (size as usize).div_ceil(4) + 1];
        size = u32::try_from(table.len() * 4).ok()?;
    }
    None
}

/// Die Zeilen hinter der Anzahl.
fn rows<T: Copy>(table: &[u32]) -> Vec<T> {
    let Some((&count, rest)) = table.split_first() else {
        return Vec::new();
    };
    let fits = size_of_val(rest) / size_of::<T>();
    let start = rest.as_ptr().cast::<T>();
    (0..fits.min(count as usize))
        // SAFETY: `i` liegt innerhalb von `rest`; `T` ist eine Zeile der Tabelle aus lauter `u32` und Bytefeldern,
        // `read_unaligned` stellt keine Ansprüche an die Ausrichtung.
        .map(|i| unsafe { start.add(i).read_unaligned() })
        .collect()
}

/// Port in Netzwerk-Reihenfolge in den unteren 16 Bit.
fn port(raw: u32) -> u16 {
    let bytes = raw.to_le_bytes();
    u16::from_be_bytes([bytes[0], bytes[1]])
}

pub(super) fn listeners() -> Vec<Conn> {
    let mut out = Vec::new();
    if let Some(table) = tcp_table(u32::from(AF_INET.0)) {
        out.extend(rows::<MIB_TCPROW_OWNER_PID>(&table).into_iter().map(|r| Conn {
            // Die Adresse steht im Speicher schon in Netzwerk-Reihenfolge.
            ip: IpAddr::V4(Ipv4Addr::from(r.dwLocalAddr.to_le_bytes())),
            port: port(r.dwLocalPort),
            pid: r.dwOwningPid,
        }));
    }
    if let Some(table) = tcp_table(u32::from(AF_INET6.0)) {
        out.extend(rows::<MIB_TCP6ROW_OWNER_PID>(&table).into_iter().map(|r| Conn {
            ip: IpAddr::V6(Ipv6Addr::from(r.ucLocalAddr)),
            port: port(r.dwLocalPort),
            pid: r.dwOwningPid,
        }));
    }
    out
}

/// # Safety
/// `handle` ist ein gültiges, offenes Handle, das sonst niemand schließt.
unsafe fn owned(handle: HANDLE) -> OwnedHandle {
    // SAFETY: Zusage des Aufrufers.
    unsafe { OwnedHandle::from_raw_handle(handle.0) }
}

fn raw(handle: &OwnedHandle) -> HANDLE {
    HANDLE(handle.as_raw_handle())
}

/// Handle nur zum Abfragen.
fn open(pid: u32) -> Result<OwnedHandle, String> {
    // SAFETY: nur Zahlen als Argumente; eine unbekannte PID liefert einen Fehler.
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }
        .map_err(|e| format!("Prozess {pid}: {e}"))?;
    // SAFETY: frisch geöffnet, gehört nur diesem Aufruf.
    Ok(unsafe { owned(handle) })
}

/// 100-ns-Schritte eines `FILETIME`.
fn ticks(t: FILETIME) -> u64 {
    u64::from(t.dwHighDateTime) << 32 | u64::from(t.dwLowDateTime)
}

/// Zeitpunkt eines `FILETIME` (zählt ab 1601).
fn time_of(t: FILETIME) -> Option<DateTime<Local>> {
    const UNIX_EPOCH_TICKS: u64 = 116_444_736_000_000_000;
    let millis = ticks(t).checked_sub(UNIX_EPOCH_TICKS)? / 10_000;
    DateTime::from_timestamp_millis(i64::try_from(millis).ok()?).map(|t| t.with_timezone(&Local))
}

pub(super) fn usage(pid: u32) -> Result<Usage, String> {
    let process = open(pid)?;
    let (mut created, mut exited, mut kernel, mut user) = (
        FILETIME::default(),
        FILETIME::default(),
        FILETIME::default(),
        FILETIME::default(),
    );
    // SAFETY: Das Handle ist offen, alle vier Ziele leben über den Aufruf.
    unsafe {
        GetProcessTimes(
            raw(&process),
            &raw mut created,
            &raw mut exited,
            &raw mut kernel,
            &raw mut user,
        )
    }
    .map_err(|e| e.to_string())?;
    let mut memory = PROCESS_MEMORY_COUNTERS::default();
    let size = u32::try_from(size_of::<PROCESS_MEMORY_COUNTERS>()).map_err(|e| e.to_string())?;
    // SAFETY: `memory` lebt über den Aufruf und ist `size` Bytes groß.
    unsafe { GetProcessMemoryInfo(raw(&process), &raw mut memory, size) }.map_err(|e| e.to_string())?;
    Ok(Usage {
        cpu: Duration::from_nanos((ticks(kernel) + ticks(user)).saturating_mul(100)),
        memory: memory.WorkingSetSize as u64,
        started: time_of(created),
    })
}

pub(super) fn parent(pid: u32) -> Option<u32> {
    // SAFETY: nur Zahlen als Argumente.
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.ok()?;
    // SAFETY: frisch angelegt, gehört nur diesem Aufruf.
    let snapshot = unsafe { owned(snapshot) };
    let mut entry = PROCESSENTRY32W {
        dwSize: u32::try_from(size_of::<PROCESSENTRY32W>()).ok()?,
        ..PROCESSENTRY32W::default()
    };
    // SAFETY: Schnappschuss offen, `entry` mit gesetzter Größe lebt über den Aufruf.
    let mut found = unsafe { Process32FirstW(raw(&snapshot), &raw mut entry) };
    while found.is_ok() {
        if entry.th32ProcessID == pid {
            return Some(entry.th32ParentProcessID);
        }
        // SAFETY: wie oben.
        found = unsafe { Process32NextW(raw(&snapshot), &raw mut entry) };
    }
    None
}

pub(super) fn image(pid: u32) -> Option<String> {
    let process = open(pid).ok()?;
    let mut buf = vec![0_u16; 32_768];
    let mut len = u32::try_from(buf.len()).ok()?;
    // SAFETY: `buf` fasst `len` Zeichen, beide leben über den Aufruf.
    unsafe { QueryFullProcessImageNameW(raw(&process), PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &raw mut len) }
        .ok()?;
    Some(String::from_utf16_lossy(buf.get(..len as usize)?))
}
