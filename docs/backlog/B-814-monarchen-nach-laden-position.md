# B-814 · Geklärt ist, warum Monarchen nach dem Laden an der Burg statt an ihrer gespeicherten Stelle stehen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Ende-zu-Ende-Lauf NG9.1 stehen die Monarchen nach dem Laden an der Burg statt an ihrer gespeicherten Stelle. Vermutet (ungeprüft): gewolltes Verhalten der Engine beim Laden (`engine/sim`, `engine/room`); nicht belegt.

## Ziel

Geklärt ist, ob das Verhalten gewollt ist; falls nicht, steht die Korrektur als Ticket.

## Beteiligte und Zielgruppen

Ralf entscheidet bei Spielregeln, Claude prüft.

## Anforderungen

- Code und Regeln (`docs/rules/`, `docs/game-design.md`) prüfen: Ist Burg-Neustart nach dem Laden gewollt?
- Ergebnis mit Beleg in den Notizen.
- Ist es nicht gewollt: Ticket Problem mit Test im Raum-Paket anlegen.

## Nicht-Ziele

Änderung des Spielstand-Formats ohne Entscheidung.

## Regeln und Einschränkungen

Sim ist deterministisch; Spielstand-Format und Spielregeln ändert nur Ralf. Keine Rücksicht auf alte Spielstände (bis auf Widerruf).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `engine/sim/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** spielstand

## Beispiele

Monarch steht bei x=40, Spiel speichern und laden → heute steht er an der Burg.

## Ausnahme- und Fehlerfälle

Spielstand ohne Monarch-Position → Burg als Rückfall (wie heute).

## Akzeptanzkriterien

- **AC-01** Belegte Antwort in den Notizen: gewollt oder nicht, mit Fundstelle in Code oder Regeln.
- **AC-02** Falls nicht gewollt: Folge-Ticket angelegt und hier verlinkt.

## Offene Fragen

Ist die Position nach dem Laden Spielregel? Entscheidet Ralf, falls die Regeln nichts sagen.

## Notizen

Gefunden in NG9.1 (2026-11-11). Bereich ist vermutet.
