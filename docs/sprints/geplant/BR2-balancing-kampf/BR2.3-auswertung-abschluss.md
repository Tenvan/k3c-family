# BR2.3 · Auswertung, Werte nachziehen, Pass/Fail festhalten und Sprint abschließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** br2/3-auswertung-abschluss
- **Abhängig von:** BR2.1, BR2.2
- **Tickets:** B-156
- **Kriterien:** AC-02, AC-04, AC-05

## Ziel

Die Werte aus Messung und Abend sind in `data/` nachgezogen und in `docs/rules/` begründet, je Zielkorridor für Kampf und Bosse ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket; der Sprint ist abgeschlossen (Doku-Sprint, kein Review).

## Kontext

- **Eingaben:** Messung und Vorschlagsliste aus BR2.1 (Abschnitt „Balancing-Runde Kampf und Bosse (BR2)“ in `docs/rules/zielkorridore.md`, sonst Ergebnis von BR2.1), Protokoll von Spieleabend 3 in `docs/playtests/` (BR2.2). **Werte ändert nur 🧑 mit Beschluss** (`docs/arbeitsweise.md`): Der Agent schlägt vor und setzt nur um, was 🧑 bestätigt hat (Quelle im Commit). Was nicht bestätigt ist, bleibt Vorschlag oder wird Ticket; nichts erfinden.
- **Werte:** nur in `data/enemies.json`, `data/waves.json`, `data/difficulty.json` (Zuständigkeit B-156 AC-03; Bosswerte dort, wo K2 sie ablegt, siehe Ergebnis von BR2.1); keine neuen Felder (SIM).
- **Begründung:** je geändertem Wert ein Eintrag in `docs/rules/gegner.md` oder `docs/rules/bosse.md`.
- **Commit-Regel (B-156/AC-02):** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung. Messung je Grad mit demselben Befehl wie in BR2.1 wiederholen (`task balance`, sonst Go-Tests oder `sim_run`).
- **Golden:** Ablauf nach `docs/arbeitsweise.md` › „Golden aktualisieren“: `task golden:update`, `git diff --stat testdata/golden` lesen, Begründung im Commit-Text (Regel, Wert, Kennzahl), Golden-Änderungen **in eigenem Commit**, `rng.json` nie ändern. Mangels Review-Session im Doku-Sprint den Diff-Stand im Ergebnis nennen.
- **Pass/Fail:** je Zielkorridor für Kampf und Bosse (Zeilen aus BR2.1) Pass oder Fail je Grad mit Messwert nach der Änderung; Fail = Abweichung mit Ticket und Entscheidung durch 🧑 (Spec › Ausnahmefälle: ein nicht erreichbarer Korridor wird Ticket, 🧑 entscheidet neu).
- **Abschluss des Doku-Sprints (Schritte 4 bis 5 der Review-Session, `docs/arbeitsweise.md` › Review-Session):** Abnahme (höchstens fünf Zeilen) in die Sprint-README, B-156 nach `docs/backlog/archiv/` (Index-Zeile in „Archiv“), Sprint-Ordner nach `docs/sprints/erledigt/`, Fahrplan anpassen, `git merge origin/develop`, den einen PR des Sprints öffnen. Versionsvorschlag in der Abnahme: **Minor**, wenn Werte in `data/` geändert wurden, sonst **Patch**; Setzen erst nach Bestätigung durch 🧑. Release-Tag nach dem Spieleabend: Release-Checkliste (RL1, Q20), nicht in dieser Session.

## Erlaubte Dateien

- `data/enemies.json`, `data/waves.json`, `data/difficulty.json` (nur Werte, nur bestätigte)
- `testdata/golden/` (nur über `task golden:update`), `testdata/balance/` (Baseline, falls vorhanden; Begründung im Commit)
- `docs/rules/` (Begründungen, Pass/Fail-Abschnitt), `docs/playtests/` (nur Verweis auf Folge-Tickets im Protokoll)
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md` (Abschluss)

## Nicht-Ziele

Wirtschaft (BR1), neue Mechaniken, Release (RL1), Änderung der Korridor-Zahlen, Werte ohne Beschluss von 🧑.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Protokoll BR2.2 und Vorschlagsliste BR2.1 lesen; bestätigte Änderungen von 🧑 auflisten.
2. Werte in `data/` ändern, Messung je Grad wiederholen, Begründung in `docs/rules/` eintragen.
3. `task golden:update` mit Begründung (eigener Commit); `git diff --stat testdata/golden` prüfen.
4. Pass/Fail je Zielkorridor festhalten; je Abweichung ein Ticket (Vorlage, Index in `docs/backlog/README.md` am Ende).
5. `task check` und `task check:go`.
6. Sprint abschließen: Abnahme in die Sprint-README, B-156 archivieren, Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan, PR öffnen.

## Fertig, wenn

- [ ] AC-02: Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen (vor und nach) und ist in `docs/rules/` begründet; die Begründungen für `enemies.json`, `waves.json` und `difficulty.json` liegen vor (Stichprobe durch 🧑).
- [ ] AC-04: Je Zielkorridor für Kampf und Bosse steht Pass oder Fail; jede Abweichung hat ein Ticket; `task check` und `task check:go` sind grün.
- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session; Abnahme mit Versionsvorschlag steht in der Sprint-README, Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
