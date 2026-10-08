# B-054 · Die Verschachtelung von Go-Code wird als Tiefe geprüft

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** L1
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit L1)

## Ausgangslage

`docs/arbeitsweise.md` › Komplexitäts-Budget setzt die Verschachtelung auf Ziel 3, harte Grenze 4. Für TypeScript
prüft Oxlint `max-depth` 4 (Tiefe 4 erlaubt, 5 scheitert). Für Go nutzt `.golangci.yml` (SP01.2) `nestif` mit
`min-complexity: 4`. `nestif` misst aber keine Tiefe, sondern eine Punktzahl nur für `if`: Probe im Review SP01.4 mit
golangci-lint 2.14 → vier verschachtelte `if` ergeben Punktzahl 6 und scheitern, drei `if` mit `else`-Zweigen können
ebenfalls scheitern, `for`, `switch` und `select` zählen gar nicht. Go ist damit bei `if` strenger und bei Schleifen
lockerer als das Budget. `revive` (in golangci-lint enthalten) hat die Regel `max-control-nesting`; mit Argument 4
meldete sie in derselben Probe genau die Tiefe 5.

## Ziel

Die Verschachtelung von Go-Code wird als Tiefe geprüft, mit derselben Grenze wie bei TypeScript. Nutzen: Das Budget
bedeutet in beiden Sprachen dasselbe, und der Go-Port (SP04–SP06) scheitert nicht an einer zu strengen Punktzahl.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Go-Code schreiben (ab SP03); Review-Session.

## Anforderungen

- Go-Code mit Verschachtelungstiefe 5 lässt `golangci-lint run` scheitern, Tiefe 4 nicht; das gilt für alle
  Kontrollstrukturen (`if`, `for`, `switch`, `select`).
- Keine neue Abhängigkeit: nur Linter, die golangci-lint mitbringt.

## Nicht-Ziele

Andere Budget-Grenzen ändern; Bestandscode umbauen (es gibt noch keinen Go-Code außer `data/`).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
Grenzen nur aus `docs/arbeitsweise.md` › Komplexitäts-Budget; `docs/arbeitsweise.md` › Werkzeuge nennt heute `nestif`
und muss mit angepasst werden.

## Beispiele

- `for` → `if` → `if` → `if` → `if` (Tiefe 5) in `engine/sim/` → `npm run check:go` scheitert.
- Vier verschachtelte `if` (Tiefe 4) → grün.

## Ausnahme- und Fehlerfälle

`max-control-nesting` zählt anders als erwartet (z. B. `else if`) → Verhalten per Probe festhalten und im Ticket notieren.

## Akzeptanzkriterien

- **AC-01** Eine Go-Probe mit Tiefe 5 lässt `golangci-lint run` scheitern, eine mit Tiefe 4 nicht (Proben zurücknehmen).
- **AC-02** `docs/arbeitsweise.md` › Werkzeuge nennt den tatsächlich genutzten Linter.

## Offene Fragen

keine. Entschieden (🧑, 2026-09-30): `nestif` ersetzen.

## Notizen

Probe (Review SP01.4): `nestif` 4 meldete `if`-Tiefe 4 (Punktzahl 6) und 5 (Punktzahl 10), nicht Tiefe 3;
`revive` `max-control-nesting` 4 meldete nur Tiefe 5. Nebenbefund: `funlen` zählt nur den Rumpf (60 Rumpfzeilen
erlaubt), Oxlint `max-lines-per-function` zählt Kopf und schließende Klammer mit (58 Rumpfzeilen erlaubt).

Umsetzung L1.1 (golangci-lint 2.14, Oxlint 1.86): `revive` › `max-control-nesting` zählt `else if` als eigene Ebene,
Oxlint `max-depth` nicht. Eine `else if`-Kette mit drei weiteren `if` darin ist in TypeScript Tiefe 4 (grün),
in Go Tiefe 5 (rot). Go ist damit nur bei `else if` um eine Ebene strenger; bewusst so belassen.

Review L1.2 (Gegenproben, `--max-same-issues=0`): Die Aussage „nur eine Ebene“ stimmt nicht, **jedes** `else if`
zählt eine Ebene: eine flache Kette mit 5 Zweigen scheitert (TypeScript Tiefe 1), ebenso `if`/3× `else if` mit einem
`if` im letzten Zweig (TypeScript Tiefe 2). Außerdem zählt `for range` gar nicht: `range` → 4× `if` bleibt grün,
erst `range` → 5× `if` scheitert. Tiefe 5 greift bei `if`, Dreiklausel-`for`, `switch`, Typ-`switch` und
`select` → `if`; `else` und Funktionsliterale zählen wie bei Oxlint nicht bzw. setzen zurück. Offene Abweichungen: B-057.
Achtung bei Proben: golangci-lint zeigt standardmäßig nur 3 gleiche Meldungen (`max-same-issues`), der Exit-Code stimmt.
