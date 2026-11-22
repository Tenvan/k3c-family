# B-1219 · Spieltest-Start nimmt Raummikro und Spielbild mit ffmpeg auf und startet das Spiel im Spieltest-Modus

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ64
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Workbench kann das Spiel starten (heute nur „Im Spiel ansehen“ in `internal/studioview/game.go`, EXE mit festem Pfad `rust/target/release/k3c-game.exe` bzw. `dist/k3c-game/k3c-game.exe`; wandert mit WZ63 ins Studio). Eine Aufnahme gibt es nicht. Ein Replay kann das Bild nicht nachträglich rendern (spielt nur Go-Traces), der F12-Screenshot lässt Sprites weg. Geprüft 2026-11-22: ffmpeg 9.0.2 (gyan full build) ist installiert, mit `ddagrab`, `h264_qsv` und `h264_amf`; GPU Intel Arc Pro, CPU Core Ultra 7 165H, keine NVIDIA-GPU.

## Ziel

Ein Knopf **Spieltest starten** auf der Seite Spieltest (B-1218) nimmt Sprache und Spielbild auf und startet das Spiel im Spieltest-Modus; endet das Spiel, endet die Aufnahme. Ralf muss vom Sofa aus nichts weiter bedienen.

## Beteiligte und Zielgruppen

Ralf (🧑) und Mitspieler am TV; die Aufnahme ist Grundlage für WZ65.

## Anforderungen

- Seite: Auswahl Mikrofon (DirectShow-Geräte aus `ffmpeg -list_devices`) und Monitor (TV), gemerkt zwischen Starts; Knopf Spieltest starten, Knopf Stopp, Laufzeit-Anzeige.
- Ein ffmpeg-Prozess mit festem Pfad aus dem PATH (Firewall-Regel: keine Netzwerk-Ausgabe): Sprache als `voice.wav` (mono, 16 kHz, PCM, ohne Spielton), Bild als `video.mkv` per `ddagrab` vom TV-Monitor, 1080p30 H.264 mit `h264_qsv`; fällt QSV aus, Fallback `libx264 -preset ultrafast` 720p30 (in `session.json` vermerkt).
- Danach Start der Spiel-EXE mit `--playtest <ordner>` (B-1220), Start- und Endzeit in `session.json` (Unix-ms).
- Spielende (Prozess beendet) oder Stopp → ffmpeg sauber beenden (`q` auf stdin), Stand „aufgenommen“.
- Laufende Aufnahme ist in der Workbench sichtbar (Kopfzeile), damit sie nicht vergessen wird.
- Logging: 🚀 Start, 🛑 Stopp, ❌ ffmpeg-Fehler mit letzter Zeile von stderr.

## Nicht-Ziele

- Transkript, Bilder, Auswertung (WZ65).
- Mini-PC am TV als eigener Aufnahmerechner (später eigenes Ticket, falls gebraucht).
- Sprecher-Erkennung, Kopfhörer je Spieler.

## Regeln und Einschränkungen

- Frame-Budget: 60 Hz mit zwei Ansichten; die Aufnahme darf es nicht reißen, gemessen im Abnahme-Ticket von WZ65.
- Keine Live-KI: während der Aufnahme wird nichts ausgewertet.
- Ordner-Vertrag: B-1218 › Anforderungen.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/frontend/src/playtest/**`
- **Abhängig von:** B-1218, B-1220
- **Braucht 🧑:** Freigabe, Gerät
- **Größe:** M
- **Sperre:** –

## Beispiele

- Ralf wählt Raummikro und TV, klickt Spieltest starten → Spiel läuft im Vollbild am TV, nach Spielende liegen `voice.wav`, `video.mkv`, `session.json` und `markers.jsonl` im neuen Session-Ordner.
- QSV nicht verfügbar → Aufnahme läuft mit libx264 720p30, `session.json` nennt den Encoder.

## Ausnahme- und Fehlerfälle

- ffmpeg fehlt oder Gerät belegt → kein Spielstart, Meldung mit Grund auf der Seite.
- Spiel stürzt ab → Aufnahme endet trotzdem sauber, Stand „aufgenommen“, Absturz in `session.json`.
- Workbench wird während der Aufnahme beendet → ffmpeg wird mit beendet (Job-Objekt); `video.mkv` bleibt abspielbar (MKV), `voice.wav` wird beim nächsten Laden repariert oder als beschädigt markiert.
- Zweiter Klick auf Spieltest starten während einer Aufnahme → gesperrt.

## Akzeptanzkriterien

- **AC-01** Aufbau des ffmpeg-Aufrufs (QSV und Fallback) und der Ablauf Start/Ende sind per Unit-Test mit Fake-Prozessen geprüft.
- **AC-02** Ein 2-Minuten-Testlauf am Gerät ergibt abspielbare `voice.wav` (16 kHz mono) und `video.mkv` (`ffprobe` zeigt H.264, 30 fps).
- **AC-03** Spielende stoppt die Aufnahme ohne Klick, Stand auf der Seite wird „aufgenommen“.
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Verworfen: gdigrab (nur CPU, kostet vermutlich Frames), Bild aus Replay (Replay rendert keine Spieler-Eingaben).
