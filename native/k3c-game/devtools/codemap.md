# native/k3c-game/devtools/

## Responsibility

Fachpaket Entwickler-Anzeigen (B-888, B-889): Tastenübersicht (F1), Cheat-Menü je Spieler (F9 bzw. View + Y), Debug-Overlay (F10) sowie Chronik und Sitzungsbericht des laufenden Spiels; nichts davon erscheint im Aufnahme-Modus.

## Design

- `devtools.go`: `New(lib)` liefert einen Drawer auf `LayerOverlay` (je Zelle in deren Rechteck aus `Frame.Cells`); F1/F10 setzt `game` über `Frame.Dev`; Paketkommentar.
- Cheat-Menü: `cheats.go` (`view.Overlay`, `Update`, je Spieler ein Menü), `cheatmenu.go` (Bedienung: Gruppe, Eintrag, Auslösen), `cheatlist.go` (Einträge in Gruppen wie Vorrat), `drawcheats.go` (Zeichnen in der Zelle).
- Debug-Overlay: `debuggeo.go` (Rahmen um Figuren, Münzen, Bauplätze, rein), `values.go` (Seite Werte aus Frame und Welt), `drawdebug.go` (Zeichnen, beschnitten je Zelle).
- `help.go`: Tastenübersicht aus `view.DevKeys` (`HelpRow`, `HelpRows`).
- Chronik: `chronicle.go` (`NewChronicle`, `view.Listener`; knappe Logs nur bei Wechseln), `chronicle_stage.go` (Wechsel je Stufe, Spieler, Takt), `report.go` (`Report`, `Finish` als `view.Finisher`: JSON `native-session-<Zeit>-t<Takte>.json` in `reports/` am Spielende).

## Flow

1. `game.Packages` meldet `devtools.New(lib)` und `NewChronicle(lib, view.ReportsDir)` vor `hud` an (Cheat-Menü nimmt Eingabe vor dem Skill-Menü).
2. Je Update: Cheat-Overlay `Update` → Dev-Aktionen über `view.DevSession` (`local/dev.go`); Chronik `Listen(frame)` loggt Wechsel.
3. Draw auf `LayerOverlay`: Hilfe, Cheat-Menüs, Debug-Formen und Werte je Zelle; am Ende eines Spiels schreibt die Chronik den Bericht, beim Programmende über `Finish` (`view.Finisher`).

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim`, `k3c/engine/level`, `k3c/data`.
- Konsumenten: `native/k3c-game/game` (`parts.go`); Berichte liest die Workbench über `report_read`.
