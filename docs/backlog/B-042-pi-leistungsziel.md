# B-042 · Pi-Modell und Leistungsziel sind festgelegt

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** LT1
- **Projekt:** WZG
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑 (mit LT1)

## Ausgangslage

Für den Pi-Betrieb (B-035) sind weder Modell noch Leistungsziel festgelegt.

## Ziel

Pi-Modell und Leistungsziel sind festgelegt. Nutzen: Ohne Ziel ist die Lastmessung (LT1) nicht bewertbar.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

- Modell festlegen.
- Zielwerte festlegen: Räume × Spieler, Tick-Dauer p99.

## Nicht-Ziele

Einrichtung des Pi (B-035).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

nicht relevant – reine Entscheidung, kein Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Entscheidung, kein Verhalten.

## Akzeptanzkriterien

- **AC-01** Modell und Zielwerte stehen in diesem Ticket.

## Offene Fragen

Pi-Modell genau (🧑 bestätigt mit der Freigabe von SP11): Angabe „Pi 3 oder älter“ vom 2026-10-02; arm64 braucht mindestens einen Pi 3 mit 64-Bit-Betriebssystem.

## Notizen

**Festgelegt (🧑, 2026-10-02):** Modell Pi 3 oder älter (vermutlich Pi 3, 64-Bit-OS). Ziel: 2 Räume × 3 Spieler parallel, Tick-Dauer p99 < 10 ms bei 30 Hz (Budget 33 ms). Gemessen wird mit dem Lasttest-Werkzeug (B-175) im Sprint LT1.3; die erste Handmessung vom 2026-10-03 (Tag p99 8,0 bis 9,5 ms, Nacht 10,2 bis 10,3 ms, ein Messpunkt) liegt knapp am Ziel.
