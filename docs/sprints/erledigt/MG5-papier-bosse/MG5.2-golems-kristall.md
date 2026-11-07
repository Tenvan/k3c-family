# MG5.2 · lava-golem, splitter-titan, kristallherz-waechter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** bosse
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg5/2-golems-kristall
- **Abhängig von:** MG5.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`lava-golem` und `splitter-titan` (Rig `figur`) sowie `kristallherz-waechter` (Rig `requisite` mit Keyframes) sind über die Asset-MCP-Tools angelegt und gerendert (Gruppe `bosse`, `hoehe_px` 288).

## Kontext

- Quelle: `data/bosses.json` › `bosses` (lavaGolem, shardTitan) und › `endBosses` (crystalHeartWarden, Endboss, `speed` 0 = steht).
- Golem und Titan sind menschenähnlich und laufen: Rig `figur` vom Krieger ohne Waffe und Schild, wuchtige Lagen. Abweichung von B-701 D-08 („Golem als `requisite`“) als D-MG5-03.
- Kristallherz-Wächter bewegt sich nicht: `requisite` mit Teilen `korpus` (Herz) und Splittern, Animationen `idle` (Pulsieren über Keyframe-Winkel) und `angriff` (Splitter schwingen aus).
- Farben (D-MG5-02): lava-golem `stein` mit `koenigsrot`/`gold`-Glut; splitter-titan `stein` mit Kristallen `purpur`/`edelstein`; Herz `purpur` mit `edelstein`-Punkten.
- Werkzeuge mit `checkout`; `task gen:paper` (B-707), fremde Gruppen zurücksetzen.

## Erlaubte Dateien

- `public/grafik/bosse/**`
- `docs/sprints/*/MG5-papier-bosse/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (Zeilen D-MG5-xx)

## Nicht-Ziele

Kein Code, kein neuer Rig-Typ, kein `assets_style_set`.

## Schritte

1. Je Boss `assets_create`, `assets_set`, `assets_preview` je Animation (Frame 0 und 4).
2. `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`, `assets_list {gruppe: bosse}`.
3. `task check:diff`, Commit, Push nach `sprint/mg5`.

## Fertig, wenn

- [ ] AC-01: drei Bosse gültig.
- [ ] AC-02: `task gen:paper` grün, Golden eingetragen.
- [ ] AC-03: jede Animation per `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne `ungueltig`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- `lava-golem` (figur): Steinkörper mit koenigsroter Glut in Fugen und Augen, Schulterblöcke, ohne Waffe, Schild und Helm; idle, lauf, angriff (Faustschlag).
- `splitter-titan` (figur): gleicher Steinkörper mit purpurner Glut, Teil `kristalle` an der Schulter, purpurne Kristallklinge; idle, lauf, angriff.
- `kristallherz-waechter` (requisite): purpurnes Herz mit Riss und Kern auf Steinsockel, drei Splitter als Kinder von `korpus`; idle (Splitter pendeln), angriff (Splitter spreizen und schlagen zurück). Rahmen [-82,-134,164,136], 347×288 px.
- Alle drei mit `hoehe_px` 288 (D-MG5-01); Rig-Wahl in D-MG5-03.
- Prüfung: assets_preview je Asset und Animation (8 Previews), nichts ragt aus dem Rahmen; assets_list `bosse` 6/6 gültig; `task gen:paper` grün, fremde Gruppen zurückgesetzt.
- `task check:diff`: 3 Workbench-Frontend-Tests rot unter Parallellast; `k3c-workbench:vitest` einzeln 432/432 grün, `task test` grün (Wackel-Tests wie D-MG2-07).
- Offen für PS1: Golems teilen die Krieger-Silhouette; ob sie als Bosse lesbar genug sind, entscheidet die Sichtprüfung.
