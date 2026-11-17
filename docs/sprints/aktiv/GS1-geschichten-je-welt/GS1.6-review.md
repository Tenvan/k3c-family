# GS1.6 · Review: Sprint-Abnahme GS1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.3, GS1.5
- **Tickets:** B-965, B-974, B-975, B-976
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Der Sprint GS1 ist gegen seine Kriterien geprüft, Befunde sind behoben oder als Ticket angelegt, die Abnahme steht in der README.

## Kontext

Ablauf: `docs/arbeitsweise.md` › Review-Session (Sprint-Abnahme). Kriterien: `docs/sprints/geplant/GS1-geschichten-je-welt/README.md` (nach dem Aktivieren unter `aktiv/`), Tickets B-965, B-974, B-975, B-976. Diff des Sprint-Branches `sprint/gs1` gegen den Start-Commit. Fachprüfer: `k3c-welten` (Welt-Format, `docs/rules/welten.md`), `k3c-hud` (Einblendung), `k3c-workbench` (Tools, Editor). Golden und Spielstand müssen unverändert sein. Offen bei 🧑 bleiben die Abnahme der Texte (B-976/AC-04) und die TV-Prüfung (B-974/AC-05).

## Erlaubte Dateien

- `docs/rules/welten.md`
- `engine/sim/**`
- `native/k3c-game/hud/**`
- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/internal/mcpsrv/**`

## Nicht-Ziele

Neue Funktionen, Umformulieren abgenommener Texte, Simulation, Balancing.

## Schritte

1. `check_run task:check`, danach `check_run task:check:go`, `check_run task:check:dev`.
2. Je Kriterium AC-01 bis AC-06 den Nachweis aus den Session-Ergebnissen prüfen.
3. Fachprüfer laufen lassen, schwere Befunde beheben (kleine Fixes), Rest als Tickets mit Projekt und Sprint.
4. Glossar-Einträge aus GS1.1 im letzten Commit eintragen.
5. Abnahme-Ticket des Sprints anlegen (Gen, Race; B-742), Abschnitt Abnahme der README füllen, offene Punkte für 🧑 nennen (AC-07).

## Fertig, wenn

- [ ] AC-01 bis AC-06 belegt, AC-07 als offen bei 🧑 vermerkt.
- [ ] Abnahme-Ticket angelegt, Abschnitt Abnahme gefüllt.

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

- AC-01 bis AC-06 geprüft: Nachweise der Session-Ergebnisse und Testnamen (Fachprüfer k3c-reviewer, keine schweren Befunde); Glossar (Abschlusszeile, Geschichte, Kunstsprache, Wendepunkt, Wortschatz) eingetragen.
- `task check` grün; `task check:go` rot: `detvec TestDataTree` (doppel-truhe aus SI2.2) → B-1030 (SI2), außerhalb der erlaubten Dateien. `task:check:dev` nicht gefahren (Review-Regel: check und check:go).
- AC-07 offen bei 🧑. Abnahme-Ticket B-1029 (Gen, Race).
- Abweichung: kein Branch `sprint/gs1` vorhanden (Sessions per Einzel-PR in `main`), Branch für den Abschluss neu von `origin/main`.
