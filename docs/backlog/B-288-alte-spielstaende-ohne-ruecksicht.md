# B-288 · Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RP1
- **Projekt:** REL
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/arbeitsweise.md` › „Spielstand-Format ändern“ (B-137) verlangt bei jeder Formatänderung eine neue `IslandSaveVersion`, eine Migration in `ParseIslandSave` und ein unverändertes Fixture je alter Version (`testdata/saves/v<n>/`, `engine/sim/save_migration_test.go`). Beschluss 🧑 2026-10-05 (Chat, Sprint W1): Bis auf Widerruf wird keine Rücksicht auf alte Spielstände genommen, es gibt noch keine relevanten.

## Ziel

Die Regel in `docs/arbeitsweise.md` entspricht dem Beschluss; Sessions bauen keine Migrationen mehr nur für alte Stände.

## Beteiligte und Zielgruppen

Entwickelnde Agenten; 🧑 entscheidet, ob Migrationscode und alte Fixtures gelöscht werden.

## Anforderungen

- Abschnitt „Spielstand-Format ändern“ nennt den Beschluss (bis auf Widerruf, Datum).
- Release-Checkliste („Spielstand-Migration“) passt dazu.

## Nicht-Ziele

Spielstand-Format selbst ändern (SIM).

## Regeln und Einschränkungen

Domäne INF (`docs/arbeitsweise.md`, Tests in `tests/`); Löschen von Code in `engine/sim/` wäre ein SIM-Ticket.

## Beispiele

Neues Feld im Spielstand → Version bleibt oder steigt ohne Migration, Fixture wird aus dem Code neu erzeugt.

## Ausnahme- und Fehlerfälle

nicht relevant (Prozessregel).

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` › „Spielstand-Format ändern“ und „Release“ nennen den Beschluss vom 2026-10-05.

## Offene Fragen

Sollen Migrationscode (`island_save*.go`) und alte Fixtures v1 bis v3 gelöscht werden (🧑)?

## Notizen

Gefunden in W1.3; dort sind die neuen Felder optional in Version 4 ergänzt.
