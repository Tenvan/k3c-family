# WZ9.4 · Review WZ9

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz9/4-review
- **Abhängig von:** WZ9.3
- **Tickets:** B-462
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-07, AC-08, AC-09, AC-10

## Ziel

Sprint WZ9 ist abgenommen (alle Kriterien außer AC-06 mit Nachweis), der Diff der Domänen SRV und DEV ist auf schwere Befunde geprüft, und der eine PR des Sprints ist offen.

## Kontext

Ablauf laut `docs/arbeitsweise.md` › Review-Session: `task check` und `task check:go`, Diff `origin/develop...origin/sprint/wz9` lesen (SRV, DEV), nur schwere Befunde (falsches Verhalten, Datenverlust, Sicherheit bei Pfaden und Dateischreiben, Nicht-Determinismus, CLAUDE.md-Regeln, Kriterium ohne Nachweis), Begriffs-Treue prüfen und angleichen. Hardware entkoppelt: WZ9.5 (🧑) blockiert den PR nicht; AC-06 wird als `angenommen, Validierung offen (WZ9.5)` geführt, der Sprint bleibt `aktiv`. Das Review läuft nie im selben Lauf wie eine Umsetzung.

## Erlaubte Dateien

- Befund-Korrekturen in den Dateien der Sprint-Domänen (SRV, DEV)
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

- [ ] Alle Kriterien außer AC-06 haben einen Nachweis in den Session-Ergebnissen; AC-06 geführt als `angenommen, Validierung offen (WZ9.5)`.
- [ ] Abnahme in der Sprint-README, Versionsvorschlag eingetragen.
- [ ] PR des Sprints offen, Auto-Merge eingeschaltet.

## Prüfen

```bash
task check
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

## Ergebnis

Umgebung offline, autonom. Getrennte Reviewer je Umsetzungs-Session über den Sprint-Diff (SRV: WZ9.1; DEV: WZ9.2, WZ9.3); Befunde jeweils behoben, siehe die Ergebnisse der Sessions.
- AC-01 geprüft: Matrix Hub-Stufe × Bauplatz (WZ9.2, Test gegen die heutige `data/hub.json`).
- AC-02 geprüft: Speichern nur nach `sim.ValidateHubData`, ungültige Kandidaten schreiben nichts (WZ9.3).
- AC-03 geprüft: Abweichung über `tabelle.Gegenprobe` (WZ9.2), Schreiben des erzeugten Blocks, „Regeltabelle veraltet“ nur bei Fehler im zweiten Schritt (WZ9.3).
- AC-04 geprüft: Live-Vorschau zeigt den Kandidaten ungespeichert über `PreviewOptions.HubCandidate` (WZ9.1, WZ9.2).
- AC-05 geprüft: REG-Hinweis nach dem Speichern (WZ9.3).
- AC-07 geprüft: format-stabil (Ein-Zeilen-Diff), atomar, Schreibregel je Datei (WZ9.3).
- AC-08 geprüft: 7-Stufen-Fixture in Snapshot, Matrix und Generator, keine feste 5 (WZ9.1, WZ9.2).
- AC-09 teilweise geprüft: Kandidat im Snapshot (fail closed, Determinismus, Kandidat = eingebettet); der Vorrang „geplante Chunks“ fehlt, weil WZ8 nicht im Branch liegt — verschoben nach B-479.
- AC-10 geprüft: `git diff --stat origin/develop -- data/hub.json testdata/golden` leer.
- AC-06: angenommen, Validierung offen (WZ9.5).
- Prüfung: `task:check`, `task:check:go`, `go:lint` grün; Studio-Vitest 148 Tests, `tsc`, `vite build`, Studio- und assetkit-Go grün.
- Abweichungen: WZ9.1 enthält eine kleine SIM-Änderung (`sim.CreateWorldWith`, Folge des Reviews); `WriteAtomic` in assetkit exportiert; kein PR (Auftrag: nur Commit und Push); beim Zusammenführen mit WZ8 `CreateOptions` vereinen (B-479). Neue Tickets: keine zusätzlichen (B-479 ergänzt).
