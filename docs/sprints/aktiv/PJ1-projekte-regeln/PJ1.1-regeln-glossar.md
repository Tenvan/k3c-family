# PJ1.1 · Arbeitsweise und Glossar für Projekte, Rang und Domäne je Session

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** pj1/1-regeln-glossar
- **Abhängig von:** –
- **Tickets:** B-355, B-356
- **Kriterien:** AC-02, AC-06

## Ziel

`docs/arbeitsweise.md` und `docs/glossar.md` beschreiben die Ebenen Projekt → Sprint → Session, den Rang als einzige Reihenfolge der Arbeit und die Domäne je Session.

## Kontext

- Specs: `docs/backlog/B-355-projekte-mit-rang.md` und `docs/backlog/B-356-domaene-je-session.md` (Anforderungen, Beispiele, Fehlerfälle). Sie sind freigegeben; nichts darüber hinaus erfinden.
- Betroffene Stellen in `docs/arbeitsweise.md`: Kopfzeile (Ticket → Sprint → Session), „Ablage“ (neu `docs/projekte/`, `vorlagen/projekt.md`), „Autonomer Ablauf“ Schritt 1 (Auswahl über Rang statt Prio, Sperre je Domäne auf Session-Ebene), „Domänen“ (gilt je Session, Sprint darf mehrere Domänen nacheinander enthalten), „Sprint-Lebenslauf“ (Aktivieren: je Projekt ein aktiver Sprint statt je Domäne; Punkte **Prio** und `Einschiebbar` ersetzen; **Klein:** 3–6 Sessions), „Review-Session“ (Diff aller Domänen des Sprints).
- **Übergang**, ausdrücklich in den Text: Bis PJ3 (B-359) haben Sprints und Tickets noch kein Projekt; solange gilt für Sprints ohne Projekt die alte Regel „je Domäne ein aktiver Sprint“. Die Felder `Prio` und `Einschiebbar` stehen noch in den Sprint-Dateien, weil k3c-dev sie liest; sie haben keine Wirkung auf die Reihenfolge mehr und entfallen mit PJ2 (B-357).
- `docs/plan-weiterentwicklung.md` § 11 bleibt bis PJ3 stehen; die Arbeitsweise verweist nicht mehr darauf als Reihenfolge.
- Glossar: Die Einträge `Projekt` und `Rang` stehen schon (Zusatz „Geplant, gilt ab PJ1“ entfernen). Anpassen: `Domäne` (je Session), `Sprint` (3–6 Sessions, ein oder mehrere Domänen, gehört zu einem Projekt), `Session` (trägt eine Domäne; Status auch `verworfen`), `Prio` (ordnet Tickets innerhalb eines Projekts), `Einschiebbar` (entfällt mit PJ2, bis dahin ohne Wirkung). Alphabetische Sortierung beibehalten (`tests/planning.test.ts › Glossar`).
- k3c-dev-Tools für Planung gibt es für Projekte noch nicht; diese Session ändert nur Text.

## Erlaubte Dateien

- `docs/arbeitsweise.md`, `docs/glossar.md`
- `docs/sprints/geplant/PJ1-projekte-regeln/`, `docs/sprints/aktiv/PJ1-projekte-regeln/` (Status), `docs/sprints/README.md` (Aktivierung), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Vorlagen, `tests/planning.test.ts`, Felder in Planungsdateien (PJ1.2, PJ1.3); `docs/plan-weiterentwicklung.md` und `CLAUDE.md` (PJ3).

## Schritte

1. Sprint aktivieren: Ordner nach `docs/sprints/aktiv/`, `Status: aktiv`, Fahrplan anpassen, Branch `sprint/pj1` von `origin/develop`, `Start-Commit` setzen; `Status: in Arbeit`, committen, pushen.
2. `docs/arbeitsweise.md` an den genannten Stellen anpassen, kurz und ohne zweite Prozessbeschreibung; Übergang als eigener Absatz im Sprint-Lebenslauf.
3. `docs/glossar.md` angleichen.
4. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `docs(inf): Projekte, Rang und Domäne je Session in der Arbeitsweise (PJ1.1)`.

## Fertig, wenn

- [x] AC-02: `docs/arbeitsweise.md` nennt die Ebenen, den Rang, die Auswahl der nächsten Session über den Rang und „je Projekt ein aktiver Sprint“; Sprint-Prio und `Einschiebbar` haben keine Wirkung mehr (Übergang beschrieben).
- [x] AC-06: `docs/arbeitsweise.md` beschreibt Domäne je Session, Sperre je Domäne auf Session-Ebene, Sprint-Größe 3–6, Review über alle Domänen; Glossar `Domäne`, `Sprint`, `Session` angeglichen.
- [x] `task test -- planning` grün (Glossar sortiert, Verweise vorhanden).

## Prüfen

```bash
task test -- planning
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-02 umgesetzt:** `docs/arbeitsweise.md` hat die Kopfzeile Projekt → Sprint → Session, `projekte/` in der Ablage und im Lesen, den neuen Abschnitt „Projekte und Rang“ (Status, Rang als einzige Reihenfolge, Sprint-Tabelle, höchstens ein aktiver Sprint je Projekt, Ticket-Prio innerhalb des Projekts, ABN). Der autonome Ablauf (Schritt 1) wählt über den Rang. Im Sprint-Lebenslauf ersetzt „Reihenfolge“ die Sprint-Prio, `Einschiebbar` ist aus Aktivieren und Blockade entfernt. Neu ist der Absatz „Übergang Projekte (bis PJ3, B-359)“: Für Sprints mit `Projekt: –` gelten Prio, Fahrplan und „ein aktiver Sprint je Domäne“ weiter, die Felder `Prio` und `Einschiebbar` entfallen mit PJ2.
- **AC-06 umgesetzt:** „Domänen“ gilt je Session, ein Sprint darf mehrere Domänen nacheinander haben, Sperre je Domäne auf Session-Ebene (auch in Schritt 1). Sprint-Größe 3–6, die Review-Session liest den Diff aller Domänen und trägt die Domäne der letzten Umsetzung. Commit-Titel nehmen die Domäne der Session. Die Feature-Kette ist als Sessions eines Sprints erlaubt, `verworfen` ist beschrieben (B-338).
- **Glossar angeglichen:** `Domäne`, `Sprint`, `Session` (mit `verworfen`), `Prio`, `Einschiebbar` (ohne Wirkung, entfällt mit PJ2), `Projekt` und `Rang` (Zusatz „Geplant“ entfernt).
- **Geprüft:** `check_run task:test planning` ist grün.
- **Hinweis:** `projekte/README.md` wird in der Arbeitsweise schon verlinkt, die Datei entsteht erst in PJ1.2. Verweise auf `plan-weiterentwicklung.md` § 11.5/11.6 (Golden, Hardware) bleiben bis PJ3 stehen.
- **Abweichung:** Der Merge von #218 enthielt den Commit mit Freigabe und Session-Dateien nicht. Er wurde als erster Commit auf `sprint/pj1` übernommen (cherry-pick `f8b7717`), danach folgte die Aktivierung.
