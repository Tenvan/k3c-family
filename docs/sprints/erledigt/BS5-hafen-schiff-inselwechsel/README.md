# BS5 · SIM, SRV · Hafen-Chunk, Schiff bauen und Inselwechsel am Hafen

- **Status:** erledigt
- **Projekt:** BAS
- **Domäne:** SIM, SRV
- **Reife:** bereit
- **Tickets:** B-637, B-638
- **Start-Commit:** 5d51f15b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-10-28, Defaults aus B-637)

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS5.1 | `BS5.1-harbor-chunk.md` | Umsetzung | autonom | fertig |
| BS5.2 | `BS5.2-schiff-bauen.md` | Umsetzung | autonom | fertig |
| BS5.3 | `BS5.3-protokoll-spielstand.md` | Umsetzung | autonom | fertig |

Reife Entwurf, Stichpunkte:
- BS5.1 · SIM · Chunk-Art `harbor` im Generator (AC-01)
- BS5.2 · SIM · Schiff bauen, Wechselbedingung (AC-02)
- BS5.3 · SRV · Protokoll und Spielstand (AC-03)
- BS5.4 · Review (alle)

## Abnahme

2026-10-29, Claude (autonom): AC-01 bis AC-03 erfüllt (BS5.1 bis BS5.3: harbor-Chunk Slot −7 in standard.json, Platz `ship` mit drei Abschnitten aus data/ship.json, Wechsel nur mit fertigem Schiff + Endboss + allen am Steg, Spielstand-Rundlauf, Protokoll-Absatz). Review (Sonnet): Gate-Reset bei Schiffverlust, Schiff aus allBuilt ausgenommen, Welten ohne Hafen auf nicht-letzter Insel werden abgelehnt (gewollt). Offen: Client (BS6, B-637/AC-05), Bots (BS9), Standard-Welt hat nur eine Insel, der Wechsel greift erst mit Insel 2.
