# B-494 · Ein Werkzeug packt die gewählten Sounds in einen Sound-Atlas

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SO6
- **Projekt:** SND
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Audio-Kern kann einen Sound-Atlas (`public/audio/atlas.*`, SO1.2) lesen, die gewählten Sounds liegen aber als Einzeldateien unter `public/audio/kandidaten/` (SO2.2, Slot-Register `data/assets.json`). Ein Werkzeug, das sie zu einem Atlas zusammenfügt, fehlt. Jede Datei ist eine eigene Anfrage.

## Ziel

Die aktiven Effekt-Sounds eines Builds stecken in einem Atlas aus dem Slot-Register; die Zahl der Audio-Anfragen beim Start bleibt klein (SO1 AC-05).

## Beteiligte und Zielgruppen

Entwickler (🧑) und der Build; Spieler profitieren von kürzerer Ladezeit am TV.

## Anforderungen

- Ein Task (`task`) erzeugt `atlas.ogg`, `atlas.mp3` und `atlas.json` aus den ersten Zuordnungen der Effekt-Slots (Bus `sfx`).
- Kurze Töne, Format mit Fallback wie SO1.2; die Größe des Atlas steht im Ergebnis.
- Musik bleibt Einzeldatei (wird je Zustand geladen, SO4.3).

## Nicht-Ziele

Auswahl der Sounds (SO2.1), Einbau ins Spiel (SO2.3), Musik-Ladestrategie (SO4.3).

## Regeln und Einschränkungen

Befehle nur über `task`; Domäne INF; Komplexitäts-Budget; keine neue Abhängigkeit ohne Zustimmung von 🧑 (ffmpeg ist schon Voraussetzung der Atlas-Erzeugung, `public/audio/README.md`).

## Beispiele

Zwölf Effekt-Slots → ein Atlas mit zwölf Einträgen und je Start und Dauer in `atlas.json`.

## Ausnahme- und Fehlerfälle

Slot ohne Datei oder Datei nicht dekodierbar → Task bricht mit dem Slot-Namen ab, schreibt keinen halben Atlas.

## Akzeptanzkriterien

- **AC-01** `task` erzeugt den Atlas aus dem Slot-Register; ein Test prüft, dass `atlas.json` jeden aktiven Effekt-Slot enthält.
- **AC-02** Beim Spielstart lädt der Client für die Effekte höchstens eine Audio-Datei je Format (Beobachtung im Browser-Pane).

## Offene Fragen

- Lohnt sich der Atlas überhaupt, oder reichen Einzeldateien mit Vorladen bei der ersten Geste? Entscheidet 🧑 nach der Messung am TV (SO2.5).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
