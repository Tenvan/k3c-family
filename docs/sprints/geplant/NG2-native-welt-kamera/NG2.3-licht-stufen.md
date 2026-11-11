# NG2.3 · Licht bei Tag und Nacht und Stufenübergang

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** welt
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng2/3-licht-stufen
- **Abhängig von:** NG2.2
- **Tickets:** B-795
- **Kriterien:** AC-03

## Ziel

Licht bei Tag und Nacht und der Übergang beim Stufen- und Inselwechsel folgen dem Zustand der Simulation.

## Kontext

Spec: B-795. Vorbild: `src/scenes/render/nightLight.ts`, `lightView.ts`, `stageView.ts`, `src/scenes/cellStages.ts`; Regeln der Stufen: `docs/rules/stufen.md`. Tageszeit und Stufe stehen in `*sim.World` (Frame). Vertrag und Importregel wie NG2.1.

## Erlaubte Dateien

- `native/k3c-game/world/**`
- Planung NG2, B-795 (über MCP)

## Nicht-Ziele

Licht-Shader (B-790 nur Fluss); Regeln ändern.

## Schritte

1. Test zuerst: Licht-Stärke je Tageszeit wie `nightLight.ts`; Übergang beim Stufenwechsel.
2. Licht zeichnen (Abdunkeln, Lichtquellen wie `lightView.ts`).
3. Übergang Stufen- und Inselwechsel.

## Fertig, wenn

- [ ] AC-03: Tests grün; Screenshot Tag und Nacht im Ergebnis.
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
