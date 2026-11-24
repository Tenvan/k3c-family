# GS4.3 · Asset: Flaschenpost (B-980)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.6
- **Tickets:** B-980
- **Kriterien:** AC-05

## Ziel

Das Papierschnitt-Asset „Flaschenpost“ liegt vor und ist in `data/paper.json` zugeordnet.

## Kontext

Spec: `docs/backlog/B-980-flaschenpost-im-spiel.md` (R1, AC-01). Asset **nur über `assets_*`** (Entscheidung 005; `assets_guide` zuerst), laufende Workbench nötig. Gruppe `objekte` (`public/grafik/objekte/`, Muster: kleine Level-Objekte wie `erzbrocken`). Die Flasche liegt am Strand der Stufe 1, ist klein, soll aber auffallen; das Glitzern zeichnet der Code (GS4.4), nicht das Asset. Zuordnung in `data/paper.json` bei den Objekten. Eingecheckt: Spec, `index.paper.json`, `golden.json`.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `data/paper.json`

## Nicht-Ziele

Code im Spiel, Pfeil und Leuchten (gezeichnet im Code), Ton.

## Schritte

1. `assets_guide`, `assets_list` (Gruppe `objekte`).
2. Asset „flaschenpost“ anlegen, rendern, Vorschau prüfen; Fachprüfung `k3c-grafik`.
3. Zuordnung in `data/paper.json`.
4. `gen:paper` samt Grafik-Golden der Gruppe, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-05: Asset über `assets_*` angelegt und zugeordnet, im Review sichtbar (B-980/AC-01).
- [ ] Grafik-Golden aktualisiert, `check_run task:check:diff` grün.

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
