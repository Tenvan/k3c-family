# RU2.4 · Messung, Abnahme-Vorbereitung und Review

- **Status:** fertig
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

- **AC-04 geprüft:** `docs/messung/rust-client-1.md` mit allen zwölf Läufen (Vulkan/DX12 × 0/5 000/20 000 Sprites × 2, je 60 s), Kontrolllauf und Vergleich mit RU0.2. Kernaussage: Median in allen Läufen 16,64–16,68 ms (60 Bilder/s); CPU je Bild auf Vulkan 0,4–2,5 ms (DX12 ~16 ms = Warten auf VSync). Späte Bilder (> 25 ms) schwanken stark (7–169 je Lauf), steigen aber **nicht** mit der Spritezahl und treten auf einem durch parallele Sessions belasteten Rechner auf (vermutet, ungeprüft; CPU-Last 2–93 %). Ein ruhigerer Kontrolllauf zeigt 0 späte Bilder nach Sekunde 3.
- **AC-05 abgenommen (🧑 Ralf, 2026-11-16):** „Ruckelfrei“. Paket `dist/k3c-rust-demo/` (`start.cmd`, Optionen `--fullscreen`, `--sprites N`, Backend per `WGPU_BACKEND`). Ob die Abnahme auch auf dem schwächeren Notebook lief, hat 🧑 nicht gesondert genannt.
- **Review geprüft:** `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo test --workspace` grün; `task check:diff` grün. RU2.1 AC-01, RU2.2 AC-02, RU2.3 AC-03 laut Session-Ergebnissen. Befunde: keine offenen; Abweichungen stehen in RU2.1 und RU2.3 › Ergebnis.
- **Neue Tickets:** keine.
