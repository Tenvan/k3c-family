# WL1.5 · Review und Abnahme-Ticket WL1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/5-review
- **Abhängig von:** WL1.4
- **Tickets:** B-1210, B-1211, B-1212, B-1213
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.
Ein Pfad oder Glob je Zeile in Backticks: daraus entsteht die Kollisionsprüfung paralleler Sessions. Sammeldateien (`docs/backlog/README.md`,
`docs/sprints/README.md`, `docs/projekte/README.md`, `docs/glossar.md`) stehen hier nicht; sie ändert `plan_*` oder der letzte Commit des Sprints.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 bis AC-05 geprüft: Diff `origin/main...sprint/wl1` (WL1.1, WL1.6) gelesen; keine schweren Befunde (Repo-Erkennung nur über `<repo>/dist/k3c-game` mit Taskfile.yml und .git, `K3C_WORLDS_DIR` hat Vorrang, Rückfall bei unlesbaren Repo-Welten, Dev-Schild nur bei `repo`; `needs`/`stopBefore` validiert, Pfade ohne `..`/Laufwerk, kein Lock-Deadlock zwischen Stop und `ended`/`crashed`, Job-Objekt unverändert). Kein Fix nötig.
- Geprüft: cargo fmt und clippy sauber; `cargo test -p k3c-game` 1049 grün, 1 rot (bekannt: `drei_stufen_mindestens_halbe_monarchenhoehe_auseinander`), dazu bekannt rot `tests/e2e_flow::ablauf_wie_go_zwei_laeufe_gleich`; go vet und go test tools/k3c-workbench (außer `internal/gitops`: 5 Tests rot durch `git` „unable to access NUL“ in der Prüfumgebung, Paket nicht im Sprint geändert) und tools/rustpaket grün; Frontend tsc und vitest (330) grün; projectRules, atomicDesign, mcpToolsDisjoint, planning (1662) grün. golangci-lint fehlt auf dem Rechner. Kein Sim, Balance, Gen, check:race.
- Neues Ticket: B-1231 (Abnahme-Läufe Sim, Balance, Gen, Race). Offen für 🧑: Abnahme am PC (Welten-Auswahl mit DEV-Schild, Spielstart als Dienste).
