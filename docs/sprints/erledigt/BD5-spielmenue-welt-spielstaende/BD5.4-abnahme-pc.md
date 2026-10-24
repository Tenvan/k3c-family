# BD5.4 · Abnahme am PC

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sprint/bd5
- **Abhängig von:** BD5.3
- **Tickets:** B-506
- **Kriterien:** AC-03

## Ziel

Ralf nimmt das Spielmenü am PC ab.

## Kontext

Server lokal (`task serve` oder Starter), `game.html` öffnen; für andere Welten Dev-Mode und `?world=`.

## Erlaubte Dateien

- `docs/sprints/**/BD5-spielmenue-welt-spielstaende/**`

## Nicht-Ziele

Kein Code.

## Schritte

1. Lobby zeigt „Welt: …“ und je Spielstand die Welt.
2. „Neuer Spielstand“ startet ein frisches Spiel `spiel-N`; Pausenmenü nennt die Welt.
3. Zurück in die Lobby, Spielstand wählen, X bzw. Entf: Rückfrage, „Abbrechen“ lässt ihn stehen, „Löschen“ entfernt ihn aus der Liste.
4. Offenen Raum löschen geht nicht (Stand erscheint nicht in der Liste).

## Fertig, wenn

- [x] AC-03: Ralf hat die Schritte 1–4 am PC gesehen.

## Prüfen

Manuell durch Ralf.

## Ergebnis

- AC-03 geprüft (Ralf, 2026-10-24, PC): Schritte 1 und 3 ok, Schritt 2 ok, Schritt 4 mit Abweichung (offene Räume stehen in der Liste, lassen sich nicht löschen: wie in BD5.1 gewollt).
- Neue Tickets: B-580 (Welt im Pausenmenü größer, Welt bei jedem Spielstand der Lobby), B-581 (offenen Raum über Entfernen schließen).
