# B-1241 · Marker-Bilder werden vorab lokal beschrieben, damit der Chat sie nicht ansehen muss

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Chat sieht Marker-Bilder mit playtest_frame und verbraucht dabei Tokens. Ein lokales Bildmodell könnte jedes Bild vorab in Text fassen (Menü, Werte, Inhalt).

## Ziel

Beim Auswerten entsteht je Marker-Bild eine kurze Textbeschreibung in der Session; der Chat liest zuerst den Text und sieht nur bei Bedarf das Bild.

## Beteiligte und Zielgruppen

Ralf (🧑), Claude als Leser der Session.

## Anforderungen

- Lokales Modell, kein Cloud-Dienst: Kandidaten sind ein Vision-Sprachmodell kleiner Größe (z. B. Moondream oder Qwen-VL über llama.cpp) und Texterkennung (OCR) für UI-Text; die Entscheidung fällt in der Spec.
- Beschreibung steht in markers/transcript-Daten und in playtest_get.
- Modell wie bei whisper aus models/ wählbar, nie automatisch geladen.
- Dauer fließt in die Statistik von B-1239.

## Nicht-Ziele

Cloud-Modelle; Beschreibung von Videos.

## Regeln und Einschränkungen

Neue Abhängigkeit: Freigabe durch Ralf; Modell und Laufzeit im Repo ignoriert (models/).

## Beispiele

„Optionen-Menü, Zeile ‹Sprache› markiert, Musik 20 %“.

## Ausnahme- und Fehlerfälle

Modell fehlt: Bilder bleiben ohne Beschreibung, Hinweis wie bei whisper.

## Akzeptanzkriterien

- **AC-01** Spec: Modell- und Laufzeitwahl entschieden und von Ralf freigegeben.
- **AC-02** Umsetzung nach freigegebener Spec.

## Offene Fragen

Welches Modell und welche Laufzeit (llama.cpp, ONNX, OCR), und reicht OCR allein?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
