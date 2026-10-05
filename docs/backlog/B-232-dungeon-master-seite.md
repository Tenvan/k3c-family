# B-232 · Eine Dungeon-Master-Seite unter /dm steuert Räume live vom Handy oder Tablet

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** DBG3
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-05, Chat, durch 🧑 (mit Sprint DBG3 Revision 2)

## Ausgangslage

Live-Eingriffe gibt es nur im Cheat-Dialog im Spiel (B-231) und über k3c-dev am PC. Wer am Handy oder Tablet neben dem
TV sitzt, kann weder Werte anpassen noch Diagnose sehen, ohne das Spiel zu unterbrechen.

## Ziel

Eine eigene, responsive Seite „Dungeon Master“ unter `/dm` für Handy und Tablet: umfangreiche Live-Anpassungen an
laufenden Räumen und Diagnosedaten, ohne das Spielbild zu stören.

## Beteiligte und Zielgruppen

🧑 als Spielleiter am Spieleabend, Tester.

## Anforderungen

- Aufruf über die URL `/dm` (nicht über die Landingpage-Kacheln), responsive für Handy und Tablet.
- Raum wählen, Diagnose (Takt, Verbindungen, Spieler, Welle, Zeit) live anzeigen.
- Live-Anpassungen über Dev-Aktionen des Servers (Gold, Material, Zeitraffer, Pause, Welle auslösen, Tageszeit setzen; später Grad B-107, Neustart B-080).
- Später passwortgeschützt.

## Nicht-Ziele

Spielen auf der Seite; Passwortschutz in der ersten Fassung (eigenes Ticket, sobald die Seite steht).

## Regeln und Einschränkungen

Dev-Aktionen nur im Dev-Mode des Servers. Die Seite ist kein Spiel-Gerät mit Monarch und arbeitet über eine HTTP-API
`/api/dev` (Entscheidung 🧑 2026-10-05, kein Protokoll-Bump).
Seiten-Regeln aus `CLAUDE.md` gelten, soweit die Seite in der Shell läuft.

## Beispiele

Handy öffnet `http://<server>/dm` → Raumliste → Raum KRNZ → „Zeit 4×“ → Raum läuft schneller, Anzeige zeigt `4×`.

## Ausnahme- und Fehlerfälle

Server ohne Dev-Mode → Seite zeigt nur Diagnose, Aktionen ausgegraut. Raum geschlossen → zurück zur Raumliste.

## Akzeptanzkriterien

- **AC-01** `/dm` liefert die Seite aus; sie ist auf 375 px Breite ohne waagerechtes Scrollen bedienbar.
- **AC-02** Die Seite zeigt Diagnosedaten eines gewählten Raums und aktualisiert sie live.
- **AC-03** Mindestens Gold, Material, Zeitraffer und Pause wirken über die Seite (Test).
- **AC-04** 🧑 hat die Seite am Handy abgenommen.
- **AC-05** Welle auslösen und Tageszeit setzen wirken über die Seite (Test).

## Offene Fragen

- Passwortschutz: Zeitpunkt und Art (🧑).

## Notizen

Aus dem Chat 2026-10-04 zusammen mit B-231.
