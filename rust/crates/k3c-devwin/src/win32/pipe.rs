//! Pipe-Server: erzeugt `\\.\pipe\<name>` (eine Instanz, nur lokal), nimmt das Spiel als Client an und tauscht
//! Zeilen-JSON-Nachrichten aus `k3c-devproto`. Ein I/O-Thread bedient Lesen und Schreiben im Wechsel (kurzes
//! Abfragen mit `PeekNamedPipe`), weil ein synchrones Pipe-Handle gleichzeitiges Lesen und Schreiben aus zwei
//! Threads serialisiert und blockieren würde. Das Fenster hängt nur an der Schnittstelle [`Sink`].

use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use k3c_devproto::{MAX_LINE, Msg, PROTO_VERSION, ProtoError, decode_line, encode_line};
use windows::Win32::Foundation::{CloseHandle, ERROR_PIPE_CONNECTED, HANDLE};
use windows::Win32::Storage::FileSystem::{FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_DUPLEX, ReadFile, WriteFile};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS,
    PIPE_TYPE_BYTE, PIPE_WAIT, PeekNamedPipe,
};
use windows::core::{Error, HRESULT, PCWSTR, Result};

use super::controls::wide;

const BUF: u32 = 64 * 1024;
const POLL: Duration = Duration::from_millis(10);

/// Sink nimmt zu, was die Pipe liefert; `false` beendet den Thread (das Fenster ist weg).
pub trait Sink: Send + Sync + 'static {
    /// Eine Nachricht des Spiels.
    fn msg(&self, m: Msg) -> bool;
    /// Die Verbindung ist zu Ende (der Server wartet danach auf einen neuen Client).
    fn lost(&self) -> bool;
}

struct Pipe(HANDLE);

// SAFETY: das Handle gehört ausschließlich dem I/O-Thread, in den es verschoben wird; kein anderer Thread greift zu.
unsafe impl Send for Pipe {}

/// Ende einer Sitzung.
enum Exit {
    /// Der Client ist weg; auf den nächsten warten.
    Lost,
    /// Nichts mehr zu tun (Fenster weg, Handle fehlerhaft).
    Stop,
}

/// start erzeugt die Pipe (scheitert, wenn der Name belegt ist) und startet den I/O-Thread; über den Sender gehen
/// Nachrichten an das Spiel (ohne Client werden sie beim Verbinden verworfen).
pub fn start(name: &str, sink: Arc<dyn Sink>) -> Result<Sender<Msg>> {
    let path = wide(&format!(r"\\.\pipe\{name}"));
    // SAFETY: path ist nullterminiert und lebt über den Aufruf; keine Sicherheitsattribute (Standard: nur lokal
    // durch PIPE_REJECT_REMOTE_CLIENTS, Zugriff für den eigenen Benutzer); FIRST_PIPE_INSTANCE lehnt einen belegten Namen ab.
    let h = unsafe {
        CreateNamedPipeW(
            PCWSTR(path.as_ptr()),
            PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            BUF,
            BUF,
            0,
            None,
        )
    };
    if h.is_invalid() {
        return Err(Error::from_thread());
    }
    let (tx, rx) = mpsc::channel();
    let pipe = Pipe(h);
    thread::Builder::new()
        .name("k3c-devwin-pipe".into())
        .spawn(move || serve(pipe, rx, sink))
        .map_err(|e| {
            // SAFETY: h ist das eben erzeugte Handle; der Thread, der es übernommen hätte, läuft nicht.
            let _ = unsafe { CloseHandle(h) };
            Error::new(HRESULT(-1), e.to_string())
        })?;
    Ok(tx)
}

fn serve(pipe: Pipe, rx: Receiver<Msg>, sink: Arc<dyn Sink>) {
    let h = pipe.0;
    loop {
        if let Err(e) = connect(h) {
            eprintln!("❌ Pipe: Warten auf das Spiel gescheitert: {e}");
            break;
        }
        while rx.try_recv().is_ok() {} // Befehle ohne Verbindung verwerfen
        let hello = Msg::Hello {
            v: PROTO_VERSION,
            game: None,
        };
        let exit = match write_msg(h, &hello) {
            Ok(()) => session(h, &rx, &*sink),
            Err(_) => Exit::Lost,
        };
        // SAFETY: h ist das eigene Pipe-Handle; danach wartet der Server wieder auf einen Client.
        let _ = unsafe { DisconnectNamedPipe(h) };
        if matches!(exit, Exit::Stop) || !sink.lost() {
            break;
        }
    }
    // SAFETY: h gehört diesem Thread und wird nicht mehr benutzt.
    let _ = unsafe { CloseHandle(h) };
}

