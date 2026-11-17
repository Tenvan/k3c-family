# RU3.4 · Daten-Schemas Teil 1: Dateien bis hub.json

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** RU3.3
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

`k3c-data` lädt die Dateien `balance-hints` bis `hub` (alphabetisch) und die Biome in typisierte Strukturen, wertgleich zu Go.

## Kontext

Die Daten liegen unter `data/*.json`, `data/biomes/` und `data/worlds/`; `data/embed.go` bettet sie ein oder lädt sie von der Platte (`K3C_DATA_DIR`). Die Go-Strukturen dazu stehen verteilt in `engine/sim` (`data.go`, `*_data.go`), `engine/level` und `native/k3c-game`; die Struktur zu einer Datei findet `grep -rn '"<name>.json"' engine data native`. Dateien dieser Session: `balance-hints`, `balance-targets`, `bosses`, `buildings`, `difficulty`, `economy`, `enemies`, `events`, `goals`, `guards`, `hub`, dazu `biomes/*.json`. Nicht zu den Spiel-Daten gehören `asset-prompts`, `assets-recherche`, `assets-vorschlaege` (Workbench) und alles unter `data/.omc`. Go ignoriert unbekannte Felder; Rust tut das auch. Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen).

## Erlaubte Dateien

- `rust/crates/k3c-data/**`
- `engine/internal/detvec/**`
- `testdata/rust/**`

## Nicht-Ziele

Werte ändern. Felder umbenennen. Logik, die über Laden und Prüfen hinausgeht.

## Schritte

1. Je Datei die Go-Struktur suchen und ganz lesen, samt der Prüfungen beim Laden.
2. Go-Test in `engine/internal/detvec`: lädt jede Datei dieser Session in ihre Go-Struktur und schreibt den JSON-Baum der Struktur nach `testdata/rust/data-tree.json.gz` (Schlüssel = Dateiname; mit `-write` schreiben, sonst vergleichen).
3. Je Datei ein Modul `src/<name>.rs` mit den Strukturen nach Leitfaden § 3 und einer Funktion `load(…)`, die dieselben Prüfungen macht wie Go, mit denselben Fehlertexten.
4. Test je Datei: laden, `golden::tree` bilden, mit dem Eintrag aus `data-tree.json.gz` vergleichen.
5. `cargo test -p k3c-data`, `go test ./engine/internal/detvec/`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-04 (Teil 1): Für jede Datei dieser Session ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur.
- [ ] Im Ergebnis steht je Datei die Go-Struktur mit Pfad.

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
