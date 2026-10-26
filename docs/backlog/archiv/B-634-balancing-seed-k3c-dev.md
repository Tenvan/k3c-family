# B-634 · Balancing läuft bis zur Abnahme nur mit Seed k3c-dev und ist schneller

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BS1
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS1

## Ausgangslage

`data/balance-targets.json` › `seeds` listet 100 Seeds ("1" bis "100"); jeder Balancing-Lauf (`sim_test`, `tools/k3c-workbench/internal/balance`, Bot-Profile wie `coop2`) rechnet sie alle. Während der Neuaufstellung der Werte (Projekt BAS) dauert jede Werte-Runde dadurch zu lange.

## Ziel

Bis zur Abnahme der neuen Werte rechnet ein Balancing-Lauf standardmäßig nur den Seed `k3c-dev` und liefert in Minuten ein Ergebnis; danach wird wieder über alle Seeds gemessen.

## Beteiligte und Zielgruppen

Claude (Werte-Runden über `sim_test`), 🧑 entscheidet über die Abnahme der Werte und das Zurückschalten.

## Anforderungen

- `k3c-dev` steht als **erster** Eintrag in `balance-targets.json` › `seeds`.
- Ein Schalter in `balance-targets.json` (Vorschlag `seedMode`: `dev` | `all`) begrenzt Läufe auf den ersten Seed; Standard während BAS ist `dev`.
- **Sauber als MCP-Tool:** Jeder BAS-Lauf startet über `sim_test` (k3c-workbench), nie über Shell oder Binaries. `sim_test` (`tools/k3c-workbench/internal/mcpsrv/simtest.go`) kann heute offline weder Bot-Profil noch Seed-Liste noch Welt wählen (`bots` nur online, `seeds` nur als Anzahl). Ergänzen: `bots` auch offline (z. B. `coop2`), `seedList` (z. B. `k3c-dev`), `world` (Standard `standard`), Seed-Modus aus den Daten als Standard.
- Tool-Beschreibung von `sim_test` und `tools/k3c-workbench/internal/mcpsrv/instructions.md` nennen den BAS-Standardlauf (`sim_test start bots=coop2 world=standard`, Seed-Modus) und die Parameter.
- **Neustart deutlich melden:** Ändert eine Session Go-Code oder Instructions der Workbench, braucht die laufende Workbench (EXE oder `k3c-workbench:web`) einen Neustart, bevor das neue Tool wirkt. Die Session schreibt das im Ergebnis als erste Zeile „⚠️ Workbench-Neustart nötig“, meldet es per `notify_ui` (level `warn`) und im Chat an 🧑; kein Lauf mit dem neuen Parameter vor dem Neustart.
- `sim_test` und der Bericht nennen den Seed-Modus, damit kein Ein-Seed-Ergebnis als Gesamtergebnis gelesen wird.
- Laufzeit eines Laufs `coop2` standard vorher/nachher messen und im Ticket notieren; weitere Tempo-Hebel (z. B. Abbruch bei klarem Scheitern, Parallelität je Seed) nur, wenn sie ohne Determinismus-Verlust gehen.
- Deterministisch: gleicher Seed, gleiche Werte → gleiches Ergebnis.

## Nicht-Ziele

Neue Bot-Profile, neue Zielkorridore (B-635), Änderungen an der Simulation.

## Regeln und Einschränkungen

Nur `engine/rng`; Läufe ausschließlich über `sim_test` (B-348). Domäne DEV (Werkzeug) plus Daten-Datei `balance-targets.json`.

## Beispiele

`sim_test start` mit Profil `coop2`, Welt standard, ohne Seed-Angabe → rechnet nur `k3c-dev`, Bericht zeigt „Seed-Modus: dev (1 Seed)“.
`seedMode: all` → alle Seeds wie bisher, `k3c-dev` zuerst.

## Ausnahme- und Fehlerfälle

Unbekannter `seedMode` → `-check-data` lehnt ab. Ausdrücklich übergebene Seeds im Aufruf gehen vor dem Modus.

## Akzeptanzkriterien

- **AC-01** `balance-targets.json` › `seeds[0]` ist `k3c-dev`, ein Test prüft das.
- **AC-02** Mit `seedMode: dev` rechnet ein `sim_test`-Lauf genau einen Seed; der Bericht nennt den Modus.
- **AC-03** Laufzeit `coop2` standard vorher/nachher ist im Ticket notiert, nachher deutlich kürzer.
- **AC-04** Mit `seedMode: all` ist das Verhalten wie vorher (Golden/Tests grün).
- **AC-05** `sim_test start` nimmt offline `bots`, `seedList` und `world`; Tool-Beschreibung und `instructions.md` nennen den BAS-Standardlauf; ein Test in `mcpsrv` prüft die Parameter.
- **AC-06** Der nötige Workbench-Neustart ist im Session-Ergebnis, per `notify_ui` und im Chat deutlich gemeldet; der Messlauf (AC-03) läuft erst nach dem Neustart über das neue Tool.

## Offene Fragen

- Zurückschalten auf `all`: am Ende von BAS (Vorschlag) oder schon nach Meilenstein 1? Entscheidet 🧑, Default: Ende BAS.

## Notizen

Wunsch 🧑 2026-10-26: „Balancing bis zur Abnahme in der Performance erhöhen, u. a. durch einen Seed k3c-dev, der auch als erster in die Balancing-Seeds kommt.“

**Messung BS1.2 (2026-10-26, Rechner mit 22 Kernen, `sim_test {action: start, bots: coop2}`, Seed-Modus dev):**

| Lauf | Seeds | Läufe | Dauer | Anmerkung |
|---|---|---|---|---|
| vorher (run-1) | 1 (`k3c-dev`) | nacheinander | 14 min 23 s | zeitweise parallel zu `check:diff` |
| nachher (run-2) | 1 (`k3c-dev`) | parallel (`runAll`, je Kern ein Lauf) | 3 min 58 s inkl. Bau des Testers | Ziele und Kennzahlen identisch zu run-1 |
| alle Seeds, alt | 101 | nacheinander | nicht gemessen; hochgerechnet aus run-1 etwa 24 h | vor B-634 der Standard ohne `--seeds` |

Ein Lauf ohne Seed-Angabe ist damit gegenüber dem alten Standard (alle Seeds, nacheinander) um mehr als das Hundertfache kürzer.
Der Tester wird bei jedem `sim_test`-Lauf aus dem Checkout gebaut: Änderungen im Paket `balance` brauchen **keinen** Workbench-Neustart, nur Änderungen an `mcpsrv` und `simrun`.
