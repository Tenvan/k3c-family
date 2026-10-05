# DBG2.3 · Abnahme am PC, Handy und Controller

- **Status:** blockiert
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

- [x] AC-05: 🧑 bestätigt im Ergebnis mit Datum „Gold droppen, Material geben und Zeitraffer am PC ausprobiert“.
- [x] AC-05: 🧑 bestätigt dasselbe für das Handy.
- [ ] AC-05: 🧑 bestätigt dasselbe mit Controller (oder nennt die Mängel als Tickets, dann bleibt die Session `blockiert`).

## Prüfen

Manuell durch 🧑 am Gerät; vorher `task check` grün auf dem getesteten Stand.

## Ergebnis

2026-10-03, geprüft von 🧑 (Ralf) im Interview mit Agent (Claude Opus 5.5), Branch `dbg2/3-abnahme-geraet`, Stand `origin/develop` 7c29a94. Vorher `task check` grün (658 Tests). Go-Server `bin/k3c-server` mit Dev-Mode (`dev=true`), `task dev` im LAN.

- **AC-05 PC: geprüft** (🧑, Maus und Ö): Overlay öffnet, „Gold 50“ lässt Münzen fallen, Material erhöht den Vorrat, „Zeit 8×“ und „Zeit 1×“ wirken und stehen im Overlay. Mängel: Overlay und Aktionsliste liegen halbtransparent über dem HUD, gewünscht links unten (**B-191**); Ö schließt die Aktionsliste nicht (**B-192**).
- **AC-05 Handy: geprüft** (🧑, Touch): alle drei Aktionen per Tippen, Schaltflächen treffbar, Lauf-Flächen nicht gestört.
- **AC-05 Controller: blockiert**, nicht geprüft (kein Gamepad zur Hand). Prüfliste 3 nachholen: Stick-Klick öffnet, RB Fokus (gelber Rahmen), D-Pad wählt, A löst aus ohne Beitritt, Stick im Fokus stumm, B ohne Wirkung, View + Menu zurück. Nach `docs/arbeitsweise.md` › Hardware entkoppelt keine Abhängigkeit für DBG2.4.
- Schritt 5 (`K3C_DEV=0`, keine Aktionsliste): von 🧑 nicht geprüft; Nachweis bisher nur aus DBG2.2 (Agent, Browser-Pane und Test). Wegen B-192 nach dessen Behebung erneut ansehen.
