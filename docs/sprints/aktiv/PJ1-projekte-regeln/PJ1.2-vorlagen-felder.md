# PJ1.2 · Vorlagen mit Projekt, Domäne und verworfen, Felder in allen Planungsdateien

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
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

- [ ] AC-01: `docs/vorlagen/projekt.md` und `docs/projekte/README.md` existieren (Glossar-Teil in PJ1.1).
- [ ] AC-03: Vorlagen Sprint und Ticket haben `Projekt`, alle Sprints und Tickets tragen es.
- [ ] AC-05: Vorlage Session hat `Domäne`, Vorlage Sprint nennt `Domäne` als Liste.
- [ ] AC-08: Alle Session-Dateien tragen `Domäne` = Domäne ihres Sprints.
- [ ] AC-09: Vorlage und Test erlauben `verworfen` für Sessions.
- [ ] `task test -- planning` und `task check` grün.

## Prüfen

```bash
task test -- planning
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
