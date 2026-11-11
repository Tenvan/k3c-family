# NG6.3 · Aktions-Hinweise, Glyphen, Anleitung und Skill-Menü

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hud
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng6/3-aktions-hinweise-skillmenue
- **Abhängig von:** NG6.2
- **Tickets:** B-799
- **Kriterien:** AC-03

## Ziel

Aktions-Hinweise mit Xbox-Glyphen, die Anleitung und das Skill-Menü erscheinen wie im Browser.

## Kontext

Spec: B-799. Vorbild: `src/scenes/hints/actionHints.ts`, `actionOverlay.ts`, `glyphs.ts`, `glyphView.ts`, `guideHints.ts`, `guideOverlay.ts`, `src/scenes/hud/skillMenuLogic.ts`, `skillMenuText.ts`, `skillMenuView.ts` samt Tests; Skills in `engine/sim/skills*.go`. Glyphen nur Xbox. Vertrag und Importregel wie NG6.1.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- Planung NG6, B-799 (über MCP)

## Nicht-Ziele

Eingabe (NG5); neue Hinweise.

## Schritte

1. Fälle aus den TS-Tests als Go-Tests (rot).
2. Aktions-Hinweise und Glyphen.
3. Anleitung und Skill-Menü.

## Fertig, wenn

- [ ] AC-03: Tests grün; Screenshot mit Hinweis und Skill-Menü im Ergebnis.
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

AC-03 umgesetzt: Aktions-Hinweise wie `playerHint`, Anleitung (Münze, Nacht, Bauer), Glyphen je Gerät, Skill-Menü als `view.Overlay` je Spieler mit Lernen und Respec über die Session. Geprüft mit `hints_test.go`, `skillmenu_test.go`, `overlay_test.go` (Fälle aus `skillMenuLogic.test.ts` mit zwei Spielern) und an Bildern `ng6-guide`, `ng6-night`.
Offen: Skill-Menü nicht im Bild (Eingabe-Folge kennt keine Menü-Aktionen); Hinweistext für die rechte Tastatur-Hälfte falsch; Debug-Schrift zeigt einige Zeichen nicht.
