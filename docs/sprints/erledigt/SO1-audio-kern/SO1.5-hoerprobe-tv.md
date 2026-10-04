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

–
