# MG4.1 · Goblins und Untote als figur

- **Status:** offen
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

- [ ] AC-01: `assets_list {gruppe: gegner}` meldet die fünf Assets gültig.
- [ ] AC-02: `golden.json` der Gruppe eingetragen, `tools/paper`-Tests grün.
- [ ] AC-03: jede Animation mit `assets_preview` geprüft; `assets_review` ohne roten Eintrag.

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
