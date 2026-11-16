# RU3.5 · Daten-Schemas Teil 2: übrige Dateien, Welten, Laden von Platte und eingebettet

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.4
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

`k3c-data` lädt alle Spiel-Daten, von der Platte oder eingebettet, wertgleich zu Go.

## Kontext

Wie RU3.4. Dateien dieser Session: `island-sizes`, `light`, `monarch`, `native`, `paper`, `render`, `ruins`, `ship`, `sounds`, `story`, `stresstests`, `troops`, `waves`, dazu `worlds/*.json` (vier Welten). Laden: `data/embed.go` nimmt den Ordner aus `K3C_DATA_DIR`, sonst die eingebetteten Dateien; eingebettet ist nur die Welt `standard` (`go:embed *.json biomes/*.json worlds/standard.json`). Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-data/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Werte ändern. Eine neue Abhängigkeit zum Einbetten (`include_str!` genügt).

## Schritte

1. Wie RU3.4, Schritte 1 bis 4, für die Dateien dieser Session; `data-tree.json.gz` um ihre Einträge ergänzen.
2. `src/source.rs`: `enum Source { Dir(PathBuf), Embedded }`; `Source::from_env()` liest `K3C_DATA_DIR` wie `data/embed.go`; eingebettet über `include_str!` genau die Dateien, die Go einbettet.
3. Test: Laden von Platte und eingebettet ergibt für jede eingebettete Datei denselben Baum; eine fehlende Datei ergibt denselben Fehlertext wie Go.
4. `cargo test -p k3c-data`, `go test ./engine/internal/detvec/`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-04: Für jede Spiel-Daten-Datei ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur.
- [ ] AC-04: Laden von Platte und eingebettet ist getestet.

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
