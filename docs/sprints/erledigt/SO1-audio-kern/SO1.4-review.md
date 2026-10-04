# SO1.4 · Review und Abnahme des Sprints SO1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Branch:** so1/4-review
- **Abhängig von:** SO1.3
- **Tickets:** B-011
- **Kriterien:** alle

## Ziel

Der Sprint SO1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-011 bleibt offen für SO2 und SO4 (Effekte, Musik), mit Notiz zum Stand.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/audio/`, `public/audio/` und `src/scenes/GameScene.ts`. Besonders prüfen: kein Audio-Fehler vor der ersten Eingabe; Audio rechnet keine Spielregel (nur Ereignisse/Snapshot); `localStorage` mit try/catch; nur selbst erzeugte Töne oder Dateien mit Credit; B nicht belegt; 2 lokale Spieler hören ihren Bereich. SO1.5 (Hörprobe am TV) ist keine Abhängigkeit: ist sie offen, führt die Abnahme AC-04 als `angenommen, Validierung offen (SO1.5)` und SO1.5 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `src/audio/`, `public/audio/`, `src/scenes/GameScene.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Klangauswahl.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/so1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-08 aus den Ergebnissen von SO1.1 bis SO1.3 (und SO1.5, falls schon da) prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-011: Status `eingeplant` lassen (SO2, SO4), Notiz „AC-03 Lautstärke: Kern fertig (SO1)“.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (SO1.5 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-03 und AC-05 bis AC-08 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] AC-04 ist am TV beobachtet oder als `angenommen, Validierung offen (SO1.5)` geführt.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Review des Diffs `origin/develop...sprint/so1`: keine schweren Befunde, keine neuen Tickets. Nachweise AC-01 bis AC-08 siehe Abnahme in der Sprint-README; AC-04 als `angenommen, Validierung offen (SO1.5)`, SO1.5 im Fahrplan unter „Offen am Gerät“. `task check` grün (925 Tests), `task check:go` grün bis auf sporadischen Windows-Rename-Fehler im Paket `store` (B-187, Wiederholung grün).
