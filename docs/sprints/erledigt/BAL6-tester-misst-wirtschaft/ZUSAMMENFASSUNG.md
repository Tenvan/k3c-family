# BAL6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Balancing-Tester misst die Wirtschaft
- **Projekt:** BAL

## Ziel

Alle Wirtschafts-Korridore aus `zielkorridore.md` › BR1 sind messbar, und der Bot des Standardszenarios spielt die Wirtschaft. Am Ende sichtbar: `task balance` zeigt je Wirtschafts-Korridor eine bewertete Zeile.

## Ergebnis

2026-10-25, Claude (BAL6.2): `task check:go`, `dev:test` und das Workbench-`go test` grün. AC-01 erfüllt durch BAL6.1 (13 Korridore bewertet), AC-02 und AC-03 durch BAL6.3. Review: keine Blocker, der Laufzeit-Hinweis ist ergänzt. Befund für BR1.3: Hub-Stufe 2 ist in Tiefe 0 mangels Stein nicht erreichbar. Neues Ticket B-625 (Baseline neu).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `task balance` bewertet alle Zeilen der Tabelle „Korridore der Wirtschaft“ (BR1) (`B-347/A … |
| AC-02 | geprüft | Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ (B-300/AC-01; aus BAL5 AC-0 … |
| AC-03 | geprüft | Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler (B-301/AC-01; aus BAL5 AC-02, PJ3) … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-347 | erledigt | AC-01 erfüllt | Der Balancing-Tester misst die Wirtschafts-Kennzahlen |
| B-300 | erledigt | AC-02 geprüft | Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ |
| B-301 | erledigt | AC-03 geprüft | Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler |
| B-099 | verworfen | – | Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele |
