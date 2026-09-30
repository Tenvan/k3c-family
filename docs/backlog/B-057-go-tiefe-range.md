# B-057 · Die Go-Verschachtelung zählt `for range` und `else if` wie TypeScript

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit L1 prüft `.golangci.yml` die Go-Verschachtelung mit `revive` › `max-control-nesting` 4 (B-054). Gegenproben im
Review L1.2 (golangci-lint 2.14) zeigen zwei Abweichungen vom Budget in `docs/arbeitsweise.md` und von Oxlint
`max-depth`: `for range` zählt keine Ebene (`range` → 4× `if`, Tiefe 5, bleibt grün), und jedes `else if` zählt eine
Ebene (flache Kette mit 5 Zweigen, Tiefe 1, scheitert). Fünf direkt verschachtelte `select` melden ebenfalls nichts.
Go ist damit bei der häufigsten Schleife lockerer und bei `else if`-Ketten strenger als das Budget.

## Ziel

Die harte Grenze Verschachtelung 4 bedeutet in Go dasselbe wie in TypeScript, auch für `for range` und `else if`.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die ab SP03 Go-Code schreiben; 🧑 entscheidet den Weg (Offene Fragen); Review-Session.

## Anforderungen

- Go-Code mit Tiefe 5 über `for range` lässt `npm run check:go` scheitern, Tiefe 4 nicht.
- Eine flache `else if`-Kette zählt als eine Ebene, wie bei Oxlint `max-depth`.

## Nicht-Ziele

Andere Budget-Grenzen ändern; Oxlint oder TypeScript-Regeln anfassen.

## Regeln und Einschränkungen

Keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget; Grenzen nur aus
`docs/arbeitsweise.md` › Komplexitäts-Budget. Domäne INF (Lint-Konfiguration, `tests/projectRules.test.ts`).

## Beispiele

- `for _, v := range xs` → `if` → `if` → `if` → `if` (Tiefe 5) → `npm run check:go` scheitert (heute grün).
- `if` / `else if` / `else if` / `else if` / `else if` ohne innere Struktur (Tiefe 1) → grün (heute rot).

## Ausnahme- und Fehlerfälle

Kein Linter aus golangci-lint zählt wie Oxlint → Abweichung bewusst annehmen und in `docs/arbeitsweise.md` nennen,
oder eigene Prüfung (Offene Fragen).

## Akzeptanzkriterien

- **AC-01** Eine Go-Probe `range` → 4× `if` lässt `npm run check:go` scheitern, `range` → 3× `if` nicht (Proben zurücknehmen).
- **AC-02** Eine Go-Probe mit flacher `else if`-Kette aus 6 Zweigen bleibt grün.

## Offene Fragen

Weg (🧑): (a) Abweichung annehmen und dokumentieren, (b) Fehler an revive melden und auf eine neue Version warten,
(c) eigene Tiefen-Prüfung für Go (z. B. kleiner `go/ast`-Test), `revive` dann entfernen.

## Notizen

Proben und Zählweise: B-054 › Notizen (Review L1.2). Ursache vermutet, ungeprüft: `max-control-nesting` wertet
`*ast.RangeStmt` nicht aus und läuft über `IfStmt.Else` mit erhöhter Ebene.
