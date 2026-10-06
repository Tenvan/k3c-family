# RL1.1 · Abschnitt „Release“ in `docs/arbeitsweise.md`

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** rl1/1-abschnitt-release
- **Abhängig von:** –
- **Tickets:** B-170
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

`docs/arbeitsweise.md` hat einen Abschnitt „Release“: eine prüfbare Liste (je Punkt Befehl oder Datei und erwartetes Ergebnis) mit Rhythmus und Auslöser aus dem Beschluss Q20.

## Kontext

- **Rhythmus und Auslöser sind beschlossen (Q20, geändert 2026-10-03):** Nach **jedem fertigen Sprint** schlägt die Abnahme eine Version vor, ein Tag wird nur bei Bestätigung durch 🧑 gesetzt (B-180); zusätzlich ein Tag nach jedem Spieleabend, mit der Release-Checkliste. Das Tag-Schema steht schon in `docs/arbeitsweise.md` › „Entscheidungen und Versionen“ (Minor `v0.<n+1>.0` bei Wirkung, Patch `v0.<n>.<m+1>` bei Doku/Korrektur). Der Workshop aus dem Entwurf (RL1.1 „Rhythmus beschließen“) entfällt damit; AC-04 belegt diese Session mit Verweis auf Q20. Hinweis: B-170 nennt noch „`v0.<n>.0` nach jeder Phase“; maßgeblich ist der neuere Beschluss Q20, die Liste verweist auf den Abschnitt „Entscheidungen und Versionen“ statt das Schema zu wiederholen.
- **Punkte (B-170 › Anforderungen, AC-02):** Golden grün auf amd64 und arm64 (CI-Jobs `go` und `go-arm64` in `.github/workflows/ci.yml`, F2); `task check:all` grün; Spielstand-Migration (`engine/sim/save_migration_test.go`, `testdata/saves/v<n>/`, Abschnitt „Spielstand-Format ändern“); Dev-Reste aus (B-098, B-107, B-080: Debug-Overlay, Dev-Tasten im Server — je Punkt angeben, woran man es prüft); Pi-Image (CI-Job `docker` baut amd64/arm64; `release.yml` schiebt beim Tag das Image nach `ghcr.io`); Version im Status und auf der Landingpage stimmt mit dem Tag überein (B-141, `VERSION` im `Taskfile.yml`, `/api/health`); Credits vollständig (B-165, Test aus GR6); Tag-Schema (Verweis).
- Tag setzen: nur 🧑 (`git tag vX.Y.Z` auf `main` nach Fast-Forward, `CLAUDE.md` › Branches); `release.yml` reagiert auf `v*`.
- Prozess steht nur in `docs/arbeitsweise.md` (kein neues Dokument); die Datei hat 202 Zeilen, der Abschnitt bleibt kurz (Richtwert ≤ 20 Zeilen, Tabelle „Punkt · Prüfen mit · erwartet“).

## Erlaubte Dateien

- `docs/arbeitsweise.md` (nur neuer Abschnitt „Release“, Verweis in „Entscheidungen und Versionen“)
- `docs/backlog/B-170-*.md` (nur Notiz zu Q20), `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Probelauf (RL1.2), Tag setzen, neue Tasks oder CI-Schritte (fehlt ein Befehl, Ticket), itch.io/Englisch (B-023).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Abschnitt „Release“: Auslöser (Q20), Liste mit Prüfung und erwartetem Ergebnis je Punkt, „ein Punkt rot → kein Tag, Ticket“.
3. Jeden Befehl der Liste einmal aufrufen bzw. jede Datei prüfen, ob es sie gibt (nicht ob grün — das ist RL1.2); fehlt einer, Ticket statt erfundenem Befehl.
4. `task check` (prüft u. a. `tests/planning.test.ts`). Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Abschnitt „Release“ vorhanden, jeder Punkt mit Befehl oder Datei und erwartetem Ergebnis.
- [x] AC-02: Liste verweist auf Golden amd64/arm64, `task check:all`, Migration, Dev-Reste, Pi-Image, Version, Credits, Tag-Schema.
- [x] AC-04: Rhythmus und Auslöser aus Q20 stehen in der Liste (Verweis auf den Beschluss).
- [x] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

- **AC-01** umgesetzt: `docs/arbeitsweise.md` › „Release“, Tabelle „Punkt · Prüfen mit · Erwartet“, 9 Punkte, 4 Zeilen Auslöser/Regeln darüber.
- **AC-02** umgesetzt: Golden amd64 (`go`), Golden arm64 (`go-arm64`), `task check:all`, Migration, Dev-Reste, Pi-Image, Version, Credits, Tag-Schema (Verweis).
- **AC-04** umgesetzt: Auslöser und Rhythmus nach Q20 stehen in der Liste (Verweis auf Q20 und B-180); B-170 trägt die Notiz dazu.
- Existenzprüfung der Befehle und Dateien (Schritt 3): `task go:test`, `check:all`, `check:race` vorhanden; CI-Jobs `go`, `go-arm64`, `docker`; `release.yml`; `engine/sim/island_save.go`, `save_migration_test.go`, `testdata/saves/v1`, `v2`; `engine/room/dev.go` (`K3C_DEV`); `src/main.ts` (`?dev=1`); `src/tools/credits.test.ts`; `/api/health` liefert `version`. Nichts fehlt, keine neuen Tickets.
- Ungeprüft (Sache von RL1.2): ob die Punkte grün sind. Der Dev-Rest-Punkt hat keinen eigenen Befehl, nur Datei und Verhalten.
- `task check` grün (siehe Commit).
