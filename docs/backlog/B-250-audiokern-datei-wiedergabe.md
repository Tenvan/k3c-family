# B-250 · Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SO5
- **Projekt:** SND
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/audio/audioCore.ts` spielt nur Sprites des Atlas (`play(name, bus)`), `ctx` und `mixer` sind privat. Die Hörprobe (`src/tools/soundtest.ts`, SO3.2) darf den Kern nicht ändern und legt deshalb einen eigenen `AudioContext` mit eigenem `Mixer` an; Formatwahl (`pickFile`) und Mixer sind wiederverwendet.

## Ziel

Der Audio-Kern bietet Wiedergabe ganzer Dateien (Schleife, Crossfade über Kurve) auf einem Bus; Hörprobe und später die Musik (SO4) nutzen denselben Kontext und Mixer.

## Beteiligte und Zielgruppen

Entwickler; 🧑 entscheidet, ob es vor SO4 nötig ist.

## Anforderungen

- Datei laden und auf Bus `music` in Schleife spielen, Crossfade zweier Stücke ohne Sprung (Kurve `crossfadeCurves` aus `src/tools/soundtestLogic.ts` in den Kern verschieben).
- Hörprobe nutzt den Kern, kein zweiter `AudioContext`.

## Nicht-Ziele

Bearbeiten oder Mischen von Sounds; Auswahl der Musik (SO4).

## Regeln und Einschränkungen

Audio rechnet keine Spielregel; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Zwei Stücke werden über 2 s überblendet, der Mixer-Bus `music` bleibt der einzige Lautstärkeregler.

## Ausnahme- und Fehlerfälle

Datei lädt nicht → kein Ton, Fehler im Log, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `audioCore().playTrack(...)` und Crossfade sind mit Vitest (gemockter Kontext) geprüft; `soundtest.ts` legt keinen eigenen `AudioContext` mehr an.

## Offene Fragen

Zeitpunkt (vor SO4?), entscheidet 🧑.

## Notizen

–
