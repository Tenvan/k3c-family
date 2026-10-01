# B-032 · GitHub Pages zeigt nur, was ohne Server geht

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP09
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („SP09 freigegeben“), Revision 2

## Ausgangslage

GitHub Pages hat keinen Server.

## Ziel

GitHub Pages zeigt nur, was ohne Server geht. Nutzen: Nach SP09 funktioniert das Spiel dort nicht mehr.

## Beteiligte und Zielgruppen

Spieler mit Controller auf der Xbox; 🧑 testet an der Xbox; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Auf Pages sind die Spiel-Kacheln deaktiviert und erklärt (nicht ausgeblendet).
- Testseiten bleiben erreichbar.

## Nicht-Ziele

Server auf Pages.

## Regeln und Einschränkungen

Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, `toggleFullscreen()`, `goHome()`); B nicht belegen, View + Menu reserviert.

## Beispiele

Aufruf der Pages-Landingpage → Gamepad-Test erreichbar, die Spiel-Kachel ist deaktiviert mit Hinweis „Braucht den Heimnetz-Server“.

## Ausnahme- und Fehlerfälle

Direkter Aufruf von `game.html` auf Pages → verständlicher Hinweis statt Fehler.

## Akzeptanzkriterien

- **AC-01** Auf Pages sind die Spiel-Kacheln deaktiviert und erklärt (nicht ausgeblendet).
- **AC-02** Die Testseiten bleiben auf Pages erreichbar.

## Offene Fragen

keine (entschieden 2026-10-01 🧑 Chat: erklären; Revision 2)

## Notizen

–
