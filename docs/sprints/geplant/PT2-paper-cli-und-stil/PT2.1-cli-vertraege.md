# PT2.1 · Paper-CLI: Fehlerpfad, Reittier-Boxen, Option --stil

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt2/1-cli-vertraege
- **Abhängig von:** –
- **Tickets:** B-708, B-709, B-705
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Die Paper-CLI nennt den vollen Fehlerpfad bei fehlendem Reittier, liefert Reittier-Boxen in `frame` und nimmt `--stil <datei>` für `frame` und `render`.

## Kontext

B-708: Ablehnung in `tools/paper/rig/pruefen.ts` bzw. `rig/reiten.ts` meldet `animationen.reittier` statt `animationen.<name>.reittier`; Test in `tools/paper/bin/paper.test.ts`. B-709 (CLI-Teil): `paper frame monarch --animation galopp` liefert Boxen nur für den Reiter; gewünscht je Reittier-Teil eine Box mit Präfix `reittier:` (auch verdeckte Teile wie `oberschenkel-h`). B-705 (CLI-Teil): `tools/paper/render/stil.ts` liest `STIL_PFAD` fest aus der Wurzel; neue Option `--stil <datei>` für `frame` und `render`, Vertrag `docs/standards/paper-spec.md` › 8. Der Workbench-Teil (Review übergibt den Stil des Schnappschusses, `assets_guide` rundet) folgt in WZ43.

## Erlaubte Dateien

- `tools/paper/rig/**`
- `tools/paper/cli/frame*.ts`
- `tools/paper/bin/paper.ts`
- `tools/paper/bin/paper.test.ts`
- `tools/paper/render/stil*.ts`
- `docs/standards/paper-spec.md`

## Nicht-Ziele

Workbench-Code (`tools/k3c-workbench/**`), Specs und Golden.

## Schritte

1. B-708: Pfad mit Animationsnamen bilden, Test.
2. B-709: Reittier-Teile in `frame` als Boxen `reittier:<teil>`, Test.
3. B-705: `--stil` durch `frame` und `render` reichen, Standard unverändert, Fehler bei fehlender Datei; Vertrag nachziehen; Test.
4. Ergebnis je Kriterium; B-708 auf `erledigt` (B-705 und B-709 schließt WZ43).

## Fertig, wenn

- [ ] AC-01: `B-708/AC-01` grün.
- [ ] AC-02: `B-709/AC-01` grün.
- [ ] AC-03: `B-705/AC-01` grün, Vertrag nennt `--stil`.
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
