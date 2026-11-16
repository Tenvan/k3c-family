# RU33 · CLI · Läufe im Game-Client: Sim, Level, Welten und Hub, headless oder mit Fenster per Parameter

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-1017
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach G4 ist das Rust-Spiel das Spiel; die Workbench rechnet Sim, Level, Welten-Prüfung und Hub-Planer aber noch mit der Go-Engine im eigenen Prozess (`engine/simcli`, B-924). Beschluss 🧑 2026-11-16: Die Läufe gehören in den Game-Client.

## Ziel

Die Spiel-EXE führt Sim-, Level-, Welten- und Hub-Läufe aus, per Parameter headless oder mit Fenster. Am Ende sichtbar: `k3c-game sim run --headless --input lauf.json` schreibt JSON-Zeilen; derselbe Aufruf mit `--window` zeigt den Lauf im Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1017 › Anforderungen` (Schnittstelle, headless und mit Fenster, MCP-Tools).

Welle 20 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Schnittstelle festschreiben (`docs/standards/k3c-game-laeufe.md`), `engine/simcli/simcli.go`, `call.go`, `input.go` portieren: `sim run`, `sim level` headless (AC-01, AC-02)
2. `engine/simcli/world.go`, `hub.go` portieren: `sim world`, `sim hub` headless (AC-02)
3. `sim run --window`: Lauf sichtbar in der Spielschleife, Zeitraffer, Vergleich mit headless (AC-03)
4. Review

### Vorentscheidungen

- Die Schnittstelle ist die von `cmd/k3c-sim/main.go` (Kopfkommentar), nur mit dem Präfix `sim` und den Schaltern `--headless` und `--window`. JSON-Zeilen und Exit-Codes bleiben gleich, damit die Workbench nur die EXE und das Präfix tauscht.
- `sim …` läuft über `platform::run_headless`; `--window` nutzt `platform::run_window` mit derselben `App`. Die `App` der Läufe steuert das Spiel über dieselben Aktionen wie ein Spieler (Bots aus `k3c-balance`).
- Mit `--window` gilt Zeitraffer 1 als Standard; `--speed N` rechnet N Schritte je Bild. Das Ergebnis hängt nicht von `--speed` ab.
- `balance` startet mehrere Läufe: headless parallel wie in Go (`engine/balance/parallel.go`), mit Fenster nacheinander.

### Hinweise für die Umsetzung

- Die Schnittstelle ist der Kopfkommentar von `cmd/k3c-sim/main.go`; sie wird übernommen, nicht neu erfunden. Neu sind nur das Präfix `sim` und die Schalter `--headless` und `--window`.
- Headless ist der Standard für `sim …`; ohne Unterbefehl startet die EXE wie bisher das Spiel mit Fenster.
- Sim-Sperre: nur kurze Testläufe mit festen Eingaben aus den Go-Tests von `engine/simcli`.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/runs/**`, `docs/standards/k3c-game-laeufe.md`
- **Abhängig von:** RU36
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`k3c-game sim run --headless --input lauf.json` → `{"event":"result",…}` und Code 0 ohne Fenster. Derselbe Aufruf mit `--window` → der Lauf ist im Spiel sichtbar, der Bericht ist gleich.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `docs/standards/k3c-game-laeufe.md` beschreibt die Schnittstelle: Unterbefehle `sim run|level|balance|world|hub`, `--input`, `--headless` (Standard), `--window`, JSON-Zeilen auf stdout und Exit-Codes 0, 1, 2, 130 wie im Kopfkommentar von `cmd/k3c-sim/main.go`.
- **AC-02** `sim run`, `level`, `world` und `hub` laufen headless ohne Grafikkontext und geben für dieselbe Eingabe dieselben JSON-Zeilen aus wie `bin/k3c-sim.exe` (Vergleichstest mit festen Eingaben).
- **AC-03** Mit `--window` läuft `sim run` sichtbar im Spiel (Zeitraffer wählbar) und liefert bei gleicher Eingabe dasselbe Ergebnis wie headless.

## Offene Fragen

Die Sim-Sperre gilt weiter: Diese Session baut und testet die Läufe nur mit kurzen Testläufen; echte Sim- und Balancing-Läufe erst nach Widerruf der Sperre (🧑).

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
