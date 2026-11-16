# RU2.4 · Messung, Abnahme-Vorbereitung und Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru2
- **Abhängig von:** RU2.3
- **Tickets:** B-1016
- **Kriterien:** AC-04, AC-05

## Ziel

Messbericht des ersten Clients, EXE für die Abnahme durch 🧑 auf PC und Notebook, Review aller Kriterien.

## Kontext

Vergleichswerte: RU0.2 › Ergebnis (Go/Ebitengine gegen Rust, Quadrat und Lasttest). Auswertung: späte Bilder = dt > 25 ms, schnelle = dt < 10 ms, je Sekunde.

## Erlaubte Dateien

- `docs/messung/rust-client-1.md`
- `rust/demo/**`
- Befunde in `rust/crates/k3c-platform/**`, `rust/crates/k3c-game/**` nur als Fix unter 30 Zeilen, sonst Ticket

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `cargo build --release`; EXE und `rust/demo/` nach `dist/k3c-rust-demo/` kopieren (mit Startdatei `start.cmd` für 🧑, Grafik-Pfad als Argument).
2. Messläufe je 60 s, je zwei: Vulkan und DX12 (`WGPU_BACKEND`), Last `--sprites 0`, `5000`, `20000`.
3. Bericht `docs/messung/rust-client-1.md` mit Tabelle und Vergleich zu RU0.2.
4. Review: alle AC, `cargo clippy -- -D warnings`, `task check:diff`.
5. 🧑 um Abnahme bitten (AC-05): PC und schwächeres Notebook, „ruckelfrei ja/nein“, Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-04: Bericht mit allen Messungen.
- [ ] AC-05: Abnahme 🧑 eingetragen (oder offen mit Grund).
- [ ] Review: alle Kriterien geprüft, Befunde behoben oder als Ticket.

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
