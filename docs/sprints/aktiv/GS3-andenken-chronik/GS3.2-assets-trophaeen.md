# GS3.2 · Assets: Trophäen der vier Welten (B-966)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.7
- **Tickets:** B-966
- **Kriterien:** AC-02

## Ziel

Je Welt liegt ein Trophäen-Asset vor und ist in `data/paper.json` › `seekarte` › `trophaeen` zugeordnet.

## Kontext

Spec: `docs/backlog/B-966-chronik-der-reise.md` (R3, AC-04). Assets **nur über `assets_*`** (Entscheidung 005; `assets_guide` zuerst), laufende Workbench nötig. Motive (Default, 🧑 bestätigt bei der Freigabe): Standard Kristallsplitter, Sengoku Maske, Nil Skarabäus, Britannien Horn; jeweils klein, mit Fahne der Spieler, lesbar auf der Insel-Miniatur aus GS2 (`public/grafik/hintergruende/`, Zuordnung `data/paper.json` › `seekarte`). Gruppe `objekte`. Eingecheckt: Specs, `index.paper.json`, `golden.json`.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `data/paper.json`

## Nicht-Ziele

Code im Spiel, Trophäen je Endboss, neue Miniaturen.

## Schritte

1. `assets_guide`, `assets_list` (Gruppe `objekte`), vorhandene Seekarten-Assets ansehen.
2. Vier Trophäen anlegen, rendern, Vorschau prüfen; Fachprüfung `k3c-grafik`.
3. `data/paper.json` › `seekarte` › `trophaeen` (Welt-ID → Asset).
4. `gen:paper` samt Grafik-Golden der Gruppe, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-02: Vier Trophäen über `assets_*` angelegt und zugeordnet (B-966/AC-04).
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
