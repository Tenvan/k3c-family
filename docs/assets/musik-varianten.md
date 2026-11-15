# Musik: Variantenvergleich für ruhigere Stücke (B-936)

Stand 2026-11-15, Grundlage für die Entscheidung von 🧑 (B-936 AC-01). Ziel: je Musik-Zustand (Tag, Abend, Nacht,
Kampf, Lobby/Menü, dazu Tiefe, Boss, Ende) ein ruhiges, langsames Stück nach dem [Klangbild](../rules/klang.md):
warm, akustisch, gedämpft, langsam, kein 8-Bit/Chiptune. Heute sind alle Slots außer `music:niederlage` Chiptune
(`data/sounds.json`, Katalog [`sounds.md`](sounds.md)). Für diesen Vergleich gab es keine Downloads und keine neuen
Audiodateien. Angaben zu Anbieter-Bedingungen sind **ungeprüft** (Stand meines Wissens) und vor der Umsetzung am
Original zu lesen.

## Übersicht

| Weg | Aufwand | Klangqualität | Lizenzlage | Hauptrisiko |
|---|---|---|---|---|
| 1. Freie Bibliotheken (CC0/CC-BY) | gering: suchen, abhören, schneiden, Ogg | hoch, echte Instrumente und Aufnahmen | klar je Stück, CC-BY braucht Nennung | passende ruhige Stücke je Zustand finden; einheitlicher Stil über 8 Stücke |
| 2. KI-Musik (Suno, Udio, ElevenLabs, Stable Audio) | gering bis mittel: Prompts, Auswahl, Nachschnitt | mittel bis hoch, oft „Stock-Musik“-Klang, Schleifen selten sauber | unsicher: Rechte nur in Bezahl-Plänen, Bedingungen ändern sich | Urheberrecht am Ergebnis fraglich, laufende Klagen um Trainingsdaten |
| 3. Prozedural in Go (Synthese → Ogg) | mittel: Generator schreiben, Klang abstimmen | gut für Ambient, Drones, Glocken; schwach für Melodien | voll eigene Rechte (CC0 wie `selbst-erzeugt-niederlage.ogg`) | klingt schnell „synthetisch“, widerspricht dann dem Klangbild |
| 4. MIDI per Skript + SoundCraft (Bounce WAV/FLAC → Ogg) | hoch: komponieren, Instrumente wählen, mischen | gut bis hoch, je nach Instrumenten; warm möglich | eigene Noten; Klangquelle (Synth/Samples) muss frei sein | SoundCraft ist Pre-Alpha, CLI-Umfang ungeprüft |

## 1. Freie Bibliotheken

- **Quellen:** OpenGameArt (Filter CC0/CC-BY, viel Fantasy- und Laute-/Harfen-Musik neben Chiptune), Kevin MacLeod
  (incompetech, CC-BY 4.0, große Auswahl ruhiger mittelalterlicher und Ambient-Stücke), Freesound (eher Loops und
  Atmos), Pixabay Music (eigene Pixabay-Lizenz, **kein** CC; Nutzung im Spiel erlaubt, Weitergabe als Einzeldatei nicht,
  ungeprüft).
- **Aufwand:** Recherche über `assets_research`/Recherche-Treffer der Workbench, Abhören durch 🧑, Schleifen mit ffmpeg
  schneiden, Lizenz in `public/audio/README.md` und im Lizenzfile (`native/k3c-game/menu/licensefile`).
- **Qualität:** echte Instrumente (Laute, Harfe, Flöte, Streicher) treffen das Klangbild am direktesten.
- **Lizenz:** CC0 ohne Pflichten; CC-BY verlangt Nennung im Spiel (Lizenzfile vorhanden). Pixabay meiden, wenn
  Weitergabe der Dateien im Paket `dist/k3c-game/` als Einzeldatei zählt (ungeprüft).
- **Risiken:** gleiche Stücke in vielen Indie-Spielen; Stilbruch zwischen Komponisten (am besten 4–5 Stücke eines
  Urhebers oder Albums); falsch deklarierte Lizenzen auf OpenGameArt (je Stück Quelle prüfen).

## 2. KI-Musik

- **Anbieter:** Suno, Udio, ElevenLabs Music, Stable Audio (Stability AI).
- **Aufwand:** Beschreibung je Zustand („slow medieval lute and harp, warm, 60 bpm, loopable“), mehrere Läufe, Auswahl,
  Schleifenschnitt.
