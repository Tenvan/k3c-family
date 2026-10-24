# SO6.2 · Task für den Sound-Atlas aus dem Slot-Register

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** so6/2-sound-atlas-task
- **Abhängig von:** SO6.1
- **Tickets:** B-494
- **Kriterien:** AC-02

## Ziel

`task gen:soundatlas` packt die erste Präferenz jedes Effekt-Slots (Bus `sfx`, ohne Schleifen) in `public/audio/atlas.ogg`/`.mp3` mit `atlas.json`; der Client spielt Effekte aus dem Atlas und lädt nur für fehlende Einträge Einzeldateien.

## Kontext

B-494; Default B-557 (Atlas bauen, Rückfall Einzeldateien). Der bisherige Atlas (drei Sinustöne, SO1.2) wird ersetzt; `play(name)` im Kern nutzt ihn, Namen werden die Slot-IDs. Werkzeug in Go (`tools/soundatlas`), ffmpeg dekodiert und kodiert (Voraussetzung wie beim alten Atlas). Schleifen-Slot `sfx:ride` bleibt draußen.

## Erlaubte Dateien

- `tools/soundatlas/` (neu), `tools/Taskfile.yml` (Task `soundatlas`)
- `public/audio/atlas.*`, `public/audio/README.md` (Abschnitt Atlas)
- `src/audio/audioCore.ts`, `src/audio/*.test.ts`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Musik im Atlas, Auswahl der Sounds, neue Slots (SO6.4).

## Schritte

1. Go-Werkzeug: Slots lesen, Dateien per ffmpeg zu PCM, mit Pausen aneinanderhängen, Ogg und MP3 schreiben, `atlas.json` mit Start/Dauer je Slot; reine Teile mit Test.
2. Task `gen:soundatlas`, Atlas erzeugen.
3. Kern: Effekte aus dem Atlas (Stimmen-Grenze wie bisher), Einzeldateien nur für Slots ohne Atlas-Eintrag; Test.
4. TS-Test: `atlas.json` enthält jeden aktiven Effekt-Slot.
5. `task check:diff`, Go-Tests.

## Fertig, wenn

- [ ] AC-02 (B-494/AC-01): Task erzeugt den Atlas, Test prüft jeden aktiven Effekt-Slot in `atlas.json`.
- [ ] AC-02 (B-494/AC-02): Effekte laden höchstens eine Audio-Datei je Format (Test der Anfragen im Kern).

## Prüfen

```bash
task gen:soundatlas
task check:diff
```

## Ergebnis

2026-10-22, autonom (Claude). Default B-557: Atlas bauen und nutzen, Einzeldateien als Rückfall.

- **AC-02 (B-494/AC-01) umgesetzt, geprüft:** `tools/soundatlas` (Go, ffmpeg) + Task `gen:soundatlas` (`tools/Taskfile.yml`, EXE `bin/soundatlas`). 11 Effekt-Slots, 9,7 s, `atlas.ogg` 66 KB, `atlas.mp3` 78 KB; zweimal erzeugt byte-gleich (deterministisch). Go-Tests `pickEntries`, `layout`; TS-Test „der erzeugte Atlas enthält jeden aktiven Effekt-Slot“ (`audioCore.test.ts`). Fehler → Slot im Text, kein halber Atlas (Temp, dann Umbenennen); Pfade außerhalb `public/` abgelehnt.
- **AC-02 (B-494/AC-02) umgesetzt, geprüft (Test):** Kern lädt nach der ersten Eingabe `atlas.json` + eine Audiodatei; Slots im Atlas spielen als Sprite (Stimmen-Grenze `MAX_SAMPLE_VOICES` auch dort), nur Slots ohne Eintrag laden einzeln (Test der Anfragen). Effekt-Anfragen: 12 → 1 (plus `atlas.json`). Browser-Pane nicht beobachtet (Audio braucht echte Geste); Messung am TV SO6.6.
- Der alte Sinus-Platzhalter-Atlas ist ersetzt (`public/audio/README.md` › Sound-Atlas).
- Review (Sonnet, getrennt): keine schweren Befunde; zwei Hinweise (Umbenennen statt Überschreiben, Pfad-Prüfung) behoben. `task check:diff`, `go:lint` grün.
- Hinweis: nach Änderung der Reihenfolge im Slot ist der Atlas veraltet, bis `task gen:soundatlas` läuft; der Test prüft nur die Slot-Menge.
