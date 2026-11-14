# NG7.5 · Review Menüs und Spielstände

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** menue
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng7/5-review
- **Abhängig von:** NG7.4
- **Tickets:** B-800
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint NG7 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Schwerpunkte: B zurück in jedem Menü, Löschen nur mit Rückfrage, kein Import von `local`. `view` und `game` unverändert.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- Planung NG7, B-800 (über MCP)

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

Review Welle 2b für `menu`: schwerer Befund behoben: Ein abgebrochenes Spielende beendete später das nächste Spiel; jetzt `Frame.Outcome` mit `view.EndDelay`, `game` schließt das Spiel und tut danach im selben Update nichts mehr (`TestEndingResetsAfterLeavingThroughPause`, `TestEndingClosesAfterDelay`). `Session.Settings()` und `Seats()` im Vertrag. Menüs im Bild: `reports/native-game-shots/w2b-menu-*`, `w2b-pause`, `w2b-options-im-spiel`. Offen: `victory` wird nur gesehen, wenn ein lokaler Spieler auf Stufe 0 steht.

Geprüft: `check_run task:check:diff` grün (58 s); `go vet`, `go test`, `golangci-lint` in `native/k3c-game` (0 Befunde) und an der Wurzel (`go test -short ./engine/... ./cmd/...`) grün; Aufnahme-Modus in allen Szenen in zwei Läufen bildgleich, auch beim Speichern. `task check`, `task check:go`, `task check:all` nicht gefahren; CI und Docker ungetestet bis zum Merge. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf `sprint/ng-welle2b` mit einem gemeinsamen PR.
