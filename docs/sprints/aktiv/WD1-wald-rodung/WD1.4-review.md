# WD1.4 · Review und Abnahme-Ticket WD1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/4-review
- **Abhängig von:** WD1.3
- **Tickets:** B-1204
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Review des Sprints WD1 und genau ein Abnahme-Ticket für Sim, Balance, Gen, Race und Frame-Messung der Baumdichte.

## Kontext

Sprint WD1, B-1204.

## Erlaubte Dateien

- `docs/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Ergebnisse WD1.1–3 gegen AC prüfen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (Sim, Balance Holz, Gen, Race, Frame-Messung bei 600–700 Bäumen, Sicht am PC).

## Fertig, wenn

- [ ] AC-01 bis AC-03 geprüft, Abnahme-Ticket angelegt.

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

AC-01 geprüft: WD1.1 (`world/forest.rs`, `forest_tests` über 40 Seeds) erfüllt Dichte, Hub-Streifen und Lichtungen laut Ergebnis der Session; Abweichung dokumentiert (Umsetzung in der Sim statt im Level-Generator, Level-Seeds und Go-Golden unverändert).
AC-02 geprüft: WD1.2 (`world/stands.rs`, `stands_tests`) erfüllt Erkennung und Rodungsregel; Abweichung: Mindestlänge 0,25 statt 0,5 der Stufenbreite (Ticket-Annahme 2), kein HUD-Hinweis bei abgelehnter Markierung (Annahme 5).
AC-03 teilweise geprüft: WD1.3 zeigt die Waldstücke im Debug-Overlay (F10) und im Cheat-Fenster (F4, Umschalter „Waldstücke aus“); die Level-Ansicht (Workbench/Studio) entfällt auf Anweisung von 🧑 Ralf (2026-11-24), weil sie aus der Go-Engine rechnet, die den Rust-Wald nicht kennt. Sicht am PC durch 🧑 im Abnahme-Ticket.

`task check` rot, nicht aus WD1: Oxlint-Funktionslänge und Komplexität in `tools/k3c-workbench/frontend/src/playtest/` (`TimelinePanel`, `RecordBar`, `SessionDetail`, `PlaytestPage`, `mockPlaytest`, `FindingsPanel`, `DescribeBar`, Tests), Vitest 2170 grün.
`task check:go` rot, nicht aus WD1 (Sprint ändert kein Go): `engine/internal/trace` (`TestAufzeichnung`, Golden `nacht-300.jsonl.gz` weicht ab), `engine/room` (`TestKampfInselwechselLetzteInselOhneTausch`, `TestVictoryEinmal`), `engine/sim` (`TestVorpostenBauNurOhneGefahr`, nil-Zeiger).

Das Abnahme-Ticket legt der Hauptrechner an (Second Worker legt keine Tickets an). Der Sprint bleibt aktiv, bis es existiert.

### Für den Hauptrechner

- **Abnahme-Ticket WD1** (Projekt und Sprint WD1, Kriterien AC-01 bis AC-03): Sim und Balance Holz, Gen, Race, Frame-Messung bei 600–700 Bäumen, Sicht am PC (Overlay F10, Umschalter F4, Rodungsregel mit 2 Spielern).
- Ticket: Oxlint-Verstöße in `tools/k3c-workbench/frontend/src/playtest/` (Funktionslänge, Komplexität), `task check` rot.
- Ticket: `task check:go` rot: Golden `testdata/rust/trace/nacht-300.jsonl.gz`, `engine/room` Inselwechsel und Sieg, `engine/sim` `TestVorpostenBauNurOhneGefahr`.
- Ticket: `props/plan_tests.rs` baut die Standard-Welt mit einer Insel („Etappe 2 hat keine Insel“), drei rote Tests in `k3c-game`.
- Optional: Waldstücke in der Level-Ansicht des Studios (Rust-Brücke oder Rust-Level-Werkzeug, Entscheidung 013).
