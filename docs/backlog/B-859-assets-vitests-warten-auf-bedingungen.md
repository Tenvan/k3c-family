# B-859 · Alle Vitests unter src/assets der Workbench warten auf Bedingungen statt flush() und bleiben unter Last grün

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ53
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

WZ50 (B-837) und WZ52 (B-858) haben Katalog- und readonly-Tests auf `until` (`src/assets/testing.tsx`) umgestellt. WZ52.1 fand `src/assets/size/SizeCompare.test.tsx` unter Last rot (`Cannot read properties of null`); weitere Tests unter `src/assets/review/` nutzen noch `flush()` mit fester Rundenzahl (gleiche Ursache vermutet, ungeprüft).

## Ziel

Kein Vitest unter `tools/k3c-workbench/frontend/src/assets/**` verlässt sich auf feste `flush()`-Runden; alle bleiben unter paralleler Last grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- Tests unter `src/assets/**` (außer den schon umgestellten) auf `until`/`mountPage` aus `src/assets/testing.tsx`.
- Feste Wartezeiten nur für Gegenproben.

## Nicht-Ziele

Tests außerhalb `src/assets` (Balancing, Git, Planung); Produktionscode.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**/*.test.tsx`, `tools/k3c-workbench/frontend/src/assets/testing.tsx`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

10 Läufe von `src/assets` parallel zu drei vollen Vitest-Läufen → alle grün.

## Ausnahme- und Fehlerfälle

nicht relevant: Test-Stabilität.

## Akzeptanzkriterien

- **AC-01** Kein `flush()` mit fester Rundenzahl mehr als Warten auf Inhalt unter `src/assets/**`.
- **AC-02** 10 Läufe von `src/assets` unter paralleler Last grün (Nachweis im Ergebnis).

## Offene Fragen

keine

## Notizen

Befund WZ52.1 (2026-11-14), zusammengefasst für alle Tests unter `src/assets`.
