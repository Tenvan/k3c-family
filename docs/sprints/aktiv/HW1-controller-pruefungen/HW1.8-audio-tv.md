# HW1.8 · Audio am TV: Entsperren, Format, Split-Screen (aus SO1.5)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** hw1/8-audio-tv
- **Abhängig von:** –
- **Tickets:** B-166
- **Kriterien:** AC-07

## Ziel

🧑 hat auf der Xbox am TV gehört, dass der Ton nach der ersten Taste startet, das gewählte Format spielt und im Split-Screen jeder seinen Bereich hört.

## Kontext

Übernommen aus SO1.5 (PJ3, 2026-10-08); dort sind der PC-Nachweis (2026-10-05) und die Sammelaussage zu Tastatur und Maus (2026-10-06) eingetragen. Offen: Entsperren, Format mit Fallback und Dämpfung am TV (Xbox, Edge); Controller-Anteil nach B-314. Bis dahin gilt die Annahme aus `docs/plan-weiterentwicklung.md` § 11.6 (Ton erst nach erster Eingabe, mp3 mit Fallback). Etwa 10 Minuten.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), HW1-README (Tabelle), `docs/backlog/` (Status, neue Tickets), `docs/plan-weiterentwicklung.md` (nur § 11.6: Messwert statt Annahme)

## Nicht-Ziele

Code-Änderungen, Klangauswahl (SO2, SO4).

## Schritte

1. Stand im Heimnetz bereitstellen, `game.html` auf der Xbox öffnen.
2. Vor der ersten Taste: kein Ton, keine Fehlermeldung; A (Beitreten) → Demo-Ton beim Bauen hörbar.
3. Zwei Spieler im Split-Screen: Bau links bzw. rechts → beim jeweiligen Spieler deutlich, beim anderen leiser.
4. Ergebnis eintragen (Datum, Gerät, gespieltes Format); Abweichung → Ticket und § 11.6; `Status: fertig`.

## Fertig, wenn

- [ ] AC-07: 🧑 bestätigt Format mit Fallback am TV (oder Abweichung als Ticket).

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
