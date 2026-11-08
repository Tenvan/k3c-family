# B-760 · assetkit: Grafik-Slots, Katalog und Pixel-Credits aus Register und Recherche entfernen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

CU1.4 hat das Slot-Register von `data/assets.json` auf `data/sounds.json` umgestellt und Atlas-, Slotdetail- und Zuordnungs-Code entfernt. In `tools/assetkit` bleibt aber die allgemeine Register-Logik mit Grafik-Slots (`KindGraphic`, `Rect`, `Size`, Zielmaß `target.go`, Ausschnitt und Raster in `Choice`), der Katalog (`catalog.go`: läuft noch über `public/grafik` und `public/sprites`, liest deren CREDITS.md) und die Recherche-, Import- und Vorrats-Pfade, die `reg.Find(slot)` brauchen (`candidates*.go`, `research*.go`, `import.go`, `credits.go`, `vorrat*.go`, `suggestions*.go`). Ohne Grafik-Slots ist die Recherche für Grafik nicht mehr möglich.

## Ziel

`assetkit` kennt nur noch Sound-Slots; Katalog und Credits der alten Pixel-Packs sind weg; die Recherche-Tools prüfen Grafik-Funde nicht mehr gegen ein Register.

## Beteiligte und Zielgruppen

Entwickler (🧑), Workbench Assets.

## Anforderungen

- `KindGraphic`, Zielmaß (`target.go`, `SetTargetIf`), `Rect`, `Size`, Ausschnitt, Raster und Sheet aus Register und `Choice` entfernen; Tests anpassen.
- `Catalog` auf `public/audio` und `public/audio-test` beschränken (oder entfernen, wenn kein Nutzer bleibt); `studioview.Sources` ohne Grafik-Katalog; Frontend-Quellenseite anpassen.
- Recherche, Import, Vorrat: `Slot nicht im Register` für Grafik-Funde streichen (freie Ziel-ID oder Papier-Asset), `instructions.md` anpassen.

## Nicht-Ziele

Neue Recherche-Funktionen; Papier-Tools (`internal/paperassets`).

## Regeln und Einschränkungen

Erst nach CU1.5 (alte Dateien weg); Workbench-EXE läuft aus der Repo-Wurzel.

## Beispiele

`assets_hit_add` mit einem Grafik-Fund ohne Register-Slot läuft durch; `grep -rn KindGraphic tools/assetkit` liefert nichts.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `git grep -n -E "KindGraphic|grafik/CREDITS|sprites/CREDITS" -- tools/assetkit tools/k3c-workbench` ohne Treffer.
- **AC-02** `go test ./...` in `tools/assetkit` und `tools/k3c-workbench` grün (die TLS-Tests laufen lokal wegen Norton rot, in der CI grün).

## Offene Fragen

Brauchen Grafik-Recherchen weiter einen Zielbezug (Papier-Asset-ID)? (🧑)

## Notizen

Aus CU1.4 abgetrennt, weil der Umbau die gesamte Recherche-Logik und ihre Tests betrifft.
