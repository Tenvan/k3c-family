# NG2.4 · Review Welt und Kamera

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** welt
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng2/4-review
- **Abhängig von:** NG2.3
- **Tickets:** B-795
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint NG2 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Batching, Allokationen je Bild, Texturbudget). Prüfen: `view` und `game` unverändert, Importregel eingehalten.

## Erlaubte Dateien

- `native/k3c-game/world/**`
- Planung NG2, B-795 (über MCP)

## Nicht-Ziele

Neue Funktionen; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03: Nachweise aus den Session-Ergebnissen geprüft.
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

Review Welle 2a (Agent `k3c-reviewer`, Opus) für `world`: Befund Fackeln doppelt gezeichnet (mit `props`) behoben: `world` zeichnet Halter und Flamme, Stärke aus `data/light.json`. Ohne Befund: Kamera und Aufblenden je Spieler getrennt, Zeit nur aus dem Takt, Lichtkarte als ein Zeichenaufruf je Zelle.

Geprüft: `go vet`, `go test`, `golangci-lint` im Modul grün (0 Befunde), `check_run task:check:diff` grün; Aufnahme-Modus bildgleich (drei Läufe Takt 300, zwei Läufe Takt 900), Bilder unter `reports/native-game-shots/w2a*`. `task check` und `task check:go` nicht gefahren: Die Welle ändert nur `native/`. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf dem Branch `sprint/ng-welle2a` mit einem gemeinsamen PR.
