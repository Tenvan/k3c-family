# MON2.4 · Abnahme am Handy

- **Status:** verworfen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** mon2/4-abnahme-handy
- **Abhängig von:** –
- **Tickets:** B-282
- **Kriterien:** AC-05

## Ziel

🧑 hat die Monitoring-Seite am Handy während eines Lasttests oder Spieleabends angesehen.

## Kontext

Server mit `K3C_STATUS_TOKEN`, Last über `task load` oder laufende Räume; Handy im selben Netz, Landingpage öffnen,
Kachel „Monitor“, Token einmal eingeben.

## Erlaubte Dateien

- Planungsdateien

## Nicht-Ziele

Änderungen am Code (Befunde → Ticket).

## Schritte

1. Last starten, Seite am Handy öffnen, Token eingeben.
2. Übersicht (Ampel), Verlauf (5 min / 1 h, Perzentile) und Ereignisse ansehen; ein Ereignis antippen.
3. Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-05: Abnahme durch 🧑 im Ergebnis vermerkt.

## Prüfen

Am Gerät, siehe Schritte.

## Ergebnis

2026-10-07, **PC-Nachweis, Handy offen.** Geprüft von 🧑 (Ralf) im Interview mit Agent (Claude Opus 5.5), Branch `sprint/mon2`. Browser am PC, Landingpage über Vite (Port 5173) → Kachel „Monitor“; Spielserver über k3c-dev mit Test-Token (`K3C_STATUS_TOKEN`), Last über `task load -- -url http://localhost:8080 -rooms 2 -players 3 -duration night`.

- **Schritt 1 (🧑):** Kachel „Monitor“ öffnet die Seite, Token angenommen und nirgends sichtbar, Übersicht mit Ampel und Zahlen passend zur Last; Fenster auf Handybreite ohne waagerechtes Scrollen.
- **Schritt 2 (🧑):** Verlauf mit Diagramm und p50/p95/p99/Max, Wechsel 5 min ↔ 1 h und Raumwahl wirken; Ereignisliste mit Filtern, ein angetipptes Ereignis springt in den Verlauf und ist markiert.
- **AC-05 am PC: geprüft** (🧑). Keine Abweichungen, keine neuen Tickets.
- **Offen:** Die Abnahme am Handy fehlt; die Session bleibt `offen`, AC-05 steht weiter als „angenommen, Validierung offen“.

2026-10-08, **verworfen (PJ3.2, B-359):** Die Abnahme am Gerät ist nach HW1.6 verschoben (Projekt ABN); bisherige Nachweise oben gelten weiter.
