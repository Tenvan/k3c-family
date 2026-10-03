# DBG2.3 · Abnahme am PC, Handy und Controller

- **Status:** in Arbeit
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** dbg2/3-abnahme-geraet
- **Abhängig von:** DBG2.1, DBG2.2
- **Tickets:** B-179
- **Kriterien:** AC-05

## Ziel

🧑 hat Gold droppen, Material geben und den Zeitraffer am PC (Maus), am Handy (Touch) und mit Controller ausprobiert; das Ergebnis steht in dieser Datei.

## Kontext

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 20 Minuten. Nach `docs/arbeitsweise.md` › SDD gilt: Abnahme gehört 🧑; wer geprüft hat, steht im Ergebnis. Der Raum muss im Dev-Mode laufen (`K3C_DEV` an, Standard in der Entwicklung; Grad `dev`). Overlay: Taste Ö oder Klick auf den linken Stick; `?dev=0` schaltet es ab. Die Belegung der Dev-Bedienung am Controller (Fokus-Knopf, D-Pad, A) steht im Ergebnis von DBG2.2.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Keine Code-Änderungen; Mängel werden Tickets (Domäne CLI oder SRV), keine Nacharbeit in dieser Session.

## Schritte

1. 🧑 startet `task start` und `task dev` (oder den gebauten Server) und öffnet `game.html` mit einem Dev-Raum, am PC im Browser.
2. PC mit Maus und Ö: Overlay öffnen. „Gold 50“ klicken: 50 Münzen fallen vor den Spieler und lassen sich aufheben. Ein Material (z. B. Holz) geben: der Vorrat im HUD steigt. „Zeit 8×“: Der Faktor steht im Overlay, die Welt läuft sichtbar schneller; „Zeit 1×“ stellt zurück.
3. Handy (`?touch=1` oder echtes Gerät im Heimnetz): dieselben drei Aktionen per Tippen; die Schaltflächen lassen sich treffen und stören die Lauf-Flächen nicht.
4. Controller (am PC oder Xbox/Edge): Overlay mit Stick-Klick öffnen, mit der Dev-Bedienung aus DBG2.2 wählen und auslösen; B und View + Menu verhalten sich wie sonst, der Spieler läuft nicht ungewollt los.
5. Raum ohne Dev-Mode (`K3C_DEV=0`): keine Aktionsliste. Mängel als Tickets festhalten.
6. Ergebnis eintragen: Geräte, Datum, was gesehen wurde, gescheiterte Punkte als Tickets, dann `Status: fertig` setzen.

## Fertig, wenn

- [ ] AC-05: 🧑 bestätigt im Ergebnis mit Datum „Gold droppen, Material geben und Zeitraffer am PC ausprobiert“.
- [ ] AC-05: 🧑 bestätigt dasselbe für das Handy.
- [ ] AC-05: 🧑 bestätigt dasselbe mit Controller (oder nennt die Mängel als Tickets, dann bleibt die Session `blockiert`).

## Prüfen

Manuell durch 🧑 am Gerät; vorher `task check` grün auf dem getesteten Stand.

## Ergebnis

–
