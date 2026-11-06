# MG4.1 · Goblins und Untote als figur

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg4/1-goblins-untote
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Goblin (`goblin`), Gier (`gier`), Goblin-Bogenschütze (`goblin-bogenschuetze`), Skelett (`skelett`) und Zombie (`zombie`) liegen als `figur`-Specs unter `public/grafik/gegner/`, mit `idle`, `lauf` und `angriff`, nur über die Asset-MCP-Tools angelegt und mit `assets_preview` geprüft.

## Kontext

- Gegner aus `data/enemies.json` (`goblin`, `greed`, `goblinArcher`, `skeleton`, `zombie`); alte Sheets in `data/sprites.json` › `enemies` (idle/run/attack).
- Vorgehen wie MG3 (Truppen): Körper nach dem Monarchen, Waffe um die Hand gedreht, `angriff` mit Keyframes (fps 14, 8 Frames), Rahmen nach Waffenreichweite (D-MG3-02). Goblins kleiner und gedrungen (grüne Haut über `smaragd`), Skelett und Zombie mit eigenen Körperformen.
- Rig `figur` kennt ohne Keyframes nur `idle` und `lauf`; nach den Änderungen `task gen:paper`, fremde Gruppen zurücksetzen (D-MG3-06).

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG4-Zeilen)

## Nicht-Ziele

Kein `assets_style_set`, keine Änderung an Renderer, Workbench oder anderen Gruppen.

## Schritte

1. `assets_guide {rig: figur}`; Baukasten aus MG3 für Gegner erweitern (Probe-Renders im Scratch-Ordner).
2. Je Gegner `assets_create {gruppe: gegner, rig: figur}` und `assets_set` mit Teilen, `idle`, `lauf`, `angriff`, Ausgabe.
3. Jede Animation mit `assets_preview` prüfen, Befunde notieren.
4. `task gen:paper`, `assets_list {gruppe: gegner}`, `assets_review`, `bunx vitest run tools/paper`.
5. Ergebnis, Commit `feat(pap): Gegner Goblins und Untote (MG4.1)`.

## Fertig, wenn

- [x] AC-01: `assets_list {gruppe: gegner}` meldet die fünf Assets gültig.
- [x] AC-02: `golden.json` der Gruppe eingetragen, `tools/paper`-Tests grün.
- [x] AC-03: jede Animation mit `assets_preview` geprüft; `assets_review` ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list {gruppe: gegner}` meldet goblin, gier, goblin-bogenschuetze, skelett, zombie gültig (Rig `figur`).
- AC-02 geprüft: `task gen:paper` schreibt `public/grafik/gegner/golden.json` und `index.paper.json`; fremde Gruppen (figuren, reittiere) danach zurückgesetzt; `bunx vitest run tools/paper tests/planning.test.ts` 1110/1110 grün.
- AC-03 geprüft: idle, lauf und angriff je Gegner mit `assets_preview` angesehen; `assets_review` gegen HEAD: fünf Einträge „neu“, kein roter.
- Abweichungen: Zombie trägt die Arme auch in `idle`/`lauf` per Keyframe nach vorn (Pose plus Keyframe); Goblins kahl mit spitzen Ohren in `smaragd`, Gier in `purpur` mit Goldsack, Bogenschütze mit Kapuze `fell-fuchs`. Rahmen je Waffenreichweite (130–160 px breit).
- `task check:diff` nicht einzeln gefahren (Diff zu develop umfasst ganz feature/pap, Laufzeit > 10 min); Gate sind die gezielten Tests, `task check` folgt in MG4.5.
- Manuell geprüft: niemand. Neue Tickets: keine.
