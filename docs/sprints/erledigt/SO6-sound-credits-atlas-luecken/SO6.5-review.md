# SO6.5 · Review und Abnahme des Sprints SO6

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** so6/5-review
- **Abhängig von:** SO6.4
- **Tickets:** B-493, B-494, B-492
- **Kriterien:** alle

## Ziel

SO6 ist nach `docs/arbeitsweise.md` › Review-Session geprüft; Abnahme in der Sprint-README.

## Kontext

Reviewer-Agent getrennt (Diff berührt Pfad-Code im Go-Werkzeug und den Audio-Kern). Die Hörprobe SO6.6 und der Workshop SO6.3 sind keine Abhängigkeit (Hardware entkoppelt): offen → Kriterien `angenommen, Validierung offen`.

## Erlaubte Dateien

- Dateien des Sprints nur für schwere Befunde; `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Geschmack der Sounds.

## Schritte

1. `task check`, `task check:go` nacheinander, Diff lesen.
2. Nachweis je Kriterium aus SO6.1, SO6.2, SO6.4.
3. Abnahme schreiben; Sprint bleibt `aktiv`, solange SO6.3/SO6.6 offen sind.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis, AC-04 offen (SO6.6) oder belegt.
- [ ] Schwere Befunde behoben.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

2026-10-22, autonom (Claude). Reviewer-Agent (Sonnet, getrennt), Diff `7d3ee511..HEAD`: **keine schweren Befunde** (bossSpawned passt zu Protokoll und `engine/sim/boss.go`, kein doppelter Hub-Ton bei zwei lokalen Spielern, keine doppelten Ladevorgänge, Grenzen eingehalten). `task check` grün; `task check:go` einmal rot nur in `engine/devrun` (Wanduhr-Test, bekannt flackernd, B-284), Einzellauf `go test ./engine/devrun` grün.

- AC-01 geprüft (SO6.1, `credits.test.ts`). AC-02 geprüft (SO6.2, SO6.4: Atlas-Test, Anfragen-Test). AC-03 umgesetzt mit Default B-558, angenommen, Bestätigung offen (SO6.3). AC-04 offen (SO6.6, TV).
- Hinweise ohne Befund: Hub-Ton kommt einen Frame später (liest den Stand des letzten Renders); Stimmen-Grenze gilt jetzt auch für Atlas-Sprites; Byte-Gleichheit des Atlas hängt an der ffmpeg-Version (hier zweimal byte-gleich).
- Sprint bleibt `aktiv` (SO6.3, SO6.6 bei 🧑).
