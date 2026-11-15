# WE7.2 · Assets Nil: Palme, Barke, Obelisk, Skorpion, Mumie (B-989)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/we7
- **Abhängig von:** WE7.1
- **Tickets:** B-989
- **Kriterien:** AC-02

## Ziel

Die fünf Nil-Assets liegen als Papierschnitt-Specs vor und sind deckungsgleich mit ihren Basis-Assets (B-989).

## Kontext

Wie WE7.1: Papierschnitt nach Entscheidung 005, nur über `assets_*` der Workbench (im Worktree `checkout` setzen), Specs und `index.paper.json` eingecheckt, PNG nicht (B-737). Basis-Assets: `objekte/tanne`, `objekte/birke`, `objekte/pappel` (Palme ersetzt alle drei), `objekte/felsen` (Obelisk), `gegner/wolf` (Skorpion), `gegner/zombie` (Mumie), das Schiff (Barke; Gruppe und Baustufen zuerst mit `assets_list` klären). Skin-Assets brauchen dieselben Zustände, Animationen, Anker und etwa dieselbe Größe wie die Basis (B-994).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `public/grafik/gegner/**`
- `public/grafik/gebaeude-paper/**`

## Nicht-Ziele

`data/paper.json` oder Weltdateien ändern, `gen:paper` und Grafik-Golden, Code.

## Schritte

1. Basis-Assets mit `assets_get` lesen; Schiff-Asset und seine Stufen finden.
2. `assets_create` für `objekte/palme`, `objekte/obelisk`, `gegner/skorpion`, `gegner/mumie` und die Barke (in der Gruppe des Schiffs); `assets_set` bis `assets_preview` passt.
3. Liegt das Schiff außerhalb der erlaubten Gruppen oder braucht die Barke mehr als diese Session: Ticket anlegen, im Ergebnis melden.
4. Deckungsgleichheit notieren, `k3c-grafik` prüfen lassen.

## Fertig, wenn

- [ ] AC-02: `assets_list` zeigt die fünf Assets (B-989/AC-01) oder das Folgeticket für die Barke ist genannt.
- [ ] AC-02: Deckungsgleichheit notiert (B-989/AC-02), `k3c-grafik` ohne schwere Befunde (B-989/AC-03).
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
