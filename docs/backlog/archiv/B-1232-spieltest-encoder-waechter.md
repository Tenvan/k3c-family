# B-1232 · Spieltest nimmt mit nutzbarem Encoder auf, überwacht die Aufnahme und zeigt Ton- und Bild-Rate

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im ersten echten Spieltest blieb `voice.wav` kurz und `video.mkv` leer. Ursache: Der Recorder (`tools/k3c-workbench/internal/playtest/record.go`) wählte fest den Intel-Encoder h264_qsv, der auf dem AMD-Rechner nicht startet. ffmpeg endete nach etwa 0,5 s, die Aufnahme lief unbemerkt weiter. Das Transkript lag nur als `transcript.json` vor.

## Ziel

Der Spieltest nimmt auf jedem Rechner mit dem besten nutzbaren Encoder auf; bricht die Aufnahme ab, sieht der Spieler es sofort statt erst nach dem Test.

## Beteiligte und Zielgruppen

Ralf (🧑) und Tester, die am PC Spieltests aufnehmen.

## Anforderungen

- Encoder-Wahl per Probe-Encode: NVENC, AMF, QSV, sonst libx264.
- Wächter misst jede Sekunde Bytes in `voice.wav` und `video.mkv` und warnt bei Stillstand oder beendetem ffmpeg.
- Zwei Diagramme (Ton, Bild in KB/s) und Warnkarte in der Aufnahmeleiste.
- Matroska und WAV werden laufend geschrieben (`-flush_packets 1`, `-cluster_time_limit 1000`).
- Nach der Auswertung steht zusätzlich `transcript.txt` (`[mm:ss] Satz`).

## Nicht-Ziele

Aufnahme ohne Ton (Idee, noch kein Ticket); Änderungen am Spiel selbst.

## Regeln und Einschränkungen

Schreiben nur im Repo; Datei ≤ 400 Zeilen; Logs mit Emoji.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/frontend/src/playtest/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

AMD-Rechner: Probe verwirft NVENC und QSV, wählt AMF, die Aufnahme wächst. Mikro abgezogen: nach 4 s „Ton kommt nicht mehr an“.

## Ausnahme- und Fehlerfälle

ffmpeg stirbt: Warnung mit letzter Meldung, `session.json` trägt den Fehler. Kein Hardware-Encoder: libx264 in 720p.

## Akzeptanzkriterien

- **AC-01** `go test ./internal/playtest` ist grün (Encoder-Wahl, Wächter, transcript.txt).
- **AC-02** Echte Aufnahme am AMD-Rechner: `voice.wav` und `video.mkv` wachsen laufend (mit ffmpeg geprüft).
- **AC-03** Aufnahmeleiste zeigt zwei Diagramme und die Warnung (Vitest `src/playtest`).
- **AC-04** Nach der Auswertung liegt `transcript.txt` neben `transcript.json`.
- **AC-05** Abnahme durch Ralf: Spieltest mit Mikro, Dateien vollständig.

## Offene Fragen

keine

## Notizen

QSV ist auf diesem AMD-Rechner nicht nutzbar, AMF läuft. Das Bluetooth-Headset muss verbunden sein, sonst findet dshow kein Mikro.
