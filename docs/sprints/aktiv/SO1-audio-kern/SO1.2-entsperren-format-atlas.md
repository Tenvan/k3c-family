# SO1.2 · Entsperren per Geste, Format mit Fallback, Sound-Atlas

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** so1/2-entsperren-format-atlas
- **Abhängig von:** SO1.1
- **Tickets:** B-011
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Vor der ersten Eingabe läuft das Spiel ohne Audio-Fehler, nach der ersten Taste wird der Ton entsperrt; Sounds werden im angenommenen Format mit Fallback aus einem Sound-Atlas geladen, die Zahl der Audio-Requests ist dokumentiert.

## Kontext

- **Entsperren:** Browser starten den `AudioContext` gesperrt (`suspended`); `resume()` nach der ersten Geste (Taste am Controller, Tastatur, Touch — bei uns typischerweise A beim Beitreten). Gamepad-Tasten zählen in Chromium nicht immer als Geste: dann bleibt der Ton aus, bis eine zählende Eingabe kommt — kein Fehler, Hinweis im Ergebnis (am Gerät in SO1.5).
- **Format (Hardware entkoppelt):** Bis X1.2/X1.3 die Xbox gemessen hat (B-166), gilt laut `docs/plan-weiterentwicklung.md` § 11.6 die Annahme **mp3 mit Fallback**. Die Spec-Anforderung nennt „ogg mit m4a-Fallback“ (Stand vor § 11.6). Umsetzung: Formatliste an **einer** Stelle (Reihenfolge per `canPlayType` bzw. Ladeversuch), Startreihenfolge laut Annahme, im Code als „angenommen (§ 11.6, B-166)“ markiert; liegt das X1-Ergebnis vor (`docs/game-design.md`), dessen Reihenfolge nehmen.
- **Sound-Atlas:** ein Audio-Sprite (eine Datei, Liste von Start/Dauer je Sound) statt vieler Requests; Beschreibung als JSON. Für den Start genügen selbst erzeugte Töne (z. B. aus `public/audio-test/`, Sinus 440 Hz, „selbst erzeugt, kein Credit nötig“) bzw. per Web Audio erzeugte Retro-Töne (Q15: Lückenfüller erlaubt). Fremde Dateien erst mit SO2 und Credit (B-165).
- Requests zählen im Browser-Pane (`read_network_requests`, nur Audio) beim Start von `game.html`.

## Erlaubte Dateien

- `src/audio/` (Entsperren, Laden, Atlas, Tests)
- `public/audio/` (neu: Atlas-Datei und Beschreibung, nur selbst erzeugte Töne)
- `src/scenes/GameScene.ts` (nur: erste Eingabe meldet sich beim Audio-Kern)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Dämpfung und Demo-Ereignis (SO1.3), SFX-Katalog (SO2), Musik (SO4), Atlas-Werkzeug für Grafiken (GR4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Entsperren: Audio-Kern startet gesperrt, erste Eingabe ruft `resume()`; keine Fehlermeldung in der Konsole vor der ersten Eingabe (Browser-Pane: `read_console_messages`).
3. Formatliste mit Fallback, Laden des Atlas; Test der reinen Auswahl-Funktion (unterstützte Formate → gewählte Datei).
4. Requests vorher/nachher messen, Zahl ins Ergebnis.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: Vor der ersten Eingabe keine Audio-Fehler, danach Ton (Konsole im Browser-Pane, Test der Entsperr-Logik).
- [x] AC-04: Format laut Annahme bzw. X1-Ergebnis mit Fallback (Test der Auswahl; Beobachtung am TV in SO1.5).
- [x] AC-05: Sounds aus einem Atlas, Zahl der Audio-Requests im Ergebnis.
- [x] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

- **AC-03: umgesetzt, geprüft** durch Vitest (`src/audio/audioCore.test.ts`): Vor der ersten Eingabe kein `AudioContext`, kein Request, `play` stumm (keine Audio-Fehler, keine Browser-Warnung); die erste Eingabe erzeugt den Context und ruft `resume()`; bleibt er gesperrt (Gamepad zählt nicht als Geste), wird bei jeder Eingabe erneut `resume()` versucht, ohne Fehler und ohne neue Requests. Anbindung: `GameScene.trackLastDevice` meldet die Eingabe (`audioCore().onInput()`). Konsole im Browser-Pane nicht geprüft (nicht freigegeben); Gamepad-Verhalten am Gerät in SO1.5.
- **AC-04: umgesetzt, geprüft** durch Test der reinen Funktion `pickFile` (`src/audio/atlas.ts`): Reihenfolge `ogg`, `mp3`, `wav` laut X1-Ergebnis (B-166, `docs/game-design.md`: ogg, mp3, wav ja, m4a nein; liegt vor, daher nicht die Annahme mp3 aus § 11.6); Fallback auf mp3, ohne Treffer keine Datei. Beobachtung am TV in SO1.5.
- **AC-05: umgesetzt, geprüft** (Test zählt Requests): `public/audio/atlas.ogg` und `atlas.mp3` (je 0,8 s, drei selbst erzeugte Sinustöne `coin`, `hit`, `warn`) mit `atlas.json`. **Audio-Requests: 0 beim Start von `game.html`, nach der ersten Eingabe genau 2** (`atlas.json` + eine Audiodatei). Messung im Browser-Pane nicht gemacht (nicht freigegeben).
- **`task check` grün:** 52 Dateien, 917 Tests.
- Abweichung: Der `AudioContext` entsteht erst bei der ersten Eingabe statt gesperrt beim Start (vermeidet die Chromium-Warnung). `audioCore().play('coin')` ist für SO1.3 bereit.
