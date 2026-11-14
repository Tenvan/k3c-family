# PJ1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Projekte, Rang und Domäne je Session in Regeln, Vorlagen und Planungstest
- **Projekt:** PRZ

## Ziel

Regeln, Glossar, Vorlagen und `tests/planning.test.ts` kennen die drei Ebenen Projekt → Sprint → Session mit Rang, die Domäne je Session und den Session-Status `verworfen`. **Am Ende sichtbar:** `docs/vorlagen/projekt.md`, `docs/projekte/README.md` (noch leer), neue Abschnitte in `docs/arbeitsweise.md`, `task test -- planning` grün mit Negativtests für die neuen Regeln.

## Ergebnis

2026-10-08, Review PJ1.4: AC-01, AC-02, AC-06 und AC-09 siehe PJ1.1 und PJ1.2, AC-03, AC-05 und AC-08 siehe PJ1.2, AC-04 und AC-07 siehe PJ1.3 (alle mit Nachweis, nichts umformuliert). Diff: 876 Dateien, je Planungsdatei genau eine Feldzeile; `task check` grün (`check:go` entfällt, keine Go-Änderung). Übergangsregel („je Domäne ein aktiver Sprint“ nur für Sprints mit `Projekt: –`) in Arbeitsweise und Test deckungsgleich. Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine. B-338 bleibt `eingeplant` (AC-02 gehört zu PJ2). Version: v0.14.1 vorgeschlagen (Patch: Doku, Planung un …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | B-355/AC-01 (Vorlage Projekt, Übersicht, Glossar). |
| AC-02 | – | B-355/AC-02 (Arbeitsweise: Projekt, Rang, Auswahl, ein aktiver Sprint je Projekt). |
| AC-03 | – | B-355/AC-03 (Feld `Projekt` an Sprint und Ticket). |
| AC-04 | – | B-355/AC-04 (Planungstest für Projekte). |
| AC-05 | – | B-356/AC-01 (Vorlagen: Domäne je Session, Liste am Sprint). |
| AC-06 | – | B-356/AC-02 (Arbeitsweise und Glossar: Domäne je Session). |
| AC-07 | – | B-356/AC-03 (Planungstest für Domänen). |
| AC-08 | – | B-356/AC-04 (alle Session-Dateien mit `Domäne`). |
| AC-09 | – | B-338/AC-01 (`verworfen` in Vorlage und Test). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-355 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Projekte bündeln Sprints zu Themen und werden nach Rang abgearbeitet |
| B-356 | erledigt | AC-05 –, AC-06 –, AC-07 –, AC-08 – | Eine Session gehört zu genau einer Domäne, ein Sprint darf mehrere Domänen nache … |
| B-338 | erledigt | AC-09 – | Sessions können den Status verworfen tragen |
