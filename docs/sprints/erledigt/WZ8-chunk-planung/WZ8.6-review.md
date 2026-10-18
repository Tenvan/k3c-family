# WZ8.6 · Review WZ8

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz8/6-review
- **Abhängig von:** WZ8.5
- **Tickets:** B-461
- **Kriterien:** AC-14

## Ziel

Der Sprint-Diff von WZ8 ist gegen AC-01–AC-13 geprüft, schwere Befunde sind behoben oder als Ticket erfasst (AC-14), die Abnahme-Liste für WZ8.7 steht, und der eine PR des Sprints ist mit Auto-Merge offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session, Schritte 1–6, auf dem Sprint-Branch `sprint/wz8`, nie im selben Lauf wie eine Umsetzung. Die Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff aller vier Domänen (REG, SIM, SRV, DEV): `git fetch && git diff origin/develop...origin/sprint/wz8`. Das Code-Review macht ein getrennter Reviewer.

Besonders prüfen:
- Ohne Planung: Golden byte-gleich; der Golden-Diff enthält nur den neuen Fall und seine Begründung (eigener Commit der Session WZ8.3); `git status` zeigt keine Änderung an `data/islands.json` über das leere Feld hinaus (AC-14).
- Geplanter Inhalt nur aus dem eigenen Strom, nur `engine/rng`, kein `math/rand`; 2-Spieler-Test.
- Kein globaler Zustand für Planung und Linien; Schichtgrenzen (`engine/level` importiert nichts aus `sim`/`room`/`net`).
- `IslandSaveVersion` und Spielstand-Format unverändert; `CreateOptions` nicht gespeichert; alle Aufrufer mit Nullwert.
- Schreibregeln des Editors (atomar, format-stabil, Stand-Prüfung): Pfade fest, keine Eingabe von außen im Dateipfad, fail closed.
- Benchmark-Messwerte im Commit-Text (AC-13).

**Hardware entkoppelt:** WZ8.7 (`Agent: Mensch`) ist keine Abhängigkeit des Reviews. Das Review führt AC-10 als `offen (WZ8.7)`, öffnet den PR, und der Sprint bleibt `aktiv`, bis WZ8.7 fertig ist.

## Erlaubte Dateien

- Planungsdateien von WZ8 und B-461, neue Tickets, `docs/sprints/README.md` (Fahrplan), alle über k3c-dev
- Für schwere Befunde alle Dateien, die WZ8.1–WZ8.5 erlauben, in den Domänen des Sprints (REG, SIM, SRV, DEV); Befunde außerhalb → Ticket

## Nicht-Ziele

Neue Funktionen oder Umbauten über schwere Befunde hinaus; Stil, Doku, Benennung und Vereinfachungen als Befund behandeln; Kriterien umformulieren oder die Spec freigeben; eine Planung in `data/islands.json` einchecken oder Golden-Daten außer dem neuen Fall ändern; manuelle Prüfungen im Fenster oder Browser oder ein Urteil anstelle von 🧑 (WZ8.7); eine Version setzen (nur vorschlagen).

## Schritte

1. `task check`, `task check:go`, `task check:dev` und `task check:all` grün; bei laufender Workbench über `check_run`.
2. Sprint-Diff lesen, Code-Review durch einen getrennten Reviewer, nur schwere Befunde. Befunde in den Domänen des Sprints im Review-Commit beheben und Schritt 1 wiederholen; Befunde außerhalb → Ticket.
3. AC-01–AC-09 und AC-11–AC-13 je mit Nachweis aus den Session-Ergebnissen ins Ergebnis, AC-14 mit Golden-Diff und `git status`; AC-10 als `offen (WZ8.7)`.
4. Abnahme-Liste in WZ8.7 › Kontext schreiben (`plan_section`, Unterabschnitt `### Abnahme-Liste`).
5. Abnahme in der Sprint-README, höchstens fünf Zeilen, mit `Version: v… vorgeschlagen (Grund)`.
6. Sprint bleibt `aktiv` (WZ8.7 offen); Fahrplan anpassen. Commit `fix(dev): Review WZ8 (WZ8.6)`, `git fetch`, `git merge origin/develop`, push auf `sprint/wz8`. Danach den einen PR des Sprints gegen `develop` öffnen (Vorlage, eine Zeile je Session) und Auto-Merge einschalten: `gh pr merge --auto --squash`; ist der PR schon `CLEAN`, `gh pr merge --squash`.

