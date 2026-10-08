# DV1 · INF · Domäne DEV für k3c-dev, Sprint ohne Prio und Einschiebbar

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-365, B-361
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

k3c-dev (`tools/k3c-dev/`) gehört heute zur Domäne SRV. Sprints am Werkzeug sperren deshalb Server-Arbeit, obwohl sie keine Dateien mit ihr teilen (B-365). Außerdem tragen Sprints noch `Prio` und `Einschiebbar`, obwohl die Reihenfolge seit PJ2 aus dem Projekt-Rang kommt. Vorlage, Planungstest und plan-Tools verlangen bzw. schreiben beide Felder weiter (B-361).

## Ziel

Arbeit am Entwickler-Werkzeug läuft in einer eigenen Domäne `DEV` und sperrt SRV nicht mehr, und Sprints planen sich nur noch über den Rang ohne `Prio`/`Einschiebbar`. Am Ende sichtbar: `plan_list domain: DEV` listet die offenen Werkzeug-Tickets, die Planungsseite hat einen Filter-Chip `DEV`, ein neuer Sprint aus `plan_create` hat weder `Prio` noch `Einschiebbar`, und `task check` ist grün.

## Beteiligte und Zielgruppen

Agenten planen und arbeiten parallel an Werkzeug und Server. 🧑 entscheidet die offenen Fragen aus B-365 (Kürzel, Umfang) und B-361 (Fahrplan-Spalte) und gibt die Spec frei.

## Anforderungen

B-365 › Anforderungen; B-361 › Anforderungen. Sprint-eigen: Beide Tickets ändern dieselben Stellen (Vorlagen, `tests/planning.test.ts`/`tests/planningDocs.ts`, `tools/k3c-dev/internal/planning/`) und werden deshalb in einem Durchgang geändert. Zuerst Regeln, Vorlagen und Test (INF), danach die plan-Tools.

## Nicht-Ziele

Erledigte Tickets und Sprints auf `DEV` umschreiben (bleiben SRV). Ticket-Prio (bleibt). Weitere Funktionen von k3c-dev (B-360, B-362, B-363). Die Werkzeug-Seiten unter `src/tools/` gehören nur dazu, wenn 🧑 das in B-365 › Offene Fragen so entscheidet.

## Regeln und Einschränkungen

- Erst nach PJ3 (B-359): B-361 setzt voraus, dass alle offenen Sprints ein Projekt haben.
- Domäne je Session: INF für Regeln, Glossar, Vorlagen und Planungstest, SRV (nach DV1.2: `DEV`) für `tools/k3c-dev/`. Neue Begriffe zuerst ins Glossar.
- Planung nur über die plan-Tools. Steht in `workbench_status` die Repo-Wurzel, gilt der Rückfall von Hand im Worktree (B-275).
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit. Go-Tests für die plan-Tools, Vitest für den Planungstest.

## Beispiele

- `plan_create {kind: ticket, fields: {Domäne: DEV}}` → Ticket angelegt, `task test -- planning` grün.
- Sprint `X` in Domäne SRV aktiv, Session in `DEV` offen → beide laufen parallel, ohne dass sich die Domänen sperren.
- `plan_create {kind: sprint}` → README ohne `Prio` und `Einschiebbar`.
- B-361 › Beispiele.

## Ausnahme- und Fehlerfälle

- Unbekannte Domäne (z. B. `TOOL`) → `plan_create`/`plan_set` lehnen mit der Liste der gültigen Domänen ab, der Planungstest meldet die Datei.
- Ein `plan_set` mit `Prio` an einem Sprint → wird abgelehnt (B-360 regelt das allgemein, hier nur kein stilles Schreiben).
- B-361 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Die Domänen-Tabelle in `docs/arbeitsweise.md` nennt `DEV` mit `tools/k3c-dev/`, SRV nennt k3c-dev nicht mehr, das Glossar hat den Begriff (`B-365/AC-01`).
- **AC-02** `docs/vorlagen/sprint.md` hat weder `Prio` noch `Einschiebbar`, und `task test -- planning` ist grün (`B-361/AC-01`).
- **AC-03** `plan_create`/`plan_set` akzeptieren `Domäne: DEV`, und `task test -- planning` ist mit einem Ticket in `DEV` grün (`B-365/AC-02`, Go-Test).
- **AC-04** `plan_create {kind: sprint}` und `plan_set` schreiben weder `Prio` noch `Einschiebbar` (`B-361/AC-02`, Go-Test).
- **AC-05** Offene Tickets am Werkzeug tragen `DEV` (`B-365/AC-03`).

## Offene Fragen

- Kürzel `DEV` oder `TOOL`? Gehören `cmd/k3c-load`, `cmd/k3c-tui` und `src/tools/` dazu? (B-365, 🧑, blockiert DV1.1)
- Fällt die Fahrplan-Spalte `Prio` in `docs/sprints/README.md` weg? (B-361, 🧑; hängt an PJ3.3, das sie durch `Projekt` ersetzt)

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- DV1.1 (INF) Domäne `DEV` in Arbeitsweise, Glossar und Planungstest; Vorlage und Sprint-Dateien ohne `Prio`/`Einschiebbar` (AC-01, AC-02).
- DV1.2 (SRV) plan-Tools: Domänenliste mit `DEV`, Filter-Chip, kein `Prio`/`Einschiebbar` mehr; offene Werkzeug-Tickets auf `DEV` umstellen (AC-03, AC-04, AC-05).
- DV1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
