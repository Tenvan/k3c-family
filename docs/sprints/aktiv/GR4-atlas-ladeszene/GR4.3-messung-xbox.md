# GR4.3 · Kaltstart auf der Xbox messen und Budget festlegen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** gr4/3-messung-xbox
- **Abhängig von:** GR4.2
- **Tickets:** B-163
- **Kriterien:** AC-04

## Ziel

🧑 hat auf der Xbox (Edge, Heimnetz) die Zeit vom Öffnen bis zum Menü gemessen und das Kaltstart-Budget in Sekunden festgelegt; Wert und Messung stehen in B-163 und in dieser Datei.

## Kontext

**Controller zurückgestellt (B-314, 2026-10-06):** 🧑 prüft vorerst nur am PC mit Tastatur und Maus (1 Spieler; 2 Spieler an einer Tastatur erst mit B-316). Controller-, Vibrations- und Xbox-Schritte dieser Session sind nach B-314 verschoben; die Kriterien bleiben, ihr Controller-Anteil gilt als `angenommen, Validierung offen (B-314)`.

Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews GR4.4. Dauer etwa 15 Minuten. „Kaltstart“ = Browser-Cache geleert bzw. erster Aufruf nach Neustart von Edge. Der Startwert des Budgets kommt aus der ersten Messung (B-163 › Anforderungen), 🧑 legt den Wert fest. Messen mit Stoppuhr oder über die Gamepad-Testseite (`gamepad-test.html`, Berichte nach `reports/`), falls sie dafür eine Zeitmessung bietet; sonst Stoppuhr, dreimal, Median.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), `docs/backlog/B-163-*.md` (Budget und Messwerte), optional `docs/game-design.md` (eine Zeile Budget)
- Sprint-README (nur Tabelle der Sessions)

## Nicht-Ziele

Code-Änderungen; Budget verfehlt → Ticket (INF bzw. CLI).

## Schritte

1. Stand mit Atlas und Lade-Szene (`task serve` oder Pi) im Heimnetz bereitstellen.
2. Auf der Xbox Edge-Cache leeren, `index.html` öffnen, Spiel starten bis Menü/Lobby; dreimal messen.
3. Maximale Texturgröße ablesen (`MAX_TEXTURE_SIZE` in Edge auf der Xbox, z. B. über eine WebGL-Report-Seite); liegt sie unter 4096 px, Ticket für GR4.1 (Startwert angenommen).
4. Budget festlegen (z. B. Median plus Reserve), in B-163 und hier eintragen; liegt der Wert über dem Gewünschten, Ticket.
5. `Status: fertig`.

## Fertig, wenn

- [ ] AC-04: Budget in Sekunden festgelegt, Messung am TV (Xbox) und maximale Texturgröße im Ergebnis und in B-163.

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

2026-10-06, **Tastatur und Maus (Sammelaussage):** 🧑 (Ralf) im Chat: „Alle Tastatur und Maussteuerungen liefen bisher wie definiert.“ Gilt für den Tastatur- und Maus-Anteil dieser Session am PC; Darstellung, Ton und Controller (B-314) sind damit nicht abgenommen, die Session bleibt `offen`.
