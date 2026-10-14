# WZ4.3 · Review WZ4

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz4/3-review
- **Abhängig von:** WZ4.2
- **Tickets:** B-406
- **Kriterien:** AC-01

## Ziel

Sprint WZ4 ist abgenommen: AC-01 mit Nachweis, `task check` grün, ein PR gegen `develop` mit Auto-Merge.

## Kontext

Review-Ablauf: `docs/arbeitsweise.md` › Review-Session. Geändert hat der Sprint nur die DOM-Testdatei `tools/k3c-dev/frontend/src/planning/SplitterDom.test.tsx`, die Root-`package.json` samt Lockfile (`happy-dom`) und die Planungsdateien zu B-406.

## Erlaubte Dateien

- `docs/sprints/geplant/WZ4-lint-splitter-test/` bzw. nach Verschieben `docs/sprints/erledigt/WZ4-lint-splitter-test/` (Abnahme, Status)
- `docs/sprints/README.md`, `docs/projekte/WZG-*.md` (Fahrplan, Sprint-Tabelle)
- Behebung schwerer Befunde in den Dateien des Sprints (DEV)

## Nicht-Ziele

Keine neuen Funktionen, keine Stil- oder Benennungsbefunde, keine Änderung am Splitter. Der Tag wird nicht gesetzt; nur Versionsvorschlag (Patch, reine Testschuld) in der Abnahme.

## Schritte

1. `task check` und `task check:dev` ausführen (über `check_run`), beide grün.
2. `git fetch` und `git diff origin/develop...origin/sprint/wz4` lesen; prüfen, dass nur `happy-dom` als Abhängigkeit dazukam und kein anderer Test die Umgebung wechselt.
3. Prüfen, dass AC-01 im Ergebnis von WZ4.2 mit Nachweis steht und der Test das Verhalten (Doppelklick, Ziehen, Klemmung, Loslassen) wirklich auslöst, nicht nur Markup.
4. Abnahme in der Sprint-README (≤ 5 Zeilen, inklusive `Version: v… vorgeschlagen (Patch)`).
5. Sprint-Ordner nach `erledigt/`, `Status: erledigt`, Fahrplan anpassen, committen, `git merge origin/develop`, pushen, PR öffnen, `gh pr merge --auto --squash`.

## Fertig, wenn

- [ ] AC-01: Nachweis aus WZ4.2 geprüft, Test löst echte Ereignisse aus; `task check` grün.
- [ ] Abnahme steht in der Sprint-README, Sprint ist `erledigt`, ein PR mit Auto-Merge ist offen.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: `task check` und `dev:test` grün; Diff gegen `origin/develop` gelesen (nur `happy-dom` neu, Umgebung per Datei-Pragma, Test löst echte Ereignisse für Doppelklick, Ziehen, Klemmung, Loslassen aus). Keine schweren Befunde, keine neuen Tickets. Version v0.16.1 vorgeschlagen (Patch), kein Tag.
