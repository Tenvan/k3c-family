# WZ2.1 · Ticket-Nummern über Worktrees und Remote-Refs, plan_set lehnt fehlendes Feld ab

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** –
- **Tickets:** B-387, B-360
- **Kriterien:** AC-01, AC-02

## Ziel

`plan_create` vergibt Ticket-Nummern eindeutig über Wurzel, Worktrees und lokale Remote-Refs, und `plan_set` lehnt ein Feld ab, das in der Datei fehlt.

## Kontext

- Spec: `docs/backlog/B-387-ticketnummern-parallele-worktrees.md` (AC-01 bis AC-04) und `docs/backlog/B-360-plan-set-fehlendes-feld.md` (AC-01); Entscheidungen 🧑 2026-10-11 stehen in der Sprint-README unter „Offene Fragen“.
- `nextTicket(root)` in `tools/k3c-dev/internal/planning/create.go` liest nur `backlog/` und `backlog/archiv/` des eigenen Checkouts.
- `Worktrees(root)` und `parseWorktrees` in `tools/k3c-dev/internal/planning/worktree.go` liefern die Git-Worktrees (`git worktree list --porcelain`).
- `setField(text, name, value)` in `tools/k3c-dev/internal/planning/edit.go` ersetzt nur vorhandene Feldzeilen; `setFields` ruft es auf und meldet bei fehlendem Feld nichts.
- Remote-Refs nur lesen: `git ls-tree` auf `origin/develop` und `origin/sprint/*` aus dem lokalen Stand, kein `git fetch`.
- Lese-Zugriff auf andere Worktrees, geschrieben wird nur im Ziel-Checkout.

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/create.go`, `worktree.go`, `edit.go` und zugehörige `*_test.go`
- neue Dateien im selben Ordner, falls eine Datei sonst 400 Zeilen überschreitet

## Nicht-Ziele

Sperren oder Reservierungsdateien; Umnummerieren bestehender Tickets; Sprint- und Session-IDs; Felder in Dateien einfügen (B-360: ablehnen); `git fetch`.

## Schritte

1. Test zuerst (rot): Temp-Repo mit zwei Worktrees, der zweite hat ein höheres Ticket → `plan_create` vergibt eine größere Nummer (B-387/AC-01).
2. `nextTicket` nimmt Wurzel, alle `Worktrees(root)` und deren `backlog/archiv/` mit; größte Nummer + 1.
3. Nicht lesbare Worktrees überspringen und in der Antwort nennen (B-387/AC-02).
4. Antwort nennt Nummer und den Checkout, der sie bestimmt hat (B-387/AC-03).
5. Remote-Refs `origin/develop`, `origin/sprint/*` über `git ls-tree` aus dem lokalen Stand zählen, ohne Fetch; `git` fehlt → nur eigener Checkout mit Warnzeile (B-387/AC-04).
6. Test: `Set` auf Datei ohne das Feld → Fehler mit Feldname und Datei, Datei unverändert (B-360/AC-01); `setFields` gibt den Fehler weiter.
7. `task check:dev` ausführen, Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-01: Go-Tests für höhere Nummer in Worktree, nicht lesbaren Worktree, Antwort mit Checkout und Remote-Ref grün (`B-387/AC-01`–`AC-04`).
- [ ] AC-02: Go-Test für fehlendes Feld: Fehler mit Grund, Datei unverändert (`B-360/AC-01`).
- [ ] Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; `task check:dev` grün.

## Prüfen

```bash
task check:dev
```

Bei laufendem k3c-dev stattdessen `check_run` mit `task:check:dev`. Manuelle Prüfungen: keine.

## Ergebnis

- AC-01 umgesetzt, geprüft: `TestTicketNummerAusAnderemWorktree`, `TestTicketNummerWorktreeNichtLesbar`, `TestTicketNummerAusRemoteRef` (B-387/AC-01–AC-04) grün; `task check:dev` grün (go test, golangci-lint 0 Issues).
- AC-02 umgesetzt, geprüft: `TestFehlendesFeldWirdAbgelehnt` (B-360/AC-01): `setFields` lehnt ein Feld ab, das in der Datei fehlt, die Datei bleibt unverändert.
- Neu: `tools/k3c-dev/internal/planning/ticketnr.go` (Nummer über Wurzel, Worktrees samt Archiv und lokale Refs `origin/develop`, `origin/sprint/*`, ohne Fetch; Antwortzeile nennt Nummer und bestimmenden Checkout; übersprungene Worktrees und fehlendes git werden genannt). Git-Auskünfte gelten nur, wenn der Ordner selbst die Repo-Wurzel ist (sonst lieferte ein Test-Ordner unterhalb des Repos dessen Worktrees).
- Abweichungen vom Plan: keine. Neue Tickets: keine. Manuell geprüft: nichts.
