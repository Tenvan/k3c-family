# KB6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Kampf-Nachträge: Wellenabstand unter Tage, stabiler Schutzplatz
- **Projekt:** KMP

## Ziel

Wellenabstand unter Tage im Korridor, Schutzplätze stabil während einer Gefahr. Am Ende sichtbar: Messtest `K3C_MESSUNG=1` im Korridor, Go-Tests für den Schutzplatz grün.

## Ergebnis

2026-11-05: AC-01 (KB6.2: `percentPerGather` 4, erste Welle 4,10 min, Abstand Median 9,42 min) und AC-02 (KB6.1) nachgewiesen; `task check`, `check:go`, `check:diff` grün, `check:race` lokal übersprungen (kein gcc). Behoben im Review: Korridor-Assertion geklammert, `docs/data/biomes.md`. Ungeprüft: Wirkung von `percentPerGather` 4 auf Höhle, Kristall, Ironhold. Neue Tickets: B-696, B-698 (offen), B-697 (erledigt).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Wellenabstand unter Tage mit besetzten Adern liegt im Korridor (`B-579/AC-01`). |
| AC-02 | geprüft | Schutzplätze bleiben bei Tod und Zugang von Truppen stabil (`B-605/AC-01`, `AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-579 | erledigt | AC-01 geprüft | Wellenabstand unter Tage mit besetzten Adern liegt über dem Korridor 4–12 Minute … |
| B-605 | erledigt | AC-02 geprüft | Schutzplatz bleibt stabil, wenn Truppen sterben oder hinzukommen |
