# B-532 · Snapshot nennt, wer einen Knoten markiert hat

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB14
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Wimpel über markierten Knoten trägt seit GEB10.3 die Farbe des markierenden Spielers. Der Snapshot kennt nur `ResourceNode.marked`; der Client leitet den Spieler aus dem Ereignis `coinGive` (`to: mark`, Ort des Spielers) ab und ordnet es dem nächsten Knoten in Zahl-Reichweite zu (`src/scenes/render/pennant.ts`). Nach Neuladen, Beitritt oder bei mehreren Knoten in Reichweite kann die Farbe fehlen (neutral) oder danebenliegen.

## Ziel

Der Snapshot nennt je Knoten, wer ihn markiert hat; der Wimpel ist immer richtig gefärbt.

## Beteiligte und Zielgruppen

Spielende mit zwei Spielern (Wimpel-Farbe); 🧑 entscheidet (B-539); Agent SIM, SRV (Protokoll), CLI.

## Anforderungen

- `markedBy` deterministisch mit der letzten Markier-Münze gesetzt, im Spielstand gespeichert.
- Zwei Spieler: jeder Knoten trägt die Farbe des letzten Markierers.

## Nicht-Ziele

Neue Wimpel-Grafik.

## Regeln und Einschränkungen

Protokolländerung (`docs/protocol.md`, beide Enden, eigene Session), Spielstand-Feld (`save.go` › `nodesMarked`) betroffen.

## Beispiele

Spieler 2 markiert einen Baum, Gerät lädt neu → Wimpel bleibt in Farbe von Spieler 2.

## Ausnahme- und Fehlerfälle

Spieler verlässt den Raum → `markedBy` bleibt, Client zeigt die gespeicherte Farbe oder neutral.

## Akzeptanzkriterien

- **AC-01** `ResourceNode` hat `markedBy` (Spieler-Index, fehlt = unbekannt), gesetzt mit der letzten Markier-Münze; Go-Test.
- **AC-02** Der Client nimmt `markedBy` statt der Ereignis-Zuordnung; Vitest.

## Offene Fragen

Entscheidung D2 in B-539 (Sprint KR2): umsetzen oder verwerfen, Default: nach GEB10.5 entscheiden.

## Notizen

Geplant in GEB12.2 (2026-10-22): Prio niedrig, wartet auf B-539.
