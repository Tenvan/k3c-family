# PS1 · CLI · Papierschnitt-Sichtprüfungen und Abnahmen gebündelt

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-699, B-701
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

🧑 hat festgelegt, dass alle Sichtprüfungen und manuellen Abnahmen des Projekts PAP so spät wie möglich gebündelt laufen. Bis dahin arbeiten die Agenten mit den Defaults aus B-701 und mit automatischen Prüfungen (Golden-Tests, `assets_preview`-Selbstkontrolle).

## Ziel

🧑 hat in einem Durchgang die Entscheidungen bestätigt, den Look geprüft und das Spiel im Papierschnitt abgenommen; Befunde sind Tickets.

Am Ende sichtbar: B-701 ohne offene Zeile, Abnahme-Notizen je Gruppe.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-701 › Anforderungen; Entscheidung 005 › AC-11.

## Nicht-Ziele

Korrekturen selbst (Folgetickets), Aufräumen (CU1).

## Regeln und Einschränkungen

Nach US1, vor CU1 (Aufräumen erst nach Abnahme). Alle Sessions `Agent: Mensch` außer PS1.5.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-701: jede Zeile bestätigt oder geändert.
- **AC-02** resvg-Look gegen Prototyp geprüft (früher PV1-Sichtprüfung).
- **AC-03** Jede Migrationsgruppe MG1–MG8 im Bereich Assets geprüft.
- **AC-04** Spiel im PC-Browser im Papierschnitt-Look abgenommen; Review einer echten Änderung am Monarchen in Assets ohne Dateien zu öffnen (005/AC-11).
- **AC-05** Alle Befunde als Tickets im Projekt PAP.

## Offene Fragen

keine

## Sessions

Reife Entwurf:

- PS1.1 Mensch: B-701 durchgehen (AC-01)
- PS1.2 Mensch: resvg-Ausgabe gegen Prototyp (AC-02)
- PS1.3 Mensch: Gruppen MG1–MG8 in Assets (AC-03)
- PS1.4 Mensch: Spiel und Review-Lauf (AC-04)
- PS1.5 Claude (Sonnet): Befunde als Tickets, Sprint schließen (AC-05)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
