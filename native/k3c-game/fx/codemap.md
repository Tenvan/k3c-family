# native/k3c-game/fx/

## Responsibility

Fachpaket Effekte (Sprint NG6, B-799): zeichnet Ringe und Partikel zu Ereignissen der Simulation, Skill-Wirkungen, Status-Effekte an Figuren, Warnkreise, Geschosse und Schadenszahlen auf `LayerEffects`.

## Design

- `fx.go`: `New(lib)` liefert Drawer und `Listener`; `Listen` legt je Frame Effekte an und verwirft abgelaufene (Zeit nur aus `Frame.Clock`, kein Zufall); Paketkommentar.
- Reine Regeln: `effects.go` (Ereignis → Effekt), `looks.go` (Aussehen je Skill, Palette DB32), `status.go` (Auren, Sturm, Schild u. a.).
- Zeichnen: `paint.go` (Scheibe, Ring, Dreieck, Bildpunkt als weiße Vorlagen über `ColorScale`), `paintmarks.go` (Status-Markierungen: Bodenflächen zuerst, Symbole danach).
- `texts.go`: einzige Textstelle des Pakets.

## Flow

1. Je Frame `Listen(frame)` → `view.EventsOf` → `effects.go` ordnet Effekte zu → Liste aktiver Effekte.
2. Draw je Zelle → aktive Effekte, Status (`status.go`) und Geschosse aus der Welt → `paint.go`/`paintmarks.go`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art` (Schrift), `k3c/engine/sim`, `k3c/data`.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`).
