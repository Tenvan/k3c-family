# B-794 · check:diff und CI prüfen das Modul des nativen Spiels

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG1
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die nativen Module (`native/k3c-ebiten`, `native/k3c-rig`) sind eigene Go-Module außerhalb von `k3c`; `tools/checkdiff` und `.github/workflows/ci.yml` erwähnen `native/` nicht (geprüft per Suche am 2026-11-11). Das Modul `native/k3c-game` (B-792) wäre damit ungeprüft.

## Ziel

`task check:diff` und die CI prüfen `native/k3c-game` wie die anderen Go-Pakete.

## Beteiligte und Zielgruppen

Claude und alle Sprints NG2–NG11 (Prüfung vor dem Commit).

## Anforderungen

- `check:diff` führt für geänderte Pakete in `native/k3c-game` `go vet`, golangci-lint und `go test -short` im eigenen Modul aus (Stufen wie im Rest).
- Die CI baut und testet das Modul unter Windows (ohne cgo); Linux-Jobs lassen es aus.
- Ein Ändern in `engine/` löst `go vet` des nativen Moduls mit aus (Abnehmer).

## Nicht-Ziele

Prüfung der Prototypen `native/k3c-ebiten` und `native/k3c-rig` (eigenes Ticket bei Bedarf); neue Lint-Regeln.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Tempo im Lauf; ein Lauf über etwa eine Minute ist ein Befund.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `tools/checkdiff/**`, `.github/workflows/ci.yml`
- **Abhängig von:** B-792
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Änderung an `native/k3c-game/view/frame.go` → `task check:diff:plan` nennt vet, Lint und Test für `native/k3c-game/view`.

## Ausnahme- und Fehlerfälle

Go fehlt auf dem Rechner → Schritt übersprungen wie die anderen Go-Schritte, mit Meldung.

## Akzeptanzkriterien

- **AC-01** `task check:diff:plan` nennt für eine Änderung in `native/k3c-game` die Schritte vet, Lint und Test dieses Moduls; ein Test in `tools/checkdiff` belegt das.
- **AC-02** Die CI hat einen Windows-Schritt für `native/k3c-game` (Build und `go test -short`).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG1).
