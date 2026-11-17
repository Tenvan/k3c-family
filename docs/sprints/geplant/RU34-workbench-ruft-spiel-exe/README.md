# RU34 · DEV · Workbench: MCP-Tools für Sim, Level, Welten und Hub rufen die Spiel-EXE

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Die MCP-Tools `sim_run`, `sim_test`, `replay_run`, `level_generate` und das Balancing rechnen die Go-Engine im Prozess der Workbench (WE1, B-924). „Im Spiel ansehen“ startet schon die Spiel-EXE.

## Ziel

Die MCP-Tools für Sim, Level, Welten und Hub starten die Spiel-EXE headless oder mit Fenster. Am Ende sichtbar: `sim_run` mit `head: true` öffnet das Spiel und zeigt den Lauf.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

Welle 21 von 26, 4 Sessions (B-1015). Modell: `opus` für RU34.1 (Starter, Gegenstück der Schnittstelle), sonst `sonnet`. Kette auf `sprint/ru34`. Schnittstelle: `docs/standards/k3c-game-laeufe.md` (RU33).

### Sessions

| Session | Inhalt | Go-Dateien der Workbench |
|---|---|---|
| 1 | Starter: EXE suchen und auf Frische prüfen, Kindprozess im Job-Objekt, JSON-Zeilen, Abbruch (AC-01) | neu `internal/gameexe/` (find, event, run); `internal/studioview/game.go` (EXE-Suche) |
| 2 | `sim_run`, `level_generate` über die EXE, Parameter `head` (AC-02) | `internal/mcpsrv/tools_engine.go`, `tools.go` (`registerEngine`), `checkout.go` (`takesCheckout`) |
| 3 | Welten-Prüfung und Hub-Planer über die EXE (AC-03) | `internal/world/level.go`, `internal/hubplan/hubplan.go`, `app_world.go`, `app_hub.go`, `internal/mcpsrv/tools_world.go`, `tools_hub.go` |
| 4 | Review | – |

### Vorentscheidungen

| MCP-Tool | Aufruf | Sprint |
|---|---|---|
| `sim_run` | `k3c-game sim run --input <Datei> --data <Checkout>/data --stop-stdin [--window [--speed N]]` | RU34.2 |
| `level_generate` | `k3c-game sim level --input <Datei> --data … --stop-stdin` | RU34.2 |
| `world_check`, Welten-Editor | `k3c-game sim world --input <Datei> --data … --stop-stdin` | RU34.3 |
| `hub_check`, Gebäude-Planer | `k3c-game sim hub --input <Datei> --data … --stop-stdin` | RU34.3 |
| `replay_run` | `k3c-game sim balance --play <Datei> …` | RU40.3 |
| `sim_test start`, Balancing | `k3c-game sim balance --input <Datei> --report-dir reports/simtest-<id>/ …` | RU40.3 |

- `head: true` hängt `--window` an (nur `sim_run`, später `sim_test` und `replay_run`); ohne Angabe läuft der Aufruf headless. `speed` (1–64) nur mit `head`.
- Die EXE kommt aus dem Checkout der Session: Suchliste `rust/target/release/k3c-game.exe`, dann `dist/k3c-game/k3c-game.exe` (`internal/gameexe`, auch für „Im Spiel ansehen“). Veraltet ist sie, wenn eine Datei unter `rust/crates/**` (`*.rs`, `Cargo.toml`) oder `rust/Cargo.lock` jünger ist; dann nennt das Tool den Bau-Befehl und rechnet nicht.
- Die Eingabe geht als Datei unter `<Checkout>/.work/runs/`, stdin bleibt offen: Schließen heißt Abbruch (Zeile `aborted`, Exit 130), nach 5 s beendet das Job-Objekt den Baum.
- `sim_run` und `level_generate` nehmen `checkout` und rechnen mit der EXE dieses Checkouts; die Ablehnung fremder Checkouts (`ownCheckoutErr`) entfällt für sie.
- Damit entfällt der Grund für die eigene Workbench je Sim-Worktree (B-925) für Level, Welten und Hub; Balancing folgt in RU40. Die Regel selbst ändert 🧑; RU34.3 legt dafür ein Ticket an (Sprint RU40).

### Hinweise für die Umsetzung

- Kindprozess über `internal/proc` (Job-Objekt, Abräumen beim Ende der Workbench); kein CTRL_BREAK an die EXE, Abbruch über stdin.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Der Go-Code der Engine bleibt unverändert lauffähig (Rückfall); `engine/simcli` und `cmd/k3c-sim` bleiben bis RU40.4. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Go-Module nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt: `sim_run` bleibt gesperrt (`simLockErr`), Tests laufen mit Fake-EXE oder `sim level`.

- **Bereich:** `tools/k3c-workbench/**`
- **Abhängig von:** RU33
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Workbench startet die Spiel-EXE des gewählten Checkouts als Kindprozess; beim Ende der Workbench bleibt kein Prozess übrig.
- **AC-02** `sim_run` und `level_generate` liefern Antworten im bisherigen Format; `head` wählt bei `sim_run` den Modus (Standard: headless). `replay_run` folgt mit RU40.3.
- **AC-03** `world_check`, `hub_check` und die Vorschau des Welten-Editors rechnen über die EXE; Balancing, `sim_test` und `replay_run` bleiben bis RU40 bei der Go-Engine.

## Offene Fragen

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **`replay_run` erst in RU40.3:** Es spielt eine Balancing-Aufnahme (`balance --play`, `engine/balance/replay.go`); die gibt es in Rust erst mit RU25.5 und `sim balance` (RU40.1). AC-02 entsprechend angepasst.
- **EXE-Pfad und Frische:** Suchliste und mtime-Vergleich wie in den Vorentscheidungen; kein git-Aufruf.
- **Abbruch über stdin statt CTRL_BREAK:** Die Workbench hat keine Konsole, CTRL_BREAK erreicht ein Kind ohne Konsole nicht zuverlässig; die EXE hat ohne neue Abhängigkeit keinen Signal-Handler.
- **Bereich verkleinert** auf `tools/k3c-workbench/**`: `cmd/k3c-sim`, `engine/simcli`, `engine/enginetools` ändert erst RU40.4.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU34.1 | `RU34.1-starter-spiel-exe.md` | Umsetzung | autonom | offen |
| RU34.2 | `RU34.2-sim-run-level-ueber-exe.md` | Umsetzung | autonom | offen |
| RU34.3 | `RU34.3-welten-hub-ueber-exe.md` | Umsetzung | autonom | offen |
| RU34.4 | `RU34.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
