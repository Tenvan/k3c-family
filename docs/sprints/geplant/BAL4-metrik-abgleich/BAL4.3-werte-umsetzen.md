# BAL4.3 · Beschlossene Werte umsetzen und Sprint abschließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** bal4/3-werte-umsetzen
- **Abhängig von:** BAL4.2
- **Tickets:** B-160
- **Kriterien:** AC-03

## Ziel

Die in BAL4.2 beschlossenen Wertänderungen stehen in `data/`, `task balance` und `task check:go` sind grün; der Sprint ist abgeschlossen (Doku-Sprint, kein Review).

## Kontext

- **Quelle der Änderungen:** nur die Beschlüsse „ändern“ in `docs/rules/abgleich-simulator.md` (BAL4.2) mit Datei, Pfad und neuem Wert. Ohne Beschluss keine Änderung; gibt es keinen Beschluss „ändern“, entfällt Schritt 2 bis 4 und das Kriterium wird mit „keine Änderung beschlossen“ nachgewiesen.
- **Domäne REG:** Werte in `data/*.json` ändern, keine neuen Felder (neue Felder wären SIM, dann Ticket). Begründung der Änderung in der zuständigen Regeldatei unter `docs/rules/` (`wirtschaft.md`, `gegner.md`, `bosse.md`, `materialien-gebaeude.md`, je nach Wert).
- **Golden:** Eine Werteänderung ändert Golden-Läufe. Ablauf nach `docs/arbeitsweise.md` › „Golden aktualisieren“: `task golden:update`, `git diff --stat testdata/golden` lesen (nur das, was die Regel erklärt), Begründung im Commit-Text (Regel, Wert, Kennzahl), Golden-Änderungen **in eigenem Commit**; `rng.json` nie ändern. Das Review der Golden-Änderung fehlt in diesem Sprint (kein Review-Session), daher im Ergebnis den Diff-Stand nennen.
- **Balance-Lauf:** `task balance` (entsteht in BAL2.2, Befehl und Bericht dort nachsehen) vor und nach der Änderung; die gekippten Kennzahlen stehen im Commit. Baseline-Update (`testdata/balance/baseline.json`, Vorschlag aus BAL2.2) nur mit Begründung im Commit.
- **Abschluss des Doku-Sprints (Schritte 4 bis 5 der Review-Session, `docs/arbeitsweise.md` › Review-Session):** Abnahme (höchstens fünf Zeilen) in die Sprint-README, B-160 archivieren, Sprint-Ordner nach `docs/sprints/erledigt/`, Fahrplan anpassen, `git merge origin/develop`, den einen PR des Sprints öffnen. Versionsvorschlag in der Abnahme: **Minor**, wenn Werte in `data/` geändert wurden (Wirkung im Spiel), sonst **Patch** (reine Doku); gesetzt wird die Version erst nach Bestätigung durch 🧑.

## Erlaubte Dateien

- `data/*.json` (nur Werte aus BAL4.2), `testdata/golden/` (nur über `task golden:update`), `testdata/balance/` (Baseline, falls vorhanden)
- `docs/rules/` (Begründung, Abgleichtabelle: Stand und Verweis auf die Änderung)
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md` (Abschluss)

## Nicht-Ziele

Werte, die BAL4.2 nicht beschlossen hat; Code-Änderungen; neue Kennzahlen oder Bot-Profile.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Beschlüsse aus BAL4.2 lesen.
2. `task balance` vor der Änderung (Stand festhalten).
3. Werte in `data/*.json` ändern, Begründung in `docs/rules/` eintragen.
4. `task golden:update` (eigener Commit, Begründung), `task balance` danach; gekippte Kennzahlen in den Commit-Text.
5. `task check:go` und `task check`.
6. Sprint abschließen: Abnahme in die Sprint-README, B-160 nach `docs/backlog/archiv/` (Index-Zeile in „Archiv“), Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan, PR öffnen.

## Fertig, wenn

- [ ] AC-03: Jede beschlossene Wertänderung steht in `data/`; `task balance` und `task check:go` sind grün (oder: „keine Änderung beschlossen“ im Ergebnis).
- [ ] Jeder geänderte Wert ist in `docs/rules/` begründet; die Golden-Änderung ist in einem eigenen Commit mit Begründung.
- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session; Abnahme mit Versionsvorschlag steht in der Sprint-README, Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task balance
task check:go
task check
```

## Ergebnis

–
