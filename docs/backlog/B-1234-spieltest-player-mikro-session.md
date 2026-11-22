# B-1234 · Spieltest: Player im Fenster, Mikro-Session ohne Spiel, bessere Transkription

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im Workbench-Fenster fehlten die Player für `voice.wav` und `video.mkv` (nur im Browser-Betrieb), sie standen unter den Kennzahlen. Eine Aufnahme ohne Spiel gab es nicht. Das Transkript war schlecht: Der Whisper-Filter erkennt standardmäßig Stücke von 3 s ohne Zusammenhang und erfindet bei Pausen „Vielen Dank.“.

## Ziel

Ralf prüft Ton und Bild selbst im Fenster, kann nur das Mikro aufnehmen (diktieren, testen) und bekommt ein brauchbares Transkript.

## Beteiligte und Zielgruppen

Ralf (🧑) als Tester.

## Anforderungen

- Audio- und Video-Player oben in der Session-Detailansicht, auch im Wails-Fenster (Asset-Server-Handler nur für Media- und Marker-Bild-Adressen).
- „Nur Mikro starten“: nimmt nur `voice.wav` auf, kein Spiel, kein Monitor, nur das Ton-Diagramm; Stopp beendet die Session.
- Whisper mit `queue=20` statt 3 s.

## Nicht-Ziele

Anderes oder größeres Modell (Modellwahl gibt es schon); Rauschfilter vor Whisper (an der Aufnahme kein Vorteil gemessen).

## Regeln und Einschränkungen

Schreiben nur im Repo; Dateien ≤ 400 Zeilen.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/frontend/src/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/main.go`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Session auswählen: oben Ton- und Bild-Player. „Nur Mikro starten“: Aufnahme läuft ohne Spiel, Stopp beendet, danach auswerten.

## Ausnahme- und Fehlerfälle

Mikro-Session ohne Spiel-EXE startet trotzdem. Mikro fällt aus: Warnung „Ton kommt nicht mehr an“, Session endet mit Fehlervermerk.

## Akzeptanzkriterien

- **AC-01** Detailansicht zeigt beide Player oberhalb der Kennzahlen, URLs wie im Client/Server-Betrieb (Vitest).
- **AC-02** `Options.VoiceOnly` startet nur ffmpeg ohne `ddagrab`, ohne Spiel; Stopp beendet sauber (Go-Test).
- **AC-03** Mikro-Session zeigt nur das Ton-Diagramm und warnt nicht wegen des Bilds (Go- und Vitest).
- **AC-04** `WhisperArgs` enthält `queue=20`; an Session 2026-11-22-5 liefert 20 s die besten Sätze (Messung dokumentiert).
- **AC-05** Abnahme durch Ralf: Player spielen im Fenster ab (Video mit Springen), Mikro-Session und Auswertung.

## Offene Fragen

Ob WebView2 `video.mkv` (H.264 in Matroska) im Fenster abspielt, ist nicht geprüft.

## Notizen

Messung 44 s Aufnahme, Pegel mittel −23,6 dB, Spitze −0,5 dB. Standard 3 s: Sätze zerhackt, „Vielen Dank.“; Pegelanhebung (dynaudnorm) brachte nichts; queue 10 und 20 deutlich besser.
