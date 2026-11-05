# B-696 · Gebäude liegen in allen vorgesehenen Auflösungen als Grafik vor

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-11-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Gebäude-Grafiken (`public/grafik/gebaeude/`, Index `index.json`) sind noch nicht in allen Auflösungen gerendert. B-685 (GRJ2) skaliert eine zugeordnete Grafik auf ein Zielmaß, erzeugt aber keine Varianten; GEB9 deckt Ausbaustufen ab, keine Auflösungen. Welche Auflösungen vorgesehen sind (z. B. 32 und 128 Pixel), ist nicht festgelegt.

## Ziel

Jedes Gebäude (je Slot und Stufe) liegt in allen festgelegten Auflösungen vor; B-685 wählt daraus passend zum Zielmaß. Die feineren Varianten sind dadurch im Spiel sichtbar besser.

## Beteiligte und Zielgruppen

🧑 Ralf legt die Auflösungen fest und nimmt ab; Sessions DEV (Asset-Studio, Erzeugung) und CLI (Auswahl im Spiel).

## Anforderungen

- Liste der Auflösungen als Daten, nicht im Code.
- Erzeugung im Asset-Studio aus Beschreibung und Prompt (Entscheidung 004, B-435), nicht von Hand.
- Fehlende Varianten je Slot und Stufe sind im Studio sichtbar.

## Nicht-Ziele

Zielmaß und Skalierung (B-685), Ausbaustufen (GEB9), Figuren und Gegner (GRA).

## Regeln und Einschränkungen

GEB-Anforderung: alte Grafik, die ersetzt wird, im selben Sprint löschen (`docs/projekte/GEB-gebaeude-grafik.md`). Datei ≤ 400 Zeilen.

## Beispiele

Taverne Stufe 2: 32 px und 128 px liegen vor → Spiel wählt nach Zielmaß aus B-685.

## Ausnahme- und Fehlerfälle

Variante fehlt → Spiel nimmt die vorhandene und skaliert (B-685), das Studio meldet die Lücke.

## Akzeptanzkriterien

- **AC-01** Für jeden Gebäude-Slot und jede Stufe liegt jede festgelegte Auflösung vor (Prüfung gegen `index.json`).
- **AC-02** Das Studio listet fehlende Varianten je Slot und Stufe.

## Offene Fragen

Welche Auflösungen? Reihenfolge zu GEB9 und B-685? Entscheidet 🧑.

## Notizen

Angelegt aus einer Frage von Ralf im Chat (2026-11-05).
