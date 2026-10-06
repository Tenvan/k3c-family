# SO1.5 · Hörprobe am TV: Entsperren, Format, Split-Screen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** so1/5-hoerprobe-tv
- **Abhängig von:** SO1.3
- **Tickets:** B-011, B-166
- **Kriterien:** AC-04

## Ziel

🧑 hat auf der Xbox am TV gehört, dass der Ton nach der ersten Taste startet, das gewählte Format spielt und im Split-Screen jeder seinen Bereich hört; das Ergebnis steht in dieser Datei.

## Kontext

**Controller zurückgestellt (B-314, 2026-10-06):** 🧑 prüft vorerst nur am PC mit Tastatur und Maus (1 Spieler; 2 Spieler an einer Tastatur erst mit B-316). Controller-, Vibrations- und Xbox-Schritte dieser Session sind nach B-314 verschoben; die Kriterien bleiben, ihr Controller-Anteil gilt als `angenommen, Validierung offen (B-314)`.

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 10 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews SO1.4. Bis dahin gilt die Annahme aus `docs/plan-weiterentwicklung.md` § 11.6 (Ton erst nach erster Eingabe, mp3 mit Fallback). Weicht das Ergebnis ab, entsteht ein Ticket (CLI) und § 11.6 bekommt den Messwert.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets), `docs/plan-weiterentwicklung.md` (nur § 11.6: Messwert statt Annahme)

## Nicht-Ziele

Code-Änderungen, Klangauswahl (SO2, SO4).

## Schritte

1. Stand im Heimnetz bereitstellen, `game.html` auf der Xbox öffnen.
2. Vor der ersten Taste: kein Ton, keine Fehlermeldung; A drücken (Beitreten) → Demo-Ton beim Bauen hörbar.
3. Zwei Spieler im Split-Screen: Bau links bzw. rechts → beim jeweiligen Spieler deutlich, beim anderen leiser.
4. Ergebnis eintragen (Datum, Gerät, welches Format gespielt hat, Befund); Abweichung → Ticket und § 11.6; `Status: fertig`.

## Fertig, wenn

- [ ] AC-04: 🧑 bestätigt Format mit Fallback am TV (oder Abweichung als Ticket).

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

2026-10-05, **PC-Nachweis, TV offen.** Geprüft von 🧑 (Ralf) im Interview mit Agent (Claude Sonnet 5.5), Branch `sprint/so1`, Chrome am PC mit Controller und Lautsprechern, Dev-Server (`task dev`), `game.html`.

- **AC-04 am PC: geprüft** (🧑): vor der ersten Taste kein Ton und keine Fehlermeldung; nach A (Beitreten) ist der Demo-Ton beim Bauen hörbar; im Split-Screen mit zwei Spielern hört der jeweilige Spieler seinen Bau deutlich, der andere leiser.
- Welches Format (ogg oder mp3) gespielt hat, wurde nicht ausgelesen; dazu gibt es keine Messung für § 11.6.
- Keine Mängel, keine neuen Tickets.
- **Offen:** Entsperren, Format mit Fallback und Dämpfung am TV (Xbox, Edge) fehlen; die Session bleibt `offen`, `plan-weiterentwicklung.md` § 11.6 behält die Annahme.

2026-10-06, **Tastatur und Maus (Sammelaussage):** 🧑 (Ralf) im Chat: „Alle Tastatur und Maussteuerungen liefen bisher wie definiert.“ Gilt für den Tastatur- und Maus-Anteil dieser Session am PC; Darstellung, Ton und Controller (B-314) sind damit nicht abgenommen, die Session bleibt `offen`.
