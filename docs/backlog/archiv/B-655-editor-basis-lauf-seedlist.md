# B-655 · Balancing-Editor übernimmt seedList einer Baseline und warnt bei abweichenden Seeds im Delta

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review BS7 (Befund 4): `BalanceSpec`/`BalanceForm` im Editor (`tools/k3c-workbench/frontend/src/api/typesBalance.ts`, `balancing/spec.ts` › `formFromBaseline`) kennen `seedList` nicht. Eine Baseline, die mit `seedList` entstand, wird mit „Übernehmen“ auf den Seed-Modus der Datei zurückgesetzt; das Delta vergleicht dann andere Seeds. `baselineView` (`insights.go`) warnt nur bei abweichender Seed-Anzahl.

## Ziel

„Übernehmen“ setzt auch die `seedList`, und das Delta warnt bei abweichenden Seeds (Namen oder Modus).

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

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-10-29 (Claude): erledigt. `BalanceSpec.seedList` und Formularfeld „Seed-Liste“ (ConfigForm, Hilfe `seedList`), „Übernehmen“ setzt die Liste; Seeds und Liste zusammen werden abgelehnt. Test in balancing.test.ts. Nicht umgesetzt: Delta-Warnung bei abweichenden Seeds (`baselineView`), dort nur die Seed-Anzahl.
