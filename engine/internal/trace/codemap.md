# engine/internal/trace/

## Responsibility

Serializer: schreibt die Aufzeichnung eines Sim-Laufs als gzip-komprimierte JSON-Zeilen (`.jsonl.gz`) für den
Rust-Port (Projekt RSU, RU38). Reine Logik ohne Abhängigkeit vom Raum; die Szenen stehen in den Tests.

## Design

- `trace.go`: Zeile 1 ist `Kopf` (`art` = `level`, `szene`, `seed`, `spieler`, `level`), danach je Takt eine Zeile `Takt`
  (`art` = `takt`, `takt`, `zustand`, `ereignisse`).
- `Schreiber` sammelt die Zeilen: `Level` (Kopfzeile, muss zuerst kommen), `Takt` (serialisiert sofort, weil sich die
  Welt weiterverändert; vor der Kopfzeile ein Fehler), `Lines` (unkomprimiert), `Gzip(n)` (Kopf plus n Takte, n < 0 alle,
  `BestCompression`). `Entpacken` liest eine `.jsonl.gz` zurück in Zeilen.
- Zahlen schreibt `encoding/json`.

## Flow

1. Test-Szene: `Level(...)`, dann je Takt `Takt(...)`.
2. `Gzip` liefert die Datei, `Entpacken` prüft sie; der Rust-Port liest dieselben Zeilen.

## Integration

- Nur Stdlib; Konsumenten sind die Tests (`trace_test.go`, `messung_test.go`) und die Rust-Seite über die Dateien.
