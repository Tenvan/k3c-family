# SO3.3 · Abnahme am TV: Bedienung und Crossfade hören

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** so3/3-abnahme-tv
- **Abhängig von:** SO3.2
- **Tickets:** B-169
- **Kriterien:** AC-03, AC-04

## Ziel

🧑 hat `soundtest.html` auf der Xbox am TV mit dem Controller bedient und die Crossfade-Probe ohne Knacken gehört; das Ergebnis steht in dieser Datei.

## Kontext

**Controller zurückgestellt (B-314, 2026-10-06):** 🧑 prüft vorerst nur am PC mit Tastatur und Maus (1 Spieler; 2 Spieler an einer Tastatur erst mit B-316). Controller-, Vibrations- und Xbox-Schritte dieser Session sind nach B-314 verschoben; die Kriterien bleiben, ihr Controller-Anteil gilt als `angenommen, Validierung offen (B-314)`.

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 10 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews SO3.4. Zu prüfen: Seite über die Landingpage öffnen, A entsperrt Audio, Stick/D-Pad wählt, A spielt, Stopp und Überblenden funktionieren, B tut nichts, View + Menu führt zur Landingpage; Crossfade zwischen zwei Stücken ohne Knacken; Lautstärke am TV angenehm.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Code-Änderungen; Auswahl der Kandidaten (SO2.1, SO4.1).

## Schritte

1. Stand im Heimnetz bereitstellen (`task serve` oder Pi), `index.html` auf der Xbox öffnen, Kachel „Hörproben“ wählen.
2. Bedienung und Crossfade wie im Kontext prüfen.
3. Ergebnis eintragen (Datum, Gerät, Befund); Mängel als Tickets; `Status: fertig` (bei Mängeln `blockiert`).

## Fertig, wenn

- [ ] AC-03: 🧑 bestätigt Controller-Bedienung am TV, B frei, View + Menu führt zurück.
- [ ] AC-04: 🧑 bestätigt Crossfade ohne Knacken am TV.

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

2026-10-05, **PC-Nachweis, TV offen.** Geprüft von 🧑 (Ralf) im Interview mit Agent (Claude Sonnet 5.5), Branch `sprint/so3`, Chrome am PC mit Controller und Lautsprechern, Dev-Server (`task dev`, Port 5173), Landingpage → Kachel „Hörprobe“.

- **AC-03 am PC: geprüft** (🧑): Controller wählt und spielt, Stopp und Überblenden funktionieren, B ohne Wirkung, View + Menu führt zur Landingpage.
- **AC-04 am PC: geprüft** (🧑): Crossfade zwischen zwei Stücken ohne Knacken, Lautstärke angenehm.
- Keine Mängel, keine neuen Tickets.
- **Offen:** Die Abnahme am TV (Xbox, Edge) fehlt; die Session bleibt `offen`, die Kriterien stehen weiter als „angenommen, Validierung offen“.

2026-10-06, **Tastatur und Maus (Sammelaussage):** 🧑 (Ralf) im Chat: „Alle Tastatur und Maussteuerungen liefen bisher wie definiert.“ Gilt für den Tastatur- und Maus-Anteil dieser Session am PC; Darstellung, Ton und Controller (B-314) sind damit nicht abgenommen, die Session bleibt `offen`.
