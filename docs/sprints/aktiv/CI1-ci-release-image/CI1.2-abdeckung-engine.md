# CI1.2 · Abdeckung pro engine-Paket

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** ci1/2-abdeckung-engine
- **Abhängig von:** –
- **Tickets:** B-019
- **Kriterien:** AC-02

## Ziel

Die Abdeckung der Go-Tests liegt pro Paket unter `engine/` vor: als Task `task coverage` lokal und als Zusammenfassung im CI-Lauf, ohne Mindestwert.

## Kontext

- Entscheidung 🧑 2026-10-18 (B-019 › Offene Fragen): Abdeckung nur anzeigen, kein Gate. Pakete ohne Tests erscheinen mit 0 % (B-019 › Ausnahme- und Fehlerfälle). Nicht gemessen wird der TypeScript-Client.
- Heute gibt es keinen Abdeckungs-Task; `Taskfile.yml` kennt `go:test` (`go test ./...`), `check:race` und `check:go`. Der CI-Job `go` (`.github/workflows/ci.yml`) ruft `task go:test` und `task check:race` auf.
- `go test -cover ./engine/...` gibt je Paket `coverage: NN.N% of statements` aus; Pakete ohne Testdateien zeigt Go mit `coverage: 0.0% of statements` bzw. `[no test files]`. Der Task rechnet nichts nach, er zeigt nur die Go-Ausgabe (je Paket eine Zeile).
- Die Abdeckung darf die CI nicht rot färben; ein Fehler des Tests selbst bleibt rot (wie `go:test`). Befehle nur über `task`; keine neue Abhängigkeit.

## Erlaubte Dateien

- `Taskfile.yml` (neuer Task `coverage`), `.github/workflows/ci.yml` (nur der Job `go`)
- `requirements.md` oder `docs/arbeitsweise.md` nur für eine Zeile zum Task, falls dort Tasks aufgelistet sind; Sprint-Dateien von CI1, `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Mindestabdeckung oder Gate, Abdeckung des TypeScript-Clients, zusätzliche Tests schreiben, Berichts-Dienste von Dritten (Codecov o. Ä.).

## Schritte

1. `Taskfile.yml`: Task `coverage` (`desc`, `deps: [tmp]`, Go-Vorbedingung wie `go:test`) mit `go test -cover ./engine/...`.
2. CI-Job `go`: Schritt „Abdeckung (engine/)“ nach `task go:test`, der `task coverage` ausführt und die Zeilen mit `coverage:` in `$GITHUB_STEP_SUMMARY` schreibt (Tabelle Paket → Abdeckung); der Schritt bricht nur bei einem Testfehler ab.
3. `task coverage` lokal laufen lassen und prüfen, dass jedes Paket unter `engine/` mit einem Wert erscheint (auch 0 %).
4. `task check` grün.

## Fertig, wenn

- [ ] AC-02 (B-019/AC-01): `task coverage` zeigt für jedes Paket unter `engine/` eine Abdeckung (Pakete ohne Tests mit 0 %), Nachweis: Ausgabe im Ergebnis.
- [ ] AC-02: Der CI-Lauf nennt die Abdeckung je Paket in der Job-Zusammenfassung, ohne Mindestwert (kein Schritt scheitert an der Zahl).
- [ ] `task check` grün; der Job `go` der CI nach dem Push grün (Nachweis im Ergebnis des Reviews).

## Prüfen

```bash
task coverage
task check
```

## Ergebnis

- **AC-02 geprüft (B-019/AC-01):** `go test -cover ./engine/...` lokal: conlog 91,2 %, internal/golden 0,0 % (keine Tests, erscheint mit 0 %), level 95,6 %, net 88,7 %, rng 96,7 %, room 91,8 %, sim 96,3 %, store 89,7 %. Neuer Task `coverage` im `Taskfile.yml`, neuer CI-Schritt „Abdeckung (engine/)“ im Job `go` mit Tabelle in `$GITHUB_STEP_SUMMARY`; `set -o pipefail`, kein Mindestwert (awk-Ausdruck an Beispielzeilen geprüft). Der CI-Nachweis folgt im Review CI1.3.
- `task check` grün. Getrenntes Review (Subagent, 2026-10-18): keine Befunde; die Anmerkung zu fehlendem `pipefail` ist umgesetzt.
- Neue Tickets: keine.
