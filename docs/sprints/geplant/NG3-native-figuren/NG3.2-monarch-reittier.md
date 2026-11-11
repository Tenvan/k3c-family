# NG3.2 · Monarch und Reittier aus dem Spielzustand

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng3/2-monarch-reittier
- **Abhängig von:** NG3.1
- **Tickets:** B-796
- **Kriterien:** AC-02

## Ziel

Paket `actors` zeichnet zwei Monarchen mit Reittier als Rig, mit Animation und Richtung aus dem Spielzustand.

## Kontext

Spec: B-796. Modul `native/k3c-game` (NG1): Vertrag `view/`, Platzhalter in `actors/` ersetzen, `game/` und `view/` unverändert; Importregel nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Rig-Bibliothek über `art` (NG1.3), Zeichnen über `native/k3c-rig/paint` (Actor, Painter). Vorbild Zustand → Animation: `src/scenes/render/monarchAnim.ts`, `mountGait.ts`, `mountView.ts` samt Tests; Monarch-Zustand in `engine/sim/monarch*.go`. Rig-Mathematik und Zuordnung: Opus.

## Erlaubte Dateien

- `native/k3c-game/actors/**`
- Planung NG3, B-796 (über MCP)

## Nicht-Ziele

Übrige Figuren (NG3.3); Waffen und Rüstungen tauschen; Effekte (NG6).

## Schritte

1. Fälle aus den TS-Tests zu `monarchAnim` und `mountGait` als Go-Tests (rot).
2. Zuordnung Zustand → Animation, Phase, Richtung.
3. Zeichnen über `paint`, zwei Monarchen unabhängig.

## Fertig, wenn

- [ ] AC-02: Tests grün; Screenshot zweier Monarchen in verschiedenen Gangarten im Ergebnis.
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
