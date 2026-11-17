# RU17 · CLI · Effekte und Ton im Spiel

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`fx` (1080 Zeilen) und das ganze Paket `audio` (1311 Zeilen: Register, Atlas, Bank, Mischen, Ducking, Wiedergabe, Klänge je Zustand, `data/sounds.json`). Die Ton-Plattform (Gerät, Dekodieren, Stimmen, `NullAudio`) liefert RU4 als `k3c_platform::AudioOut`.

## Ziel

Effekte und Klänge folgen dem Sim-Zustand. Am Ende sichtbar: Nacht mit Gegnern mit Effekten und Ton.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 6 von 26, 6 Sessions (B-1015; nach der Überarbeitung von RU4 zwei Sessions mehr, weil RU4 nur die Plattform `k3c_platform::AudioOut` liefert und das ganze Go-Paket `audio` hier portiert wird).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/fx/` → `rust/crates/k3c-game/src/fx/`, `native/k3c-game/audio/` → `rust/crates/k3c-game/src/audio/`; Dateinamen nach Leitfaden § 2. Ebitengine-Audio und oto (`sink.go`, `output.go`, Dekodieren in `bank.go`) gehen auf `k3c_platform::AudioOut` (RU4.8).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/fx/`: effects (203, T), fx (267) | 470 |
| 2 | `native/k3c-game/fx/`: looks (117), paint (215), paintmarks (93), status (172, T) | 597 |
| 3 | `native/k3c-game/fx/`: texts (13); `native/k3c-game/audio/`: register (153), atlas (65, T), duck (52), sink (47) | 330 |
| 4 | `native/k3c-game/audio/`: bank (140), output (299, T) | 439 |
| 5 | `native/k3c-game/audio/`: mix (237, T), events (127, T), music (143, T), audio (48, T), dazu `klang_test.go` | 555 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2391 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/audio/**`
- **Abhängig von:** RU13
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Effekte erscheinen bei denselben Ereignissen wie im Go-Spiel.
- **AC-02** Klänge aus `data/sounds.json` spielen je Ereignis und Spielerhälfte; die Paket-Tests sind portiert.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI3.4 Vorboten, Taifun und Flut mit Ambient (B-999)
- SI6.4 Jubel, Tanz und Himmels-Effekt je Welt (B-1002)
- SI6.5 Festmusik je Welt (B-1002)
- WE8.4 Ankündigung und Schleier für Überfall, Sandsturm, Nebel (B-995)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
