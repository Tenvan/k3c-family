# PJ1.3 · Planungstest für Projekte, Rang und Domänen-Sperre

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** pj1/3-planungstest-regeln
- **Abhängig von:** PJ1.2
- **Tickets:** B-355, B-356
- **Kriterien:** AC-04, AC-07

## Ziel

`tests/planning.test.ts` prüft die Regeln von B-355 und B-356 mit je einem Negativtest, und die Prüfung „je Domäne ein aktiver Sprint“ gilt nur noch für Sprints ohne Projekt.

## Kontext

- Stand nach PJ1.2: Vorlagen und alle Dateien tragen `Projekt` bzw. `Domäne`; `docs/projekte/README.md` ist leer; `docs/vorlagen/projekt.md` existiert.
- Muster im Test: reine Hilfsfunktionen (`crowdedDomains`, `waitsForDevice`) mit eigenen `it`-Fällen für die Regel und ein Lauf über die echten Dateien. Neue Regeln genauso: Funktion + Negativtest mit kleinen Beispieldaten + Prüfung der echten Daten.
- Zu prüfende Regeln (B-355 › Ausnahme- und Fehlerfälle, B-356 › Ausnahme- und Fehlerfälle):
  1. Jede Datei unter `docs/projekte/` außer `README.md` folgt `vorlagen/projekt.md` (Felder, Überschriften, Werte), Name `<KÜRZEL>-…md` mit Kürzel aus drei Großbuchstaben = Überschrift.
  2. Ränge der aktiven Projekte sind eindeutig und lückenlos ab 1; `ruht`/`erledigt` haben `Rang: –`.
  3. Sprint mit `Projekt` ≠ `–` → Projekt existiert und nennt den Sprint in seiner Sprint-Tabelle; jeder Sprint der Tabelle existiert und nennt das Projekt.
  4. Je Projekt höchstens ein aktiver Sprint, der nicht nur auf Mensch-Sessions wartet (`waitsForDevice`).
  5. Ticket mit `Projekt` ≠ `–` → Projekt existiert.
  6. Sprint-Feld `Domäne` = Domänen seiner Sessions in Reihenfolge des ersten Auftretens (nur bei `Reife: bereit`).
  7. Höchstens eine Session je Domäne mit `Status: in Arbeit` über alle Sprints des Checkouts.
  8. Übergang: `crowdedDomains` nur für aktive Sprints mit `Projekt: –`.
- Projekt-Übersicht `docs/projekte/README.md` nennt jedes Projekt (wie „Fahrplan nennt jeden Sprint-Ordner“).
- Datei-Budget: `tests/planning.test.ts` hat 203 Zeilen; wird es mehr als 400, die Projekt-Regeln nach `tests/planningProjects.test.ts` auslagern und die Hilfsfunktionen gemeinsam nutzen (keine Kopien).

## Erlaubte Dateien

- `tests/planning.test.ts`, `tests/planningProjects.test.ts` (neu, falls nötig)
- `docs/sprints/aktiv/PJ1-projekte-regeln/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Projekte anlegen (PJ3), strenge Pflicht „jeder Sprint hat ein Projekt“ (PJ3, B-359/AC-06), Änderungen an k3c-dev (PJ2).

## Schritte

1. Branch `sprint/pj1` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. Je Regel: Negativtest mit Beispieldaten (rot ohne Prüfung), dann Prüfung, dann Lauf über die echten Dateien.
3. Mit einer Projekt-Datei im Scratchpad-Ordner von Hand gegenprüfen, dass die Prüfung der echten Dateien greift (nicht einchecken).
4. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `test(inf): Planungstest für Projekte und Domänen (PJ1.3)`.

## Fertig, wenn

- [ ] AC-04: Regeln 1–5 und die Übersicht geprüft, je Regel ein Negativtest.
- [ ] AC-07: Regeln 6–8 geprüft, je Regel ein Negativtest.
- [ ] `task test -- planning` und `task check` grün.

## Prüfen

```bash
task test -- planning
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
