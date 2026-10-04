# SO4.4 · Review und Abnahme des Sprints SO4

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** so4/4-review
- **Abhängig von:** SO4.3
- **Tickets:** B-168
- **Kriterien:** alle

## Ziel

Der Sprint SO4 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-168 ist archiviert; der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/audio/`, `public/audio/` (Musik, `CREDITS.md`), `docs/assets/sounds.md` und `src/scenes/GameScene.ts` bzw. `LobbyScene.ts` (nur Meldung an den Automaten). Besonders prüfen: nur CC0 oder CC-BY, jede Musik-Datei mit Credit (CC-BY mit Urheber und Quelle), keine nicht gewählten Kandidaten im Repo; Zustands-Automat und Ducking sind reine Funktionen über Snapshot und Ereignisse, `src/scenes/noSim.test.ts` grün, kein `Math.random()` für Spiel-Logik; fehlendes Stück lässt das Spiel weiterlaufen; eine Musik je Gerät (nicht je Viertel); Audio-Kern aus SO1, `src/tools/` und `lizenzen.html` unverändert (PLAT); keine Musik wird vor der ersten Eingabe gestartet (kein Audio-Fehler); B-Taste nicht belegt; Gesamtgröße der Musik-Dateien vertretbar (Kaltstart-Budget GR4). **Das Hören am TV (SO4.5) ist keine Abhängigkeit** (Regel „Hardware entkoppelt“): ist sie offen, führt die Abnahme AC-02 und AC-03 als `angenommen, Validierung offen (SO4.5)` und SO4.5 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `src/audio/`, `public/audio/`, `docs/assets/sounds.md`, `src/scenes/GameScene.ts`, `src/scenes/LobbyScene.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Stückwahl, Optimierung.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von SO4.2 und SO4.3 (und SO4.5, falls schon da) prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-168 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); B-011 mit Notiz zum Stand (Musik: SO4 fertig) im Ticket belassen oder archivieren, je nachdem, ob alle Kriterien von B-011 erfüllt sind.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (SO4.5 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-02 und AC-03 sind am TV beobachtet oder als `angenommen, Validierung offen (SO4.5)` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

–
