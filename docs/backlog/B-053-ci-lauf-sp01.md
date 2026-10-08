# B-053 · Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** CI1
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SP01 wurde nur lokal umgesetzt und abgenommen (🧑: kein Push, keine CI). `.github/workflows/ci.yml` hat seitdem den
Schritt `npm run lint` im Job `check` und den neuen Job `go` (`setup-go@v7`, `go test ./...`,
`golangci-lint-action@v9` mit `v2.14`, Cross-Build `windows/amd64` und `linux/arm64`). Keiner davon ist je auf GitHub
gelaufen. Offen sind damit SP01/AC-02 und SP01/AC-04 (CI-Teil), B-009/AC-01 und B-009/AC-02 (jeweils „in der CI“).

## Ziel

Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen. Nutzen: Die Nachweise gelten nicht nur auf dem
Entwickler-PC, und ein Fehler im Workflow (Action-Version, Pfad, Go-Version aus `go.mod`) fällt vor SP02 auf.

## Beteiligte und Zielgruppen

🧑 (pusht und gibt die CI frei); Agenten, die ab SP02 auf eine grüne CI bauen.

## Anforderungen

- Der Stand nach SP01 (Review SP01.4) läuft auf GitHub durch beide Jobs `check` und `go`.
- Scheitert ein Schritt, wird nur der Workflow oder die Konfiguration korrigiert, keine Grenze gelockert.

## Nicht-Ziele

Neue CI-Schritte oder Werkzeuge; Branch-Schutz einrichten.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
Versionen bleiben passend zu `requirements.md`, `.nvmrc` und `go.mod` (B-052).

## Beispiele

Push von `main` nach SP01 → Workflow „CI“ zeigt `check` und `go` grün.

## Ausnahme- und Fehlerfälle

`golangci-lint-action` findet `v2.14` nicht oder `setup-go` kennt Go 1.27 nicht → Version im Workflow korrigieren,
`requirements.md` mit anpassen.

## Akzeptanzkriterien

- **AC-01** Der Job `check` (mit `npm run lint`) ist für den Stand nach SP01 auf GitHub grün (SP01/AC-02, B-009/AC-01).
- **AC-02** Der Job `go` (Tests, `golangci-lint`, beide Cross-Builds) ist für denselben Stand grün (SP01/AC-04, B-009/AC-02).

## Offene Fragen

Wann pusht 🧑 den lokalen Stand (7 Commits vor `origin/main` plus Review)? (🧑)

## Notizen

Lokal geprüft im Review SP01.4 (Go 1.27.0, golangci-lint 2.14.0, Node 26.7): `npm run check`, `npm run build`,
`npm run check:go`, beide Cross-Builds und `golangci-lint config verify` grün.
