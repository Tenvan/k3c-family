# NG3.4 · Messung Rig gegen Sheet mit vielen Gegnern und Empfehlung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** ng3/4-messung-gegnerzahl
- **Abhängig von:** NG3.3
- **Tickets:** B-796, B-780
- **Kriterien:** AC-04

## Ziel

Ein Messbericht zeigt Rig gegen Sheet mit 2, 50 und 200 Gegnern am Entwicklungs-PC, dazu eine Empfehlung zur Reihenfolge der Umstellung.

## Kontext

Spec: B-796/AC-03, B-780 (AC-01 Teil Entwicklungs-PC, AC-02 Empfehlung). Kein `k3c-load`, kein `sim_test` (Sperre 2026-11-11): Messmodus im Spiel-Modul mit fester Szene (Gegner per Testaufbau in `actors`, ohne Simulation zu verändern), je Fassung ein Lauf. Vorbild Messung: `native/k3c-rig/measure.go`, `native/k3c-ebiten/measure.go`, `native/measure.ps1`. Vergleichswert B-778: 200 Reiter Rig 126 FPS, Sheet 981 FPS (Entwicklungs-PC). Maßstab aus der Entscheidung 🧑 zu B-780 (Default 60 FPS bei 2 Spielern).

## Erlaubte Dateien

- `native/k3c-game/actors/**`
- `reports/native-actors-*.json`
- Planung NG3, B-796, B-780 (über MCP)

## Nicht-Ziele

Messung am Mini-PC (NG9.5); Balancing-Werte; Mischbetrieb einbauen (nur empfehlen).

## Schritte

1. Messmodus `--measure-actors` (2, 50, 200; Rig und Sheet) mit Ausgabe JSON nach `reports/`.
2. Je Fassung ein Lauf.
3. Empfehlung (Reihenfolge, ggf. Mischbetrieb) in B-780 › Notizen; Entscheidung 🧑 bleibt offen.

## Fertig, wenn

- [ ] AC-04: Bericht unter `reports/`, Werte und Empfehlung in B-780 › Notizen.
- [ ] `task check:diff` grün.

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
