# native/k3c-game/hud/

## Responsibility

Fachpaket Anzeigen (Sprint NG6, B-799): zeichnet je Zelle Spielerkarte, Uhr, Vorrat, Kampf- und Boss-Leiste, Radar, Meldungen, Skill-Leiste, Aktions-Hinweise, geführte erste Nacht und Labels über der Welt auf `LayerHUD`, dazu das Skill-Menü je Spieler.

## Design

- `hud.go`: `New(lib)` liefert den Drawer auf `LayerHUD`; zugleich `Listener`, `Overlay` (Skill-Menü) und `AssetSource` (Icons); Paketkommentar.
- Zustand: `state.go` schreibt je Frame Texte, Meldungen und Spielerzustand fort (`Listen`); `overlay.go` bedient das Skill-Menü als `view.Overlay` (`Update` vor Pause, Beitritt und Befehlen).
- Reine Regeln: `layout.go` (Anker je Element), `values.go` (Texte der Werte), `banner.go` (Ereignis → Meldung), `radar.go`, `kampf.go`, `hints.go` (Aktions- und Guide-Hinweise), `glyphs.go` (Tasten-Glyphen, `KeyLabel`, `PadLabel`, `KeyName`, `IconAsset`), `skillmenu.go`, `skilltext.go`, `fonts.go` (Schriftrollen), `data.go` (Skill-Katalog aus `data/monarch.json`), `texts.go` (Spieltexte de/en).
- Zeichnen: `draw.go` (Elemente der Zelle, Layout-Cache), `drawmenu.go` (Skill-Menü), `drawstock.go` (Vorrat mit Icons), `drawworld.go` (an Orten der Welt), `icons.go` (Papierschnitt-Icons, `Assets`), `paint.go` (Flächen aus weißen Vorlagen).

## Flow

1. Je Update im Spiel: `game` ruft `Overlay.Update` (Skill-Menü verbraucht Eingabe) → `view.Session.Learn`/`Respec` sowie gesehene Hinweise über `SetSettings`.
2. Je Frame `Listen(frame)` → `state.go` aktualisiert Texte und Meldungen (`banner.go`).
3. Draw je Zelle → `layout.go` → `draw.go`/`drawstock.go`/`drawworld.go`/`drawmenu.go` mit `art.Font` und Icons aus `art.Library.Sheet`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim`, `k3c/engine/level`, `k3c/data`.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`; Tastentabelle `keytable.go` nutzt die Glyphen).
