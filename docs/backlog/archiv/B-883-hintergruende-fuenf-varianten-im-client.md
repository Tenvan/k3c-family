# B-883 · Der Client nutzt alle fünf Hintergrund-Varianten

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

MG11 (B-722) liefert je Biom und Ebene fünf Hintergrund-Varianten `hg-<biom>-<ebene>-v1` … `v5`. `data/paper.json` › `hintergruende.varianten` steht noch auf 2, der Client wiederholt also v1/v2; Tests (`tests/paperAssets.test.ts`, `src/scenes/render/worldSprites.test.ts`) erwarten v1/v2.

## Ziel

Das Spiel zeigt alle fünf Varianten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

`hintergruende.varianten` auf 5, Tests anpassen; Ladeliste (LoadScene) lädt die neuen Dateien.

## Nicht-Ziele

Neue Grafik, Parallax-Code.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test: `paperBackgroundFile('wald', 'fern', 5)` liefert `hintergruende/hg-wald-fern-v5/stand.png`, v6 wieder v1.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Quelle: MG11.2 (B-722).
