# NG3.5 · Review Figuren

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng3/5-review
- **Abhängig von:** NG3.4
- **Tickets:** B-796, B-791, B-780
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint NG3 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Rig-Mathematik, Allokationen je Bild, Batching). Prüfen: `view` und `game` unverändert, Importregel.

## Erlaubte Dateien

- `native/k3c-game/actors/**`
- `native/k3c-rig/rig/**`
- Planung NG3, B-796, B-791, B-780 (über MCP)

## Nicht-Ziele

Neue Funktionen; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game` und `native/k3c-rig`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge; B-780 bleibt offen bis NG9.5.

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

Review Welle 2a für `actors` und `k3c-rig/rig`: Befunde behoben: Umhang und Animation werden im festen Takt fortgeschrieben statt beim Zeichnen (`TestPoseIndependentOfDraws`); Ecke der Bein-Bänder per Lot stetig (`TestUnfoldIsContinuous`, `TestLegStripContinuous`); Sheets nur für Figuren, die im Level vorkommen können; Rig-Figuren mit Deckkraft (fliehende Gegner 0,7, Monarch mit Smoke Bomb 0,35). Die Korrektur der Bein-Bänder ist laut Review mathematisch sauber. Grenze: überlappende Rig-Teile scheinen bei Deckkraft < 1 durch.

Geprüft: `go vet`, `go test`, `golangci-lint` im Modul grün (0 Befunde), `check_run task:check:diff` grün; Aufnahme-Modus bildgleich (drei Läufe Takt 300, zwei Läufe Takt 900), Bilder unter `reports/native-game-shots/w2a*`. `task check` und `task check:go` nicht gefahren: Die Welle ändert nur `native/`. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf dem Branch `sprint/ng-welle2a` mit einem gemeinsamen PR.
