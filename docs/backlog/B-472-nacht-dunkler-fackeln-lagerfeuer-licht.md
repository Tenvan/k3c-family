# B-472 · Nacht und Morgengrauen sind dunkler, Fackeln und Lagerfeuer machen Licht

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W13
- **Projekt:** WRT
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Der Client zeichnet die Dunkelheit der Nacht über `daylight()` in `src/scenes/render/viewRules.ts` (Morgengrauen = letzte 10 % der Nacht). Nacht und Morgengrauen sind nach Ralf (2026-10-17, Chat zu `B-460 › Offene Fragen` 13) zu hell: sie sollen deutlich dunkler werden, dafür sollen Fackeln und Lagerfeuer Licht abgeben. Berührt B-460 (Welt-Betrachter, Tageszeit-Auswahl) und B-470 (Morgengrauen als echte Phase). Die Details (Dunkelstufe, Lichtradius, Lichtquellen) sind offen.

## Ziel

Nacht und Morgengrauen sind im Spiel deutlich dunkler als heute. Fackeln und Lagerfeuer geben Licht ab und hellen ihre Umgebung auf. Nutzen: Die Nacht wirkt bedrohlich und Lichtquellen haben einen spielerischen und optischen Wert. Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Spieler sehen die Nacht im Spiel (alle Spieler, auch Split-Screen). 🧑 entscheidet über Dunkelstufe und Lichtwirkung und nimmt die Darstellung ab. Der Welt-Betrachter im Asset-Studio (B-460) zeigt dieselbe Darstellung.

## Anforderungen

- Nacht und Morgengrauen werden gegenüber heute (`daylight()` in `src/scenes/render/viewRules.ts`) dunkler; die genaue Dunkelstufe legt 🧑 fest.
- Fackeln und Lagerfeuer erhellen ihre Umgebung (Lichtkegel mit Radius); Radius und Stärke sind Werte in `data/`, nicht im Code.
- Die Darstellung ist deterministisch, rein Client-seitig (keine Änderung der Simulation) und läuft mit 2 Spielern im Split-Screen sowie auf der Xbox (Edge) ohne spürbaren Leistungseinbruch.
- Der Welt-Betrachter (B-460) übernimmt die Änderung über dieselbe Funktion `daylight()` und dieselben Renderer; es entsteht keine zweite Kopie.

## Nicht-Ziele

Keine neue Phase Morgengrauen in der Simulation (B-470). Keine Änderung der Spielregeln für Nacht und Gegner. Keine neuen Gebäude oder Objekte; falls es Fackeln oder Lagerfeuer als Spielobjekte noch nicht gibt, ist deren Einführung ein eigenes Ticket.

## Regeln und Einschränkungen

Domäne CLI: Änderungen unter `src/scenes/` und `data/`. Abhängigkeit zu B-460: WZ7.4 testet die Abbildung des Morgengrauens gegen `daylight()` (AC-05). Wer von beiden zuletzt landet, zieht den Test und die Live-Vorschau nach. Verträge (Protokoll, Spielstand) bleiben unverändert. Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- Nacht → der Bildschirm ist deutlich dunkler als heute; um ein Lagerfeuer oder eine Fackel bleibt ein heller Kreis.
- Morgengrauen (letzte 10 % der Nacht) → dunkel, hellt zum Tagesbeginn auf.
- Tag → unverändert.

## Ausnahme- und Fehlerfälle

- Lichtquellen überlappen → die Aufhellung addiert sich nicht über die Helligkeit des Tages hinaus.
- Keine Lichtquelle in Sicht → reine Nachtdunkelheit, Spielelemente bleiben erkennbar (Mindesthelligkeit, Wert legt 🧑 fest).
- Zwei Spieler mit getrennten Kameras → jede Kamera zeichnet ihr Licht selbst.

## Akzeptanzkriterien

Entwurf, Werte offen (siehe Offene Fragen):
- **AC-01** Nacht und Morgengrauen sind dunkler als heute: Vitest auf `daylight()` mit der von 🧑 festgelegten Mindesthelligkeit für Nacht und letzte 10 % der Nacht; der Tag bleibt unverändert.
- **AC-02** Fackeln und Lagerfeuer hellen ihre Umgebung auf: Radius und Stärke stehen in `data/`, Vitest auf die Lichtberechnung (innen heller als außen, überlappende Quellen überschreiten die Tageshelligkeit nicht); Beobachtung durch 🧑.
- **AC-03** Der Welt-Betrachter (B-460) zeigt dieselbe Darstellung ohne zweite Kopie: Test gegen `daylight()` bleibt grün.

## Offene Fragen

Entscheidet 🧑:
1. Wie dunkel sind Nacht und Morgengrauen (Mindesthelligkeit)?
2. Gibt es Fackeln und Lagerfeuer schon als Spielobjekte (Grafik, Platz), oder braucht es dafür ein eigenes Ticket?
3. Radius und Stärke des Lichts je Quelle.
4. Landet B-472 vor oder nach WZ7 (B-460)?

## Notizen

Herkunft: Ralf, 2026-10-17, Chat zu `B-460 › Offene Fragen` 13. Verwandt: B-460 (Welt-Betrachter), B-470 (Morgengrauen als echte Phase).
