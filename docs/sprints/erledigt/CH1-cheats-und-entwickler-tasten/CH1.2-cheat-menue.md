# CH1.2 · Cheat-Menü F9 je Spieler

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.1
- **Tickets:** B-888
- **Kriterien:** AC-03

## Ziel

F9 (Tastatur) bzw. View + Y (Gamepad) öffnet je Spieler ein Cheat-Menü mit den Gruppen Vorrat, Ausbau, Zeit, Grad, Kampf; Befehle gehen über `Room.Dev` an die Sim.

## Kontext

- Alter Dialog: `src/scenes/debug/debugActions.ts` (nur lesen).
- Dev-Befehle: `engine/room/dev.go`, `engine/room/dev_combat.go`, `engine/sim/dev*.go` (🐛-Log je Aktion schon im Raum).
- Overlay je Spieler wie das Skill-Menü (`hud/overlay.go`, `view/roles.go` › Overlay).
- Dev-Schnittstelle der Session aus CH1.1.

## Erlaubte Dateien

- `native/k3c-game/view/**`
- `native/k3c-game/devtools/**`
- `native/k3c-game/input/**`
- `native/k3c-game/local/**`
- `native/k3c-game/game/**`

## Nicht-Ziele

- Neue Dev-Befehle in der Sim.

## Schritte

1. Menü-Modell (Gruppen, Einträge, Dev-Aktion) rein und getestet in `devtools`.
2. Overlay: öffnen/schließen je Spieler, Eingabe verbrauchen, Aktion über die Session.
3. Zeichnen als Kasten in der Zelle des Spielers.
4. Go-Test über local: Gold, Material, Zeitraffer, nächste Tageszeit, Nacht wirken.

## Fertig, wenn

- [ ] AC-03: Go-Tests grün (Menü-Modell, Wirkung über local).
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

- AC-03 geprüft: `devtools/cheatlist.go` (Gruppen Vorrat, Ausbau, Zeit, Grad, Kampf; Bosse aus `data/bosses.json`), `devtools/cheatmenu.go` (Bedienung), `devtools/cheats.go` (Overlay je Spieler, F9 der linken Tastatur-Hälfte, sonst Ersatz-Spieler; Pad View + Y), `devtools/drawcheats.go`. Wirkung über `local.Dev` → `Room.DevLocal` (🐛-Log des Raums mit Slot und Wert); Tests `TestCheatOverlayPerPlayer`, `TestCheatOverlayCloses`, `TestCheatKeyFallback`, `TestCheatGroups`, `local/TestDevActions` grün.
- game: Ein verbrauchter Spieler fehlt in der Eingabe späterer Overlays (`play.go` › `hideConsumed`), devtools steht vor hud (Cheat-Menü vor Skill-Menü); Test `TestOverlayHidesConsumedPlayer`.
- Grad „dev“ fehlt im Menü (braucht Dev-Mode des Raums).
- `check_run task:check:diff:go` grün.
