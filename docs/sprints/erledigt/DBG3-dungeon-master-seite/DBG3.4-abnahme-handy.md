# DBG3.4 · Abnahme am Handy

- **Status:** verworfen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** dbg3/4-abnahme-handy
- **Abhängig von:** –
- **Tickets:** B-232
- **Kriterien:** AC-04

## Ziel

🧑 hat `/dm` am Handy neben dem laufenden Spiel am TV ausprobiert.

## Kontext

Server mit `K3C_DEV=1` (Dev-Mode), Spiel am TV oder PC, Handy im selben Netz: `http://<server>:8080/dm`.

## Erlaubte Dateien

- Planungsdateien

## Nicht-Ziele

Änderungen am Code (Befunde → Ticket).

## Schritte

1. Raum am TV starten, `/dm` am Handy öffnen, Raum wählen.
2. Zeit 4×, Pause, Gold, Material, Welle, Nacht ausprobieren und am TV beobachten.
3. Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-04: Abnahme durch 🧑 im Ergebnis vermerkt.

## Prüfen

Am Gerät, siehe Schritte.

## Ergebnis

2026-10-07, **PC-Nachweis, Handy offen.** Geprüft von 🧑 (Ralf) im Interview mit Agent (Claude Opus 5.5), Branch `sprint/dbg3`. Spiel und `/dm` in zwei Browserfenstern am PC (Tastatur), Spielserver über k3c-dev (`task start`, Dev-Mode an, Port 8080, `/dm` aus `dist/`), Spiel über Vite (Port 5173).

- **Schritt 1 (🧑):** `/dm` zeigt die Raumliste, Raum wählbar, Diagnose (Takt, Zeit, Welle, Burg, Gold, Truppen, Verbindungen, Stufen) aktualisiert live; Fenster auf Handybreite schmal gezogen ohne waagerechtes Scrollen.
- **Schritt 2 (🧑):** Zeit 4×, Pause, Weiter, 1× wirken sichtbar im Spiel.
- **Schritt 3 (🧑):** +50 Gold und +20 Material kommen im Spiel an.
- **Schritt 4 (🧑):** Dämmerung, Nacht (Nachtwelle startet), Welle +1 und Tag wirken im Spiel.
- **AC-04 am PC: geprüft** (🧑). Keine Abweichungen von der Spec.
- **Neues Ticket:** B-333 (CLI): Bei Pause fehlt im Spielbild eine Pause-Anzeige, und die Figuren-Animationen laufen weiter (Anmerkung 🧑).
- **Offen:** Die Abnahme am Handy (neben Spiel am TV oder PC) fehlt; die Session bleibt `offen`, AC-04 steht weiter als „angenommen, Validierung offen“.

2026-10-08, **verworfen (PJ3.2, B-359):** Die Abnahme am Gerät ist nach HW1.3 verschoben (Projekt ABN); bisherige Nachweise oben gelten weiter.
