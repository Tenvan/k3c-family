# RU36 · CLI, SIM · Abnahme am PC: Rust-Spiel gegen Go-Spiel (Gate G4)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Beide Spiele sind baubar; das Go-Spiel ist die Referenz.

## Ziel

🧑 nimmt das Rust-Spiel am PC mit zwei Pads ab. Am Ende sichtbar: Abnahme-Bericht mit Liste, Messwerten und Entscheidung G4.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 19 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`, G4-Bericht `opus`, Session 2 ist Mensch.

### Sessions

1. Abnahme-Liste aus den Stationen des Go-Spiels, Schwere-Skala, Messwerkzeug (`measure.ps1 -Engine rustgame`, Startzeit) und Probelauf (AC-01, AC-02 Teil Werkzeug)
2. Abnahme am PC mit zwei Pads und Messläufe auf ruhigem Rechner (Mensch, AC-02, AC-03)
3. Befunde beheben, Runde 1: schwere Befunde mit Test, übrige als Ticket (AC-04)
4. Befunde beheben, Runde 2: Nachprüfung durch 🧑, Rest der schweren Befunde (AC-04)
5. G4-Bericht, Entscheidung 🧑, Review

### Vorentscheidungen

- Schwere: `schwer` = Absturz/Hänger, Datenverlust beim Speichern/Laden, Station nicht erreichbar, Pad-Eingabe falsch, sichtbares Ruckeln, Regel anders als Go; `mittel`/`leicht` werden Tickets.
- Schwellen aus G1 (B-1016): längstes Bild ≤ 2 × Median, GPU-Speicher ≤ 1330 MiB, EXE ≤ 40 MB; dazu FPS, Startzeit und Arbeitsspeicher nicht schlechter als Go. Die 1-ms-Schwelle des Spike-Modus `level` entfällt.
- Startzeit = Prozessstart bis sichtbares Fenster (`MainWindowHandle`), gemessen von `measure.ps1` für beide Spiele.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Fixes bilden das Go-Verhalten nach. G4 entscheidet 🧑 (RU36.5 hält an); ein schwerer Befund wird nur mit Zustimmung von 🧑 verschoben.

- **Bereich:** `docs/messung/rust-abnahme.md`, `rust/crates/**`, `native/measure.ps1`
- **Abhängig von:** RU32, RU35, RU53
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Abnahme-Liste nennt jede Station des Go-Spiels (Menüs, Spiel, Nacht, Bau, Skills, Boss, Seekarte, Speichern, Laden).
- **AC-02** Der Messlauf zeigt FPS, Grafikspeicher und Startzeit beider Spiele am selben PC; Rust ist in keinem Wert schlechter als die Schwellen aus G1.
- **AC-03** 🧑 hat jede Station mit zwei Pads abgenommen oder als Befund notiert.
- **AC-04** Schwere Befunde sind behoben, die übrigen sind Tickets.

## Offene Fragen

Entscheidung 🧑 am Gate G4 (RU36.5 hält an).

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Die Gate-Messwerte erhebt 🧑 in Session 2 auf ruhigem Rechner zusammen mit der Abnahme; Session 1 baut das Werkzeug und macht einen Probelauf (parallele Sessions verfälschen Messungen, `docs/messung/rust-client-1.md`).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Bereich um `native/measure.ps1` erweitert (Engine `rustgame`, Startzeit für beide Spiele).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Runde 2 beginnt mit einer kurzen Nachprüfung der Fixes aus Runde 1 durch 🧑 (Haltepunkt in RU36.4); gibt es nichts mehr zu tun, endet sie ohne Codeänderung.
- **Ralf 2026-11-19 (Go-Abbau): Das Go-Spiel wird vor G4 entfernt (RU58, RU37, RU59; alle drei Schritte sofort).** Der Vergleich Rust gegen Go in dieser Abnahme läuft gegen die gesicherte Referenz (Messwerte `docs/messung/go-referenz.json`, Bilder `testdata/rust/e2e/go/`, Farben und Pause `testdata/rust/go-referenz/`; RU58.2, RU58.3) oder gegen eine Go-EXE, die sich 🧑 aus dem Tag `go-referenz-final` baut (`docs/standards/go-referenz.md`, RU58.1). Das Messskript zieht in RU37.1 von `native/measure.ps1` nach `tools/measure/measure.ps1`; sein Zweig `-Engine game` gibt ohne Go-EXE die Referenzwerte aus. RU36 hängt nicht mehr an RU37; RU37 hängt nicht mehr an RU36.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU36.1 | `RU36.1-abnahme-liste-und-messwerkzeug.md` | Umsetzung | autonom | fertig |
| RU36.2 | `RU36.2-abnahme-am-pc-zwei-pads.md` | Workshop | Mensch | fertig |
| RU36.3 | `RU36.3-befunde-runde-1.md` | Umsetzung | autonom | fertig |
| RU36.4 | `RU36.4-befunde-runde-2.md` | Umsetzung | autonom | offen |
| RU36.5 | `RU36.5-bericht-g4-und-review.md` | Review | autonom | offen |
| RU36.6 | `RU36.6-weltwahl-alle-welten.md` | Umsetzung | autonom | offen |
| RU36.7 | `RU36.7-wald-dicht-standard-seed.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
