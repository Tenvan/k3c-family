# MG4.3 · Fledermaus, Rattenschwarm, Grubengeist, Lavaschleim

- **Status:** fertig
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

- [x] AC-01: vier Assets gültig.
- [x] AC-02: `golden.json` eingetragen, `tools/paper`-Tests grün.
- [x] AC-03: jede Animation mit `assets_preview` geprüft; `assets_review` ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list` meldet fledermaus, rattenschwarm, grubengeist, lavaschleim (requisite) gültig.
- AC-02 geprüft: `task gen:paper` trägt sie in `public/grafik/gegner/golden.json` ein; fremde Gruppen zurückgesetzt; `bunx vitest run tools/paper tests/planning.test.ts` 1110/1110 grün.
- AC-03 geprüft: idle, lauf, angriff je Kreatur mit `assets_preview` angesehen; `assets_review` ohne roten Eintrag.
- Umsetzung: Fledermaus mit zwei Flügel-Teilen (Flügelschlag, Sturzflug im Angriff), Rattenschwarm als drei Ratten-Teile mit versetztem Wippen (Angriff: vordere Ratte bäumt sich auf), Grubengeist mit Helm und Grubenlampe, pendelt am Kopf-Pivot, Arme greifen nach vorn; Lavaschleim aus `koenigsrot`/`gold`/`fell-fuchs` (kein neues Material), Zunge liegt in idle/lauf verdeckt hinter dem Körper. Größe 1,06 px je Einheit wie die Figuren.
- Abweichung: ein `assets_set` brach einmal mit EPERM beim Umbenennen im Render-Cache ab (zwei Aufrufe parallel); einzeln wiederholt, ohne Folgen. D-Zeilen trägt MG4.4 gesammelt ein.
- Manuell geprüft: niemand. Neue Tickets: keine.
