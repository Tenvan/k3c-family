//! Betriebssystem-Abfragen (Port von `system.go`; Go nutzt gopsutil, hier direkt die Windows-API): wer am Port lauscht,
//! CPU und Speicher eines Prozesses, sein Supervisor.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use chrono::{DateTime, Local};

use super::options::Sampler;

#[cfg(windows)]
#[allow(unsafe_code)]
mod windows;
#[cfg(windows)]
use self::windows as sys;

#[cfg(not(windows))]
mod other;
#[cfg(not(windows))]
use self::other as sys;

/// Messwerte eines Prozesses.
#[derive(Clone, Copy, Debug, Default)]
pub struct Metrics {
    /// Prozent eines Kerns seit der letzten Messung.
    pub cpu: f64,
    /// RSS in Bytes.
    pub memory: u64,
    /// Startzeit des Prozesses.
    pub started: Option<DateTime<Local>>,
}

/// Rohwerte eines Prozesses.
pub(crate) struct Usage {
    /// Rechenzeit in Benutzer- und Systemmodus.
    pub cpu: Duration,
    pub memory: u64,
    pub started: Option<DateTime<Local>>,
}

/// Ein lauschender Eintrag der Verbindungstabelle.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Conn {
    pub ip: IpAddr,
    pub port: u16,
    pub pid: u32,
}

/// Höchste PID, die Windows selbst gehört (0 Leerlauf, 4 System, z. B. http.sys); sie wird nie als Besitzer genommen,
/// damit ein Stopp sie nicht treffen kann.
pub(crate) const SYSTEM_PID: u32 = 4;

/// Prüft per Verbindung, ob jemand am Port lauscht, und sucht die PID in der Verbindungstabelle.
pub fn port_listener(port: u16) -> (u32, bool) {
    if port == 0 {
        return (0, false);
    }
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    if TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_err() {
        return (0, false);
    }
    (owner_pid(&sys::listeners(), port), true)
}

/// Der Prozess, den eine Verbindung an `127.0.0.1:port` erreicht: wer dort, an `0.0.0.0` oder (Dual-Stack) an `::`
/// lauscht. Andere Adressen zählen nicht, sonst träfe ein Stopp ein fremdes Programm mit demselben Port.
pub(crate) fn owner_pid(conns: &[Conn], port: u16) -> u32 {
    let rank = |ip: IpAddr| match ip {
        ip if ip == IpAddr::V4(Ipv4Addr::LOCALHOST) => 3,
        ip if ip == IpAddr::V4(Ipv4Addr::UNSPECIFIED) => 2,
        ip if ip == IpAddr::V6(Ipv6Addr::UNSPECIFIED) => 1,
        _ => 0,
    };
    let mut best = (0, 0);
    for c in conns.iter().filter(|c| c.port == port && c.pid > SYSTEM_PID) {
        if rank(c.ip) > best.1 {
            best = (c.pid, rank(c.ip));
        }
    }
    best.0
}

/// So viele Prozesse merkt sich der Sampler; jeder Neustart bringt eine neue PID.
const MAX_CACHED: usize = 64;

/// Sampler, der je PID die letzte Messung merkt; die erste Messung liefert 0 % CPU.
pub fn new_sampler() -> Sampler {
    let last: Mutex<HashMap<u32, (Duration, Instant)>> = Mutex::new(HashMap::new());
    Arc::new(move |pid| {
        let usage = sys::usage(pid)?;
        let now = Instant::now();
        let mut last = last.lock().unwrap_or_else(PoisonError::into_inner);
        if last.len() >= MAX_CACHED && !last.contains_key(&pid) {
            last.clear();
        }
        let cpu = last.insert(pid, (usage.cpu, now)).map_or(0.0, |(before, at)| {
            let wall = now.duration_since(at).as_secs_f64();
            if wall > 0.0 {
                usage.cpu.saturating_sub(before).as_secs_f64() / wall * 100.0
            } else {
                0.0
            }
        });
        Ok(Metrics {
            cpu,
            memory: usage.memory,
            started: usage.started,
        })
    })
}

/// PID des Elternprozesses, wenn er dieselbe Programmdatei ist (ein Dienst mit Supervisor startet sich als Supervisor
/// und Kind aus derselben EXE), sonst 0. Ein fremder Elternprozess (Shell, task) zählt nicht, ein Stopp träfe ihn nie.
pub fn supervisor_of(pid: u32) -> u32 {
    let Some(parent) = sys::parent(pid).filter(|p| *p > SYSTEM_PID) else {
        return 0;
    };
    match (sys::image(pid), sys::image(parent)) {
        (Some(child), Some(other)) if !child.is_empty() && child.to_lowercase() == other.to_lowercase() => parent,
        _ => 0,
    }
}
