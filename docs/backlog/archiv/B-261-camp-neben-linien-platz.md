# B-261 · Ein Camp liegt nie so nah an einem Linien-Platz, dass Zahlziele sich überlagern

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** W0
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Generator legt Camps bevorzugt in die Chunks neben dem Hub (±75, seltener ±125 Units ab Hub-Mitte, `engine/level/level.go` › `placeEvents`). Die Mauerlinien aus W0.2 (`engine/level/lines.go`) liegen bei ±44 bis ±124 plus Streuung 0…+4 mit Turm 8 innen und Tor 4 außen. Die Messung in B-206 › Notizen zeigt: rund 70 % der Camps liegen näher als 4 Units (2 × `payRangeUnits`) an einem Mauer-, Turm- oder Tor-Platz, bei ±125 teils genau auf der Mauer der Linie 5. Q57 erlaubt Camps innerhalb der Linien, sagt aber nichts zur Überlagerung von Zahlzielen (Münze an Landstreicher oder an den Platz).

## Ziel

Ein Spieler, der neben einem Camp steht, weiß eindeutig, ob seine Münze an den Landstreicher oder an den Bauplatz geht.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); 🧑 entscheidet die Regel.

## Anforderungen

- Regel für Zahlziele in der Nähe eines Camps (Vorrang, Abstand oder Verschieben), deterministisch, mit 2 Spielern gleichzeitig.

## Nicht-Ziele

Camps verschieben, solange 🧑 nicht entschieden hat (Q50: Golden-Level bleiben unverändert).

## Regeln und Einschränkungen

Q50, Q57; Level-Golden (`testdata/golden/level-*.json`) ändern sich nur nach `docs/arbeitsweise.md` › „Golden aktualisieren“.

## Beispiele

Camp bei Hub-Mitte +125, Mauer der Linie 5 rechts bei +125 (Streuung 1) → heute liegen Landstreicher und Mauer-Platz am selben Ort.

## Ausnahme- und Fehlerfälle

nicht relevant, solange die Regel offen ist.

## Akzeptanzkriterien

- **AC-01** Ein Test über 500 Seeds je Biom belegt die von 🧑 gewählte Regel (Abstand oder Vorrang).

## Offene Fragen

Vorrang des Bauplatzes, Mindestabstand im Generator (ändert Level-Golden) oder Camp bleibt und Landstreicher laufen weg? Entscheidet 🧑.

## Notizen

Messung: B-206 › Notizen (W0.2).

**Beschluss 2026-10-04 (🧑, Chat, über den Orchestrator von W0.3):** Camps werden auf Abstand zu den Linien-Plätzen verschoben; Zahlziele halten ≥ 4 Units Abstand wie die übrigen Zahlziele. Das Level-Golden-Update ist durch diesen Beschluss gedeckt (Begründung „B-261: Camps auf Abstand zu Linien-Plätzen“). Umsetzung offen: Die Camp-Platzierung liegt in `engine/level/`, das in W0.3 nicht erlaubt ist; bis dahin gilt in der Sim „Bauplatz vor Landstreicher“ (`findPayTarget`), gesperrte Linien-Plätze nehmen keine Münzen.
