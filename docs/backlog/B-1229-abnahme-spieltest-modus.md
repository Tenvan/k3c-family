# B-1229 · Abnahme-Läufe WZ66: Sim, Balance, Gen samt Golden und Race laufen grün

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review WZ66.4 (2026-11-22): `cargo fmt --check`, `cargo clippy --all-targets` und `cargo test -p k3c-game` grün bis auf einen Test, der `gen:paper` braucht. Sim, Balance, Gen und Race laufen nach B-742 nur hier. Der Sprint ändert nur `rust/crates/k3c-game` (Spieltest-Modus), keine Sim, keine Daten.

## Ziel

Die Abnahme-Läufe des Sprints WZ66 sind gelaufen, Abweichungen als Tickets erfasst.

## Beteiligte und Zielgruppen

Claude arbeitet autonom ab; Ralf (🧑) gibt die Spec frei.

## Anforderungen

- **AC-01** Sim: entfällt bis zur Aufhebung der Sperre (Beschluss 🧑 2026-11-11), danach `sim_test` ohne Abweichung zur Baseline.
- **AC-02** Balance: entfällt bis zur Aufhebung der Sperre; der Sprint ändert keine Werte.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün, darunter `props::plan_objects_stage_tests::drei_stufen_mindestens_halbe_monarchenhoehe_auseinander`.
- **AC-04** Race: `check_run task:check:race` grün (ohne gcc: als übersprungen nennen).

## Nicht-Ziele

Neue Funktionen des Spieltest-Modus; Behebung der unten genannten, vom Sprint unabhängigen Fehler auf main (eigene Tickets).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidung 012 und 013; Sim und Balancing bleiben gesperrt, bis 🧑 widerruft.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/playtest/**`, `.work/**`
- **Abhängig von:** WZ66.4
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: reines Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

Ein Lauf rot → neues Ticket, Messwerte in die Notizen.

## Akzeptanzkriterien

- **AC-01** Sim (`sim_test`): entfällt bis zur Aufhebung der Sperre, danach ohne Abweichung zur Baseline.
- **AC-02** Balance: entfällt bis zur Aufhebung der Sperre.
- **AC-03** Gen (`check_run task:test` samt `gen:paper` und Golden) grün.
- **AC-04** Race (`check_run task:check:race`) grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Beim Review rot, aber nicht vom Sprint verursacht (Sprint ändert weder `engine/`, `data/` noch `testdata/`): `task check:go` rot in `engine/internal/trace` (TestAufzeichnung, Abweichung `testdata/rust/trace/nacht-300.jsonl.gz`), `engine/room` (TestKampfInselwechselLetzteInselOhneTausch, TestVictoryEinmal) und `engine/sim` (TestVorpostenBauNurOhneGefahr, Nil-Zeiger); `task check` rot in `tests/planning.test.ts` (WZ65: AC-04 ohne Session; B-1225: Bereich leer) und `tools/paper/rig/animation.test.ts` (EPERM beim Umbenennen im Temp-Ordner, Umgebung). golangci-lint ist auf diesem Rechner nicht installiert.
