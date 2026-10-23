# B-572 · Baugerüst steht immer bei waitingWorker, auch bei Gefahr

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** GEB14
- **Projekt:** GEB
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-545 (D8, KR2.1): Das Baugerüst steht wieder immer bei `hubUpgrade.state = waitingWorker`, auch nachts bei Gefahr (Stand GEB11). GEB12.3 hatte es auf „nur ohne Gefahr“ begrenzt.

## Ziel

`hubLookOf` zeigt das Gerüst bei jedem `waitingWorker`, unabhängig von `danger`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Eine Zeile in `src/scenes/render/hubGrowth.ts` (`hubLookOf`) und der Test von GEB12.3 zurück.
- Kein Server betroffen.
- Regel `docs/rules/materialien-gebaeude.md` § 2.1 („ein Bauer baut“) und Text dazu prüfen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `hubLookOf` liefert bei `waitingWorker` das Gerüst, auch mit `danger`; Test angepasst, `task check` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-545 (KR2.1, Alternative).
