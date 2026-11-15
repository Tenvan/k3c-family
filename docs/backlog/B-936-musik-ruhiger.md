# B-936 · Ruhigere, langsamere Musikstücke passend zur Atmosphäre finden oder erzeugen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** SR1
- **Projekt:** SND
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Die Musik ist zu schnell und unruhig für Couch-Abende. Gefragt ist: ruhigere Stücke finden oder selbst erstellen.

## Ziel

Je Musik-Zustand (Tag, Abend, Nacht, Kampf, Menü) ein ruhiges, langsames Stück passend zur Papierschnitt-Atmosphäre, mit geklärter Lizenz.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Wege, abgewogen in einem Variantenvergleich vor der Umsetzung:
1. **Freie Bibliotheken** mit CC0/CC-BY (z. B. OpenGameArt, Freesound, Kevin MacLeod/incompetech, Pixabay Music): schnell, Lizenz je Stück prüfen.
2. **KI-Musik** (z. B. Suno, Udio, ElevenLabs Music, Stable Audio): eigene Stücke nach Beschreibung; Nutzungsrechte des Anbieters prüfen.
3. **Prozedural selbst erzeugt** in Go (Synthese aus Sinus/Rauschen plus Hüllkurven und Hall, rendert `.ogg`): voll eigene Rechte, gut für Ambient und Glöckchen, schwach für echte Melodien.
4. **Komponieren als Noten** (MIDI per Skript) und Rendern mit freiem Soundfont (z. B. FluidSynth): eigene Melodien, warmer Klang, mehr Aufwand.
- „SoundCraft“ gibt es als Unity-Plugin; ein Werkzeug „AniCraft“ ist nicht auffindbar (ungeprüft, Name bei 🧑 nachfragen). Für K3C (Go, kein Unity) taugt das Plugin nicht direkt.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `data/sounds.json`, `public/audio/**`, `native/k3c-game/menu/licensefile/**`
- **Abhängig von:** B-935
- **Braucht 🧑:** Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Variantenvergleich mit Empfehlung, Entscheidung durch 🧑.
- **AC-02** Neue Stücke je Zustand in `data/sounds.json`, Lizenz im Lizenzfile.
- **AC-03** Abhören am TV durch 🧑.

## Offene Fragen

Welcher Weg (1–4)? Entscheidung nach dem Variantenvergleich (🧑).

## Notizen

SoundCraft (ArtCraft, https://craft.wiki/apps/soundcraft/overview/, gelesen 2026-11-15): freie Open-Source-DAW (MIT/Apache-2.0), Windows/macOS/Linux/Web, Pre-Alpha. Eingebaut: Subtraktiv-Synth, Drum-Synth, Sampler, MIDI-Editor, Effekte (EQ, Hall, Delay), Bounce nach WAV/AIFF/FLAC mit LUFS-Angabe; jeder Befehl per Kommandozeile oder KI-Agent steuerbar. Passt zu Weg 4 (Noten per Skript, Rendern in SoundCraft) und Weg 3 (Ambient aus Synth): Claude kann Sessions bauen, mischen und bouncen; für das Spiel danach nach Ogg wandeln (SoundCraft exportiert kein Ogg). Risiko: Pre-Alpha, CLI-Umfang ungeprüft.
