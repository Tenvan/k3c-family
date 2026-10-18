# RP1.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** rp1/3-review
- **Abhängig von:** RP1.1, RP1.2
- **Tickets:** B-302, B-288, B-094, B-058
- **Kriterien:** alle

## Ziel

Ein getrennter Reviewer hat Diff und Kriterien von RP1 geprüft; Befunde sind behoben oder als Ticket festgehalten, der Sprint ist abgeschlossen und der PR geöffnet.

## Kontext

Review-Session nach `docs/arbeitsweise.md` (getrennt vom Schreiben): Diff `origin/develop...sprint/rp1` lesen, AC-01 bis AC-04 an den Ergebnissen von RP1.1 und RP1.2 halten. Besonderes Augenmerk: `tools/aufraeumen` darf nur sichere Fälle löschen (Schutzliste, `--dry`, Worktree mit Änderungen). Version vorschlagen (Patch, reine Werkzeug- und Doku-Änderung).

## Erlaubte Dateien

- Befunde: nur kleine Korrekturen in den Dateien von RP1.1 und RP1.2; größere Befunde als Ticket
- Sprint-Dateien von RP1 (Abnahme, Status, Ordner nach `erledigt/`), `docs/sprints/README.md`, `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Neue Funktionen, Ändern von Kriterien, Löschen von Branches außerhalb von `task aufraeumen`.

## Schritte

1. `task check` und `task check:go` ausführen.
2. Diff `origin/develop...sprint/rp1` lesen, Kriterien AC-01 bis AC-04 gegen die Ergebnisse halten.
3. Befunde beheben oder als Ticket anlegen.
4. Abnahme der Sprint-README füllen (höchstens fünf Zeilen, Versionsvorschlag), Tickets B-302, B-288, B-094 und B-058 `erledigt` setzen, Sprint abschließen, PR öffnen und Auto-Merge einschalten.

## Fertig, wenn

- [ ] AC-01 bis AC-04 sind am Diff belegt (Nachweis im Ergebnis).
- [ ] Befunde sind behoben oder als Ticket festgehalten; `task check` und `task check:go` grün.
- [ ] Abnahme gefüllt, Tickets und Sprint `erledigt`, PR geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
