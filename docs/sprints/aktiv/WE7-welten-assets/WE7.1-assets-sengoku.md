# WE7.1 · Assets Sengoku: Torii, Pagode, Ninja, Samurai, Samurai-Lager (B-988)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/we7
- **Abhängig von:** –
- **Tickets:** B-988
- **Kriterien:** AC-01

## Ziel

Die fünf Sengoku-Assets liegen als Papierschnitt-Specs vor und sind deckungsgleich mit ihren Basis-Assets (B-988).

## Kontext

Papierschnitt nach Entscheidung 005 (`docs/decisions/`): Assets entstehen aus Specs `public/grafik/<gruppe>/<asset>.paper.json`, nur über die MCP-Tools `assets_guide`, `assets_style_get`, `assets_list`, `assets_get`, `assets_create`, `assets_set`, `assets_render`, `assets_preview`, `assets_review` der Workbench (muss laufen; im Worktree immer `checkout` setzen). Eingecheckt sind Specs und `index.paper.json`, PNG und `manifest.json` nicht (B-737). Basis-Assets: `objekte/ausgang`, `objekte/portal`, `gegner/wolf`, `gegner/hoehlentroll`, der Turm in `gebaeude-paper`. Ein Skin-Asset braucht dieselben Zustände, Animationen, Ankerpunkte und etwa dieselbe Größe wie die Basis (das Spiel tauscht nur den Pfad, B-994).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `public/grafik/gegner/**`
- `public/grafik/gebaeude-paper/**`

## Nicht-Ziele

`data/paper.json` oder Weltdateien ändern, `gen:paper` und Grafik-Golden (Abnahme-Ticket), Code.

## Schritte

1. `assets_guide`, `assets_style_get` lesen; je Basis-Asset `assets_get` (Zustände, Animationen, Anker notieren).
2. Je Asset `assets_create` (`objekte/torii`, `gebaeude-paper/pagode`, `gegner/ninja`, `gegner/samurai`, `objekte/samurai-lager`), dann `assets_set` bis `assets_preview` passt.
3. Deckungsgleichheit mit der Basis je Asset im Ergebnis notieren.
4. Agent `k3c-grafik` prüfen lassen, Befunde beheben; `assets_review` für 🧑 vorbereiten.

## Fertig, wenn

- [ ] AC-01: `assets_list` zeigt die fünf Assets (B-988/AC-01).
- [ ] AC-01: Deckungsgleichheit je Asset notiert (B-988/AC-02), `k3c-grafik` ohne schwere Befunde (B-988/AC-03).
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
