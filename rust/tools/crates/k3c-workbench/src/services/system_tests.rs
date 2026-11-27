//! Port von `TestOwnerPIDNurErreichbareAdressenOhneSystem`, `TestGopsutilGegenEchtenProzess` (echter Attrappen-Prozess,
//! echte Windows-Abfragen) und `TestHealthCheck`.

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use std::process::Stdio;
use std::time::Duration;

use chrono::Local;
use k3c_kit::proc::Cmd;

use super::runtime::health_check;
use super::system::{Conn, new_sampler, owner_pid, port_listener, supervisor_of};
use super::testutil::{free_port, helper, wait_for};
use super::*;

#[test]
fn owner_pid_nur_erreichbare_adressen_ohne_system() {
    let conn = |ip: &str, pid| Conn {
        ip: ip.parse::<IpAddr>().unwrap(),
        port: 5173,
        pid,
    };
    let cases = [
        (vec![conn("::1", 100), conn("192.168.2.10", 200), conn("::", 300)], 300),
        (vec![conn("::", 300), conn("0.0.0.0", 400), conn("127.0.0.1", 500)], 500),
        (vec![conn("0.0.0.0", 4), conn("::1", 100)], 0),
        (
            vec![Conn {
                port: 5174,
                ..conn("127.0.0.1", 600)
            }],
            0,
        ),
    ];
    for (i, (conns, want)) in cases.iter().enumerate() {
        assert_eq!(owner_pid(conns, 5173), *want, "Fall {i}");
    }
}

#[cfg(windows)]
#[tokio::test(flavor = "multi_thread")]
async fn windows_abfragen_gegen_echten_prozess() {
    let port = free_port();
    let service = helper("Helfer", port);
    let mut cmd = Cmd::new(&service.command);
    cmd.command_mut()
        .envs(&service.env)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut proc = cmd.start().unwrap();
    let pid = proc.id();
    wait_for("Port", || port_listener(port).1).await;
    assert_eq!(port_listener(port), (pid, true));
    let sample = new_sampler();
    let m = sample(pid).unwrap();
    assert!(m.memory > 0 && m.cpu == 0.0, "{m:?}");
    assert!(m.started.is_some_and(|t| t <= Local::now()), "{m:?}");
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(sample(pid).unwrap().cpu >= 0.0);
    // Der Elternprozess ist dieses Test-Binary, also dieselbe Programmdatei.
    assert_eq!(supervisor_of(pid), std::process::id());
    proc.kill_handle().kill().unwrap();
    proc.wait().await.unwrap();
    assert_eq!(port_listener(port), (0, false));
    assert_eq!(port_listener(0), (0, false));
}

/// Antwortet auf jede Anfrage mit dem nächsten Code.
fn server(codes: Vec<u16>) -> u16 {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for code in codes {
            let (mut stream, _) = listener.accept().unwrap();
            // Erst die ganze Anfrage lesen und in einem Stück antworten: Der Prüfer legt nach der Statuszeile auf,
            // ein zweiter Schreibzugriff scheiterte dann (10053) und beendete den Server.
            let mut request = Vec::new();
            let mut buf = [0; 1024];
            while !request.ends_with(b"\r\n\r\n") {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0, "Anfrage abgebrochen");
                request.extend_from_slice(&buf[..n]);
            }
            let head = "Location: http://127.0.0.1:1/woanders\r\nContent-Length: 0\r\nConnection: close";
            let answer = format!("HTTP/1.1 {code} X\r\n{head}\r\n\r\n");
            let _ = stream.write_all(answer.as_bytes());
        }
    });
    port
}

#[test]
fn health_check_http_und_tcp() {
    let port = server(vec![200, 302, 500]);
    let http = Service {
        port: i64::from(port),
        health: "http".to_owned(),
        ..Service::default()
    };
    assert_eq!(health_check(&http), Ok(()));
    assert_eq!(health_check(&http), Ok(()));
    assert_eq!(health_check(&http), Err("antwort 500".to_owned()));
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let tcp = Service {
        port: i64::from(listener.local_addr().unwrap().port()),
        health: "tcp".to_owned(),
        ..Service::default()
    };
    assert_eq!(health_check(&tcp), Ok(()));
    drop(listener);
    assert!(health_check(&tcp).is_err());
}
