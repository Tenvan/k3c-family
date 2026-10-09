# B-355 · Projekte bündeln Sprints zu Themen und werden nach Rang abgearbeitet

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** PJ1
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Die Planung kennt nur Sprint (2–4 Sessions, eine Domäne) und Session. Stand 2026-10-07: 11 aktive und 41 geplante Sprints; 7 der aktiven warten nur noch auf eine Abnahme am Gerät, 18 der geplanten tragen Prio `hoch`, weil ein Sprint die höchste Prio seiner Tickets erbt (`docs/arbeitsweise.md` › Sprint-Lebenslauf). Themen wie Grafik, Sound oder Balancing stehen nur als „Schienen“ in `docs/plan-weiterentwicklung.md` § 3 und § 11 und über das Feld `Einschiebbar`; die Planung selbst kennt sie nicht. Ein Thema lässt sich so kaum am Stück abarbeiten.

## Ziel

Ein **Projekt** (Thema) hält seine Sprints in fester Reihenfolge zusammen, 🧑 legt eine Rangfolge der Projekte fest, und die nächste Session ergibt sich daraus eindeutig: Projekt → Sprint → Session.

## Beteiligte und Zielgruppen

🧑 setzt Rang und Status der Projekte. Agenten wählen im autonomen Ablauf die nächste Session. Werkzeug: `plan_*`-Tools (B-357) und Planungsseite der Workbench (B-358).

## Anforderungen

- Drei Ebenen: **Projekt → Sprint → Session**. Der Sprint behält Branch `sprint/<präfix>`, einen PR und die Review-Session.
- Ein Projekt ist eine Datei `docs/projekte/<KÜRZEL>-name.md` nach neuer Vorlage `docs/vorlagen/projekt.md`. Kürzel: drei Großbuchstaben (z. B. `GRA`).
  - Felder: `Status` (`aktiv` | `ruht` | `erledigt`), `Rang` (ganze Zahl; nur bei `aktiv`, sonst `–`), `Ziel-Tickets` (Tickets, die das Thema als Ganzes beschreiben, z. B. B-011).
  - Abschnitte: Ziel · Sprints (Tabelle in Abarbeitungs-Reihenfolge: Sprint, Thema, Status) · Nicht-Ziele · Notizen.
- Übersicht `docs/projekte/README.md`: alle Projekte nach Rang, dann ruhende, dann erledigte.
- Sprint und Ticket bekommen das Feld `Projekt` (Kürzel). Ein Ticket ohne Sprint gehört damit trotzdem zu einem Thema.
- **Reihenfolge:** Der autonome Ablauf nimmt das aktive Projekt mit dem kleinsten Rang, darin den ersten nicht erledigten Sprint der Tabelle und darin die erste offene Session (`Agent: autonom`, Abhängigkeiten erledigt). Gibt es dort keine, kommt das nächste Projekt. Je Projekt ist höchstens ein Sprint aktiv; ein Sprint, in dem nur noch Sessions mit `Agent: Mensch` offen sind, zählt nicht.
- Die Sprint-Prio (höchste Ticket-Prio) entfällt, ebenso das Feld `Einschiebbar`. Die Ticket-Prio bleibt und ordnet Tickets innerhalb eines Projekts.
- Abnahmen am Gerät sammelt ein Projekt `ABN` ohne Rang (ersetzt „Offen am Gerät“ im Fahrplan).
- Glossar: `Projekt`, `Rang` neu; `Sprint`, `Prio`, `Einschiebbar` angepasst bzw. entfernt.
- `tests/planning.test.ts` prüft die neuen Regeln.

## Nicht-Ziele

Domäne je Session (B-356), `plan_*`-Tools (B-357), Planungsseite der Workbench (B-358), Umzug der bestehenden Sprints und Tickets (B-359). Termine oder Zeitschätzungen je Projekt.

## Regeln und Einschränkungen

- Domäne INF: `docs/arbeitsweise.md`, `docs/vorlagen/`, `docs/glossar.md`, `tests/planning.test.ts`, `docs/projekte/`.
- Prozess steht weiter nur in `docs/arbeitsweise.md`; `docs/projekte/README.md` ist wie `docs/sprints/README.md` eine Übersicht, keine Prozessbeschreibung.
- **Übergang:** Bis B-359 umgesetzt ist, darf ein Sprint oder Ticket ohne Feld `Projekt` sein; die strenge Prüfung (jeder geplante und aktive Sprint hat ein Projekt) schaltet B-359 ein.
- Bis B-357 umgesetzt ist, entstehen Projekt-Dateien von Hand nach Vorlage (Rückfall laut `CLAUDE.md`).
- Begriffe entschieden von 🧑 am 2026-10-07 im Chat: Projekt → Sprint → Session, Rangfolge LST, GRA, SND, BED, WRT, SKL, KMP, WZ, REL; BAL ruht.

## Beispiele

- `LST` hat Rang 1, sein aktiver Sprint PF1 hat eine offene autonome Session → der Agent nimmt sie.
- In `LST` sind nur Sessions mit `Agent: Mensch` offen → der Agent geht zu `GRA` (Rang 2).
- `BAL` hat `Status: ruht` → wird nie gewählt, seine Sprints bleiben in `geplant/`.
- Ticket B-331 ohne Sprint trägt `Projekt: GRA` → erscheint in der Projekt-Übersicht von GRA.

## Ausnahme- und Fehlerfälle

- Zwei aktive Projekte mit demselben Rang oder eine Lücke in den Rängen → `tests/planning.test.ts` rot.
- Sprint nennt ein Projekt, das es nicht gibt, oder fehlt in dessen Sprint-Tabelle (oder umgekehrt) → rot.
- Zwei aktive Sprints im selben Projekt (beide mit offener autonomer Session) → rot.
- Ruhendes oder erledigtes Projekt mit Rang → rot.
- Aktives Projekt ohne offenen Sprint → erlaubt (wartet auf Planung), der Agent überspringt es.

## Akzeptanzkriterien

- **AC-01** `docs/vorlagen/projekt.md` und `docs/projekte/README.md` existieren; `docs/glossar.md` erklärt `Projekt` und `Rang`.
- **AC-02** `docs/arbeitsweise.md` beschreibt Projekt-Ebene, Rang, Auswahl der nächsten Session über den Rang und „je Projekt ein aktiver Sprint“; Sprint-Prio und `Einschiebbar` sind entfernt.
- **AC-03** Die Vorlagen für Sprint und Ticket haben das Feld `Projekt`.
- **AC-04** `tests/planning.test.ts` prüft Projekt-Vorlage, eindeutige lückenlose Ränge, Sprint ↔ Sprint-Tabelle des Projekts und höchstens einen aktiven Sprint je Projekt; je Regel ein Negativtest, `task test -- planning` grün.

## Offene Fragen

keine (Begriffe und Rangfolge von 🧑 am 2026-10-07 entschieden)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
