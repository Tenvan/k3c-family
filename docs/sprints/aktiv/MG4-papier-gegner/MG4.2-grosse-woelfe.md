# MG4.2 · Höhlentroll, Kristallwächter und Wölfe

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg4/2-grosse-woelfe
- **Abhängig von:** MG4.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Höhlentroll (`hoehlentroll`) und Kristallwächter (`kristallwaechter`) als große `figur`-Specs, Wolf (`wolf`) und Alphawolf (`alphawolf`) als `vierbeiner` unter `public/grafik/gegner/`, gerendert und geprüft.

## Kontext

- `caveTroll` (altes Sheet: Höllenkatze als Platzhalter) wird ein Troll, `crystalGuardian` ein Kristallgolem; beide `figur` mit Keyframe-Angriff, größerer Rahmen.
- Wolf und Alphawolf als `vierbeiner` nach dem Pferd (`public/grafik/reittiere/pferd.paper.json`): Beinpfad aus `bein`-Breiten, Bewegung nur über Gangarten (`schritt`, `trab`, `galopp`), der Renderer kennt für Vierbeiner keine Keyframes (D-MG4-Zeile).
- B-701 D-08: Spinne und Golem als `requisite` mit Keyframes; hier abweichend Golem als `figur` (menschenähnlich), D-Zeile.

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG4-Zeilen)

## Nicht-Ziele

Keine neuen Rig-Typen, kein `assets_style_set`.

## Schritte

1. Ergebnis MG4.1 lesen; Pferd per `assets_get` als Vorlage für die Wölfe.
2. Je Gegner `assets_create` und `assets_set`.
3. Jede Animation mit `assets_preview` prüfen.
4. `task gen:paper`, `assets_list`, `assets_review`, `tools/paper`-Tests.
5. Ergebnis, Commit `feat(pap): Gegner Troll, Wächter, Wölfe (MG4.2)`.

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
