# WZ37 · DEV · Welt-Overrides im Welten-Editor und im Studio-Chat

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-682, B-683
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sobald Welten Werte übersteuern können (BAL8), fehlt die Bedienung im Welten-Editor (B-682) und im Studio-Chat (B-683).

## Ziel

Im Welten-Editor und im Chat lassen sich Overrides einer Welt setzen, zurücksetzen und gegen den Basiswert vergleichen. Am Ende sichtbar: Editor zeigt je Wert „geerbt“ oder „überschrieben“; Chat setzt einen Override als Aktionskarte.

## Beteiligte und Zielgruppen

Wer Welten pflegt; Agent DEV; 🧑 nimmt am PC ab.

## Anforderungen

`B-682 › Anforderungen`, `B-683 › Anforderungen`.

## Nicht-Ziele

Regeln und Simulation der Overrides (BAL8).

## Regeln und Einschränkungen

Beginnt erst nach BAL8; Oberfläche über `@webkit`; Studio schreibt `data/**`; Port 5196 für Tests.

## Beispiele

Basiswert 100, Override 150 → Editor zeigt beide, Zurücksetzen entfernt den Override.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Editor lehnt ab, nennt den Pfad.

## Akzeptanzkriterien

- **AC-01** Der Editor übersteuert und setzt zurück, zeigt geerbt und überschrieben (`B-682/AC-01` bis `AC-03`).
- **AC-02** Der Chat liest, setzt und entfernt Overrides als Aktionskarte (`B-683/AC-01`, `AC-02`).

## Offene Fragen

keine.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ37.1 (DEV) Editor für Overrides (AC-01).
- WZ37.2 (DEV) Chat-Aktionen (AC-02).
- WZ37.3 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
