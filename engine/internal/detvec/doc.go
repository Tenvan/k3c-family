// Package detvec hält die Prüf-Vektoren für den Rust-Port (Sprint RU3, B-1014): Gleitkomma-Funktionen
// (`testdata/rust/det-vectors.json.gz`) und später die JSON-Bäume der Spiel-Daten. Die Tests schreiben die
// Dateien nur mit `-write` (`go test ./engine/internal/detvec/ -run TestWriteVectors -write`); ohne `-write`
// vergleichen sie Go mit der Datei.
package detvec
