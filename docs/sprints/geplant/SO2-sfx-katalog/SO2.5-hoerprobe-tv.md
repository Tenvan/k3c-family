# SO2.5 · Hörprobe am TV: Effekte, Lautheit, Kindertauglichkeit

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** so2/5-hoerprobe-tv
- **Abhängig von:** SO2.3
- **Tickets:** B-167
- **Kriterien:** AC-03

## Ziel

🧑 hat auf der Xbox am TV gehört, dass Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht je ihren Sound auslösen, dass nichts erschreckt und die Lautheit passt; das Ergebnis steht in dieser Datei.

## Kontext

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 15 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews SO2.4; bis dahin gilt AC-03 als angenommen (Beobachtung im Browser-Pane in SO2.3). Maßstab ist Beschluss Q15: Retro/Chiptune, kindgerecht, Nacht leise statt laut. Weicht das Ergebnis ab (Sound zu laut, erschreckend, falsch zugeordnet), entsteht ein Ticket (CLI, bzw. neue Auswahl über `soundtest.html`) statt einer Änderung in dieser Session. Ton wird in Edge erst nach der ersten Eingabe entsperrt (Annahme aus SO1, `docs/plan-weiterentwicklung.md` § 11.6).

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Code-Änderungen, neue Sounds, Musik (SO4).

## Schritte

1. Stand im Heimnetz bereitstellen, `game.html` auf der Xbox öffnen, mit A beitreten.
2. Nacheinander auslösen und hören: Münze aufheben, Schlag, Gegner-Tod, Bauen fertig, Nacht naht (Tageszeit-Zeitraffer über `?fast=1`, falls nötig).
3. Zwei Spieler im Split-Screen: Sound des anderen Bereichs leiser, Warnung bei beiden laut.
4. Ergebnis eintragen (Datum, Gerät, Befund je Ereignis, Lautheit); Abweichung → Ticket; `Status: fertig`.

## Fertig, wenn

- [ ] AC-03: 🧑 bestätigt am TV, dass die fünf Ereignisse ihren Sound auslösen (oder Abweichung als Ticket).

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

–
