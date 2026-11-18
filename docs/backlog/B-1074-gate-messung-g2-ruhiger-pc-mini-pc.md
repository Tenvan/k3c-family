# B-1074 · Gate-Messung G2 am ruhigen PC und Mini-PC samt Sichtprüfung (aus RU39.2)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU36
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU39.2 sah Messläufe am ruhigen Entwicklungs-PC und Mini-PC samt Sichtprüfung vor. Ralf hat am 2026-11-18 entschieden: Messungen für Optimierungen machen wir zum Schluss. Bis dahin liegt nur die vorläufige Messung mit eigenem Messcode vor (`docs/messung/rust-ruckler.md`, unruhiger Rechner).

## Ziel

Die Gate-Werte für G2 (Entwicklungs-PC ruhig, Mini-PC) liegen vor, Ralf hat beide Programme mit derselben Szene nebeneinander gesehen, G2 ist entschieden.

## Beteiligte und Zielgruppen

Ralf (🧑) misst und entscheidet G2; ein Agent überträgt die Werte in den Bericht.

## Anforderungen

- Werkzeug und Anleitung aus RU39.1: `docs/messung/rust-ruckler.md` › Messanleitung, `native/measure.ps1`.
- Ruhiger Rechner: keine anderen Claude-Sessions, keine Builds, Workbench ohne laufende Tasks.
- Absolut performanter und stabiler Code bleibt das Ziel.

## Nicht-Ziele

Codeänderungen. Messungen mit parallelen Sessions.

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11 gilt (kein sim_test, kein Balancing).

- **Bereich:** `docs/messung/rust-ruckler.md`
- **Abhängig von:** RU39
- **Braucht 🧑:** Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Fehlt der Mini-PC, gilt AC-02 mit „angenommen, Validierung offen“.

## Akzeptanzkriterien

- **AC-01** Je vier Läufe (Go, Rust, Go, Rust; `-Seconds 300 -Vsync -Frames`) auf dem ruhigen Entwicklungs-PC, Berichte unter `reports/`, Werte im Bericht `docs/messung/rust-ruckler.md`.
- **AC-02** Dieselben vier Läufe auf dem Mini-PC oder Vermerk „Gerät fehlt“.
- **AC-03** Sichtprüfung: Go-Spiel (`k3c-game.exe --seed k3c --players 2 --tick 15600 --input 0@0=-,1@0=-`) und Rust-Wiedergabe (`k3c-game.exe replay trace\messung-nacht.jsonl.gz --window`) je zwei Minuten am selben Bildschirm, wenn möglich am TV; Notizen: Ruckeln ja/nein, wann, wie oft, Unterschiede im Bild.

## Offene Fragen

keine

## Notizen

Schritte aus RU39.2: 1. `task gen:paper`, `task native:game:build`, `task rust:game:dist`; Aufzeichnung `.work/trace/messung-nacht.jsonl.gz` nach `dist/k3c-rust/trace/`. 2. Alle anderen Sessions und Builds beenden, vier Läufe abwechselnd. 3. Beide Ordner auf den Mini-PC, dieselben vier Läufe, Rechnername notieren. 4. Sichtprüfung wie AC-03. 5. Pfade und Notizen ins Ergebnis. Typ „Schuld“ gewählt, weil „Aufgabe“ kein gültiger Typ ist.
