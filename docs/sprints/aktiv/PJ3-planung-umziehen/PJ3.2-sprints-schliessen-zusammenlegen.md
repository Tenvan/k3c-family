# PJ3.2 · Sprints schließen, zusammenlegen, RM1 teilen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** live
- **Branch:** pj3/2-sprints-schliessen
- **Abhängig von:** PJ3.1
- **Tickets:** B-359
- **Kriterien:** AC-02, AC-03

## Ziel

`aktiv/` enthält nur noch Sprints mit offener autonomer Arbeit: Die sieben Sprints, die nur auf ein Gerät warten, sind erledigt und ihre Gerät-Sessions stehen in HW1. RG3, BAL5, SO5 und DBG4 sind in ihren Zielsprints aufgegangen, B-214 hängt ohne Sprint an BED.

## Kontext

- Spec: B-359 › Anforderungen (Schließen, Zusammenlegen, RM1 teilen) und › Beispiele (SO1, RG3). Projekte und Zuordnung stehen seit PJ3.1.
- **Schließen** (offene Session in Klammern, alle `Agent: Mensch`): DBG3 (DBG3.4), LP1 (LP1.4), LT1 (LT1.3), MON2 (MON2.4), RL1 (RL1.2), SO1 (SO1.5), SO3 (SO3.3). Je Sprint: für die Session eine neue Session in HW1 anlegen (`plan_create kind=session`, `Agent: Mensch`, Domäne und Kriterien-Bezug der alten Session, Titel „… (aus SO1.5)“, Inhalt von Ziel und Schritte übernehmen), alte Session `Status: verworfen` mit Ergebnis „nach HW1.n verschoben“, in der Abnahme des alten Sprints eine Zeile „AC-xx angenommen, Validierung in HW1.n“, dann Sprint `Status: erledigt` (das Tool verschiebt den Ordner). HW1 ist ein Entwurf mit HW1.1/HW1.2 als Stichpunkten; neue Sessions ab HW1.3. HW1 bleibt `Spec: Entwurf`.
- **Zusammenlegen** (Ziel ← Quelle): RG1 ← RG3, BAL6 ← BAL5, SO4 ← SO5, M10 ← DBG4. Tickets der Quelle per `plan_set {Sprint}` an das Ziel; Kriterien der Quelle als neue IDs am Ende der Ziel-Kriterien anhängen (nie umnummerieren), Stichpunkte der Sessions übernehmen; Ziel-Spec `Entwurf`, `Revision` + 1, `Freigabe: –` (SO4 verliert damit seine Freigabe vom alten Stand, B-359 › Regeln); Quelle mit `plan_delete` löschen und aus der Projekt-Tabelle nehmen.
- **RM1 teilen:** B-214 `Sprint: –`, `Projekt: BED`; RM1 behält B-285, sein Kriterium zur Pause bleibt mit Vermerk „entfällt, B-214 an BED“ stehen (keine Umnummerierung), `Revision` + 1.
- Fallstrick: Ein Sprint der Liste ist inzwischen erledigt oder hat wieder autonome Arbeit → nicht schließen, im Ergebnis nennen.
- Im Checkout der Repo-Wurzel arbeiten (B-275), `Checkout:`-Zeile prüfen.

## Erlaubte Dateien

- `docs/sprints/` (Status, Sessions, Abnahme, Zusammenlegen, Ordner, Fahrplan)
- `docs/projekte/` (Sprint-Tabellen)
- `docs/backlog/` (Feld `Sprint`/`Projekt`, neue Tickets)

## Nicht-Ziele

Gerät-Prüfungen durchführen, HW1 bereit machen oder freigeben, Inhalte der Zielsprints über das Übernehmen hinaus ändern, Fahrplan umgliedern (PJ3.3).

## Schritte

1. Die sieben Sprints der Reihe nach schließen (Kontext › Schließen).
2. RG3, BAL5, SO5, DBG4 in RG1, BAL6, SO4, M10 aufgehen lassen (Kontext › Zusammenlegen), danach je Projekt die Reihenfolge per `plan_set {Sprints}` prüfen.
3. RM1 teilen.
4. `check_run task:test pattern=planning` grün, committen.

## Fertig, wenn

- [ ] AC-02: DBG3, LP1, LT1, MON2, RL1, SO1, SO3 liegen in `docs/sprints/erledigt/`, ihre Gerät-Sessions sind dort `verworfen` und stehen als Sessions in HW1.
- [ ] AC-03: RG3, BAL5, SO5, DBG4 gelöscht, ihre Tickets und Kriterien in RG1, BAL6, SO4, M10; B-214 mit `Projekt: BED` ohne Sprint.
- [ ] `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
```

`check_run task:test pattern=planning` über k3c-dev. Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
