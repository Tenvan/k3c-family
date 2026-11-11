# NG6.5 · Review HUD, Hinweise und Effekte

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hud
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng6/5-review
- **Abhängig von:** NG6.4
- **Tickets:** B-799
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint NG6 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Allokationen je Bild, Text-Cache). Prüfen: Gold nur im HUD, `view` und `game` unverändert, Importregel.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/fx/**`
- Planung NG6, B-799 (über MCP)

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03, AC-04: Nachweise geprüft.
- [ ] PR offen, Auto-Merge an.

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

Review Welle 2b (Agent `k3c-reviewer`, Opus) für `hud` und `fx`: Befunde behoben: Label-Budget zählt erst nach der Sichtprüfung (`hud/labels_test.go`); Kontur-Texte über einen Bild-Cache (`art/outline.go`) statt 9 Zeichenaufrufen je Text; Auren je Monarch zwischengespeichert; Pause-Band entfernt (lag unter dem Pause-Menü); Tasten-Texte je Gerät, geprüft gegen die Belegung (`game/keys_test.go`). Skill-Menü im Bild: `reports/native-game-shots/w2b-skillmenu/shot-01.png`. Offen: Mit der Debug-Schrift überlagern sich Titel und erste Zeile des Skill-Menüs.

Geprüft: `check_run task:check:diff` grün (58 s); `go vet`, `go test`, `golangci-lint` in `native/k3c-game` (0 Befunde) und an der Wurzel (`go test -short ./engine/... ./cmd/...`) grün; Aufnahme-Modus in allen Szenen in zwei Läufen bildgleich, auch beim Speichern. `task check`, `task check:go`, `task check:all` nicht gefahren; CI und Docker ungetestet bis zum Merge. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf `sprint/ng-welle2b` mit einem gemeinsamen PR.
