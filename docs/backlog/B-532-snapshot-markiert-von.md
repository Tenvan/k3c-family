# B-532 · Snapshot nennt, wer einen Knoten markiert hat

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
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

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Protokolländerung (`docs/protocol.md`, beide Enden, eigene Session), Spielstand-Feld (`save.go` › `nodesMarked`) betroffen.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `ResourceNode` hat `markedBy` (Spieler-Index, fehlt = unbekannt), gesetzt mit der letzten Markier-Münze; Go-Test.
- **AC-02** Der Client nimmt `markedBy` statt der Ereignis-Zuordnung; Vitest.

## Offene Fragen

🧑: Lohnt sich die Protokolländerung, oder reicht die Annäherung aus GEB10.3?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
