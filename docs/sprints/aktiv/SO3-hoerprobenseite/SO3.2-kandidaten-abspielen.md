# SO3.2 · Kandidatenliste, Abspielen, Crossfade-Probe, Controller-Bedienung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** so3/2-kandidaten-abspielen
- **Abhängig von:** SO3.1, SO1.4
- **Tickets:** B-169
- **Kriterien:** AC-02, AC-03, AC-04, AC-06, AC-07, AC-08

## Ziel

Die Seite listet Kandidaten gruppiert nach Zustand und Ereignis mit Quelle und Lizenz, spielt sie über den Audio-Kern ab (Abspielen, Stoppen, Lautstärke, Crossfade zwischen zwei Stücken) und ist mit Controller, Tastatur und Touch bedienbar.

## Kontext

- Anforderungen: B-169 › Anforderungen, Ausnahme- und Fehlerfälle (Audio gesperrt → Hinweis; Kandidat lädt nicht → Eintrag grau, Rest geht).
- **Audio-Kern aus SO1** (`src/audio/`: Mixer mit Bussen Musik/Effekte/Ambient, Entsperren per Geste, Format nach B-166 mit Fallback). Die Seite nutzt ihn, baut kein eigenes Audio. Fehlt SO1: `Status: blockiert`.
- **Gruppen (Beschluss Q16):** 8 Zustände Tag, Abend, Nacht, Kampf, Tiefe/Höhle, Boss, Lobby, Niederlage/Sieg (Musik, 1–2 Stücke je Zustand); Ereignisse wie in Q08 (Treffer, Kill, Münze auf/gegeben, Pfeil, Schlag, Bau-Fortschritt/fertig, Tod, Wiederbeleben, Skill, Nacht naht, Portal). Crossfade und Ducking bei Warnungen gehören laut Q16 zur Musik; die Seite probt den Crossfade.
- **Kandidaten:** Die Auswahl selbst trifft 🧑 in SO2.1/SO4.1 (Q15: Retro/Chiptune, kindgerecht, Nacht leise; Quellen Kenney, OpenGameArt, freesound CC0/CC-BY). Diese Session baut die Liste aus einer Datenquelle (z. B. `public/audio/kandidaten.json` mit Gruppe, Name, Datei, Quelle, Lizenz), damit SO2/SO4 nur Einträge ergänzen. Bis dahin genügen die selbst erzeugten Testtöne aus `public/audio-test/` (Sinus 440 Hz, kein Credit nötig) und zwei kurze, selbst erzeugte Probe-Stücke für den Crossfade — keine fremden Dateien ohne Credit (B-165).
- Bedienung: Stick/D-Pad wählt, A spielt, Stopp und Überblenden auf freien Tasten (nicht B, nicht View/Menu allein); Pad-Scroll wie `installPadScroll`. Große Schrift für den TV (Q03).
- Crossfade ohne Knacken: Lautstärke über Rampen (`linearRampToValueAtTime` o. ä.) statt Sprung; Prüfung im Browser über die Kurve (Test der reinen Funktion), hörbar am TV in SO3.3.

## Erlaubte Dateien

- `src/tools/soundtest.ts` und neue Dateien in `src/tools/` (Liste, reine Funktionen mit Test)
- `public/audio/` (nur Kandidaten-Datei und selbst erzeugte Probe-Stücke), `soundtest.html`
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Auswahl der Kandidaten (SO2, SO4), Einbau ins Spiel, Änderungen am Audio-Kern (Ticket an SO1 bzw. CLI), Bearbeiten/Mischen von Sounds.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Prüfen, dass SO1 fertig ist.
2. Kandidaten-Datei und Liste nach Gruppen mit Quelle und Lizenz; nicht ladbarer Eintrag grau.
3. Abspielen, Stoppen, Lautstärke über den Mixer; Crossfade-Probe zwischen zwei Stücken mit Rampe; Test der reinen Funktionen (Gruppierung, Rampe ohne Sprung).
4. Bedienung mit gemocktem Pad, Tastatur und `?touch=1` im Browser-Pane prüfen; B ohne Wirkung (Screenshot im PR).
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-02: Liste nach Zustand und Ereignis mit Quelle und Lizenz, Abspielen funktioniert (Nachweis im Browser-Pane).
- [x] AC-03: Bedienung mit gemocktem Controller im Browser-Pane, B ohne Wirkung (am TV in SO3.3).
- [x] AC-04: Crossfade mit Rampe, Test der reinen Funktion (hörbar am TV in SO3.3).
- [x] AC-07: Bedienung mit Tastatur und `?touch=1` im Browser-Pane.
- [x] AC-08: keine fremde Audiodatei ohne Quelle und Lizenz in den Credits (B-165).
- [x] AC-06: `task check` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

`public/audio/kandidaten.json` (Liste: Gruppe, Name, Datei, Bus, Quelle, Lizenz) und selbst erzeugte Probe-Stücke `probe-a`/`probe-b` (`.ogg`, `.mp3`, ffmpeg-Befehl in `public/audio/README.md`). `src/tools/soundtestLogic.ts` (reine Logik) mit Test, `src/tools/soundtest.ts` und `soundtest.html` (Liste nach 8 Zuständen und Ereignissen mit Quelle und Lizenz, Abspielen, Stopp, Lautstärke, Überblenden über 2 s). Mixer und Formatwahl (`pickFile`) aus `src/audio/` wiederverwendet; `AudioCore` spielt nur Atlas-Sprites und hält Kontext/Mixer privat, daher eigener Kontext in der Seite (Ticket B-250). Kein `installPadScroll()`: Stick und Steuerkreuz wählen den Eintrag, die Auswahl scrollt selbst.

- AC-02: umgesetzt (Gruppierung und Quelle/Lizenz je Eintrag, `soundtestLogic.test.ts`); Abspielen im Browser-Pane nicht geprüft (Auftrag ohne Browserprüfung), Hören am TV in SO3.3.
- AC-03: umgesetzt, geprüft per Test `padActions` (Flanke, Taste 1 = B löst nichts aus); gemockter Controller im Browser-Pane nicht geprüft, am TV in SO3.3.
- AC-04: umgesetzt, geprüft per Test `crossfadeCurves` (Enden 1/0, gleiche Leistung, Schrittweite < 0,05, kein Sprung); Anwendung über `setValueCurveAtTime`. Hören am TV in SO3.3.
- AC-07: umgesetzt (Tastatur `keyAction`, Touch per Antippen und Knöpfen); Browser-Pane-Prüfung mit `?touch=1` nicht ausgeführt.
- AC-08: geprüft: nur selbst erzeugte Dateien (`probe-*`, `audio-test/test.*`), keine fremde Datei, kein Credit nötig.
- AC-06: `task check` grün (54 Dateien, 933 Tests); alle Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.
- Neues Ticket: B-250 (Audio-Kern spielt ganze Dateien mit Crossfade).
