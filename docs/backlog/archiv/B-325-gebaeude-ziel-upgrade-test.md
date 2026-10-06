# B-325 · K1.2 darf den Testaufbau des Elite-Bogenschützen anpassen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K1
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K1.2 setzt `gegner.md` § 4 um: Übrige gebaute Gebäude sind Ziele der Gegner, Gegner ohne `prefersBuildings` greifen sie an, wenn nichts anderes in Reichweite ist (`engine/sim/enemies.go`, `isBuilding`, `chooseTarget`). Dadurch scheitert `TestUpgradeEliteBogenschuetze` (`engine/sim/upgrades_test.go`): Das Skelett startet bei `HubX+40`, bleibt an der gebauten Schmiede (x 578) stehen und schlägt auf sie ein; der Elite-Bogenschütze (x 560, Reichweite 12) erreicht es dort nicht und schießt in 20 s nicht. Die Regel wirkt wie verlangt, der Testaufbau rechnet noch mit dem alten Verhalten. `upgrades_test.go` steht nicht in den Erlaubten Dateien von K1.2.

## Ziel

K1.2 kann abgeschlossen werden, ohne die Regel aus § 4 aufzuweichen.

## Beteiligte und Zielgruppen

Entwicklung (SIM); 🧑 entscheidet.

## Anforderungen

- Entscheidung, ob K1.2 `engine/sim/upgrades_test.go` (nur den Aufbau von `TestUpgradeEliteBogenschuetze`) ändern darf.

## Nicht-Ziele

Änderung der Regel § 4 oder der Elite-Werte.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › „Wenn etwas nicht passt“; Erlaubte Dateien von K1.2.

## Beispiele

Vorschlag: Das Skelett im Test auf der anderen Seite der Burg starten (`HubX-40`, dort steht kein gebautes Gebäude) oder in Reichweite des Bogenschützen (`HubX+20`). Ein Einzeiler, der Test prüft weiter nur den Schaden des Elite-Geschosses.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Freigabe einer Datei.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden; bei Ja ist `upgrades_test.go` in den Erlaubten Dateien von K1.2 und `task check:go` grün.

## Offene Fragen

Darf K1.2 `engine/sim/upgrades_test.go` anpassen (Startpunkt des Skeletts)? 🧑

**Entscheidung 🧑 2026-10-06 (Chat):** „B-325: Test anpassen“. Umgesetzt in K1.2: Skelett startet bei `HubX+20` statt `HubX+40`, Prüfungen unverändert; `task check:go` grün.

## Notizen

Golden-Läufe bleiben mit der Änderung unverändert grün (`go test ./engine/sim`, nur dieser eine Test rot).
