# MG4.3 · Fledermaus, Rattenschwarm, Grubengeist, Lavaschleim

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg4/3-kreaturen-a
- **Abhängig von:** MG4.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Fledermaus (`fledermaus`), Rattenschwarm (`rattenschwarm`), Grubengeist (`grubengeist`) und Lavaschleim (`lavaschleim`) als `requisite`-Specs mit Keyframe-Animationen `idle`, `lauf`, `angriff` unter `public/grafik/gegner/`.

## Kontext

- `bat`, `ratSwarm`, `mineGhost`, `lavaSlime` aus `data/enemies.json`; die alten Sheets sind teils Platzhalter (Feuerschädel, Pilz) – gezeichnet wird, was der Name sagt.
- `requisite` bewegt Teile nur per Keyframes (Drehung um den Pivot); Wippen über ein Wurzel-Teil mit tiefem Pivot.
- Keine Rot-/Orange-Materialien für Lava außer `koenigsrot`, `fell-fuchs`, `gold` (kein neues Material, D-Zeile).

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG4-Zeilen)

## Nicht-Ziele

Keine neuen Rig-Typen, kein `assets_style_set`.

## Schritte

1. `assets_guide {rig: requisite}`.
2. Je Gegner `assets_create {rig: requisite}` und `assets_set`.
3. Jede Animation mit `assets_preview` prüfen.
4. `task gen:paper`, `assets_list`, `assets_review`, `tools/paper`-Tests.
5. Ergebnis, Commit `feat(pap): Gegner Kreaturen A (MG4.3)`.

## Fertig, wenn

- [ ] AC-01: die vier Assets gültig.
- [ ] AC-02: Golden eingetragen, `tools/paper` grün.
- [ ] AC-03: jede Animation geprüft; Review ohne roten Eintrag.

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
