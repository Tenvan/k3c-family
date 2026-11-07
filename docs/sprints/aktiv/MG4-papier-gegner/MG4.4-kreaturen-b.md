# MG4.4 · Eisenkäfer, Feuergeist, Kristallspinne, Splitterling

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg4/4-kreaturen-b
- **Abhängig von:** MG4.3
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Eisenkäfer (`eisenkaefer`), Feuergeist (`feuergeist`), Kristallspinne (`kristallspinne`) und Splitterling (`splitterling`) als `requisite`-Specs mit Keyframe-Animationen `idle`, `lauf`, `angriff` unter `public/grafik/gegner/`.

## Kontext

- `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling` aus `data/enemies.json`; Spinne als `requisite` mit Keyframes (B-701 D-08), Beine als eigene Teile.
- Aufbau wie MG4.3.

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG4-Zeilen)

## Nicht-Ziele

Keine neuen Rig-Typen, kein `assets_style_set`.

## Schritte

1. Ergebnis MG4.3 lesen.
2. Je Gegner `assets_create {rig: requisite}` und `assets_set`.
3. Jede Animation mit `assets_preview` prüfen.
4. `task gen:paper`, `assets_list`, `assets_review`, `tools/paper`-Tests.
5. Ergebnis, Commit `feat(pap): Gegner Kreaturen B (MG4.4)`.

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

- AC-01 geprüft: `assets_list {gruppe: gegner}` meldet eisenkaefer, feuergeist, kristallspinne, splitterling (requisite) gültig, die Gruppe hat 17 gültige Gegner.
- AC-02 geprüft: `task gen:paper` trägt sie in `public/grafik/gegner/golden.json` ein; fremde Gruppen zurückgesetzt; `bunx vitest run tools/paper tests/planning.test.ts` 1110/1110 grün.
- AC-03 geprüft: alle Animationen mit `assets_preview` angesehen. Befunde behoben: Kristallspinne hob im Angriff die Hinterbeine (bein-1 ist hinten) → jetzt bein-3/-4; Rahmen der Spinne auf `[-46,-50,92,56]` verbreitert. Feuergeist trug die Feuerball-Hand dauerhaft wie eine Pfeife → Hand liegt in idle/lauf hinter dem Körper.
- B-701: D-MG4-01 bis -06 eingetragen (Rigs, Vierbeiner-Animationen, Größe, Platzhalter-Sheets, Farben, Teile nur im Angriff).
- Manuell geprüft: niemand. Neue Tickets: keine.
