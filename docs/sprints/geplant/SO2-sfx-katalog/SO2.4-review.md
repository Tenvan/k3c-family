# SO2.4 · Review und Abnahme des Sprints SO2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so2/4-review
- **Abhängig von:** SO2.3
- **Tickets:** B-167
- **Kriterien:** alle

## Ziel

Der Sprint SO2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-167 ist archiviert; der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `docs/assets/sounds.md`, `public/audio/` (Sounds, `CREDITS.md`, Atlas), `src/audio/` und eventuell `src/scenes/GameScene.ts`. Besonders prüfen: nur CC0 oder CC-BY, jede Sound-Datei mit Credit (CC-BY mit Urheber und Quelle); keine nicht gewählten Kandidaten und keine Quelldateien im Repo; Mapping und Audio rechnen nichts (`src/scenes/noSim.test.ts` grün, nur Ereignisse); fehlende Datei lässt das Spiel weiterlaufen; vor der ersten Eingabe keine Audio-Fehler; Audio-Kern aus SO1 unverändert; `src/tools/` und `lizenzen.html` unverändert (PLAT, Ticket); 2 lokale Spieler hören ihren Bereich. **Das Hören am TV (SO2.5) ist keine Abhängigkeit** (Regel „Hardware entkoppelt“): ist sie offen, führt die Abnahme AC-03 als `angenommen, Validierung offen (SO2.5)` und SO2.5 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `src/audio/`, `public/audio/`, `docs/assets/sounds.md`, `src/scenes/GameScene.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Klänge, Optimierung.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/so2` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von SO2.1 bis SO2.3 (und SO2.5, falls schon da) prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-167 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); B-011 bleibt eingeplant (SO4), Notiz „Effekte: SO2 fertig“.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (SO2.5 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-03 ist am TV beobachtet oder als `angenommen, Validierung offen (SO2.5)` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

–
