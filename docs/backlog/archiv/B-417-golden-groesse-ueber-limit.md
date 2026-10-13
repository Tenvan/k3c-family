# B-417 · Golden-Daten liegen mit 12,7 MB über der Grenze von 8 MB aus SP06

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** TST
- **Erstellt:** 2026-10-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-075 und W7.3 nennen „Golden-Daten gesamt höchstens 8 MB (SP06)“. Gemessen am 2026-10-13 (`git ls-tree -r -l HEAD testdata/golden`): schon vor W7.3 12,66 MB in 18 Dateien, W7.3 legt `campaign-ausbau.json` mit 0,4 MB dazu. Kein Test prüft die Grenze; die Angabe „heute ca. 7,9 MB“ in W7.3 war veraltet.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entschieden 2026-10-13 (🧑, Chat): Grenze auf 14 MB anheben. Umgesetzt als Test `Golden-Daten gesamt höchstens 14 MB (B-417)` in `tests/projectRules.test.ts` (vorher gab es keine Prüfung).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
