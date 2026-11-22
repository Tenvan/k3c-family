# B-1223 · Knopf Transkribieren erzeugt per ffmpeg-whisper ein Transkript mit Zeitstempeln und ein Bild je Marker

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Nach WZ64 liegen je Session `voice.wav`, `video.mkv` und `markers.jsonl` in `playtests/<id>/`. Geprüft 2026-11-22: Das installierte ffmpeg 9.0.2 (gyan full build) hat den Audio-Filter `whisper` (whisper.cpp eingebaut); Python oder eine eigene whisper-EXE sind nicht nötig, nur eine Modelldatei (ggml). Rechner: Core Ultra 7 165H, Intel Arc, keine NVIDIA-GPU.

## Ziel

Ein Klick auf **Transkribieren** macht aus der Sprachspur ein deutsches Transkript mit Zeitstempeln und zieht aus dem Video ein Bild je Marker; keine KI während des Spiels, nur auf diese manuelle Aktion.

## Beteiligte und Zielgruppen

Ralf (🧑) löst aus und liest; Claude nutzt das Ergebnis in B-1224.

## Anforderungen

- Knopf **Modell laden** auf der Seite Spieltest: lädt `ggml-large-v3-turbo-q5_0.bin` (Quelle: Hugging Face `ggerganov/whisper.cpp`, Größe und SHA-256 in der Session prüfen und fest eintragen) nach `models/whisper/` (gitignored), mit Fortschritt; vorhandenes Modell wird erkannt.
- Knopf **Transkribieren** je Session: `ffmpeg -i voice.wav -af whisper=model=…:language=de:format=json:destination=transcript.json` als Hintergrundauftrag mit Fortschritt und Abbrechen; Arbeitsordner = Session-Ordner (relative Pfade, damit `C:` im Filter nicht gequotet werden muss).
- `transcript.json`: Segmente mit Start/Ende (Unix-ms, aus Startzeit in `session.json` umgerechnet) und Text.
- Danach je Marker aus `markers.jsonl` ein Bild `frames/marker-<nr>.png` aus `video.mkv` (ffmpeg `-ss`), Stand „transkribiert“.
- Seite zeigt das Transkript als Zeitleiste mit Markern und Bildern; Klick springt im Video-Player an die Stelle.
- Start der Workbench prüft ffmpeg-Version ≥ 8 und Filter `whisper`; fehlt er, ist der Knopf gesperrt mit Hinweis.
- `requirements.md`: ffmpeg ≥ 8 mit whisper-Filter (gyan full build) und Modell `ggml-large-v3-turbo-q5_0`.

## Nicht-Ziele

- Live-Transkription während der Aufnahme.
- Sprecher-Erkennung.
- Cloud-Dienste (Familienstimmen bleiben lokal).
- Python/faster-whisper (verworfen 🧑 2026-11-22).

## Regeln und Einschränkungen

- Neue Abhängigkeit (Modelldatei, ffmpeg-Mindestversion): Freigabe durch 🧑, nicht Klasse A.
- Schreiben nur im Repo (`models/`, `playtests/`).
- Ordner-Vertrag: B-1218 › Anforderungen.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/frontend/src/playtest/**`, `requirements.md`, `.gitignore`
- **Abhängig von:** WZ64, B-1220
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

- 45 min Spieltest mit 6 Markern → nach Klick liegen `transcript.json` und 6 Bilder vor; die Zeitleiste zeigt Sätze und Marker in zeitlicher Folge.

## Ausnahme- und Fehlerfälle

- Modell fehlt → Transkribieren gesperrt, Hinweis auf Modell laden.
- Download bricht ab oder Prüfsumme falsch → Datei verworfen, Meldung, erneuter Versuch möglich.
- Video schon gelöscht → Transkript läuft, Bilder entfallen mit Hinweis.
- Abbrechen → kein halbes `transcript.json` bleibt liegen.

## Akzeptanzkriterien

- **AC-01** Unit-Test: Aufbau der ffmpeg-Aufrufe (whisper, Bild an Zeitpunkt) und Umrechnung der Zeitstempel.
- **AC-02** Ein Test-WAV mit bekanntem deutschem Satz ergibt ein Transkript, das den Satz enthält (Test mit `-short` übersprungen, wenn Modell fehlt).
- **AC-03** Dauer für 10 min Sprache ist gemessen und im Ergebnis der Session notiert.
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Vermutet: q5_0-Modell ca. 550 MB, CPU-Tempo etwa Echtzeit; AC-03 misst es.
