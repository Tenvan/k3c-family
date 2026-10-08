# GR5.3 · Review und Abnahme des Sprints GR5

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr5/3-review
- **Abhängig von:** GR5.2
- **Tickets:** B-164
- **Kriterien:** alle

## Ziel

Der Sprint GR5 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-164 ist archiviert; der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/` (`GameScene.ts`, neue `effects*.ts`, Tests). Besonders prüfen: Effekte lesen nur Events und Einstellungen und ändern keinen Spielzustand (`src/scenes/noSim.test.ts` grün, kein `Math.random()` für Spiel-Logik); Effekte nutzen eine **eigene** Event-Warteschlange und leeren `pendingEvents` der HudScene nicht; Screenshake nur in der Kamera des betroffenen Spielers (2 Spieler im Split-Screen); höchstens 3 Blitze pro Sekunde; kaputter oder fehlender Einstellungs-Speicher und fehlende Vibration führen nicht zum Absturz; `src/input/` und `src/core/settings.ts` sind unverändert; B-Taste nicht belegt. **Die Abnahme am TV (AC-02 sichtbar mit „an“, Blitz-Eindruck) gehört 🧑 und ist keine Abhängigkeit** (Regel „Hardware entkoppelt“); ist sie offen, führt die Abnahme AC-02 als `angenommen, Validierung offen (🧑, TV)`.

## Erlaubte Dateien

- `src/scenes/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack bei Stärke und Aussehen der Effekte, Optimierung.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr5` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-07 aus den Ergebnissen von GR5.1 und GR5.2 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-164 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-07 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-02 ist am TV beobachtet oder als `angenommen, Validierung offen` geführt.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Review abgeschlossen, Abnahme in der Sprint-README; keine schweren Befunde, kein Eingriff in `src/`.
