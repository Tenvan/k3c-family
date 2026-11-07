# PS1 · CLI · Papierschnitt-Sichtprüfungen und Abnahmen gebündelt

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-699, B-701
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

🧑 hat festgelegt, dass alle Sichtprüfungen und manuellen Abnahmen des Projekts PAP so spät wie möglich gebündelt laufen. Bis dahin arbeiten die Agenten mit den Defaults aus B-701 und mit automatischen Prüfungen (Golden-Tests, `assets_preview`-Selbstkontrolle).

## Ziel

🧑 hat in einem Durchgang die Entscheidungen bestätigt, den Look geprüft und das Spiel im Papierschnitt abgenommen; Befunde sind Tickets.

Am Ende sichtbar: B-701 ohne offene Zeile, Abnahme-Notizen je Gruppe.

## Beteiligte und Zielgruppen

- **Ralf (🧑):** entscheidet in PS1.1–PS1.4, prüft Look und Spiel.
- **Claude:** bereitet vor (Prüflisten), setzt in PS1.4 die Review-Änderung, schreibt in PS1.5 Tickets und schließt ab.
- **Profitieren:** Spieler (einheitlicher Look), Lanes der Folgesprints (klare Defaults).

## Anforderungen

B-701 › Anforderungen; Entscheidung 005 › AC-11.

## Nicht-Ziele

Korrekturen selbst (Folgetickets), Aufräumen (CU1).

## Regeln und Einschränkungen

Nach US1, vor CU1 (Aufräumen erst nach Abnahme). Alle Sessions `Agent: Mensch` außer PS1.5.

## Beispiele

- D-Zeile passt → ✅, Status in PS1.5 `bestätigt`.
- D-Zeile soll anders sein, z. B. Rappe schwarz statt dunkelbraun (D-MG2-01) → ✏️, PS1.5 legt ein Ticket in der Lane `reittiere` an.
- Lesbarkeits-Befund „Lava-Golem gleich Splitter-Titan“ (D-MG5-04) → Ralf entscheidet lassen oder nachbessern.

## Ausnahme- und Fehlerfälle

- US1 noch nicht gemerged → PS1.4 wartet; PS1.1–PS1.3 laufen schon, Sprint bleibt `geplant`, bis US1 gemerged ist.
- MG8 noch nicht gemerged → PS1.3 Schritt 8 offen, Session wartet.
- Workbench läuft nicht → Assets über `task k3c-workbench:web` öffnen.
- Ralf bricht ab → Teilergebnis in die Session, Status bleibt `in Arbeit`.

## Akzeptanzkriterien

- **AC-01** B-701: jede Zeile bestätigt oder geändert.
- **AC-02** resvg-Look gegen Prototyp geprüft (früher PV1-Sichtprüfung).
- **AC-03** Jede Migrationsgruppe MG1–MG8 im Bereich Assets geprüft.
- **AC-04** Spiel im PC-Browser im Papierschnitt-Look abgenommen; Review einer echten Änderung am Monarchen in Assets ohne Dateien zu öffnen (005/AC-11).
- **AC-05** Alle Befunde als Tickets im Projekt PAP.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PS1.1 | `PS1.1-b701-durchgehen.md` | Workshop | Mensch | offen |
| PS1.2 | `PS1.2-resvg-gegen-prototyp.md` | Workshop | Mensch | offen |
| PS1.3 | `PS1.3-gruppen-in-assets.md` | Workshop | Mensch | offen |
| PS1.4 | `PS1.4-spiel-und-review-lauf.md` | Workshop | Mensch | offen |
| PS1.5 | `PS1.5-befunde-tickets-abschluss.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
