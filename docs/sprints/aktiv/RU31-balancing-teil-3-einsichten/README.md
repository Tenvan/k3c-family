# RU31 · SIM · Rust-Balance-Crate Teil 3: Einsichten, Berichte, Tuning

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU28 fehlen die Einsichten (`insights*.go`, 1721 Zeilen), der HTML-Bericht und das Tuning.

## Ziel

`k3c-balance` ist vollständig. Am Ende sichtbar: alle Tests aus `engine/balance` portiert und grün (ohne Balancing-Lauf).

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 24 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `sonnet`. Kette auf `sprint/ru31`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/balance/` → `rust/crates/k3c-balance/src/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Tests ohne eigene Quelle | Zeilen |
|---|---|---|---:|
| 1 | `engine/balance/`: insights (245, T), insights_figures (174), insights_files (88) | – | 507 |
| 2 | `engine/balance/`: insights_levers (95), insights_markdown (148), insights_next (211) | – | 454 |
| 3 | `engine/balance/`: insights_plain (298, T), insights_progress (210) | – | 508 |
| 4 | `engine/balance/`: insights_text (252), report_html (126); Vorlagen `report/*` als Kopie | fullgame_test, milestone_test (lang) | 378 |
| 5 | `engine/balance/`: tune (347, T), tuneparam (209) | – | 556 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – | – |

Summe: 2403 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Vorentscheidungen

- Feste Eingabe für AC-01 ist `testdata/rust/balance/insights-run7.json` (Kopie aus RU25.3); `engine/balance/rustref_test.go` schreibt dazu `Insights`, Markdown und HTML als Vergleichsdateien. Zeitzeilen und PNG-Bytes sind ausgenommen (Pixel gleich).
- Vorlagen des HTML-Berichts als Kopie unter `rust/crates/k3c-balance/report/` (`include_str!`), PNG über die Workspace-Abhängigkeit `png`, Base64 als eigene Datei.
- Zeit in UTC (std hat keine Zeitzone).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: neue Fälle in `engine/balance/rustref_test.go`.

- **Bereich:** `rust/crates/k3c-balance/**`, `engine/balance/rustref_test.go`, `testdata/rust/balance/**`
- **Abhängig von:** RU28
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Einsichten und Berichte ergeben für eine feste Eingabe (Bericht-JSON aus `testdata/balance`) denselben Text wie Go.
- **AC-02** Das Tuning wählt für feste Eingaben dieselben Parameter wie Go; ausgeführt wird kein Balancing-Lauf, solange die Sperre gilt.
- **AC-03** Kein Test aus `engine/balance` fehlt ohne Begründung.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Bericht-JSON für AC-01:** `testdata/rust/balance/insights-run7.json` (unter `testdata/balance/` liegt nur `baseline.json`).
- **HTML-Bericht:** Vorlagen kopiert, `png` aus dem Workspace (keine neue Abhängigkeit), eigenes Base64; PNG-Bytes dürfen abweichen, Pixel nicht.
- **Lange Tests** (`fullgame`, `milestone`) portiert, aber `ignore = "lang"`; sie laufen erst nach Widerruf der Sperre im Abnahme-Ticket.
- **Zeit** in UTC statt Ortszeit.
- **Abweichung vom Schnitt:** `fullgame_test.go`, `milestone_test.go` (ohne eigene Quelle) in Session 4.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU31.1 | `RU31.1-balance-insights-figures-files.md` | Umsetzung | autonom | fertig |
| RU31.2 | `RU31.2-balance-insights-levers-markdown-next.md` | Umsetzung | autonom | fertig |
| RU31.3 | `RU31.3-balance-insights-plain-progress.md` | Umsetzung | autonom | fertig |
| RU31.4 | `RU31.4-balance-insights-text-report-html.md` | Umsetzung | autonom | fertig |
| RU31.5 | `RU31.5-balance-tune-tuneparam.md` | Umsetzung | autonom | fertig |
| RU31.6 | `RU31.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
