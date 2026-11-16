# RU40 · CLI · Balancing-Läufe im Game-Client, sim_test und Balancing der Workbench über die Spiel-EXE

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

Nach RU34 laufen Sim, Level, Welten und Hub über die Spiel-EXE; Balancing rechnet die Workbench noch mit der Go-Engine im eigenen Prozess.

## Ziel

Auch Balancing läuft mit der Spiel-EXE, headless oder mit Fenster; die Workbench linkt keine Engine mehr. Am Ende sichtbar: `sim_test start` mit `head: true` zeigt die Bots im Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1017 › Anforderungen` (Schnittstelle, headless und mit Fenster, MCP-Tools).

Welle 25 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. `sim balance` headless: `engine/simcli/balance.go`, `targets.go`, `tune.go`, `sensitivity.go` portieren (CLI, AC-01)
2. `sim balance --window`: Bots sichtbar, Läufe nacheinander, Vergleich mit headless (CLI, AC-02)
3. `sim_test` und Balancing der Workbench über die EXE, Parameter `head`; Engine-Importe, `engine/simcli` und `cmd/k3c-sim` entfernen (DEV, AC-03, AC-04)
4. Review

### Hinweise für die Umsetzung

- Die Sim-Sperre gilt bis zum Widerruf durch 🧑: Diese Sessions testen nur mit kurzen, festen Eingaben aus den Go-Tests von `engine/simcli`.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Stand vor diesem Sprint bleibt über Git erreichbar (B-1015 › Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/runs/**`, `tools/k3c-workbench/**`, `cmd/k3c-sim/**`, `engine/simcli/**`
- **Abhängig von:** RU31, RU34
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `k3c-game sim balance` meldet Fortschritt zeilenweise, schreibt die Berichtsdateien wie `bin/k3c-sim.exe balance` und endet bei CTRL_BREAK mit `{"event":"aborted"}` und Code 130.
- **AC-02** Mit `--window` laufen dieselben Läufe sichtbar nacheinander und liefern bei gleicher Eingabe dieselben Berichte wie headless.
- **AC-03** `sim_test` und Balancing der Workbench starten die Spiel-EXE; `head` wählt den Modus (Standard: headless); `stop` beendet den Lauf, es bleibt kein Prozess übrig.
- **AC-04** `tools/k3c-workbench` importiert kein `k3c/engine/*` mehr außer `conlog` und `devrun`; `cmd/k3c-sim` und `engine/simcli` sind entfernt.

## Offene Fragen

keine

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
