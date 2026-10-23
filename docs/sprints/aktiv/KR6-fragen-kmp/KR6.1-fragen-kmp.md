# KR6.1 · Workshop: kritische Fragen zu KMP entscheiden (🧑)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kr6/1-fragen-kmp
- **Abhängig von:** –
- **Tickets:** B-343, B-386, B-328
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat B-343, B-386 und B-328 entschieden; bestätigte sind erledigt, geänderte haben ein Umsetzungs-Ticket oder einen Sprint im Projekt KMP.

## Kontext

Sprint-Spec: `docs/sprints/geplant/KR6-fragen-kmp/README.md` › Anforderungen. Details je Ticket unter `Offene Fragen` und `Beispiele`: B-343 (`docs/backlog/B-343-bau-vor-dem-ausgang.md`, `engine/level/level.go` › `lairX`, B-261, Q28), B-386 (`docs/backlog/B-386-netz-budget-kampf-q08.md`, Messung in `docs/protocol.md` › Snapshot-Größe, B-154/AC-05, `docs/fragenkatalog.md` Q08), B-328 (`docs/backlog/B-328-feuergeist-flammen-flaeche.md`, `docs/rules/gegner.md` § 1 und § 2). Alle drei sind offline entscheidbar; keine Hörprobe oder Abnahme am Gerät nötig.

## Erlaubte Dateien

- `docs/backlog/B-343*`, `B-386*`, `B-328*` (über `plan_*`)
- `docs/fragenkatalog.md` (nur Q08, falls B-386 entschieden wird)
- neue Tickets oder Sprints im Projekt KMP (über `plan_create`)

## Nicht-Ziele

Umsetzung der Folgearbeit; B-327; Spec-Änderungen von B-154 ohne neue Freigabe.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die gewählte Alternative.
2. Claude setzt bestätigte Tickets auf `erledigt` und plant geänderte als Ticket oder Sprint im Projekt KMP; bei B-386 Q08 in `docs/fragenkatalog.md` nachtragen.
3. KR6.1 auf `fertig`.

## Fertig, wenn

- [ ] AC-01: B-343, B-386 und B-328 tragen Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Umsetzungs-Ticket oder Sprint.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Workshop 2026-10-23 mit Ralf (Claude moderiert und protokolliert).

- **AC-01 geprüft:** B-343, B-386 und B-328 tragen Ralfs Entscheidung unter Notizen.
- **AC-02 geprüft:** Alle drei bestätigt oder ohne neue Umsetzung entschieden, daher kein neues Ticket nötig und alle `erledigt`: B-343 (Lage und Abstand bleiben), B-386 (Variante B, Q08 nur für Ereignisse), B-328 (Flammen-Fläche aus der Regel gestrichen).
- **Folgearbeit in vorhandenen Sessions:** KB1.2 dokumentiert B-343, KB1.4 passt `docs/rules/gegner.md` § 1 an (B-328).
- **Nachgezogen von Claude:** B-154/AC-05 in Revision 3 umformuliert (Freigabe im Chat durch Ralf), B-154 `erledigt` (`go:test` `TestEventsBudgetJeTick` grün); Q08 in `docs/fragenkatalog.md` ergänzt.
- **Abweichung:** Ob K5, LV1 und KB2 weitere offene Fragen tragen, wurde nicht geprüft.
