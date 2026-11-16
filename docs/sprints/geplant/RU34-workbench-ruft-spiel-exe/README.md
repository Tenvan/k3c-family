# RU34 · DEV · Workbench: MCP-Tools für Sim, Level, Welten und Hub rufen die Spiel-EXE

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die MCP-Tools `sim_run`, `sim_test`, `replay_run`, `level_generate` und das Balancing rechnen die Go-Engine im Prozess der Workbench (WE1, B-924). „Im Spiel ansehen“ startet schon die Spiel-EXE.

## Ziel

Die MCP-Tools für Sim, Level, Welten und Hub starten die Spiel-EXE headless oder mit Fenster. Am Ende sichtbar: `sim_run` mit `head: true` öffnet das Spiel und zeigt den Lauf.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1017 › Anforderungen` (Schnittstelle, headless und mit Fenster, MCP-Tools).

Welle 21 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Starter für die Spiel-EXE: Kindprozess im Job-Objekt (Muster `engine/devrun`), JSON-Zeilen lesen (AC-01)
2. `sim_run`, `replay_run`, `level_generate` über die EXE, Parameter `head` (AC-02)
3. Welten-Prüfung und Hub-Planer über die EXE (AC-03)
4. Review

### Vorentscheidungen

| MCP-Tool | Aufruf |
|---|---|
| `sim_run`, `replay_run` | `k3c-game sim run --input -` (JSON auf stdin) |
| `level_generate` | `k3c-game sim level --input -` |
| `sim_test start`, Balancing (erst RU40) | `k3c-game sim balance --input - --report-dir reports/simtest-<id>/` |
| Welten-Editor | `k3c-game sim world --input -` |
| Gebäude-Planer | `k3c-game sim hub --input -` |

- `head: true` hängt `--window` an; ohne Angabe läuft der Aufruf headless.
- Die EXE kommt aus dem gewählten Checkout (Suchliste wie in `tools/k3c-workbench/internal/studioview/game.go`). Fehlt sie oder ist sie älter als der letzte Commit an `rust/`, nennt das Tool den Bau-Befehl und rechnet nicht.
- Damit entfällt der Grund für die eigene Workbench je Sim-Worktree (B-925): Die Workbench rechnet nichts mehr im eigenen Prozess. Die Regel selbst ändert 🧑; die Session legt dafür ein Ticket an.

### Hinweise für die Umsetzung

- Der Starter bekommt den Pfad der EXE aus derselben Liste wie „Im Spiel ansehen“ (`tools/k3c-workbench/internal/studioview/game.go`).
- Kindprozess im Job-Objekt wie `engine/devrun`; `stop` beendet per CTRL_BREAK, nach 5 s hart.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Stand vor diesem Sprint bleibt über Git erreichbar (B-1015 › Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `tools/k3c-workbench/**`, `cmd/k3c-sim/**`, `engine/simcli/**`, `engine/enginetools/**`
- **Abhängig von:** RU33
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Workbench startet die Spiel-EXE des gewählten Checkouts als Kindprozess; beim Ende der Workbench bleibt kein Prozess übrig.
- **AC-02** `sim_run`, `replay_run` und `level_generate` liefern Berichte im bisherigen Format; `head` wählt den Modus (Standard: headless).
- **AC-03** `world_check`, `hub_check` und die Vorschau des Welten-Editors rechnen über die EXE; Balancing und `sim_test` bleiben bis RU40 bei der Go-Engine.

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
