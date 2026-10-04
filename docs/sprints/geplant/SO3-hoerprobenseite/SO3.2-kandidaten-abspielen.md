# SO3.2 · Kandidatenliste, Abspielen, Crossfade-Probe, Controller-Bedienung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
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

- [ ] AC-02: Liste nach Zustand und Ereignis mit Quelle und Lizenz, Abspielen funktioniert (Nachweis im Browser-Pane).
- [ ] AC-03: Bedienung mit gemocktem Controller im Browser-Pane, B ohne Wirkung (am TV in SO3.3).
- [ ] AC-04: Crossfade mit Rampe, Test der reinen Funktion (hörbar am TV in SO3.3).
- [ ] AC-07: Bedienung mit Tastatur und `?touch=1` im Browser-Pane.
- [ ] AC-08: keine fremde Audiodatei ohne Quelle und Lizenz in den Credits (B-165).
- [ ] AC-06: `task check` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

–
