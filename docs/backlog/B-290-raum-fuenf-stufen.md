# B-290 · Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit W2.2 gibt es die Biome `ironhold` (Tiefe 3) und `crystal` (Tiefe 4); die Sim baut Inseln mit fünf Stufen (`CreateIsland(…, []int{0,1,2,3,4}, …)`). Der Raum erzeugt die Insel aber fest mit `[]int{0, 1, 2}` (`engine/room/manager.go`), und der Client importiert nur `forest`, `cave` und `mine` (`src/model/biome.ts`). In der Mine steht jetzt ein Platz `stairsDown` und ein Eingang nach Tiefe 3, die im Raum ins Leere führen (der Spieler bleibt stehen, `moveToStage`).

## Ziel

Im Raum sind alle fünf Stufen erreichbar und werden gezeichnet.

## Beteiligte und Zielgruppen

Spieler; SRV (Raum), CLI (Biome im Client).

## Anforderungen

- Der Raum erzeugt die Stufen aus den Daten (bis B-103/K2: alle Biome).
- Der Client kennt die neuen Biome (Palette, Name).

## Nicht-Ziele

Gegner der neuen Stufen (K1, B-129), Grafik (B-010), `data/islands.json` (B-103).

## Regeln und Einschränkungen

Domänen SRV und CLI getrennt (je eine Session); Protokoll unverändert, `biomeId` gibt es schon.

## Beispiele

Spieler geht in der Mine über den Eingang → landet im Eisenstollen, der Client zeichnet ihn.

## Ausnahme- und Fehlerfälle

Unbekanntes Biom im Client → klarer Fehler statt Absturz.

## Akzeptanzkriterien

- **AC-01** Test: Ein neuer Raum hat fünf Stufen.
- **AC-02** Test: Der Client lädt alle Biome aus `data/biomes/`.

## Offene Fragen

Kommt das mit K2/B-103 (Inseldaten) oder vorher (🧑)?

## Notizen

Gefunden in W2.2.
