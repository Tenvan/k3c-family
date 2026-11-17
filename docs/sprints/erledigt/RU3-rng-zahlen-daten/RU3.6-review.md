# RU3.6 · Review rng, Zahlen, Daten

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.5
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Zusätzlich gegen den Leitfaden prüfen: kein `HashMap`, kein `powf`, `exp`, `f64::max`, `f64::min` aus der Standardbibliothek in `k3c-rng`, `k3c-det` (außer in den eigenen Funktionen), `k3c-data`; kein `unsafe`; `engine/sim` und `testdata/golden` sind im Diff unverändert.

## Erlaubte Dateien

- `rust/crates/k3c-rng/**`
- `rust/crates/k3c-det/**`
- `rust/crates/k3c-data/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium mit dem Befehl aus der jeweiligen Session nachprüfen.
2. `grep -rn -E "HashMap|powf|\.exp\(|f64::max|f64::min|unsafe" rust/crates/k3c-rng rust/crates/k3c-det rust/crates/k3c-data` auswerten.
3. `task check` und `task check:go` nacheinander.
4. Stellen notieren, an denen der Leitfaden nicht reichte (für B-1018/AC-03).
5. Abschnitt Abnahme der Sprint-README füllen, PR öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` und `task check:go` sind grün.

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

- Jedes Kriterium nachgeprüft (Abschnitt Abnahme der Sprint-README): `cargo test -p k3c-rng -p k3c-det -p k3c-data` grün (5, 11, 25 Tests), `go test ./engine/internal/detvec/` grün, `task check` (7 min 33 s) und `task check:go` (2 min 18 s) grün, `cargo clippy` und `cargo fmt --check` grün.
- Grep `HashMap|powf|.exp(|f64::max|f64::min|unsafe` in den drei Crates: kein Treffer außer `mul_add` in `k3c-det/src/exp.rs` (FMA-Pfad von Go, einzige erlaubte Stelle).
- Wo der Leitfaden nicht reichte (für B-1018/AC-03): (1) § 5 nennt `math.Exp` nicht als CPU-abhängig; Go wählt auf amd64 zur Laufzeit FMA oder SSE (B-1023). (2) § 3 sagt nichts zu Go-Feldern ohne Tag: Go schreibt den Go-Namen (`HPFactor`) und liest ohne Groß-/Kleinschreibung; Rust braucht `rename` plus `alias`. (3) Nicht exportierte Go-Typen (`economyData`, `shipFile`, `rawWorld`) sind aus einem Test-Paket nur über `reflect` oder eine Kopie erreichbar. (4) Für Dateien, die nur das native Spiel in Teilen liest, gibt es keine eine Go-Struktur; Vorbild war der Aufbau der Datei. (5) `null` ≡ `[]` reicht nicht für Maps; fehlende Maps sind `Option<BTreeMap>`.
- PR: nicht geöffnet (Auftrag: kein PR, kein Merge); Branch gepusht.
- Neue Tickets: B-1027 (Abnahme-Läufe RU3).
