# B-539 · Entscheidung D2: Wimpel-Farbe bleibt Client-Annäherung, markedBy erst nach GEB10.5

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR2
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Wimpel über markierten Knoten trägt die Farbe des markierenden Spielers, abgeleitet im Client aus `coinGive` (`src/scenes/render/pennant.ts`). Nach Neuladen oder bei mehreren Knoten in Reichweite kann sie fehlen oder danebenliegen. Umsetzungs-Ticket mit Protokolländerung: B-532.

## Ziel

🧑 entscheidet, ob B-532 (`markedBy` im Snapshot) umgesetzt wird.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Protokoll (`docs/protocol.md`) und Spielstand (`nodesMarked`) betroffen, nur mit Entscheidung von 🧑.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen entschieden: B-532 umsetzen oder verwerfen.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Lohnt die Protokolländerung `markedBy`, oder reicht die Annäherung?

**Entscheidung 🧑 (KR2.1, 2026-10-23):** Default gilt, die Entscheidung fällt nach der Abnahme GEB10.5 (zwei Spieler am Gerät). Die Annäherung bleibt bis dahin, B-532 bleibt offen mit Prio niedrig. Dieses Ticket bleibt `eingeplant`, KR2.1 bleibt deshalb offen. Danach: B-532 zum Sprint im Projekt GEB (Ja) oder verwerfen (Nein).
