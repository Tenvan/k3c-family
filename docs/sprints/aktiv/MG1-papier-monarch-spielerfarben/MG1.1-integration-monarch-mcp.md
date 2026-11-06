# MG1.1 · Integration: Monarch durchgängig über MCP

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** Sonnet
- **Umgebung:** lokal (Worktree, Workbench-MCP)
- **Branch:** mg1/1-integration-monarch-mcp
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Monarch (`public/grafik/figuren/monarch.paper.json`, Rig `figur`) ist durchgängig nur über die Asset-MCP-Tools geprüft, geändert und gerendert: alle Animationen (idle, lauf, schritt, trab, galopp mit Reiten) sind im Preview angesehen, Befunde zu Renderer und Tools liegen als Tickets im Projekt PAP. Das Ergebnis entscheidet über den Start von MG2–MG8.

## Kontext

- Monarch und Pferd liegen seit RD1 unter `public/grafik/figuren/` bzw. `reittiere/` (gerendert, `golden.json`).
- Werkzeuge (MC1): `assets_guide`, `assets_get`, `assets_set` (JSON-Merge-Patch, atomar), `assets_preview`, `assets_render`, `assets_review`, `assets_list`; immer mit `checkout` = Worktree.
- Vertrag: `docs/standards/paper-spec.md`, Entscheidung 005 › 2–4 (005/AC-05: erstes Asset nur über MCP).
- Fallstrick: Reit-Animationen brauchen das Pferd im Nachbar-Ordner `reittiere/` (D-RD1-04/05).

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `docs/sprints/aktiv/MG1-papier-monarch-spielerfarben/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-Zeilen), neue Tickets über `plan_create`

## Nicht-Ziele

Kein Code im Renderer oder in der Workbench; kein `assets_style_set`; keine Spielerfarben (MG1.2).

## Schritte

1. `assets_guide {rig: figur}` lesen, `assets_list` und `assets_get monarch` prüfen.
2. Jede Animation Frame 0 und einen Mittel-Frame mit `assets_preview` ansehen; Befunde notieren.
3. Eine kleine echte Änderung per `assets_set` (Beschreibung bzw. Version) mit Preview gegen `basis HEAD` und `assets_review`.
4. Befunde zu Renderer/Tools als Tickets (Projekt PAP) anlegen.
5. `task gen:paper` bzw. `assets_render` und Golden prüfen, `check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `assets_list` meldet monarch gültig.
- [ ] AC-02: `golden.json` der Gruppe passt zur Ausgabe (`task test -- paper` grün).
- [ ] AC-03: alle Animationen mit `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

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