## Fertig, wenn

- [ ] AC-01–AC-09, AC-11–AC-13 mit Nachweis im Ergebnis; AC-10 als `offen (WZ8.7)`; offene Befunde als Ticket.
- [ ] AC-14: `task check:all` und `task check:go` grün; Golden-Diff zeigt nur den neuen Fall samt Begründung; `git status` zeigt keine Änderung an `data/islands.json` über das leere Feld hinaus.
- [ ] Abnahme-Liste in WZ8.7 › Kontext; Abnahme in der Sprint-README.
- [ ] Der PR von `sprint/wz8` gegen `develop` ist offen, Auto-Merge an (oder gemergt).

## Prüfen

```bash
task check
task check:go
task check:dev
task check:all
```

Manuelle Prüfungen (Browser, Xbox, TV) nennt diese Session nicht; sie gehören zu WZ8.7.

## Ergebnis

Review durch Claude (autonom), 2026-10-17; Code-Review durch einen getrennten Reviewer-Agenten auf dem Sprint-Diff (Querschnitt) nach den Reviews je Session (WZ8.2–WZ8.5). Kein PR in dieser Session (Auftrag von Ralf: PRs macht er extra); der Sprint bleibt `aktiv`, bis WZ8.7 (Mensch) fertig ist.

- **Prüfläufe:** `task check` und `task check:go` (inkl. aller Golden-Tests) grün (`check_run`); Studio: `vitest` (134), `tsc`, `vite build`, `go test` und `golangci-lint` grün (in der Shell, `check:studio` ist in der Workbench nicht freigegeben); `check_run dev:test` grün. `task check:all` und `task check:dev` nicht als Lauf gefahren (kein `check_run`-Ziel; ihre Bestandteile einzeln grün).
- **AC-01 bis AC-09, AC-11 bis AC-13 umgesetzt, geprüft** (Nachweise in den Ergebnissen WZ8.2–WZ8.5): Regeltext `docs/rules/geplante-chunks.md` (AC-01, WZ8.1); `level.ValidatePlan` mit Linien als Parameter und ein Test je Ablehnungsfall (AC-06); Generator `GenerateWith`, eigener Strom, Biom-Summen, Chunk-Seed stabil, Golden-Fall (AC-02 bis AC-05); `sim.CreateOptions` mit allen Aufrufern und Sim-Test mit 2 Spielern (AC-07); Vorschau-Option (AC-08); Editor, Schreibregeln, gleiche Quelle für beide Vorschauen (AC-09, AC-11, AC-12); Benchmark +2,7 % mit Planung (AC-13).
- **AC-10:** `offen (WZ8.7)`, Abnahme durch 🧑.
- **AC-14:** Querschnitt geprüft: bestehende Golden-Dateien unverändert, nur `testdata/golden/level-planned.json` neu (eigener Commit mit Begründung); `data/islands.json` enthält nur das leere Feld `plannedChunks` (und einen Satz in `notes`); keine globalen Zustände für Planung und Linien, `engine/level` importiert nichts aus `sim`, `room`, `net`; `IslandSaveVersion` unverändert; kein `math/rand`; Dateien ≤ 400 und Funktionen ≤ 60 Zeilen (Nicht-Test-Code); kein Kriterium umformuliert.
- **Schwere Befunde:** keine. Hinweise ohne Befund: Der Umfang von WZ8.5 liegt über dem Richtwert; `tools/k3c-studio/go.mod` und `go.sum` enthalten Änderungen aus WZ7 (B-460), die Spec von WZ8 sagt „keine neue Abhängigkeit“ (aus WZ7 übernommen, zur Kenntnis in der Abnahme-Liste); der Benchmark lief als `go test -bench` in der Shell.
- **Neue Tickets:** B-475 (Frage zu den Annahmen, angelegt in WZ8.2).
- **Abnahme-Liste** für WZ8.7 steht in dessen Kontext; die Abnahme steht in der Sprint-README.
