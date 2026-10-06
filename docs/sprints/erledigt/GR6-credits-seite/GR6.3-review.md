# GR6.3 · Review und Abnahme des Sprints GR6

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** gr6/3-review
- **Abhängig von:** GR6.2
- **Tickets:** B-165
- **Kriterien:** alle

## Ziel

Der Sprint GR6 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-165 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/tools/`, `lizenzen.html` und die beiden CREDITS-Dateien. Besonders prüfen: keine CC-BY-Namensnennung ist verloren gegangen (Warped Caves, LPC-Reittiere); kein Credit-Text wird doppelt gepflegt; der Vollständigkeits-Test liest die echten Verzeichnisse und wurde nicht gelockert; Seiten-Regeln aus `CLAUDE.md` (Home-Button, `pages.ts`, keine direkten Seitenwechsel, B frei).

## Erlaubte Dateien

- `src/tools/`, `lizenzen.html`, `public/grafik/CREDITS.md`, `public/sprites/CREDITS.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, neue Assets.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-04 aus den Ergebnissen von GR6.1 und GR6.2 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-165 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt.
- [x] `task check` grün; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Review auf dem Sprint-Branch `sprint/gr6` (Diff gegen `origin/develop`, nur `src/tools/`, `lizenzen.html`, `public/sprites/CREDITS.md`).

- **AC-01 bis AC-04** aus den Ergebnissen von GR6.1 und GR6.2 nachgewiesen (Test `src/tools/credits.test.ts`), nichts verschoben; Sichtprüfung am TV angenommen, Validierung offen.
- Befunde: keine schweren. Namensnennungen vollständig erhalten, Test nicht gelockert, `installPageChrome()` und `pages.ts`-Eintrag vorhanden, keine B-Belegung, keine Seitenwechsel.
- `task check` grün (672 Tests); keine Go-Änderungen, daher kein `task check:go`.
- B-165 archiviert, Sprint nach `erledigt/`, Version `v0.7.0` vorgeschlagen (siehe Abnahme).
