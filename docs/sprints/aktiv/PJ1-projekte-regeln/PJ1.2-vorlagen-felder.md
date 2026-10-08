# PJ1.2 · Vorlagen mit Projekt, Domäne und verworfen, Felder in allen Planungsdateien

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** pj1/2-vorlagen-felder
- **Abhängig von:** PJ1.1
- **Tickets:** B-355, B-356, B-338
- **Kriterien:** AC-01, AC-03, AC-05, AC-08, AC-09

## Ziel

Die Vorlagen kennen Projekt, Domäne je Session und den Session-Status `verworfen`; alle bestehenden Tickets, Sprints und Sessions tragen die neuen Felder, und `docs/projekte/README.md` steht bereit.

## Kontext

- `tests/planning.test.ts › checkTemplate` verlangt, dass jede Datei **genau** die Felder und Überschriften ihrer Vorlage hat. Ein neues Vorlagen-Feld muss deshalb im selben Commit in alle Dateien dieser Art (Stand 2026-10-07: 314 Tickets in `docs/backlog/` und `docs/backlog/archiv/`, 136 Sprint-READMEs, 411 Sessions unter `docs/sprints/*/*/`).
- Neue Felder und ihre Position:
  - `docs/vorlagen/ticket.md`: `- **Projekt:** – (Kürzel, z. B. GRA)` direkt nach `Sprint`. Bestehende Tickets: `- **Projekt:** –`.
  - `docs/vorlagen/sprint.md`: `- **Projekt:** – (Kürzel)` direkt nach `Status`; Feld `Domäne` mit Hinweis „eine oder mehrere, Reihenfolge der Sessions“. Bestehende Sprints: `- **Projekt:** –`, Domäne unverändert. `Prio` und `Einschiebbar` bleiben bis PJ2 stehen (k3c-dev liest sie).
  - `docs/vorlagen/session.md`: `- **Domäne:** REG | SIM | SRV | CLI | PLAT | INF` direkt nach `Agent`; Status `offen | in Arbeit | fertig | blockiert | verworfen`. Bestehende Sessions: Domäne ihres Sprints (Feld `Domäne` der Sprint-README).
- Neue Vorlage `docs/vorlagen/projekt.md`: Überschrift `# XXX · Titel des Projekts`; Felder `Status` (`aktiv | ruht | erledigt`), `Rang` (Zahl, bei `ruht`/`erledigt` `–`), `Ziel-Tickets`; Abschnitte `Ziel`, `Sprints` (Tabelle `| Sprint | Thema | Status |` in Abarbeitungs-Reihenfolge), `Nicht-Ziele`, `Notizen`. Spec: B-355 › Anforderungen.
- Neue Übersicht `docs/projekte/README.md`: Kopf mit Verweis auf `../arbeitsweise.md`, Tabelle `| Rang | Projekt | Status | Ziel | Datei |`, vorerst ohne Zeilen („noch keine Projekte, Umzug in PJ3“).
- Die Felder setzt ein einmaliges Skript (Node, im Scratchpad, nicht einchecken): Zeile nach dem Bezugsfeld einfügen, CRLF/LF der Datei beibehalten, nichts sonst ändern. Danach `git diff --stat` prüfen: je Datei genau eine Zeile mehr.
- `tests/planning.test.ts`: Wertelisten in `ALLOWED` ergänzen (Session-`Status` mit `verworfen`, Session-`Domäne`), Sprint-`Domäne` als Liste erlauben (jedes Element in `DOMAINS`), Überschrift-Prüfung `# ID · Domäne · ` auf die Liste anpassen. Weitere Regeln folgen in PJ1.3.
- k3c-dev liest Feldnamen aus den Vorlagen (`tools/k3c-dev/internal/planning/edit.go › templateFields`), legt aber neue Dateien ohne Werte für neue Felder an; bis PJ2 bekommen neue Planungsdateien die Felder notfalls von Hand.

## Erlaubte Dateien

- `docs/vorlagen/`, `docs/projekte/README.md`, `tests/planning.test.ts`
- Feld-Zeilen in `docs/backlog/**/*.md` und `docs/sprints/**/*.md` (nur die neuen Felder)
- `docs/sprints/aktiv/PJ1-projekte-regeln/` (Status), `docs/backlog/README.md` (neue Tickets)

