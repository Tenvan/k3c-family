# NG2.3 · Licht bei Tag und Nacht und Stufenübergang

- **Status:** fertig
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

AC-03 umgesetzt: Tag-Nacht-Licht aus `data/light.json` als Lichtkarte über der Zelle, Lagerfeuer und Fackeln als Lichtquellen, Aufblenden 0,6 s nach Stufen- oder Inselwechsel. Geprüft mit `light_test.go` (Werte wie `nightLight.test.ts`, Fackeln gleich `sim.TorchXs`) und am Nachtbild `reports/native-game-shots/ng2-nacht/shot-01.png` (Agent).
Abweichungen: Lava leuchtet nicht (keine Lichtwerte in den Daten); der Lichtschein ist nur im Test belegt, im geprüften Ausschnitt stand kein Licht.
