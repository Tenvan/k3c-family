# GR3.3 · Review und Abnahme des Sprints GR3

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** gr3/3-review
- **Abhängig von:** GR3.2
- **Tickets:** B-010
- **Kriterien:** alle

## Ziel

Der Sprint GR3 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-010 ist archiviert (oder mit Grund und Ticket offen, falls ein Kriterium nur teilweise erfüllt ist); der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/` (`worldRenderer.ts`, `stageView.ts`, `LoadScene.ts`, neue Dateien), `public/grafik/CREDITS.md` und Tests. Besonders prüfen: Client rechnet nichts (`src/scenes/noSim.test.ts`), kein `Math.random()` für Spiel-Logik; jeder eingebaute Sprite hat einen Credit (CC-BY-Namensnennung für Warped Caves erhalten); fehlende Textur lässt den Platzhalter stehen statt zu crashen; mit 2 Spielern im Split-Screen keine Darstellungsfehler; B-Taste nicht belegt; `worldRenderer.ts` ≤ 400 Zeilen. **Die Sichtprüfung am TV (AC-04) gehört 🧑 und ist keine Abhängigkeit** (Regel „Hardware entkoppelt“): ist sie offen, führt die Abnahme AC-04 als `angenommen, Validierung offen (🧑, TV)`.

## Erlaubte Dateien

- `src/scenes/`, `public/grafik/CREDITS.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Grafikauswahl, Optimierung, Atlas.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr3` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von GR3.1 und GR3.2 prüfen (AC-06: `task check` grün, 2 Spieler im Split-Screen im Browser-Pane ansehen).
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-010 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-04 ist am TV beobachtet oder als `angenommen, Validierung offen` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

–