- **Qualität:** überzeugend für Hintergrundmusik, aber oft generisch; saubere Loops und gleichbleibender Stil über
  mehrere Stücke brauchen Nacharbeit.
- **Lizenz (ungeprüft):** kommerzielle Nutzung meist nur in Bezahl-Plänen; das Recht am Ergebnis gewährt der Anbieter
  vertraglich, ein Urheberrecht entsteht an reiner KI-Ausgabe nach US-Praxis nicht (Dritte dürften die Stücke dann
  frei nutzen). Klagen der Musikindustrie gegen Suno und Udio (2024) und die folgenden Vergleiche und Lizenzdeals
  können Bedingungen nachträglich ändern.
- **Risiken:** Rechtslage und Bedingungen im Fluss, Abo-Kosten, keine nachvollziehbare Herkunft für das Lizenzfile.

## 3. Prozedural in Go

- **Ansatz:** kleines Werkzeug unter `tools/` (eigenes Taskfile-Ziel), Sinus/Rauschen mit Hüllkurven, Filtern und Hall
  nach WAV, ffmpeg nach Ogg; deterministisch über `engine/rng`. Vorbild: `selbst-erzeugt-niederlage.ogg` (Rezept in
  `public/audio/README.md`).
- **Aufwand:** 1–2 Sessions für Generator und Ambient-Stücke; Melodien deutlich mehr.
- **Qualität:** gut für Nacht-Drone, Wind, Glockenspiel, Menü-Ambient; ohne Samples klingen Melodie-Instrumente
  künstlich, das widerspricht „akustisch“.
- **Lizenz:** vollständig eigen, CC0.
- **Risiken:** Klang wirkt elektronisch; Abstimmung nur über Abhören durch 🧑 (Claude hört nicht).

## 4. MIDI per Skript + SoundCraft

- **Ansatz:** Noten als MIDI per Skript (Go oder TS), Session in SoundCraft (ArtCraft, MIT/Apache-2.0, Windows,
  per Kommandozeile oder Agent steuerbar) bauen, mischen (EQ, Hall), Bounce nach WAV/FLAC mit LUFS-Angabe, dann
  ffmpeg nach Ogg (SoundCraft exportiert kein Ogg).
- **Aufwand:** hoch: Komposition je Zustand, Instrumentwahl, Mischung, Abhörschleifen mit 🧑; Einrichtung des
  Werkzeugs und Taskfile-Ziel.
- **Qualität:** eigene Melodien und einheitlicher Stil über alle Zustände; warm, wenn der Sampler echte
  Instrumenten-Samples (freie SoundFonts/Samples) nutzt; mit dem eingebauten Subtraktiv-Synth eher elektronisch.
- **Lizenz:** Noten eigen; Lizenz der Samples prüfen (z. B. freie SoundFonts mit permissiver Lizenz).
- **Risiken:** SoundCraft ist Pre-Alpha, CLI-Umfang und Stabilität ungeprüft; neues Werkzeug in der Kette
  (Requirements, Windows-Build). Rückfall bei Problemen: FluidSynth mit freiem General-MIDI-SoundFont (ausgereift,
  CLI-fähig).

## Empfehlung

**Weg 1 zuerst, ergänzt durch Weg 3 für Ambient; Weg 4 als Ausbau, Weg 2 nicht.**

1. Je Zustand 2–3 ruhige akustische Kandidaten aus OpenGameArt (CC0) und Kevin MacLeod (CC-BY 4.0) recherchieren,
   möglichst aus einer Hand für einen einheitlichen Stil; 🧑 hört am TV ab und wählt (B-936 AC-02/AC-03). Schnellster
   Weg zu echtem, warmem Klang mit klarer Lizenz.
2. Nacht und Tiefe zusätzlich als prozedurales Ambient (Drone, Wind, Glocke) erzeugen, falls die Bibliotheken dafür
   nichts Ruhiges liefern; eigene Rechte, passt zum Klangbild „Glocken, gedämpft“.
3. Weg 4 erst, wenn eigene Melodien gewünscht sind (eigene Erkennungsmelodie, Leitmotiv); vorher SoundCraft in einer
   kurzen Probe-Session auf CLI-Bounce prüfen, sonst FluidSynth.
4. KI-Musik nicht einsetzen, solange Rechte am Ergebnis und Bedingungen der Anbieter unsicher sind.

Entscheidung: 🧑 (B-936, offene Frage „Welcher Weg?“).
