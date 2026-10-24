# B-600 · Der AI-Chat unterstützt die Zuordnung auf Sound- und Grafik-Seite

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ29
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Studio hat einen Chat rechts (WZ19, Claude Code CLI), der bisher vor allem Welten füllt. Ralf (2026-10-24): Sound- und Grafik-Seite sollen vom AI-Chat unterstützt werden; die Figuren und Grafiken sollen später damit besser gelöst werden.

## Ziel

Auf Sound- und Grafik-Seite kennt der Chat den Kontext und kann Zuordnungen vorbereiten, die Ralf bestätigt.

## Beteiligte und Zielgruppen

Ralf (bestätigt); Agent DEV; Claude als Modell des Chats (braucht Internet; offline ist der Chat aus und die Seiten funktionieren weiter).

## Anforderungen

- Kontext: Der Chat bekommt automatisch Seite, gewählten Slot oder Szene, Filter, Slot-Details (B-594: Definition, Pack, Lizenz, Verwendung) und Hörprobe/Vorschau-Stand.
- Aktionen als Vorschläge, nie stilles Schreiben: Kandidaten suchen (`assets_research`), Kandidaten je Slot vorschlagen, Platz 1 ändern, Lücken füllen (z. B. passiver Skill ohne Ton), Credits prüfen, Atlas neu erzeugen; jeder Vorschlag erscheint als Karte mit Vorschau/Hörprobe und „Übernehmen“/„Verwerfen“.
- Mehrere Vorschläge in einer Liste (z. B. „Töne für alle passiven Skills“), einzeln übernehmbar.
- Downloads fremder Dateien nur mit Bestätigung durch Ralf (Quelle, Datei, Größe, Lizenz sichtbar), nur erlaubte Lizenzen (Q70).
- Schreibt nur über das Studio-Backend in `data/assets.json` und `public/`; Entwurfsschutz wie beim Welten-Chat (WZ21).
- Ohne Internet: Chat-Bereich zeigt „offline“, alle Seiten bleiben nutzbar.

## Nicht-Ziele

Autonomes Beschaffen ohne Bestätigung; Welt-Chat (WZ19); eigene Modelle.

## Regeln und Einschränkungen

Chat-Bausteine aus `tools/k3c-studio/frontend/src/chat` (`chatBridge.ts`, Aufträge wie `chatTask`), Backend `tools/k3c-studio/backend` (Chat, assetkit); Lizenzregel Q70, Credits-Test; Prompt-Injection aus Webseiten gilt als Daten, nie als Auftrag.

## Beispiele

Slot `sfx:skill:healer` leer → „Vorschlag suchen“ → drei Kandidaten mit Hörprobe, Ralf übernimmt einen, Credit und Atlas folgen.

## Ausnahme- und Fehlerfälle

Chat nicht erreichbar → „offline“. Vorschlag widerspricht der Lizenzregel → Karte zeigt es und lässt „Übernehmen“ nicht zu.

## Akzeptanzkriterien

- **AC-01** Der Chat erhält den Seitenkontext (Test der Kontext-Aufbereitung).
- **AC-02** Vorschläge erscheinen als Karten mit Vorschau/Hörprobe und werden nur nach „Übernehmen“ geschrieben (Go-Test, Vitest).
- **AC-03** Lizenz- und Download-Regeln sind erzwungen (Test).
- **AC-04** Ohne Internet bleiben Seiten nutzbar, Chat zeigt „offline“.
- **AC-05** Ralf nimmt den Ablauf für einen Ton und eine Grafik am PC ab.

## Offene Fragen

Welche Aktionen zuerst: Default Kandidaten suchen, Platz 1 ändern, Lücken füllen; Ralf ergänzt.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ29. Zusammenhang: WZ19, WZ21, B-576/SO8, B-594.