fn connect(h: HANDLE) -> Result<()> {
    // SAFETY: h ist ein Server-Handle ohne OVERLAPPED; der Aufruf blockiert, bis ein Client kommt.
    match unsafe { ConnectNamedPipe(h, None) } {
        Err(e) if e.code() != HRESULT::from_win32(ERROR_PIPE_CONNECTED.0) => Err(e),
        _ => Ok(()),
    }
}

fn write_msg(h: HANDLE, m: &Msg) -> Result<()> {
    let line = encode_line(m).map_err(|e| Error::new(HRESULT(-1), e.to_string()))?;
    let mut rest = line.as_bytes();
    while !rest.is_empty() {
        let mut n = 0u32;
        // SAFETY: rest und n leben über den Aufruf; kein OVERLAPPED (synchron).
        unsafe { WriteFile(h, Some(rest), Some(&mut n), None) }?;
        rest = rest.get(n as usize..).unwrap_or_default();
    }
    Ok(())
}

fn available(h: HANDLE) -> Result<u32> {
    let mut n = 0u32;
    // SAFETY: n lebt über den Aufruf; die übrigen Zeiger sind None (nur die Zahl wartender Bytes wird gelesen).
    unsafe { PeekNamedPipe(h, None, 0, None, Some(&mut n), None) }?;
    Ok(n)
}

fn session(h: HANDLE, rx: &Receiver<Msg>, sink: &dyn Sink) -> Exit {
    let mut acc: Vec<u8> = Vec::new();
    loop {
        loop {
            match rx.try_recv() {
                Ok(m) => {
                    if write_msg(h, &m).is_err() {
                        return Exit::Lost;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return Exit::Stop,
            }
        }
        match available(h) {
            Err(_) => return Exit::Lost,
            Ok(0) => match rx.recv_timeout(POLL) {
                Ok(m) if write_msg(h, &m).is_err() => return Exit::Lost,
                Err(RecvTimeoutError::Disconnected) => return Exit::Stop,
                _ => {}
            },
            Ok(n) => {
                if let Some(exit) = read_some(h, n, &mut acc, sink) {
                    return exit;
                }
            }
        }
    }
}

/// read_some liest wartende Bytes und gibt vollständige Zeilen weiter; `Some` beendet die Sitzung.
fn read_some(h: HANDLE, avail: u32, acc: &mut Vec<u8>, sink: &dyn Sink) -> Option<Exit> {
    let mut buf = [0u8; 4096];
    let want = (avail as usize).min(buf.len());
    let mut got = 0u32;
    // SAFETY: buf[..want] und got leben über den Aufruf; kein OVERLAPPED (synchron).
    if unsafe { ReadFile(h, Some(&mut buf[..want]), Some(&mut got), None) }.is_err() {
        return Some(Exit::Lost);
    }
    acc.extend_from_slice(buf.get(..got as usize).unwrap_or_default());
    while let Some(pos) = acc.iter().position(|b| *b == b'\n') {
        let line: Vec<u8> = acc.drain(..=pos).collect();
        match decode_line(&String::from_utf8_lossy(&line)) {
            Ok(m) => {
                if !sink.msg(m) {
                    // Queue voll oder Fenster weg: Verbindung kappen; `lost` entscheidet, ob es weitergeht.
                    return Some(Exit::Lost);
                }
            }
            Err(ProtoError::Version(v)) => {
                eprintln!("🚫 Pipe: Protokollversion {v} nicht unterstützt");
                return Some(Exit::Lost);
            }
            Err(e) => eprintln!("🚫 Pipe: Zeile verworfen: {e}"),
        }
    }
    if acc.len() > MAX_LINE {
        eprintln!("🚫 Pipe: Zeile ohne Ende über {MAX_LINE} Bytes, Verbindung getrennt");
        return Some(Exit::Lost);
    }
    None
}
