# SO7 · CLI · Niederlage-Stück, Hufschlag, UI-Klicks und Skill-Sounds je Klasse

- **Status:** erledigt
- **Projekt:** SND
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-573, B-574, B-575
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Niederlage-Stück, Hufschlag, UI-Klicks und klassenspezifische Skill-Sounds fehlen (B-573 bis B-576).

## Ziel

Die vier Sounds sind eingebunden. Am Ende sichtbar: Sound im Spiel und in den Menüs, Credits ergänzt.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-573, B-574, B-575, B-576 › Anforderungen.

## Nicht-Ziele

Neuer Musik-Katalog über diese Stücke hinaus.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Nach dem Fall der Burg spielt ein eigenes Stück (B-573/AC).
- **AC-02** Reiten hat eine Hufschlag-Schleife je Spieler (B-574/AC).
- **AC-03** Die Menü-Szenen haben UI-Klick-Töne (B-575/AC).
- **AC-04** Jede Klasse hat einen eigenen Skill-Sound (B-576/AC). Verschoben: braucht ein Skill-Ereignis oder Klassenfeld im Protokoll (B-576 bleibt offen).
- **AC-05** Quellen und Lizenzen stehen in den Sound-Credits.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO7.1 | `SO7.1-sounds-umsetzen.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-03 und AC-05 umgesetzt, geprüft mit Vitest und tsc. AC-04 verschoben (B-576). Hörprobe der selbst erzeugten Töne am TV bleibt bei 🧑.
