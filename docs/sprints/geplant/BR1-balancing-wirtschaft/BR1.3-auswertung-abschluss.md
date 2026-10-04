# BR1.3 · Auswertung, Werte nachziehen, Pass/Fail festhalten und Sprint abschließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** br1/3-auswertung-abschluss
- **Abhängig von:** BR1.1, BR1.2
- **Tickets:** B-155, B-015
- **Kriterien:** AC-02, AC-04

## Ziel

Die Werte aus Messung und Abend sind in `data/` nachgezogen und in `docs/rules/` begründet, je Zielkorridor der Wirtschaft ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket; der Sprint ist abgeschlossen (Doku-Sprint, kein Review).

## Kontext

- **Eingaben:** Messung und Vorschlagsliste aus BR1.1 (Abschnitt „Balancing-Runde Wirtschaft (BR1)“ in `docs/rules/zielkorridore.md`, sonst Ergebnis von BR1.1), Protokoll von Spieleabend 2 in `docs/playtests/` (BR1.2). Die Auswertung berücksichtigt Spielmetrik und Fragebogen. **Werte ändert nur 🧑 mit Beschluss** (B-155, `docs/arbeitsweise.md`): Der Agent schlägt vor und setzt nur um, was 🧑 bestätigt hat (im Protokoll oder im Chat dokumentiert, Quelle im Commit). Was nicht bestätigt ist, bleibt Vorschlag oder wird Ticket; nichts erfinden.
- **Werte:** nur in `data/hub.json`, `data/buildings.json`, `data/economy.json`, `data/troops.json` (Zuständigkeit B-155 AC-03); keine neuen Felder (SIM).
- **Begründung:** je geändertem Wert ein Eintrag in `docs/rules/wirtschaft.md` oder `docs/rules/materialien-gebaeude.md` (Truppen: `docs/rules/buerger.md`); HP und Kosten aller Gebäude (B-015/AC-01) stehen begründet in `materialien-gebaeude.md` § 3.
- **Commit-Regel (B-155/AC-02):** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung. Messung mit demselben Befehl wie in BR1.1 wiederholen (`task balance`, sonst Go-Tests oder `sim_run`).
- **Golden:** Ablauf nach `docs/arbeitsweise.md` › „Golden aktualisieren“: `task golden:update`, `git diff --stat testdata/golden` lesen, Begründung im Commit-Text (Regel, Wert, Kennzahl), Golden-Änderungen **in eigenem Commit**, `rng.json` nie ändern. Mangels Review-Session im Doku-Sprint den Diff-Stand im Ergebnis nennen.
- **Pass/Fail:** je Zielkorridor der Wirtschaft (Zeilen aus BR1.1) Pass oder Fail mit Messwert nach der Änderung; Fail = Abweichung mit Ticket (Typ Idee oder Problem) und Entscheidung durch 🧑 (Spec › Ausnahmefälle: ein nicht erreichbarer Korridor wird Ticket, 🧑 entscheidet neu).
- **Abschluss des Doku-Sprints (Schritte 4 bis 5 der Review-Session, `docs/arbeitsweise.md` › Review-Session):** Abnahme (höchstens fünf Zeilen) in die Sprint-README, B-155 und B-015 nach `docs/backlog/archiv/` (Index-Zeile in „Archiv“; B-015 erst, wenn AC-01 und AC-02 beide belegt sind), Sprint-Ordner nach `docs/sprints/erledigt/`, Fahrplan anpassen, `git merge origin/develop`, den einen PR des Sprints öffnen. Versionsvorschlag in der Abnahme: **Minor**, wenn Werte in `data/` geändert wurden, sonst **Patch**; Setzen erst nach Bestätigung durch 🧑. Ein Release-Tag nach dem Spieleabend ist Sache der Release-Checkliste (RL1, Q20).

## Erlaubte Dateien

- `data/hub.json`, `data/buildings.json`, `data/economy.json`, `data/troops.json` (nur Werte, nur bestätigte)
- `testdata/golden/` (nur über `task golden:update`), `testdata/balance/` (Baseline, falls vorhanden; Begründung im Commit)
- `docs/rules/` (Begründungen, Pass/Fail-Abschnitt), `docs/playtests/` (nur Verweis auf Folge-Tickets im Protokoll)
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md` (Abschluss)

## Nicht-Ziele

Kampf und Bosse (BR2), neue Mechaniken, Ausbau des Testers, Änderung der Korridor-Zahlen, Werte ohne Beschluss von 🧑.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Protokoll BR1.2 und Vorschlagsliste BR1.1 lesen; bestätigte Änderungen von 🧑 auflisten.
2. Werte in `data/` ändern, Messung wiederholen, Begründung in `docs/rules/` eintragen.
3. `task golden:update` mit Begründung (eigener Commit); `git diff --stat testdata/golden` prüfen.
4. Pass/Fail je Zielkorridor der Wirtschaft festhalten; je Abweichung ein Ticket (Vorlage, Index in `docs/backlog/README.md` am Ende).
5. `task check` und `task check:go`.
6. Sprint abschließen: Abnahme in die Sprint-README, B-155 und B-015 archivieren, Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan, PR öffnen.

## Fertig, wenn

- [ ] AC-02: Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen (vor und nach) und ist in `docs/rules/` begründet; die Begründungen für `hub.json`, `buildings.json`, `economy.json` und `troops.json` liegen vor (Stichprobe durch 🧑).
- [ ] AC-04: Je Zielkorridor der Wirtschaft steht Pass oder Fail; jede Abweichung hat ein Ticket; `task check` und `task check:go` sind grün.
- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session; Abnahme mit Versionsvorschlag steht in der Sprint-README, Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
