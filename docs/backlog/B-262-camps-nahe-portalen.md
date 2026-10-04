# B-262 · Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit W0.3b (B-261) wählt der Level-Generator für Camps zuerst Chunks, deren Mitte bei jeder Streuung ≥ 4 Units von allen Linien-Plätzen liegt (`engine/level/level.go` › `placeEvents`, `engine/level/lines.go` › `clearOfLines`). Die Linien belegen ±32 bis ±136 Units ab Hub-Mitte lückenlos, deshalb liegen Camps jetzt bei ±175 bis ±275 statt ±75/±125 (Golden-Level: forest 21 × 175, 5 × 225, 2 × 275). Portale liegen ab ±150 (Chunk-Mitten ab ±175), also teils im Nachbar-Chunk eines Camps.

## Ziel

🧑 weiß, ob Landstreicher so weit draußen und nah an den Portalen spielbar sind, und entscheidet gegebenenfalls eine andere Regel.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen), 🧑 entscheidet; SIM setzt um.

## Anforderungen

- Messung oder Spieltest: Wie oft liegt ein Camp im Chunk neben einem Portal, und überleben die Landstreicher die erste Nacht?

## Nicht-Ziele

Abstand zu Hub-Plätzen und Ressourcen; Rückbau von B-261.

## Regeln und Einschränkungen

Level-Golden nur nach `docs/arbeitsweise.md` › „Golden aktualisieren“; deterministisch, nur `engine/rng`.

## Beispiele

Forest, Seed `golden-1`: Camp bei Hub-Mitte +175, Portal im Chunk daneben → Gegner laufen in der ersten Nacht durch das Camp.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** Die gewählte Regel (so lassen, Mindestabstand Camp ↔ Portal oder Camps zwischen die Linien mit Vorrang des Bauplatzes) ist im Ticket vermerkt.

## Offene Fragen

keine: entschieden 2026-10-04 (🧑, Chat): so lassen; offen ist nur die Bewertung über Balancing-Läufe bzw. Spieleabend.

## Notizen

Aufgefallen beim Golden-Update in W0.3b (2026-10-04).

**Entscheidung 2026-10-04 (🧑, Chat, vermerkt in W0.4):** Camps bleiben vorerst bei ±175–275 nahe den Portalen (Regel „so lassen“, AC-01). Bewertung später über Balancing-Läufe (BAL3/BR1) bzw. den Spieleabend; ohne eigenen Sprint, das Ticket bleibt bis dahin offen.
