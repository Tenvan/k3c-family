# RU3.3 · Golden lesen und als JSON-Baum vergleichen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.2
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

`k3c_det::golden` liest alle Golden und nennt bei zwei JSON-Bäumen den Pfad des ersten Unterschieds.

## Kontext

Vorbild: `engine/internal/golden/golden.go` (`Tree`, `Diff`, `Read`, `ReadFile`). Die Go-Tests vergleichen Golden als JSON-Baum, nicht als Text. 23 Golden unter `testdata/golden/`, davon 15 als `.json.gz`. Abweichung von Go, bewusst: `null` und leere Liste gelten als gleich (Leitfaden § 3). Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-det/**`

## Nicht-Ziele

Golden schreiben oder ändern. Ein eigenes JSON-Format.

## Schritte

1. `golden.go` ganz lesen; notieren, wie `Diff` Zahlen, Listen und fehlende Felder vergleicht und wie die Meldung aussieht.
2. `src/golden.rs`: `read(path) -> Value` (`.json` direkt, `.json.gz` über `flate2`), `tree(&impl Serialize) -> Value`, `diff(at, want, got) -> String` mit derselben Meldungsform wie Go; Zahlen über `f64::to_bits`, `null` gleich `[]`.
3. Tests: alle 23 Golden laden; `diff` eines Golden mit sich selbst ist leer; eine geänderte Zahl, ein fehlendes Feld und eine vertauschte Liste ergeben je den richtigen Pfad.
4. `cargo test -p k3c-det`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-03: Die vier Testfälle sind grün; alle 23 Golden laden.

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