## Nicht-Ziele

Neue Prüfregeln für Rang, Projekt-Zuordnung und Domänen-Sperre (PJ1.3); Projekte anlegen oder zuordnen (PJ3); `Prio` und `Einschiebbar` entfernen (PJ2); k3c-dev ändern (PJ2).

## Schritte

1. Branch `sprint/pj1` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. Vorlagen `ticket.md`, `sprint.md`, `session.md` ändern, `projekt.md` und `docs/projekte/README.md` anlegen.
3. Skript laufen lassen, `git diff --stat` und Stichproben prüfen (je eine Datei aus `backlog/`, `backlog/archiv/`, `sprints/erledigt/`).
4. `tests/planning.test.ts` für die neuen Werte anpassen.
5. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `docs(inf): Vorlagen mit Projekt und Domäne je Session (PJ1.2)`.

## Fertig, wenn

- [x] AC-01: `docs/vorlagen/projekt.md` und `docs/projekte/README.md` existieren (Glossar-Teil in PJ1.1).
- [x] AC-03: Vorlagen Sprint und Ticket haben `Projekt`, alle Sprints und Tickets tragen es.
- [x] AC-05: Vorlage Session hat `Domäne`, Vorlage Sprint nennt `Domäne` als Liste.
- [x] AC-08: Alle Session-Dateien tragen `Domäne` = Domäne ihres Sprints.
- [x] AC-09: Vorlage und Test erlauben `verworfen` für Sessions.
- [x] `task test -- planning` und `task check` grün.

## Prüfen

```bash
task test -- planning
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 umgesetzt:** `docs/vorlagen/projekt.md` (Felder `Status`, `Rang`, `Ziel-Tickets`; Abschnitte Ziel, Sprints, Nicht-Ziele, Notizen) und `docs/projekte/README.md` (Abschnitte Aktiv, Ruht, Erledigt, noch ohne Projekte) angelegt. Den Glossar-Teil hat PJ1.1 erledigt.
- **AC-03 umgesetzt:** Die Vorlagen für Ticket (nach `Sprint`) und Sprint (nach `Status`) haben das Feld `Projekt: – (oder Kürzel …)`. Alle 314 Tickets und 136 Sprint-READMEs tragen `- **Projekt:** –`.
- **AC-05 umgesetzt:** Die Vorlage Session hat `Domäne` (nach `Agent`, genau eine). In der Vorlage Sprint heißt es bei `Domäne` „eine oder mehrere, kommagetrennt“. Bei `Prio` und `Einschiebbar` steht der Übergangshinweis (gilt nur ohne Projekt, entfällt mit PJ2).
- **AC-08 umgesetzt:** Alle 415 Session-Dateien tragen `- **Domäne:**` mit der Domäne ihres Sprints. Geprüft mit `git diff --numstat`: Jede Planungsdatei hat genau +1 Zeile, nur die Vorlagen weichen ab. Stichproben aus `backlog/archiv/` (B-001), `sprints/erledigt/` (K1.1) und `sprints/aktiv/` (W6) sind in Ordnung.
- **AC-09 umgesetzt:** Die Vorlage Session erlaubt `verworfen`, ebenso `tests/planning.test.ts` (`ALLOWED.session.Status`).
- **Test angepasst:** Session-`Domäne` steht in `ALLOWED`. Die Sprint-`Domäne` darf eine Liste sein, jedes Element aus `DOMAINS` und ohne Dopplung.
- **Geprüft:** `task test -- planning` ist grün, `task check` ist grün (1543 Tests). Beides lief in der Shell, weil k3c-dev nicht erreichbar war (ECONNREFUSED).
- **Werkzeug:** Die Felder setzte ein einmaliges Node-Skript im Scratchpad, das nicht eingecheckt ist. Es fügt die Zeile nach dem Bezugsfeld ein und behält das Zeilenende der Datei. Der erste Lauf brach nach den Tickets an einem Regex-Fehler ab, der zweite Lauf war idempotent und hat Sprints und Sessions ergänzt.
- **Hinweis für PJ2:** `plan_create` übernimmt Felder ohne Vorgabe als Vorlagentext. Eine neue Session bekäme `Domäne: REG | SIM | …` und wäre damit ungültig. Bis PJ2 deshalb `Domäne` beim Anlegen immer mitgeben.
