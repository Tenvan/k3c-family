# GR2.4 · Review und Abnahme des Sprints GR2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** gr2/4-review
- **Abhängig von:** GR2.3
- **Tickets:** B-162
- **Kriterien:** alle

## Ziel

Der Sprint GR2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-162 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `docs/funde/`, `public/grafik/`, Credits und `docs/assets/zuordnung.md`. Besonders prüfen: nur CC0/CC-BY, jede neue Datei mit Lizenzdatei und Credit (CC-BY mit Urheber und Quelle); keine Musik, kein Demo-Code, keine Quelldateien im Repo; nur gewählte Kandidaten eingebunden (Entscheidungen von 🧑 auf der Referenzseite); Tests nicht gelockert (Zahl der Packs angepasst, nicht entfernt).

## Erlaubte Dateien

- `public/grafik/`, Credits, `docs/assets/zuordnung.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Auswahl, Einbau.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-05 aus den Ergebnissen von GR2.1 bis GR2.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-162 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-05 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

–
