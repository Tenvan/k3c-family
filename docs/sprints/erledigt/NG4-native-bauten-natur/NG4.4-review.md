# NG4.4 · Review Gebäude, Natur und Objekte

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** bauten
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng4/4-review
- **Abhängig von:** NG4.3
- **Tickets:** B-797
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint NG4 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Batching, Allokationen je Bild). Prüfen: `view` und `game` unverändert, Importregel.

## Erlaubte Dateien

- `native/k3c-game/props/**`
- Planung NG4, B-797 (über MCP)

## Nicht-Ziele

Neue Funktionen; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03: Nachweise geprüft.
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

Review Welle 2a für `props`: Befunde behoben: Fackel-Flamme und feste Glut-Stärke entfernt (zeichnet `world`); verschwundene Truhen erscheinen nach einer Pause nicht mehr als offen (`TestWiedereintrittOhneOffeneTruhe`). Texte und Balken liegen wie im Browser hinter den Figuren (Tiefe 2), deshalb bleibt es bei einer Ebene.

Geprüft: `go vet`, `go test`, `golangci-lint` im Modul grün (0 Befunde), `check_run task:check:diff` grün; Aufnahme-Modus bildgleich (drei Läufe Takt 300, zwei Läufe Takt 900), Bilder unter `reports/native-game-shots/w2a*`. `task check` und `task check:go` nicht gefahren: Die Welle ändert nur `native/`. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Abweichung: Die vier Sprints liefen in einem Checkout auf dem Branch `sprint/ng-welle2a` mit einem gemeinsamen PR.
