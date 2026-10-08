# B-361 · Die Sprint-Felder Prio und Einschiebbar entfallen in Vorlage, Planungstest und plan-Tools

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit PJ2.2 ordnen die plan-Tools Sprints mit Projekt nur nach Rang und Platz in der Sprint-Tabelle. Die Felder `Prio` und `Einschiebbar` stehen trotzdem noch in `docs/vorlagen/sprint.md`, und `tests/planning.test.ts` verlangt `Prio = höchste Ticket-Prio` für jeden Sprint mit Tickets (Zeile 117) sowie „je Domäne ein aktiver, nicht einschiebbarer Sprint“. Deshalb schreiben `plan_create`/`plan_set` (`tools/k3c-dev/internal/planning/create.go`, `edit.go`) die Prio weiter für alle Sprints, auch für Sprints mit Projekt (Beschluss 🧑 2026-10-08 in PJ2.2). `plan_create` setzt `Einschiebbar: nein`, weil die Vorlage sonst einen Platzhalter hinterlässt. B-359 (PJ3) zieht nur die Daten um.

## Ziel

Sprints tragen keine Prio und kein `Einschiebbar` mehr; die Reihenfolge kommt allein aus dem Rang (arbeitsweise.md › Projekte und Rang).

## Beteiligte und Zielgruppen

🧑 priorisiert nur noch über den Rang; Agenten planen über die plan-Tools.

## Anforderungen

- Vorlage `sprint.md` ohne `Prio` und `Einschiebbar`; Planungstest ohne Prio-Ableitung und ohne Domänen-Sperre für einschiebbare Sprints.
- plan-Tools (SRV-Folge-Session) schreiben beide Felder nicht mehr, `SprintPrio` und `Einschiebbar`-Tabelle im Fahrplan entfallen.
- Bestehende Sprint-Dateien verlieren beide Felder (nach dem Umzug B-359).

## Nicht-Ziele

Ticket-Prio (bleibt, ordnet Tickets im Projekt).

## Regeln und Einschränkungen

Erst nach B-359, wenn alle offenen Sprints ein Projekt haben. INF ändert Vorlage und Test, SRV die plan-Tools, als eigene Sessions.

## Beispiele

Sprint-README nach Vorlage ohne `Prio` → Planungstest grün.

## Ausnahme- und Fehlerfälle

Sprint ohne Projekt nach dem Umzug → Planungstest meldet ihn (Regel aus B-359).

## Akzeptanzkriterien

- **AC-01** `docs/vorlagen/sprint.md` hat weder `Prio` noch `Einschiebbar`; `task test -- planning` grün.
- **AC-02** `plan_create {kind: sprint}` und `plan_set` schreiben weder `Prio` noch `Einschiebbar` (Go-Test in `tools/k3c-dev`).

## Offene Fragen

Ob die Fahrplan-Spalte `Prio` in `docs/sprints/README.md` mit entfällt (🧑).

## Notizen

Entstanden in PJ2.2 (Übergang Sprint-Prio).
