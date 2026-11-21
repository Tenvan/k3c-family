# rust/crates/k3c-devwin/src/pages/

## Responsibility

Die fünf Reiter des Cheat-Fensters; jede Seite baut aus dem Katalog des Spiels ihre Gruppen von Controls (ohne Win32).

## Design

- `mod.rs`: `PageDef { key, title, build }` und `PAGES` mit fünf Einträgen: `world` (Welt), `eco` (Wirtschaft), `players` (Spieler), `foes` (Gegner), `view` (Anzeige).
- `build.rs`: kurze Konstruktoren `act`, `dev`, `ui`, `button` und weitere für Controls und Nachrichten; `MAX_PLAYERS`.
- `world.rs`: Welt, Insel, Stufe, Tageszeit, Zeitraffer, Pause, Einzelschritt, Grad, Speichern, Neu laden. `eco.rs`: Gold, Ressourcen, Lager, Baukosten, Ausbau, Burgfall. `players.rs`: Zielspieler, Skills lernen und zurücksetzen, Skillpunkte. `foes.rs`: Welle, Nacht, Boss und Phase, Ereignisse, Gegner und Truppen erzeugen. `view.rs`: Debug-Overlay, Leistung, Fluss, Tastenübersicht, Vollbild, Screenshot, Spielfenster nach vorn.

## Flow

`PAGES[i].build(&Catalog)` → `Vec<Group>` → `layout` → `win32/page.rs`; ein Klick löst die in `button` hinterlegte `Msg` aus (`Dev`, `Ui`, `Goto`, `Focus`).

## Integration

- Abhängigkeiten: `k3c_devproto::Catalog`/`Msg`, `crate::layout::Group`.
- Konsument: `win32/` (Reiterleiste, Seitenfläche, Ereignisse).
