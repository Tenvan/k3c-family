//! Pipe-Client des Cheat-Fensters (RU56.5, B-1133): ein Lese-Thread und ein Schreib-Thread je Verbindung, ohne
//! `unsafe` (`std::fs`). Das Spiel ruft nur `poll`, `send_state` und `send_ctl`; nichts davon blockiert.
//!
//! Ein synchrones Pipe-Handle serialisiert Lesen und Schreiben: ein wartendes `ReadFile` hält jedes `WriteFile` auf
//! (belegt, RU56.5). Darum liest der Lese-Thread nur, solange der Schreib-Thread nichts zu tun hat (`GatedRead`), und
//! der Server des Fensters sendet alle 50 ms einen Herzschlag, der das Lesen beendet.
//!
//! Schreiben: Zustand (`state`) darf bei voller Warteschlange verworfen werden, Steuernachrichten (`hello`, `catalog`,
//! `ack`, `bye`) nie. Lesen: eine defekte, zu lange oder fremde Zeile beendet die Verbindung (Ereignis `Lost`).

use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread;

use k3c_devproto::{MAX_LINE, Msg, decode_line, encode_line};

/// Größe der Warteschlange, ab der `state` verworfen wird.
pub const STATE_QUEUE: usize = 8;

/// Streams sind die beiden Enden einer Verbindung (Lesen und Schreiben).
pub type Streams = (Box<dyn Read + Send>, Box<dyn Write + Send>);

/// Opener öffnet eine Verbindung; er darf nicht blockieren (Pipe fehlt oder belegt: sofort ein Fehler).
pub type Opener = Box<dyn FnMut() -> io::Result<Streams> + Send>;

/// open_pipe öffnet die Pipe `path` (`\\.\pipe\k3c-devwin-<id>`) zum Lesen und Schreiben.
pub fn open_pipe(path: String) -> Opener {
    Box::new(move || {
        let reader = OpenOptions::new().read(true).write(true).open(&path)?;
        let writer = reader.try_clone()?;
        Ok((Box::new(reader), Box::new(writer)))
    })
}

/// Event ist, was der Lese- oder Schreib-Thread dem Spiel meldet.
#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)] // wenige Nachrichten je Sekunde, Box brächte nur eine Allokation je Nachricht
pub enum Event {
    /// Eine Nachricht des Fensters.
    Msg(Msg),
    /// Die Verbindung ist zu Ende (Pipe weg, defekte Zeile, Schreibfehler).
    Lost,
}

#[derive(Default)]
struct Queue {
    items: VecDeque<Msg>,
    /// Der Schreib-Thread schreibt gerade (die Nachricht ist schon aus der Warteschlange).
    writing: bool,
    closed: bool,
}

struct Shared {
    queue: Mutex<Queue>,
    cv: Condvar,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, Queue> {
        self.queue.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Link ist eine Verbindung zum Fenster; beim Löschen geht `bye` hinaus und die Threads enden.
pub struct Link {
    shared: Arc<Shared>,
    rx: Receiver<Event>,
}

impl Link {
    /// connect startet Lese- und Schreib-Thread auf den Streams.
    pub fn connect((reader, writer): Streams) -> io::Result<Link> {
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue::default()),
            cv: Condvar::new(),
        });
        let (tx, rx) = mpsc::channel();
        let (sh, txw, sh_read) = (Arc::clone(&shared), tx.clone(), Arc::clone(&shared));
        thread::Builder::new()
            .name("k3c-devwin-write".into())
            .spawn(move || write_loop(&sh, writer, &txw))?;
        let res = thread::Builder::new()
            .name("k3c-devwin-read".into())
            .spawn(move || read_loop(reader, sh_read, &tx));
        if let Err(e) = res {
            close(&shared);
            return Err(e);
        }
        Ok(Link { shared, rx })
    }

    /// poll liefert das nächste Ereignis, ohne zu warten.
    pub fn poll(&mut self) -> Option<Event> {
        self.rx.try_recv().ok()
    }

    /// send_state reiht den Zustand ein; bei voller Warteschlange fällt er weg (`false`).
    pub fn send_state(&self, m: Msg) -> bool {
        let mut q = self.shared.lock();
        if q.closed || q.items.len() >= STATE_QUEUE {
            return false;
        }
        q.items.push_back(m);
        self.shared.cv.notify_all(); // Lese- und Schreib-Thread warten auf dieselbe Bedingungsvariable
        true
    }

    /// send_ctl reiht eine Steuernachricht ein; sie fällt nie weg.
    pub fn send_ctl(&self, m: Msg) {
        let mut q = self.shared.lock();
        if !q.closed {
            q.items.push_back(m);
            self.shared.cv.notify_all(); // Lese- und Schreib-Thread warten auf dieselbe Bedingungsvariable
        }
    }

    /// pending ist die Zahl der Nachrichten, die noch nicht geschrieben sind.
    pub fn pending(&self) -> usize {
        self.shared.lock().items.len()
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        self.send_ctl(Msg::Bye);
        close(&self.shared);
    }
}

fn close(shared: &Shared) {
    shared.lock().closed = true;
    shared.cv.notify_all();
}

fn write_loop(shared: &Shared, mut w: Box<dyn Write + Send>, tx: &Sender<Event>) {
    loop {
        let msg = {
            let mut q = shared.lock();
            loop {
                if let Some(m) = q.items.pop_front() {
                    q.writing = true;
                    break Some(m);
                }
                if q.closed {
                    break None;
                }
                q = shared.cv.wait(q).unwrap_or_else(PoisonError::into_inner);
            }
        };
        let Some(msg) = msg else { return };
        let sent = encode_line(&msg)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
            .and_then(|line| w.write_all(line.as_bytes()))
            .and_then(|()| w.flush());
        shared.lock().writing = false;
        shared.cv.notify_all();
        if sent.is_err() {
            close(shared);
            let _ = tx.send(Event::Lost);
            return;
        }
    }
}

/// GatedRead liest erst, wenn der Schreib-Thread nichts mehr zu schreiben hat.
struct GatedRead {
    inner: Box<dyn Read + Send>,
    shared: Arc<Shared>,
}

impl Read for GatedRead {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let mut q = self.shared.lock();
        while (q.writing || !q.items.is_empty()) && !q.closed {
            q = self.shared.cv.wait(q).unwrap_or_else(PoisonError::into_inner);
        }
        drop(q);
        self.inner.read(out)
    }
}

fn read_loop(r: Box<dyn Read + Send>, shared: Arc<Shared>, tx: &Sender<Event>) {
    let mut r = BufReader::new(GatedRead { inner: r, shared });
    let mut buf = Vec::new();
    loop {
        buf.clear();
        // Höchstens MAX_LINE + Zeilenende: eine Zeile ohne Ende kann den Speicher nicht füllen.
        let got = r.by_ref().take(MAX_LINE as u64 + 2).read_until(b'\n', &mut buf);
        if !matches!(got, Ok(n) if n > 0) || buf.last() != Some(&b'\n') {
            break;
        }
        match decode_line(&String::from_utf8_lossy(&buf)) {
            Ok(m) => {
                if tx.send(Event::Msg(m)).is_err() {
                    return;
                }
            }
            Err(_) => break,
        }
    }
    let _ = tx.send(Event::Lost);
}
