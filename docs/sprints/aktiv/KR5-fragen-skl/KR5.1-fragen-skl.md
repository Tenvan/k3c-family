# KR5.1 · Workshop: kritische Fragen zu SKL entscheiden (🧑)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** kr5/1-fragen-skl
- **Abhängig von:** –
- **Tickets:** B-559, B-560, B-561
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat B-559 bis B-561 entschieden; bestätigte sind erledigt, geänderte haben Folgearbeit im Projekt SKL.

## Kontext

Tabelle in `docs/sprints/aktiv/KR5-fragen-skl/README.md` › Anforderungen, Optionen je Ticket unter Anforderungen. B-560/B-561 mit SK2.1 und SK2.5 am TV ansehen (Skill-Ringe, Skill-Menü mit `learn.skills`).

## Erlaubte Dateien

- `docs/backlog/B-559*` … `B-561*`, `B-503*`, `B-504*` (über `plan_*`)
- neue Tickets/Sprints im Projekt SKL (über `plan_create`)

## Nicht-Ziele

Umsetzung der Folgearbeit; die TV-Abnahme selbst.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude setzt bestätigte auf `erledigt`, plant geänderte als Ticket/Sprint im Projekt SKL; SK2.1 und Regeltext (`docs/rules/monarch.md`) nachziehen.
3. KR5.1 auf `fertig`.

## Fertig, wenn

- [ ] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Folgearbeit.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Workshop 2026-10-23 mit Ralf (Claude moderiert und protokolliert).

- **AC-01 geprüft:** B-559, B-560, B-561 tragen Ralfs Entscheidung unter Notizen.
- **AC-02 geprüft:** Bestätigt und `erledigt`: B-559 (a, Feld `skills` in `learn`). Geändert und `erledigt` mit Verweis: B-560 (b, Dieb-Skills bauen → B-577, SIM-Sprint mit Golden und BAL-Lauf), B-561 (b, eigener Effekt je Skill → B-578, CLI).
- **Neue Tickets (Projekt SKL):** B-577, B-578 (Spec Entwurf; B-578 nach B-577).
- **Abweichungen:** B-561 hing laut Plan an der TV-Abnahme SK2.5; Ralf hat sie vorab entschieden. SK2.5 prüft weiter den Ring aus SK2.3. Der Regeltext `docs/rules/monarch.md` („der Dieb folgt später“) wird erst mit B-577 nachgezogen, deshalb bleibt SK2.1 AC-01 offen.
