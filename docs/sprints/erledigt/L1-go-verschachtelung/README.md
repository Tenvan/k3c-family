# L1 · INF · Go-Verschachtelung als Tiefe prüfen

- **Status:** erledigt
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-054
- **Start-Commit:** 20c5530
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 1, nestif ersetzen)

## Ausgangslage

Seit SP01 prüft `.golangci.yml` die Verschachtelung mit `nestif` (`min-complexity: 4`). Das Review SP01.4 hat gezeigt,
dass `nestif` eine Punktzahl nur für `if` misst, keine Tiefe: vier verschachtelte `if` scheitern schon, Schleifen,
`switch` und `select` zählen gar nicht (B-054). Für TypeScript prüft Oxlint `max-depth` 4 korrekt.

## Ziel

Die Verschachtelung von Go-Code wird als Tiefe geprüft, mit derselben Grenze wie bei TypeScript, bevor in SP03 der
erste echte Go-Code entsteht. Am Ende sichtbar: `npm run check:go` scheitert bei Tiefe 5 und bleibt bei Tiefe 4 grün.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die ab SP03 Go-Code schreiben; 🧑 entscheidet die Offene Frage; die Review-Session.

## Anforderungen

B-054 › Anforderungen.

## Nicht-Ziele

Andere Budget-Grenzen ändern; die Ratsche nachziehen (B-056); Oxlint oder TypeScript-Regeln anfassen.

## Regeln und Einschränkungen

Einschiebbar, soll vor SP03 laufen. Nur Linter, die golangci-lint 2.14 mitbringt (keine neue Abhängigkeit).
Grenzen nur aus `docs/arbeitsweise.md` › Komplexitäts-Budget.

## Beispiele

B-054 › Beispiele: `for` → `if` → `if` → `if` → `if` (Tiefe 5) scheitert, vier verschachtelte `if` (Tiefe 4) bleiben grün.

## Ausnahme- und Fehlerfälle

B-054 › Ausnahme- und Fehlerfälle. Aktiviert `revive` mehr als die eine Regel → nur `max-control-nesting` einschalten,
`golangci-lint run` muss auf dem Bestand 0 Befunde behalten.

## Akzeptanzkriterien

- **AC-01** Go-Code mit Verschachtelungstiefe 5 lässt `npm run check:go` scheitern, Tiefe 4 nicht, für `if`, `for` und `switch` (B-054/AC-01).
- **AC-02** `docs/arbeitsweise.md` › Werkzeuge und der Kommentar in `.golangci.yml` nennen den tatsächlich genutzten Linter (B-054/AC-02).

## Offene Fragen

keine. Entschieden (🧑, 2026-09-30): `nestif` wird durch `revive` › `max-control-nesting` 4 ersetzt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| L1.1 | `L1.1-go-verschachtelung.md` | Umsetzung | autonom | fertig |
| L1.2 | `L1.2-review.md` | Review | autonom | fertig |

## Abnahme

**2026-09-30, L1.2 (autonomer Agent, nur lokal).** Geprüft: 8 Dateien aus `git diff --stat 20c5530..main`
(lokales `main` 9aee6a5; kein `git fetch`, 🧑: nur lokal), jede vollständig gelesen: `.golangci.yml`,
`docs/arbeitsweise.md`, `docs/backlog/B-054-go-verschachtelung.md`, `docs/sprints/README.md`,
`docs/sprints/aktiv/.gitkeep` und die drei Dateien dieses Sprints. Lokal: Go 1.27.0, golangci-lint 2.14.0.

- **AC-01 geprüft, mit Lücke (Teil verschoben nach B-057):** Gegenproben in `engine/probe/` mit
  `--max-same-issues=0` (Standard zeigt nur 3 gleiche Meldungen), danach gelöscht. Scheitern (Tiefe 5): `for` → 4× `if`,
  5× `if`, `switch` → `for` → 3× `if`, `switch` → 4× `if`, Typ-`switch` → 4× `if`, `for` → Typ-`switch` → `for` → 2× `if`,
  `for` → `select` → 3× `if`. Grün (Tiefe 4): 4× `if`, `for` → 3× `if`, `switch` → `for` → 2× `if`, `switch` → 3× `if`;
  `else` und Funktionsliterale zählen wie bei Oxlint nicht bzw. setzen zurück. `revive` meldet nur `max-control-nesting`
  (exportierte Funktion ohne Doc-Kommentar mit unbenutztem Parameter, `if … return … else`-Muster für `indent-error-flow`: keine
  revive-Meldung; `staticcheck`/`unused` kommen aus dem Standard-Satz von golangci-lint, nicht aus revive).
  **Lücke:** `for range` zählt keine Ebene (`range` → 4× `if` grün, erst 5× `if` rot); jedes `else if` zählt eine
  Ebene (flache Kette mit 5 Zweigen rot), L1.1 hatte nur „eine Ebene“ festgehalten. Beides widerspricht „Tiefe 5
  scheitert, Tiefe 4 nicht“ für `for` bzw. `if`; ohne neue Regel nicht behebbar (Nicht-Ziel) → B-057.
- **AC-02 geprüft:** `docs/arbeitsweise.md` › Werkzeuge nennt `revive` › `max-control-nesting`, der Kopfkommentar in
  `.golangci.yml` ebenso. Die Ergänzung „Dateilänge beider Sprachen“ steht in derselben Werkzeug-Zeile (von L1.1 erlaubt)
  und stimmt: `tests/projectRules.test.ts` prüft die Ratsche für beide Sprachen und die harte Grenze für Go.
- **Weiter geprüft:** `golangci-lint config verify`, `npm run check` (198 Tests) und `npm run check:go` grün; Grenze 4
  passt zum Budget. `docs/sprints/aktiv/.gitkeep` ist leer, `tests/planning.test.ts` liest nur Unterordner und bleibt
  grün. Fahrplan hatte L1 korrekt unter „Aktiv“.
- **Behoben:** Kopfkommentar `.golangci.yml` („wie Oxlint max-depth“ ohne Abweichungen); B-054 › Notizen und
  L1.1 › Ergebnis zur `else if`-Zählung korrigiert und um `for range` ergänzt.
- **Neue Tickets:** B-057 (`for range` und `else if` zählen anders als in TypeScript, Weg entscheidet 🧑).
