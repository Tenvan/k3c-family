# SO4.5 · Hörprobe am TV: Crossfade, Ducking, Lautheit der Musik

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** so4/5-hoerprobe-tv
- **Abhängig von:** SO4.3
- **Tickets:** B-168
- **Kriterien:** AC-02, AC-03, AC-07

## Ziel

🧑 hat auf der Xbox am TV gehört, dass Tag → Abend → Nacht ohne Pause und Knacken überblendet, Warnungen die Musik absenken und wieder zurückkehren und die Lautheit passt; das Ergebnis steht in dieser Datei.

## Kontext

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 20 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews SO4.4; bis dahin gelten AC-02 und AC-03 als angenommen (Beobachtung im Browser-Pane in SO4.2 und SO4.3). Maßstab sind die Beschlüsse Q15 (Retro/Chiptune, kindgerecht, Nacht leise) und Q16 (Crossfade, Ducking, Lautheit am TV). Weicht das Ergebnis ab (Knacken, Pause, Stück zu laut), entsteht ein Ticket (CLI) statt einer Änderung in dieser Session. Ton wird in Edge erst nach der ersten Eingabe entsperrt (Annahme aus SO1, `docs/plan-weiterentwicklung.md` § 11.6).

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Code-Änderungen, neue Stücke, Pegel-Anpassung (eigenes Ticket).

## Schritte

1. Stand im Heimnetz bereitstellen, `game.html` auf der Xbox öffnen, mit A beitreten.
2. Tag → Abend → Nacht durchlaufen (Zeitraffer `?fast=1`): Übergänge ohne Pause und Knacken?
3. Warnung auslösen (Nacht naht, Welle): Musik sinkt hörbar und kehrt zurück?
4. Lobby und Tiefe/Höhle ansehen und hören; Lautheit der Stücke untereinander und gegenüber den Effekten (SO2) beurteilen.
5. Ergebnis eintragen (Datum, Gerät, Befund je Übergang, Lautheit); Abweichung → Ticket; `Status: fertig`.

## Fertig, wenn

- [ ] AC-02: 🧑 bestätigt den hörbar sauberen Wechsel Tag → Abend → Nacht (oder Abweichung als Ticket).
- [ ] AC-03: 🧑 bestätigt, dass Warnungen die Musik absenken und diese zurückkehrt (oder Abweichung als Ticket).

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

–
