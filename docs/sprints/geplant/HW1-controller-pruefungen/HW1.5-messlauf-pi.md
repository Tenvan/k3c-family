# HW1.5 · Messlauf am Pi über eine Nacht (aus LT1.3)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** hw1/5-messlauf-pi
- **Abhängig von:** –
- **Tickets:** B-175, B-042
- **Kriterien:** AC-04

## Ziel

🧑 hat am Pi 2 Räume × 3 Spieler über eine Nacht gemessen; Tabelle und Bewertung stehen in dieser Datei und in B-042.

## Kontext

Übernommen aus LT1.3 (PJ3, 2026-10-08); dort steht der PC-Lauf vom 2026-10-07 (p99 < 10 ms erreicht, sagt nichts über den Pi). Bis zur Messung gilt das angenommene Ziel aus B-042 (Tick-p99 < 10 ms bei 30 Hz, 2 Räume × 3 Spieler; Handmessung 2026-10-03: Nacht 10,2 bis 10,3 ms). Etwa 15 Minuten Arbeit plus eine Nacht Laufzeit. Der Pi-Server braucht `K3C_STATUS_TOKEN`. Gestartet wird der Lauf über `sim_test` der Workbench (B-348), nicht über `task load` in der Shell; ein Agent kann den Bericht danach auswerten.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), `docs/backlog/B-042-*.md` (Ergebnis, Status, Archiv), `docs/backlog/README.md`, HW1-README (Tabelle)

## Nicht-Ziele

Code-Änderungen; Optimierungen am Server. Ziel verfehlt → Ticket (SRV), keine stille Absenkung des Ziels.

## Schritte

1. Lauf gegen den Pi starten (2 Räume × 3 Spieler, eine Nacht).
2. Tabelle und Bewertung ins Ergebnis und in B-042 übernehmen.
3. Bewertung `verfehlt` oder `knapp` → Ticket mit Messwerten (SRV).
4. B-042 `erledigt` (archiviert das Tool), `Status: fertig`.

## Fertig, wenn

- [ ] AC-04: Tabelle und Bewertung des Pi-Laufs stehen im Ergebnis und in B-042; B-042 ist archiviert.

## Prüfen

Manuell durch 🧑 am Pi.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
