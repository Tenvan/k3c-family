# MG7.1 · Hintergründe Wald und Höhle

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hintergruende
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg7/1-wald-hoehle
- **Abhängig von:** –
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Hintergründe Wald (Tannenhang) und Höhle (Tropfsteinhalle) als Rig `hintergrund`: `hg-wald-<fern|mitte|nah>-v1/v2`, `hg-hoehle-<fern|mitte|nah>-v1/v2`, gerendert, Golden eingetragen.

## Kontext

Sprint-README (Anforderungen, D-MG7-01 bis D-MG7-04); Motive aus `public/grafik/gebaeude/hg-wald-*.json` und `hg-hoehle-*.json` (Feld `vorlage`). Wald: `smaragd`, `holz`; Höhle: `stein`, Moos `smaragd`.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `docs/sprints/*/MG7-papier-hintergruende/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine anderen Biome, kein Laden im Spiel.

## Schritte

1. `assets_guide {rig: hintergrund}`, Entwurf mit Generator im Scratchpad, Probe-Render mit der CLI.
2. `assets_create` und `assets_set` je Asset.
3. `assets_preview` je Asset, `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`.

## Fertig, wenn

- [ ] 12 Assets gültig und gerendert, jedes per Preview geprüft.
- [ ] Golden und Index der Gruppe aktuell.
- [ ] Ergebnis mit Nachweis je Kriterium.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
