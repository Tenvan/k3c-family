//! Prozesswerte (Port von `proc.go`, `proc_windows.go`): eigener Prozess und alle Nachkommen (Tests, Vite, Dienste,
//! WebView2). Nur lesende Zugriffe; anderswo bleibt alles null.

/// Ein Kindprozess.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    /// Arbeitssatz.
    pub rss: u64,
}

/// Schnappschuss des eigenen Prozesses.
#[derive(Debug, Default)]
pub(super) struct Snapshot {
    pub rss: u64,
    /// Privater Speicher (Commit).
    pub private: u64,
    pub handles: usize,
    pub threads: usize,
    /// Alle Nachkommen.
    pub children: Vec<ProcInfo>,
}

#[cfg(windows)]
pub(super) use self::windows::read;

#[cfg(not(windows))]
pub(super) fn read() -> Snapshot {
    Snapshot::default()
}

#[cfg(windows)]
mod windows {
    #![allow(unsafe_code)]

    use std::collections::HashMap;
    use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};

    use ::windows::Win32::Foundation::HANDLE;
    use ::windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
    };
    use ::windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    use ::windows::Win32::System::Threading::{
        GetCurrentProcess, GetProcessHandleCount, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    use super::{ProcInfo, Snapshot};

    /// Begrenzt das Folgen der Eltern-Kette (wiederverwendete PIDs können Zyklen bilden).
    const MAX_PARENT_CHAIN: usize = 64;

    /// Eine Zeile der Prozessliste.
    struct Entry {
        parent: u32,
        threads: u32,
        name: String,
    }

    /// Arbeitssatz und privater Speicher; bei Fehler null.
    fn memory(process: HANDLE) -> (u64, u64) {
        let mut counters = PROCESS_MEMORY_COUNTERS::default();
        let Ok(size) = u32::try_from(size_of::<PROCESS_MEMORY_COUNTERS>()) else {
            return (0, 0);
        };
        // SAFETY: `counters` lebt über den Aufruf und ist `size` Bytes groß.
        match unsafe { GetProcessMemoryInfo(process, &raw mut counters, size) } {
            Ok(()) => (counters.WorkingSetSize as u64, counters.PagefileUsage as u64),
            Err(_) => (0, 0),
        }
    }

    /// Alle Prozesse des Systems nach PID.
    fn processes() -> HashMap<u32, Entry> {
        let mut out = HashMap::new();
        // SAFETY: nur Zahlen als Argumente.
        let Ok(handle) = (unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }) else {
            return out;
        };
        // SAFETY: frisch angelegt, gehört nur diesem Aufruf; `OwnedHandle` schließt ihn.
        let snapshot = unsafe { OwnedHandle::from_raw_handle(handle.0) };
        let handle = HANDLE(snapshot.as_raw_handle());
        let Ok(size) = u32::try_from(size_of::<PROCESSENTRY32W>()) else {
            return out;
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: size,
            ..PROCESSENTRY32W::default()
        };
        // SAFETY: Schnappschuss offen, `entry` mit gesetzter Größe lebt über den Aufruf.
        let mut found = unsafe { Process32FirstW(handle, &raw mut entry) };
        while found.is_ok() {
            let end = entry
                .szExeFile
                .iter()
                .position(|c| *c == 0)
                .unwrap_or(entry.szExeFile.len());
            let line = Entry {
                parent: entry.th32ParentProcessID,
                threads: entry.cntThreads,
                name: String::from_utf16_lossy(&entry.szExeFile[..end]),
            };
            out.insert(entry.th32ProcessID, line);
            // SAFETY: wie oben.
            found = unsafe { Process32NextW(handle, &raw mut entry) };
        }
        out
    }

    fn is_descendant(mut pid: u32, root: u32, all: &HashMap<u32, Entry>) -> bool {
        for _ in 0..MAX_PARENT_CHAIN {
            match all.get(&pid).map(|e| e.parent) {
                None | Some(0) => return false,
                Some(parent) if parent == root => return true,
                Some(parent) => pid = parent,
            }
        }
        false
    }

    fn child(pid: u32, name: &str) -> ProcInfo {
        let mut info = ProcInfo {
            pid,
            name: name.to_owned(),
            rss: 0,
        };
        // SAFETY: nur Zahlen als Argumente; eine unbekannte oder gesperrte PID liefert einen Fehler.
        if let Ok(handle) = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) } {
            // SAFETY: frisch geöffnet, gehört nur diesem Aufruf; `OwnedHandle` schließt ihn.
            let process = unsafe { OwnedHandle::from_raw_handle(handle.0) };
            info.rss = memory(HANDLE(process.as_raw_handle())).0;
        }
        info
    }

    pub(in super::super) fn read() -> Snapshot {
        // SAFETY: liefert ein Pseudo-Handle auf den eigenen Prozess, das niemand schließen muss.
        let own = unsafe { GetCurrentProcess() };
        let (rss, private) = memory(own);
        let mut handles = 0_u32;
        // SAFETY: `handles` lebt über den Aufruf. Ein Fehler lässt die Null stehen.
        let _ = unsafe { GetProcessHandleCount(own, &raw mut handles) };
        let all = processes();
        let root = std::process::id();
        let mut children: Vec<ProcInfo> = all
            .iter()
            .filter(|(pid, _)| is_descendant(**pid, root, &all))
            .map(|(pid, e)| child(*pid, &e.name))
            .collect();
        children.sort_by_key(|c| c.pid);
        Snapshot {
            rss,
            private,
            handles: handles as usize,
            threads: all.get(&root).map_or(0, |e| e.threads as usize),
            children,
        }
    }
}
