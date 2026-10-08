# PL2.5 · Abnahme: Werkzeug-Seiten nach Sprachwechsel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** pl2/5-abnahme-sprachwechsel
- **Abhängig von:** PL2.3
- **Tickets:** B-322
- **Kriterien:** AC-01

## Ziel

🧑 hat bestätigt, dass die Werkzeug-Seiten nach einem Sprachwechsel Englisch zeigen (B-322/AC-02); danach ist der Sprint erledigt.

## Kontext

Die Sprache wählt man in den Optionen des Spiels (`game.html`); sie steht in den Einstellungen des Geräts. Die Werkzeug-Seiten lesen sie beim Öffnen.

## Erlaubte Dateien

- Planungs-Dateien des Sprints, `docs/backlog/`

## Nicht-Ziele

Code ändern; Abweichungen werden Tickets.

## Schritte

1. Im Spiel die Sprache auf Englisch stellen.
2. Über die Landingpage bzw. die Entwicklerseite jede Werkzeug-Seite öffnen und auf deutsche Reste achten.
3. Zurück auf Deutsch und Stichprobe wiederholen.
4. Ergebnis eintragen; Reste als Ticket. Sprint-Ordner nach `erledigt/`, B-322 auf `erledigt`.

## Fertig, wenn

- [ ] AC-01: 🧑 bestätigt Englisch und Deutsch auf allen Werkzeug-Seiten (B-322/AC-02).

## Prüfen

Manuell durch 🧑 im Browser (PC oder Xbox).

## Ergebnis

2026-10-08, geprüft von 🧑 am PC (Chrome/Edge, Dev-Server, Sprachwahl über die Optionen im Spiel).

- **AC-01 geprüft** (B-322/AC-02): Nach Wechsel auf English zeigten alle Werkzeug-Seiten (Lizenzen, Entwicklerseite mit allen Kacheln, Monitor, DM) Englisch; nach Rückwechsel auf Deutsch war die Stichprobe deutsch. PC-Nachweis; Xbox nicht geprüft (Session erlaubt PC oder Xbox).
- **Abweichung:** Die Landingpage blieb deutsch. Sie liegt außerhalb von PL2 (`src/tools/`) und B-215 → neues Ticket B-369.
