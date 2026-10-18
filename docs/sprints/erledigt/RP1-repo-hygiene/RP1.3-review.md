# RP1.3 · Review

- **Status:** fertig
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

- [x] AC-01 bis AC-04 sind am Diff belegt (Nachweis im Ergebnis).
- [x] Befunde sind behoben oder als Ticket festgehalten; `task check:go` grün, `task check` nur durch den bestehenden Oxlint-Fehler `src/tools/dm.ts:92` rot (auch auf `develop`).
- [x] Abnahme gefüllt, Tickets und Sprint `erledigt`, PR geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **Review (getrennter Reviewer, 2026-10-18):** keine schweren Befunde. AC-01 (B-302/AC-01 bis AC-05) an `docs/arbeitsweise.md`, `tools/aufraeumen` samt Tests, `Taskfile.yml` und PR-Vorlage belegt; AC-02 am Beschluss in Spielstand-Abschnitt und Release-Zeile; AC-03 im Stand erfüllt (Binärdateien seit 2026-10-18 ausgetragen, `git grep "npm run"` ohne Treffer, `task check:dev` grün); AC-04 vorläufig (Antwort von 🧑 auf B-058 steht aus).
- **Behoben:** Fixture-Satz im Spielstand-Abschnitt klargestellt (Fixture der neuen Version aus dem Code erzeugen).
- **Restrisiken, so in B-302 vorgesehen:** PR-Status nur über den Branchnamen; ein frischer Branch ohne Commit gilt als gemergt (sein sauberer Worktree würde mit entfernt).
- B-302, B-288, B-094 und B-058 `erledigt`; neue Tickets: keine.
