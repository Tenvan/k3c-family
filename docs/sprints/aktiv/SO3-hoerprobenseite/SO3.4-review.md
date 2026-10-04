# SO3.4 · Review und Abnahme des Sprints SO3

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** so3/4-review
- **Abhängig von:** SO3.2
- **Tickets:** B-169
- **Kriterien:** alle

## Ziel

Der Sprint SO3 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-169 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `soundtest.html`, `src/tools/`, `src/landing/pages.ts` und `public/audio/`. Besonders prüfen: Seiten-Regeln aus `CLAUDE.md` (Home-Button, `pages.ts`, nur `toggleFullscreen()`, nur `openPage()`/`goHome()`, B frei, View + Menu reserviert; `projectRules.test.ts` grün); keine fremde Audiodatei ohne Credit; Audio nur über den Kern aus SO1. SO3.3 (Abnahme am TV) ist keine Abhängigkeit: ist sie offen, führt die Abnahme AC-03 und AC-04 als `angenommen, Validierung offen (SO3.3)` und SO3.3 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `soundtest.html`, `src/tools/`, `public/audio/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Kandidatenauswahl.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/so3` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von SO3.1 bis SO3.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-169 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (bei offener SO3.3 erst nach deren Ergebnis).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (SO3.3 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-05 bis AC-08 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] AC-03 und AC-04 sind abgenommen oder als `angenommen, Validierung offen (SO3.3)` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

–
