# NA1.7 · Review NA1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.6
- **Tickets:** B-1004, B-1005, B-912, B-1006, B-911, B-885
- **Kriterien:** alle

## Ziel

Der Sprint NA1 ist geprüft: alle Kriterien nachgewiesen, Befunde behoben oder als Ticket, Abnahme-Ticket angelegt.

## Kontext

Sprint-README `docs/sprints/geplant/NA1-nachtraege-natives-spiel/README.md` (nach dem Aktivieren unter `aktiv/`). Sessions NA1.1 bis NA1.6: Audio-Tests und Lint, `TestFlow`, Steg-Eingabe, Tasten-Icons (Grafik und HUD), Rig-Export. Ablauf der Review-Session: `docs/arbeitsweise.md` › Review-Session. Fachprüfer: `k3c-hud` (HUD, Eingabe), `k3c-grafik` (Icons, Rig-Export), `k3c-reviewer` (Rest). AC-08 (Sichtprüfung) bleibt bei 🧑 in NA1.8.

## Erlaubte Dateien

- `native/k3c-game/audio/**`
- `native/k3c-game/menu/licensefile/**`
- `native/k3c-game/e2e/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/game/**`
- `public/grafik/icons/**`
- `data/paper.json`
- `native/k3c-rig/assets/**`

## Nicht-Ziele

Neue Funktionen; Simulation und Balancing (gesperrt); manuelle Prüfung am PC (NA1.8).

## Schritte

1. Diff des Sprints gegen den Start-Commit lesen, je Kriterium AC-01 bis AC-07 den Nachweis aus den Session-Ergebnissen prüfen.
2. Fachprüfer laufen lassen; Fixes unter etwa 30 Zeilen direkt, sonst Ticket (Projekt NAT).
3. `task check`, danach `task check:go`; im Modul `native/k3c-game`: `go test ./...` (ohne `-short`, damit `TestFlow` läuft) und `golangci-lint run ./...`.
4. Abnahme-Ticket des Sprints anlegen (Gen: `task test` samt `gen:paper` und Golden, `check:race`; keine Sim, kein Balancing).
5. Abnahme in der Sprint-README (höchstens fünf Zeilen).

## Fertig, wenn

- [ ] AC-01 bis AC-07 je mit Nachweis im Ergebnis.
- [ ] `task check`, `task check:go` und `go test ./...` in `native/k3c-game` grün.
- [ ] Abnahme-Ticket angelegt, Abnahme in der README.

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
