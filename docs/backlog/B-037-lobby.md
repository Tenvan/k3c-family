# B-037 · Lobby zeigt Räume und startet Spiele

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP08
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Einem Raum tritt man nur per URL-Parameter `?online=RAUM` bei.

## Ziel

Lobby zeigt Räume und startet Spiele. Nutzen: Ohne Lobby muss man Raumcodes kennen.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Laufende Räume sehen.
- Einem Raum beitreten und ein neues Spiel starten.
- Spielstand pro Raum wählen.

## Nicht-Ziele

In SP08 nur die einfache Raumwahl; laufende Räume sehen und Spielstand wählen kommen später.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen).

## Beispiele

Die Xbox erstellt einen Raum → das Handy tritt ihm bei, ohne einen Code einzutippen.

## Ausnahme- und Fehlerfälle

Raum ist voll oder weg → Hinweis, zurück zur Auswahl.

## Akzeptanzkriterien

- **AC-01** Ein Gerät kann einen Raum erstellen (SP08).
- **AC-02** Ein anderes Gerät kann ihm beitreten (SP08).
- **AC-03** Laufende Räume sind sichtbar.
- **AC-04** Der Spielstand ist pro Raum wählbar.

## Offene Fragen

keine

## Notizen

–
