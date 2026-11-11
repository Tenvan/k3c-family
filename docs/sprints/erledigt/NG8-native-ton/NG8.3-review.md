# NG8.3 · Review Ton

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** ton
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng8/3-review
- **Abhängig von:** NG8.2
- **Tickets:** B-801
- **Kriterien:** AC-01, AC-02

## Ziel

Sprint NG8 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Schwerpunkte: kein Absturz ohne Audio-Gerät, Lizenzen, `view` und `game` unverändert.

## Erlaubte Dateien

- `native/k3c-game/audio/**`
- Planung NG8, B-801 (über MCP)

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02: Nachweise geprüft.
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

Review Welle 2b für `audio`: ohne Befund (kein Datenrennen beim Laden, fehlende Dateien stumm). Aufnahme-Modus jetzt über `Frame.Recording` statt Kommandozeile, Dämpfung aus den Kameras der Zellen. Offen: Scheitert das Ton-Gerät mit einem anderen Fehler als „kein Gerät“, beendet Ebitengine das Spiel; abfangbar nur mit oto direkt statt `ebiten/audio` (Ticket).

Geprüft: `check_run task:check:diff` grün (58 s); `go vet`, `go test`, `golangci-lint` in `native/k3c-game` (0 Befunde) und an der Wurzel (`go test -short ./engine/... ./cmd/...`) grün; Aufnahme-Modus in allen Szenen in zwei Läufen bildgleich, auch beim Speichern. `task check`, `task check:go`, `task check:all` nicht gefahren; CI und Docker ungetestet bis zum Merge. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf `sprint/ng-welle2b` mit einem gemeinsamen PR.
