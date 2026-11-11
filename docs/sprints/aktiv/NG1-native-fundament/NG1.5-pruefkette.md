# NG1.5 · check:diff und CI kennen das Modul

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng1/5-pruefkette
- **Abhängig von:** NG1.4
- **Tickets:** B-794
- **Kriterien:** AC-08, AC-09

## Ziel

`task check:diff` und die CI prüfen `native/k3c-game` (vet, Lint, Tests).

## Kontext

Spec: B-794. `tools/checkdiff` (Stufen: Plan, vet der geänderten Pakete und Abnehmer, golangci-lint, `go test -short` der geänderten Pakete) kennt heute nur das Wurzel-Modul und eigene Module unter `tools/`; `native/` kommt dort nicht vor. CI: `.github/workflows/ci.yml` (Windows- und Linux-Jobs). Ebitengine nur unter Windows bauen (unter Linux vermutet cgo und X11 nötig). Shell- und Pfad-Code: Reviewer (Sonnet) parallel zu `task check:diff`.

## Erlaubte Dateien

- `tools/checkdiff/**`
- `.github/workflows/ci.yml`
- Planung NG1, B-794 (über MCP)

## Nicht-Ziele

Prüfung der Prototypen; neue Lint-Regeln; Änderungen am Spiel-Modul.

## Schritte

1. Test zuerst in `tools/checkdiff`: geänderte Datei unter `native/k3c-game/view` → Schritte vet, Lint, Test im Modul `native/k3c-game`.
2. Modul-Erkennung erweitern (eigenes `go.mod` unter `native/k3c-game`); Änderung in `engine/` zählt das native Modul als Abnehmer für vet.
3. CI: Windows-Schritt Build und `go test -short ./...` in `native/k3c-game`.

## Fertig, wenn

- [ ] AC-08: Test in `tools/checkdiff` grün; `task check:diff:plan` nennt die Schritte für eine Änderung in `native/k3c-game`.
- [ ] AC-09: `ci.yml` enthält den Windows-Schritt; Lauf auf dem Sprint-Branch grün oder im Ergebnis begründet.
- [ ] Review ohne schwere Befunde.

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
