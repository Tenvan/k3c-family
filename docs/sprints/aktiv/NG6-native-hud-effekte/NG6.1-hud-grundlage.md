# NG6.1 · HUD-Grundlage: Layout, Schrift, Gold, Vorrat, Banner

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hud
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng6/1-hud-grundlage
- **Abhängig von:** NG1
- **Tickets:** B-799
- **Kriterien:** AC-01

## Ziel

Paket `hud` zeigt je Zelle Gold, Vorrat, HP und die Banner-Warteschlange mit Schriftgrößen und Kontur nach den heutigen Regeln.

## Kontext

Spec: B-799. Modul `native/k3c-game` (NG1): Vertrag `view/`, Platzhalter in `hud/` und `fx/` ersetzen, `game/` und `view/` unverändert; Importregel nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Schrift aus `art` (NG1.3). Vorbild: `src/scenes/hud/hudLayout.ts`, `hudElements.ts`, `goldFlash.ts`, `bannerQueue.ts`, `economyLabels.ts`, `src/scenes/render/fontRules.ts` samt Tests; Regel „Gold nur im HUD“ (`src/scenes/goldOnlyInHud.test.ts`), Textregel (`textRule.test.ts`). Superpowers TDD.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- Planung NG6, B-799 (über MCP)

## Nicht-Ziele

Radar und Hinweise (NG6.2, NG6.3), Effekte (NG6.4); neue Texte.

## Schritte

1. Fälle aus den TS-Tests zu `hudLayout`, `fontRules`, `bannerQueue` als Go-Tests (rot).
2. Layout je Zelle (1 und 2 Spieler), Schriftrollen.
3. Gold, Vorrat, HP, Banner.

## Fertig, wenn

- [ ] AC-01: Tests grün; Screenshot HUD bei 2 Spielern im Ergebnis.
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
