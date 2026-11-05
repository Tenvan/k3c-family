# KB6 · SIM, REG · Kampf-Nachträge: Wellenabstand unter Tage, stabiler Schutzplatz

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** SIM, REG
- **Reife:** bereit
- **Tickets:** B-579, B-605
- **Start-Commit:** 29493645
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

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

keine. Entschieden (🧑, 2026-10-31): Hebel für B-579 ist Adern `percentPerGather` 1 → 2 (B-626). Der Messtest `K3C_MESSUNG=1` prüft den Korridor 4–12 min; weitere Hebel nur, wenn er rot bleibt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB6.1 | `KB6.1-schutzplatz-stabil.md` | Umsetzung | autonom | fertig |
| KB6.2 | `KB6.2-wellenabstand-messen.md` | Umsetzung | autonom | fertig |
| KB6.3 | `KB6.3-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- KB6.1 (SIM) Schutzplatz stabil (AC-02).
- KB6.2 (REG) Wellenabstand messen und Hebel wählen (AC-01).
- KB6.3 Review, alle Kriterien, schließt ab.

## Abnahme

2026-11-05: AC-01 (KB6.2: `percentPerGather` 4, erste Welle 4,10 min, Abstand Median 9,42 min) und AC-02 (KB6.1) nachgewiesen; `task check`, `check:go`, `check:diff` grün, `check:race` lokal übersprungen (kein gcc).
Behoben im Review: Korridor-Assertion geklammert, `docs/data/biomes.md`. Ungeprüft: Wirkung von `percentPerGather` 4 auf Höhle, Kristall, Ironhold.
Neue Tickets: B-696, B-698 (offen), B-697 (erledigt).
