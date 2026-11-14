# B-881 · Burg-Ausbau-Überlage zeigt die Türmchen im Material der Burg-Stufe

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

D-MG6-08: Die Wehrtürmchen der Überlage `burg-ausbau` sind immer Holz, auch über einer Burg aus Stein, Kupfer, Eisen oder Kristall. MG11.1 hat dem Asset die Varianten `stein`, `kupfer`, `eisen`, `kristall` gegeben (`stand@<material>.s<n>.png`); der Client lädt bisher nur `stand.s<n>.png`.

## Ziel

Die Überlage passt im Material zur Burg-Stufe darunter.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

`castleOverlay`/`BUILDING_TEXTURES` (`src/scenes/render/buildingSprites.ts`) wählen die Variante nach der Hub-Stufe (2 stein, 3 kupfer, 4 eisen, 5 kristall) und laden die Dateien.

## Nicht-Ziele

Neue Grafik.

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

- **AC-01** Test: Ausbauschritt über Hub-Stufe n lädt `burg-ausbau/stand@<material>.s<schritt>.png` mit dem Material der Stufe n.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Quelle: MG11.1 (B-721), D-MG6-08.
