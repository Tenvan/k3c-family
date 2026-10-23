# GEB14 · REG · Wald und Höhle dichter, Gerüst bei Wartenden, Knoten-Markierer im Snapshot

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-531, B-532, B-572
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wald hat wenige Bäume, Höhle keine Deko (B-531); das Gerüst steht nur ohne Gefahr (B-572); der Snapshot nennt nicht, wer einen Knoten markiert hat (B-532).

## Ziel

Wald dichter, Höhle mit Büschen, Gerüst bei jedem wartenden Arbeiter, Markierer im Snapshot. Am Ende sichtbar: Level im Browser, Tests grün.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-531, B-532, B-572 › Anforderungen.

## Nicht-Ziele

Neue Biome.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Wald hat mehr Bäume je Abschnitt, Höhle Deko-Büsche (B-531/AC).
- **AC-02** `hubLookOf` zeigt das Gerüst bei jedem `waitingWorker` (B-572/AC).
- **AC-03** Der Snapshot nennt den Markierer eines Knotens, Protokoll nachgezogen (B-532/AC).
- **AC-04** Golden-Änderungen sind begründet.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|


## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
