# GR4.3 · Kaltstart auf der Xbox messen und Budget festlegen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** gr4/3-messung-xbox
- **Abhängig von:** GR4.2
- **Tickets:** B-163
- **Kriterien:** AC-04

## Ziel

🧑 hat auf der Xbox (Edge, Heimnetz) die Zeit vom Öffnen bis zum Menü gemessen und das Kaltstart-Budget in Sekunden festgelegt; Wert und Messung stehen in B-163 und in dieser Datei.

## Kontext

Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews GR4.4. Dauer etwa 15 Minuten. „Kaltstart“ = Browser-Cache geleert bzw. erster Aufruf nach Neustart von Edge. Der Startwert des Budgets kommt aus der ersten Messung (B-163 › Anforderungen), 🧑 legt den Wert fest. Messen mit Stoppuhr oder über die Gamepad-Testseite (`gamepad-test.html`, Berichte nach `reports/`), falls sie dafür eine Zeitmessung bietet; sonst Stoppuhr, dreimal, Median.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), `docs/backlog/B-163-*.md` (Budget und Messwerte), optional `docs/game-design.md` (eine Zeile Budget)
- Sprint-README (nur Tabelle der Sessions)

## Nicht-Ziele

Code-Änderungen; Budget verfehlt → Ticket (INF bzw. CLI).

## Schritte

1. Stand mit Atlas und Lade-Szene (`task serve` oder Pi) im Heimnetz bereitstellen.
2. Auf der Xbox Edge-Cache leeren, `index.html` öffnen, Spiel starten bis Menü/Lobby; dreimal messen.
3. Budget festlegen (z. B. Median plus Reserve), in B-163 und hier eintragen; liegt der Wert über dem Gewünschten, Ticket.
4. `Status: fertig`.

## Fertig, wenn

- [ ] AC-04: Budget in Sekunden festgelegt, Messung am TV (Xbox) im Ergebnis und in B-163.

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

–
