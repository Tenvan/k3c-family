# L1 · INF · Go-Verschachtelung als Tiefe prüfen

- **Status:** geplant
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-054
- **Start-Commit:** –
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
| L1.1 | `L1.1-go-verschachtelung.md` | Umsetzung | autonom | offen |
| L1.2 | `L1.2-review.md` | Review | autonom | offen |

## Abnahme

–
