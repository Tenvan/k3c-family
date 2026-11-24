# GS3.6 · Review: Sprint-Abnahme GS3

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.8
- **Tickets:** B-966, B-978
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint GS3 ist gegen seine Kriterien geprüft, Befunde sind behoben oder als Ticket angelegt, die Abnahme steht in der README.

## Kontext

Ablauf: `docs/arbeitsweise.md` › Review-Session. Kriterien: README des Sprints GS3 (`docs/sprints/geplant/GS3-andenken-chronik/README.md`, nach dem Aktivieren unter `aktiv/`), Tickets B-966, B-978. Diff von `sprint/gs3` gegen den Start-Commit. Fachprüfer: `k3c-reviewer` (Spielstand-Format, Fixture), `k3c-grafik` (Trophäen), `k3c-hud` (Seekarte, Menü). TV-Abnahmen (AC-06) bleiben bei 🧑.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`
- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/menu/**`

## Nicht-Ziele

Neue Funktionen oder Assets, Migration alter Spielstände.

## Schritte

1. `check_run task:check`, danach `check_run task:check:go`.
2. Nachweise AC-01 bis AC-05 prüfen; Spielstand-Version und Fixture gegen `docs/arbeitsweise.md` › Spielstand-Format ändern.
3. Fachprüfer, schwere Befunde beheben, Rest als Tickets.
4. Glossar-Einträge (Andenken, Chronik, Trophäe) im letzten Commit.
5. Abnahme-Ticket anlegen (Gen, Race; B-742), Abschnitt Abnahme füllen, AC-06 als offen bei 🧑 nennen.

## Fertig, wenn

- [ ] AC-01 bis AC-05 belegt, AC-06 als offen bei 🧑 vermerkt.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
