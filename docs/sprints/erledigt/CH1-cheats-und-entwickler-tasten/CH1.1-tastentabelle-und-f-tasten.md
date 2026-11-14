# CH1.1 · Tastentabelle, F1-Übersicht und F-Tasten F5–F8, F11, F12

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** –
- **Tickets:** B-888
- **Kriterien:** AC-01, AC-02

## Ziel

Eine zentrale Tabelle der Entwickler-Tasten steuert Auswertung und F1-Übersicht; F2/F3 ziehen dorthin um, F5–F8, F11 und F12 wirken.

## Kontext

- `native/k3c-game/game/devkeys.go` wertet heute F2/F3 aus (`g.pressed`), nicht im Aufnahme-Modus.
- Vollbild liegt auf `view.ActionFullscreen` (Taste F, `input/bindings.go`), Pad: rechter Stick.
- `local.Session` (`local/session.go`, `local/step.go`) taktet den Raum; Zeitraffer und Dev-Pause kennt `engine/room/dev.go` (`Room.Dev`, Aktionen `timescale`, `pause`), Manager-Feld `Dev` muss gesetzt sein.
- Screenshot: `game/shots.go` schreibt PNG im Aufnahme-Modus, Muster für F12.
- Importregel: `game/imports_test.go`.

## Erlaubte Dateien

- `native/k3c-game/view/**`
- `native/k3c-game/game/**`
- `native/k3c-game/input/**`
- `native/k3c-game/local/**`
- `native/k3c-game/devtools/**`
- `native/k3c-game/menu/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/e2e/**`

## Nicht-Ziele

- Cheat-Menü (CH1.2), Debug-Overlay (CH1.3).

## Schritte

1. `view/devkeys.go`: Tabelle `DevKeys` (Taste, Beschriftung, Text, Befehl).
2. game wertet die Tabelle aus (F1, F2, F3, F5, F6, F7, F8, F10, F12); F9 und F11 über input-Aktionen aus derselben Tabelle.
3. local: Dev-Schnittstelle (Dev-Aktion, Einzelschritt, Zeitraffer im Frame).
4. Neues Fachpaket `devtools` zeichnet die F1-Übersicht (LayerOverlay).
5. Tests: jede Tabellen-Taste hat einen Befehl und steht in der Übersicht; Pause/Einzelschritt/Zeitraffer in local.

## Fertig, wenn

- [ ] AC-01: Go-Test Tabelle ↔ Auswertung ↔ Übersicht grün.
- [ ] AC-02: Go-Tests für F7/F8 (local) und F11 (input) grün.
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

- AC-01 geprüft: `view/devkeys.go` (Tabelle `DevKeys`), `game/devkeys.go` wertet sie aus, `devtools/help.go` zeichnet F1 daraus; Tests `TestDevKeyTableEvaluated`, `TestHelpRowsCoverTable`, `TestDevKeysOnlyFunctionKeys` grün.
- AC-02 geprüft: F5 `Session.Save`, F6 `local.Reload` (lädt ohne Speichern), F7 `Halt`/`StepOnce`, F8 Zeitraffer über `Room.DevLocal` (in der Pause: weiter), F11 Vollbild (Taste F entfällt), F12 PNG nach `reports/`; Tests `TestDevKeySession`, `TestHaltStepOnce`, `TestReload`, Input-Tabellen grün.
- Neu: `engine/room/dev.go` › `DevLocal` (ohne Dev-Mode, sonst ändert `Manager.Dev` Grad und Welten) und `Timescale`.
- `check_run task:check:diff:go` grün; `:js` rot nur durch B-887/WR1.2 (vorbestehend auf dem Basis-Commit, Sprint WR1).
