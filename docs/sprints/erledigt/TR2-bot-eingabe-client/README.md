# TR2 · PLAT · Bot-Eingabe im Client für Testläufe

- **Status:** erledigt
- **Domäne:** PLAT
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-349
- **Start-Commit:** 4048467
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1 (mit TR1)

## Ausgangslage

Der Client kennt nur Tastatur, Gamepad und Touch als Eingabe; für `sim_test` mit laufenden Clients (TR1.3) fehlt eine Eingabe, über die Bots die Monarchen steuern (B-349).

## Ziel

Ein Client mit `?botfeed=…&players=n` nimmt die Eingaben seiner lokalen Spieler aus dem Bot-Feed der Workbench.

Am Ende sichtbar: Test der `BotInput` grün; im Browser bewegen sich n Monarchen nach Bot-Kommandos (mit TR1.3).

## Beteiligte und Zielgruppen

Workbench (`sim_test`), Agenten und 🧑 bei Testläufen.

## Anforderungen

B-349 › Anforderungen.

## Nicht-Ziele

Bot-Entscheidung im Client, Browser-Start (TR1.3), neue Anzeigen.

## Regeln und Einschränkungen

Domäne PLAT (`src/input/`, `game.html`, `src/core/shell.ts` nur falls nötig). Regeln aus `CLAUDE.md` (Eingabe, Seiten, B-Taste, kein `Math.random()`). Einschiebbar, weil TR1.3 darauf wartet.

## Beispiele

B-349 › Beispiele.

## Ausnahme- und Fehlerfälle

B-349 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** `BotInput` liefert je Slot die Aktionen aus dem Feed, ohne Feed keine (B-349/AC-01).
- **AC-02** Fremder Host abgelehnt, ohne Parameter unverändert (B-349/AC-02).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| TR2.1 | `TR2.1-bot-eingabe.md` | Umsetzung | autonom | fertig |
| TR2.2 | `TR2.2-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-07: Kriterien siehe Ergebnisse TR2.1 und TR2.2 (AC-01, AC-02 per Test). Einbindung ins Spiel und Browser-Nachweis → B-353 (TR3).
Keine schweren Befunde; leichte Befunde zu `botInput.ts` → B-354. Neue Tickets: B-353, B-354.

