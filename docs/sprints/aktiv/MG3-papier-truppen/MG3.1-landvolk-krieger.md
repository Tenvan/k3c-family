# MG3.1 · Landstreicher, Bauer und Krieger als figur

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** truppen
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg3/1-landvolk-krieger
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Landstreicher (`landstreicher`), Bauer (`bauer`) und Krieger (`krieger`) liegen als `figur`-Specs unter `public/grafik/truppen/`, gerendert mit `idle`, `lauf` und `angriff`, nur über die Asset-MCP-Tools angelegt und mit `assets_preview` geprüft.

## Kontext

- Truppen: `data/troops.json` (`vagrant` Landstreicher, `peasant` Bauer, `warrior` Krieger); Animationen der alten Sheets `data/sprites.json` › `sheets` (idle 10/8, run 8, attack 7/4 Frames bei fps 14).
- Rig `figur` (`assets_guide {rig: figur}`): Pflicht-Teile, Pivots, Proportion 0,286 ± 0,15. Der Renderer kennt ohne Keyframes nur `idle` und `lauf` (`tools/paper/rig/figur.ts`); `angriff` braucht Keyframes (Winkel je Teil, Grad, positiv = Uhrzeigersinn).
- Vorbild: `public/grafik/figuren/monarch.paper.json` (Körperformen, Kopf, Schwert-Teil); Truppen ohne Krone und Umhang, eigene Materialien aus der Stil-Datei (keine Farben).
- B-707/D-MG1-02: `assets_set` zieht `golden.json` nicht nach, daher nach den Änderungen `task gen:paper`.

## Erlaubte Dateien

- `public/grafik/truppen/**` (Specs, PNGs, Manifeste, `index.paper.json`, `golden.json`)
- `docs/sprints/aktiv/MG3-papier-truppen/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG3-Zeilen)

## Nicht-Ziele

Kein `assets_style_set`, keine Änderung an Renderer, Workbench oder anderen Gruppen; Spiel lädt die Grafiken noch nicht (US1).

## Schritte

1. `assets_guide {rig: figur}`, `assets_get monarch` als Vorbild.
2. Je Truppe `assets_create {gruppe: truppen, rig: figur}` und `assets_set` mit Teilen, Animationen `idle` (8), `lauf` (8), `angriff` (Keyframes, fps 14) und Ausgabe.
3. Jede Animation mit `assets_preview` ansehen (Frame 0 und Mitte), Befunde notieren; Fehlerfälle (unbekanntes Material, Frame jenseits) einmal prüfen.
4. `task gen:paper`, `assets_list {gruppe: truppen}`, `assets_review`.
5. `task check:diff`, Ergebnis, Commit `feat(pap): Truppen Landstreicher, Bauer, Krieger (MG3.1)`.

## Fertig, wenn

- [ ] AC-01: `assets_list {gruppe: truppen}` meldet die drei Assets gültig.
- [ ] AC-02: `task gen:paper` grün, `golden.json` der Gruppe eingetragen; `task test -- paper` grün.
- [ ] AC-03: jede Animation jedes Assets mit `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

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
