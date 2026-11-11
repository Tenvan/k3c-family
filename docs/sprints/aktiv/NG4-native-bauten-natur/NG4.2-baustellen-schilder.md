# NG4.2 · Baustellen, Gerüste, Schilder und Preis-Anzeigen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** bauten
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng4/2-baustellen-schilder
- **Abhängig von:** NG4.1
- **Tickets:** B-797
- **Kriterien:** AC-02

## Ziel

Baustellen, Gerüste, Bau-Schilder, Preis-Anzeigen, Minen-Schilder, Wimpel und Platz-Status erscheinen wie im Browser.

## Kontext

Spec: B-797. Vorbild: `src/scenes/render/siteView.ts`, `siteStatusView.ts`, `scaffoldView.ts`, `buildSignView.ts`, `priceTagRules.ts`, `mineSignView.ts`, `pennant.ts`, `targetFit.ts` samt Tests; Plätze: `engine/sim/site_view.go`, `node_mark.go`. Texte mit der Schrift aus `art`. Vertrag und Importregel wie NG4.1.

## Erlaubte Dateien

- `native/k3c-game/props/**`
- Planung NG4, B-797 (über MCP)

## Nicht-Ziele

HUD-Texte (NG6); neue Grafik.

## Schritte

1. Fälle aus den TS-Tests zu `priceTagRules` und `siteView` als Go-Tests (rot).
2. Baustellen und Gerüste.
3. Schilder, Preis-Anzeigen, Wimpel, Platz-Status.

## Fertig, wenn

- [ ] AC-02: Tests grün; Screenshot einer Baustelle mit Preis-Anzeige im Ergebnis.
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
