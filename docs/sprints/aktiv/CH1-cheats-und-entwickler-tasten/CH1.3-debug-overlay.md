# CH1.3 · Debug-Overlay F10: Hitboxen, Chunk-Grenzen, Kamera

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.1
- **Tickets:** B-888
- **Kriterien:** AC-04

## Ziel

F10 schaltet ein Debug-Overlay: Hitboxen der Figuren und Bauten, Chunk-Grenzen, Kamera-Mitte und -Spanne je Zelle.

## Kontext

- Zellen und Kamera: `view/cell.go` (`Camera.ScreenX`, `Cell.ScreenY`).
- Welt der Stufe: `view.Frame.Stages[i].World` (`*sim.World`).
- Zustand des Overlays trägt der Frame (`view.Frame.Dev`, CH1.1).

## Erlaubte Dateien

- `native/k3c-game/devtools/**`
- `native/k3c-game/view/**`
- `native/k3c-game/game/**`

## Nicht-Ziele

- Werte-Seite (CH1.4).

## Schritte

1. Geometrie (Rechtecke in Fenster-Koordinaten) rein und getestet.
2. Drawer auf LayerHUD zeichnet Rahmen und Linien.

## Fertig, wenn

- [ ] AC-04: Go-Test der Geometrie grün.
- [ ] `check_run task:check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
