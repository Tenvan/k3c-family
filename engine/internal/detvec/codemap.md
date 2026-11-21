# engine/internal/detvec/

## Responsibility

Test Fixture Generator (Projekt RSU, Sprint RU3, B-1014): hält die Prüf-Vektoren, mit denen der Rust-Port gegen Go
verglichen wird, zunächst Gleitkomma-Funktionen, später die JSON-Bäume der Spiel-Daten.

## Design

- `doc.go` ist die einzige Quelldatei (Paketkommentar); Inhalt und Logik stehen in den Tests (`detvec_test.go`,
  `data_test.go`, `inputs_test.go`).
- Die Vektoren liegen in `testdata/rust/det-vectors.json.gz`. Die Tests schreiben sie nur mit dem Flag `-write`
  (`go test ./engine/internal/detvec/ -run TestWriteVectors -write`); ohne `-write` vergleichen sie Go mit der Datei.

## Flow

1. Bei gewollter Änderung: Test mit `-write` schreibt die Datei neu.
2. Sonst vergleicht der Test die Go-Ergebnisse mit der eingecheckten Datei; der Rust-Port liest dieselbe Datei.

## Integration

- Abhängig von `engine/sim` und `data` über die Tests; Konsument der Datei ist der Rust-Workspace (`rust/`).
