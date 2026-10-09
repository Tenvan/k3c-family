# GR4.4 · Review und Abnahme des Sprints GR4

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** gr4/4-review
- **Abhängig von:** GR4.2
- **Tickets:** B-163, B-029
- **Kriterien:** alle

## Ziel

Der Sprint GR4 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-163 und B-029 sind archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt das Atlas-Werkzeug, `Taskfile.yml`, `src/scenes/sprites.ts`, die Lade-Szene und den Spiel-Einstieg. Besonders prüfen: Atlas-Erzeugung deterministisch (Test vorhanden, nicht gelockert); keine neue schwere Abhängigkeit; fehlende Assets führen zu einer Meldung, nicht zu einem hängenden Balken; B nicht belegt, Home-Button nicht verdeckt; Animationen unverändert (Screenshot aus GR4.2). GR4.3 (Messung auf der Xbox) ist keine Abhängigkeit: ist sie offen, führt die Abnahme AC-04 als `angenommen, Validierung offen (GR4.3)` und GR4.3 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- Atlas-Werkzeug, `Taskfile.yml`, `src/scenes/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Messung an der Xbox.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` und `task build` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von GR4.1 bis GR4.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-029 auf `erledigt` setzen und archivieren; B-163 erst, wenn GR4.3 das Budget eingetragen hat (sonst offen lassen, Hinweis in der Abnahme).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (GR4.3 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-03, AC-05 und AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] AC-04 ist nachgewiesen oder als `angenommen, Validierung offen (GR4.3)` geführt.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task build
```

## Ergebnis

`task check` und `task check:go` grün (Race-Test lokal übersprungen, kein C-Compiler; die CI prüft ihn). Diff `origin/develop...origin/sprint/gr4` gelesen. Atlas-Erzeugung deterministisch (Test aus GR4.1 unverändert), keine neue Abhängigkeit, Ladefehler zeigen Meldung, B nicht belegt.

Nachweis: AC-01 (GR4.1), AC-02, AC-03, AC-06 (GR4.2, im Browser-Pane geprüft), AC-05 (`task check`, `task build` in GR4.1/GR4.2; hier `task check` erneut), AC-04 angenommen, Validierung offen (GR4.3).

Schwere Befunde, behoben in INF:
- B-196: CI-, Pages- und Release-Workflow bauten ohne `setup-go` (Atlas braucht Go).
- Dockerfile baute das Web-Bundle per `vite build` ohne Atlas und `.dockerignore` schloss `tools/` aus; das Image hätte die Lade-Szene mit Fehlermeldung gezeigt. Neue Stufe `atlas` (Image nicht lokal gebaut, ungeprüft; der CI-Job `docker` prüft `/game.html`, aber nicht den Atlas).
