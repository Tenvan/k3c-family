# B-222 · Das Glossar nennt Spielstand-Version 3

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** erledigt
- **Sprint:** S1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S1.4 hat `IslandSaveVersion` auf 3 gehoben (`engine/sim/island_save.go`, Fixture `testdata/saves/v3/`). `docs/glossar.md` › Spielstand-Version sagt noch „heute 2“; die Datei gehörte nicht zu den erlaubten Dateien von S1.4.

## Ziel

Das Glossar beschreibt den Ist-Stand der Spielstand-Version.

## Beteiligte und Zielgruppen

Agenten und 🧑, die das Glossar vor jeder Session lesen.

## Anforderungen

- Der Eintrag nennt Version 3 und was sie enthält (Fund-Pool `skillPool`, je Spieler `skills` und `slots`); Hub- und Platz-Stufen kommen mit W1.3 optional in Version 3 dazu (Q42).

## Nicht-Ziele

Weitere Glossar-Änderungen; Version 4 (B-201).

## Regeln und Einschränkungen

`CLAUDE.md` › Glossar ist verbindlich.

## Beispiele

nicht relevant (reine Doku-Korrektur).

## Ausnahme- und Fehlerfälle

nicht relevant (reine Doku-Korrektur).

## Akzeptanzkriterien

- **AC-01** `docs/glossar.md` › Spielstand-Version nennt „heute 3“ mit Pool und Verteilung je Spieler.

## Offene Fragen

keine

## Notizen

Gefunden in S1.4 (Sprint S1).
