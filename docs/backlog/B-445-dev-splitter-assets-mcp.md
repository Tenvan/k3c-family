# B-445 · Assets- und MCP-Seite von k3c-dev haben ziehbare Splitter zwischen den Panelen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-16, Chat): „Assets fehlen noch Splitter zwischen den Panelen · MCP fehlen noch Splitter zwischen den Panelen“. Tasks- und Planungsseite haben schon den `Splitter` aus `tasks/TaskBars.tsx` (B-400, B-403), der wird wiederverwendet.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Assets: zwischen Slot-Liste und Detail ein senkrechter Splitter (20–80 %, Doppelklick = Vorgabe, gemerkt), unter 1100 px ausgeblendet.
- **AC-02** MCP: Splitter zwischen Tool-Kacheln und Live-Monitoren (Widgets) sowie zwischen Tool-Tabelle und Seitenspalte (Statistiken), gemerkt, im schmalen Layout ausgeblendet.
- **AC-03** Keine neue Abhängigkeit; Ziehen im Mock geprüft.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
