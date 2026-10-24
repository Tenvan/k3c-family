# B-607 · Der Studio-Test levelView erwartet die Baum-Zahl vor dem dichten Wald

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Zwei Studio-Tests sind auf `develop` rot, seit der dichte Wald (#455, 87e0831c) Level und Vorschau geändert hat:

- `tools/k3c-studio/frontend/src/world/levelView.test.ts` („Objekte nach x sortiert, Anzahl je Art wie in der Golden-Datei“) erwartet `tree: 42`, `skillPoint: 5`; die Golden-Datei liefert `tree: 88`, `skillPoint: 4`.
- `tools/k3c-studio/backend` › `TestMockVorschauIstDieVorschauDesServers`: `previewFixture.json` weicht von `net.Preview(k3c, forest, Hub 5)` ab.

`task check:studio` ist deshalb rot, und jede Session mit Diff in `engine/` oder `data/` scheitert in `check:diff`. Gefunden in KB2.1.

## Ziel

`task check:studio` ist auf `develop` wieder grün.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Erwartete Zahlen in `levelView.test.ts` an die aktuelle Golden-Datei anpassen oder aus ihr ableiten.
- `previewFixture.json` neu erzeugen (Anleitung in `app_preview_fixture_test.go`).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task check:studio` grün auf `develop`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Duplikat von B-606 (auf `sprint/wz28`), dort erfasst samt `TestChatZeitlimit`. Verworfen.
