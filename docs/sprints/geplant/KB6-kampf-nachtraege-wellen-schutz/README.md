# KB6 · REG · Kampf-Nachträge: Wellenabstand unter Tage, stabiler Schutzplatz

- **Status:** geplant
- **Projekt:** KMP
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-579, B-605
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Unter Tage liegt der Wellenabstand mit besetzten Adern über dem Korridor 4–12 Minuten (B-579); Truppen verlieren ihren Schutzplatz, wenn andere sterben oder hinzukommen (B-605).

## Ziel

Wellenabstand unter Tage im Korridor, Schutzplätze stabil während einer Gefahr. Am Ende sichtbar: Messtest `K3C_MESSUNG=1` im Korridor, Go-Tests für den Schutzplatz grün.

## Beteiligte und Zielgruppen

Wer Kampf und Balancing pflegt; Agenten REG und SIM; 🧑 entscheidet über die Wertanpassung.

## Anforderungen

`B-579 › Anforderungen`, `B-605 › Anforderungen`.

## Nicht-Ziele

Neue Gegner oder Events.

## Regeln und Einschränkungen

Golden nur mit Begründung; Messläufe nur über `sim_test`; Werte in `data/`; deterministisch.

## Beispiele

Besetzte Adern unter Tage → Wellenabstand zwischen 4 und 12 Minuten.

## Ausnahme- und Fehlerfälle

Truppe stirbt während der Gefahr → übrige Truppen behalten ihren Platz.

## Akzeptanzkriterien

- **AC-01** Wellenabstand unter Tage mit besetzten Adern liegt im Korridor (`B-579/AC-01`).
- **AC-02** Schutzplätze bleiben bei Tod und Zugang von Truppen stabil (`B-605/AC-01`, `AC-02`).

## Offene Fragen

B-579 hat Spec Entwurf: Welcher Hebel (Adern-Takt, Pool) gilt? Entscheidet 🧑 vor der Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- KB6.1 (SIM) Schutzplatz stabil (AC-02).
- KB6.2 (REG) Wellenabstand messen und Hebel wählen (AC-01).
- KB6.3 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
