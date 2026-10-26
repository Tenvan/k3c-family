# BS5 · SIM · Hafen-Chunk, Schiff bauen und Inselwechsel am Hafen

- **Status:** geplant
- **Projekt:** BAS
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-637, B-638
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Inselwechsel heute: Endboss besiegt → `SwitchReady` → `engine/room/island_switch.go`. Regeln zu Hafen und Schiff aus BS2, Schiffskosten in `data/` und Bedarf im Bericht aus BS3.

## Ziel

Auf Inselstufe 1 liegt der Hafen-Chunk, dort wird das Schiff in Abschnitten mit Münzen und Material gebaut, und der Inselwechsel läuft am Hafen. Bots, Bericht und Werte folgen in BS9 (Meilenstein 3).

**Am Ende sichtbar:** Generator- und Sim-Tests grün, Protokoll und Spielstand tragen den Schiff-Fortschritt.

## Beteiligte und Zielgruppen

Claude, 🧑 gibt frei und nimmt die Werte ab.

## Anforderungen

B-637 › Anforderungen.

## Nicht-Ziele

Client (BS6), Inhalte weiterer Inseln.

## Regeln und Einschränkungen

Abhängig von BS3 (BS4 nicht nötig, parallel möglich, solange die Domänen-Sperre es zulässt). Protokoll und Spielstand in eigener Session. Keine Werte-Justierung hier (BS9).

## Beispiele

B-637 › Beispiele, B-638 › Beispiele.

## Ausnahme- und Fehlerfälle

B-637 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Generator-Test: genau ein Hafen-Chunk auf Insel 1 Tiefe 0 am geplanten Platz (B-637/AC-02).
- **AC-02** Sim-Test: Schiff in Abschnitten baubar; Wechsel nur mit fertigem Schiff, besiegtem Endboss und allen Spielern am Hafen (B-637/AC-03).
- **AC-03** Protokoll und Spielstand tragen den Schiff-Fortschritt (B-637/AC-04).

## Offene Fragen

keine (Defaults aus BS2).

## Sessions

Reife Entwurf, Stichpunkte:
- BS5.1 · SIM · Chunk-Art `harbor` im Generator (AC-01)
- BS5.2 · SIM · Schiff bauen, Wechselbedingung (AC-02)
- BS5.3 · SRV · Protokoll und Spielstand (AC-03)
- BS5.4 · Review (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
