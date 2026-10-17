# WZ9.7 · Review WZ9

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz9/7-review
- **Abhängig von:** WZ9.6
- **Tickets:** B-462
- **Kriterien:** alle außer AC-06

## Ziel

Sprint WZ9 ist abgenommen (alle Kriterien außer AC-06 mit Nachweis), der Diff aller fünf Domänen ist auf schwere Befunde geprüft, und der eine PR des Sprints ist offen.

## Kontext

Ablauf laut `docs/arbeitsweise.md` › Review-Session: `task check` und `task check:go`, Diff `origin/develop...origin/sprint/wz9` lesen (REG, INF, SIM, SRV, DEV), nur schwere Befunde (falsches Verhalten, Datenverlust, Sicherheit bei Pfaden und Dateischreiben, Nicht-Determinismus, CLAUDE.md-Regeln, Kriterium ohne Nachweis), Begriffs-Treue prüfen und angleichen. Hardware entkoppelt: WZ9.8 (🧑) blockiert den PR nicht; AC-06 wird als `angenommen, Validierung offen (WZ9.8)` geführt, der Sprint bleibt `aktiv`. Das Review läuft nie im selben Lauf wie eine Umsetzung.

## Erlaubte Dateien

- Befund-Korrekturen in den Dateien der Sprint-Domänen (REG, INF, SIM, SRV, DEV)
- `docs/sprints/` und `docs/backlog/` (Abnahme, Fahrplan, Tickets)

## Nicht-Ziele

Kein Stil-Review, keine Vereinfachungen als Befund, keine Änderung an `data/hub.json` oder `testdata/golden/`, kein Setzen eines Release-Tags ohne Bestätigung von 🧑.

## Schritte

1. `task check` und `task check:go` grün.
2. `git fetch` und Diff `origin/develop...origin/sprint/wz9` lesen.
3. Schwere Befunde in den Sprint-Domänen beheben, andere als Ticket.
4. AC-10 prüfen: `git diff --stat origin/develop -- data/hub.json testdata/golden` leer.
5. Abnahme in der Sprint-README (höchstens fünf Zeilen) und Versionsvorschlag.
6. `git merge origin/develop`, pushen, PR des Sprints öffnen (eine Zeile je Session), Auto-Merge: `gh pr merge --auto --squash`.

## Fertig, wenn

- [ ] Alle Kriterien außer AC-06 haben einen Nachweis in den Session-Ergebnissen; AC-06 geführt als `angenommen, Validierung offen (WZ9.8)`.
- [ ] Abnahme in der Sprint-README, Versionsvorschlag eingetragen.
- [ ] PR des Sprints offen, Auto-Merge eingeschaltet.

## Prüfen

```bash
task check
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
