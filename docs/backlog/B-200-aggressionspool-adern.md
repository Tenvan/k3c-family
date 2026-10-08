# B-200 · Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** LV1
- **Projekt:** KMP
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`gatherPressure` (`engine/sim/units.go`, Zeile 186) erhöht den Aggressionspool je vollständig abgegebener Lieferung um `cycle.percentPerGather` (1 % in `data/biomes/cave.json` und `mine.json`); Aufrufer sind `units.go` (Zeile 181) und `carryToStock` (`engine/sim/island_storage.go`, Zeile 70). Bisher begrenzt der endliche Startvorrat die Lieferungen. Mit den unendlichen Adern aus W2 (B-114) gilt das nicht mehr: Bei 10 Einheiten je Lieferung (wie `data/economy.json` › `gatherables`, Menge der Adern legt W2.1 fest) liefern 2 Stein-Adern mit je 2 Bauern 120 Stein/min = 12 Lieferungen/min, also +12 %/min zusätzlich zum Grundwert +1 %/min. Der Pool ist damit ohne Kills nach etwa 7,7 min voll, mit Kills (+5 % je Kill) deutlich früher. Rechnung, nicht gemessen.

## Ziel

Der Abstand der Wellen unter Tage liegt auch mit besetzten Adern im Korridor 4–12 min (`docs/rules/zielkorridore.md`, Zeile „Abstand der Wellen unter Tage“; `docs/rules/stufen.md` § 2). Abbauen bleibt ein Risiko, wird aber nicht zur Wellen-Pumpe.

## Beteiligte und Zielgruppen

Spieler unter Tage; REG und Balancing-Tester (B-099) liefern die Messung; 🧑 bestätigt die Werte.

## Anforderungen

- Der Druck durch Abbau ist in `data/` einstellbar, auch getrennt für Adern und endliche Objekte, falls die Messung es verlangt.
- Messung mit dem Szenario des Korridors (Mine, Normal, 2 Spieler, Ressourcen gesammelt) und zusätzlich 2 besetzten Adern.
- Deterministisch, gilt mit 2+ Spielern.

## Nicht-Ziele

Adern selbst (B-114, W2.1), Wellengröße und Gegnerwerte (Regelwerk III, K1), der Balancing-Tester als Werkzeug (B-099, BAL1).

## Regeln und Einschränkungen

Werte nur in `data/`; `docs/rules/stufen.md` § 2 (+1 %/min, +5 % je Kill, +1 % je gesammelter Ressource) ist eine Regel: eine Änderung der Bedeutung braucht einen Beschluss von 🧑 in `docs/rules/`. Golden nach `docs/arbeitsweise.md` › „Golden aktualisieren“.

## Beispiele

Mine, 2 Spieler, beide Kupfer-Adern mit je 2 Bauern besetzt, keine Kills → erste Welle frühestens nach 4 min, Median der Abstände ≤ 12 min.

## Ausnahme- und Fehlerfälle

Lager voll → keine Lieferung, kein Druck (wie heute). Höhle ohne Bauern an den Adern → nur Grundwert und Kills.

## Akzeptanzkriterien

- **AC-01** Messung (100 Seeds, Szenario wie oben) als Tabelle in diesem Ticket › Notizen: Wellenabstand Median und p10 mit und ohne besetzte Adern.
- **AC-02** Go-Test mit festen Seeds: Mit 2 besetzten Adern und ohne Kills ist der Pool frühestens nach 4 min Spielzeit voll (Werte aus den Daten).
- **AC-03** `task check:go` grün, geänderte Werte stehen nur in `data/`.

## Offene Fragen

Lösungsweg (🧑 mit B-099): (a) `percentPerGather` senken, (b) Druck je Menge statt je Lieferung, (c) Adern mit eigenem, kleinerem Wert. Ob eine Regeländerung nötig ist, entscheidet 🧑.

## Notizen

Entstanden bei der Vorbereitung von W2.1 (Kontext „Rate“, Fallstrick `gatherPressure`). W2.1 ändert den Pool nicht und nennt im Ergebnis die gemessene Größenordnung. Kupfer: 2 × 45/min = 9 Lieferungen/min → +9 %/min; Eisen +7 %/min; Kristall +5 %/min (jeweils bei 10 Einheiten je Lieferung).
