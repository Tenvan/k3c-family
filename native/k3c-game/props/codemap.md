# native/k3c-game/props/

## Responsibility

Fachpaket Bauten, Natur und Objekte (Sprint NG4, B-797): zeichnet Burg, Bauplätze mit Stufe, Schaden und Ruine, Baustellen, Preis-Anzeigen, Bäume, Felsen, Adern, Plantage, Portal, Hafen, Lager, Truhen, Sterne und Münzen auf `LayerBuildings`.

## Design

- `props.go`: `New(lib)` liefert einen Drawer auf `LayerBuildings`, zugleich `Listener` und `AssetSource`; Paketkommentar.
- `listen.go`: abgeleiteter Zustand je Update (bezahlbar, roden, Wartegründe), den die Simulation nur indirekt liefert.
- Auswahl-Regeln (rein): `buildings.go` (Gebäude-Grafik, Wachstumsreihe, Hintergrundmauer), `sites.go` (Preis-Anzeigen, Wartegründe), `nature.go` (Natur und Objekte), `decor.go`/`decor_glow.go` (Natur-Eigenschaften mit `display: true`, BI2), `data.go` (Namen, Baukosten aus `k3c/data`).
- Zeichenliste je Zelle (rein, getestet): `plan.go`, `plan_sites.go` (Bauplätze, Gerüst, Bauschild, Schaden), `plan_objects.go` (Plantage, Truhen, Sterne, Münzen).
- `paint.go`: zeichnet die Liste mit wenigen weißen Vorlagen (Bildpunkt, Scheibe, Ring) und Papier-Sheets.
- `assets.go`: alle Papier-Assets, die props in einem Level zeichnen kann (`view.AssetSource`).

## Flow

1. Levelwechsel: `Assets(level, world)` → Liste für `art.Library.SetLevels`.
2. Je Frame `Listen(frame)` → abgeleiteter Zustand für Plätze und Burg.
3. Draw je Zelle → `plan` baut aus Welt, Zustand und Kamera die Zeichenliste → `paint` zeichnet Bilder und Formen.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim`, `k3c/engine/level`, `k3c/data`.
- Fackeln und Licht zeichnet `native/k3c-game/world`.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`).
