# B-1241 · Marker-Bilder werden vorab lokal beschrieben, damit der Chat sie nicht ansehen muss

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Chat sieht Marker-Bilder mit playtest_frame und verbraucht dabei Tokens. Ein lokales Bildmodell könnte jedes Bild vorab in Text fassen (Menü, Werte, Inhalt).

## Ziel

Beim Auswerten entsteht je Marker-Bild eine kurze Textbeschreibung in der Session; der Chat liest zuerst den Text und sieht nur bei Bedarf das Bild.

## Beteiligte und Zielgruppen

Ralf (🧑), Claude als Leser der Session.

## Anforderungen

- Lokales Modell, kein Cloud-Dienst: Qwen2.5-VL-3B (GGUF samt mmproj) über llama.cpp (llama-mtmd-cli, Vulkan-Build), beides in models/vision/ (ignoriert, nie automatisch geladen).
- Bildmodelle sind wählbar (alle Hauptdateien in models/vision/ mit passender mmproj-Datei); Laufzeit aus models/vision/llama oder K3C_LLAMA_DIR.
- Beschreiben: alle ausgewählten Marker-Bilder (B-1240) oder ein einzelnes Bild; abgewählte nie. Fortschritt, Abbrechen, fertige Beschreibungen bleiben.
- Ergebnis in descriptions.json der Session, in der Zeitleiste unter dem Marker und in playtest_get als „Bildbeschreibung“.

## Nicht-Ziele

Cloud-Modelle; Beschreibung von Videos.

## Regeln und Einschränkungen

Neue Abhängigkeit: Freigabe durch Ralf; Modell und Laufzeit im Repo ignoriert (models/).

## Beispiele

„Optionen-Menü, Zeile ‹Sprache› markiert, Musik 20 %“.

## Ausnahme- und Fehlerfälle

Modell fehlt: Bilder bleiben ohne Beschreibung, Hinweis wie bei whisper.

## Akzeptanzkriterien

- **AC-01** Spec: Modell- und Laufzeitwahl entschieden (Qwen2.5-VL-3B, llama.cpp Vulkan) und von Ralf im Chat freigegeben.
- **AC-02** Go-Tests: Modellliste mit mmproj, nur ausgewählte Bilder, gezielte Nummern, Ablehnungen; Vitest für Modellwahl, Beschreiben und Einzel-Knopf; Paritaet der Bindings grün.
- **AC-03** Abnahme durch Ralf: Beschreibung einer echten Session ist brauchbar (Messung: etwa 26 s je Bild mit Grafikkarte).

## Offene Fragen

keine (Qwen2.5-VL-3B gewählt; OCR allein reicht nicht für Szenen)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
