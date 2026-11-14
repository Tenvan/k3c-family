# BI2 · CLI · Biom-Eigenschaften sichtbar: Kristallfarben, Decke, Tropfsteine, Verfall, Glut, Leuchten

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit BI1 (B-891) haben die Biome Darstellungs-Eigenschaften und Kristallfarben in den Daten; das native Spiel zeigt davon nur die Baumart.

## Ziel

Kristallfarben, Deckenhöhe, Tropfsteine, Stützbalken und Verfall, Hitze und Glut sowie Leuchtstärke sind im nativen Spiel sichtbar.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Siehe B-900 bis B-906.

## Nicht-Ziele

Neue Eigenschaften; Balancing.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `pfad/**` (Vereinigung der Erlaubten Dateien der Sessions ohne Sammeldateien; ein Sprint = eine Dateimenge)
- **Abhängig von:** – (nur IDs: SP00, B-000, SP00.1; erst starten, wenn erledigt und gemergt)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; Abnahmen am Gerät sind keine Abhängigkeit)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Kristall-Adern in drei Farben als Papierschnitt (B-900/AC-01).
- **AC-02** Kristall-Adern zeigen ihre Farbe (B-901/AC-01).
- **AC-03** Deckenhöhe sichtbar (B-902/AC-01).
- **AC-04** Tropfsteine nach Wert (B-903/AC-01).
- **AC-05** Stützbalken und Verfall nach Wert (B-904/AC-01).
- **AC-06** Glut nach Wert (B-905/AC-01).
- **AC-07** Leuchtstärke nach Wert (B-906/AC-01).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
