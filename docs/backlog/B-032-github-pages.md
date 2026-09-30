# B-032 · GitHub Pages zeigt nur, was ohne Server geht

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP09
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GitHub Pages hat keinen Server.

## Ziel

GitHub Pages zeigt nur, was ohne Server geht. Nutzen: Nach SP09 funktioniert das Spiel dort nicht mehr.

## Beteiligte und Zielgruppen

Spieler mit Controller auf der Xbox; 🧑 testet an der Xbox; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Auf Pages sind Spiel-Kacheln ausgeblendet oder erklärt.
- Testseiten bleiben erreichbar.

## Nicht-Ziele

Server auf Pages.

## Regeln und Einschränkungen

Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, `toggleFullscreen()`, `goHome()`); B nicht belegen, View + Menu reserviert.

## Beispiele

Aufruf der Pages-Landingpage → Gamepad-Test erreichbar, die Spiel-Kachel ist ausgeblendet oder erklärt.

## Ausnahme- und Fehlerfälle

Direkter Aufruf von `game.html` auf Pages → verständlicher Hinweis statt Fehler.

## Akzeptanzkriterien

- **AC-01** Auf Pages sind Spiel-Kacheln ausgeblendet oder erklärt.
- **AC-02** Die Testseiten bleiben auf Pages erreichbar.

## Offene Fragen

Ausblenden oder erklären? (🧑)

## Notizen

–
