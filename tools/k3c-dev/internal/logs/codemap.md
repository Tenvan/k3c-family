# tools/k3c-dev/internal/logs/

## Responsibility
Liest die JSON-Logs unter `logs/*.jsonl` (B-046, Format von `log/slog`) effizient von hinten, filtert sie und verdichtet gleichartige Meldungen. Reine Lese-Schicht ohne Schreibzugriff.

## Design
- `entry.go`: `Entry` (Felder `time`, `level`, `msg`, `ns`, Rest in `Data`), `Levels`, `LevelRank` (unbekanntes Level = niedrigster Rang), Zeilenparser `parse`.
- `reader.go`: Rückwärts-Scan `Scan(path, Query)` → `Result` (neueste zuerst) über `reverseLines` mit Blockgröße `blockSize` und Byte-Budget `DefaultBudget`=8 MiB; Vorwärts-Lesen `ReadSince(path, cursor, limit, budget)` → `SinceResult` mit Byte-Cursor (nur vollständige Zeilen).
- `digest.go`: `Fingerprint` maskiert variable Teile (Pfade, Zahlen; `masks`, Länge `fingerprintMax`), `Digest` bildet `Group`s je Namespace + Fingerabdruck, häufigste zuerst.
- Fehlende Datei = leeres Ergebnis.

## Flow
1. `logs_errors`: `Scan` mit `minLevel` WARN → `Digest` → eine Zeile je Gruppe.
2. `logs_query`: `Scan` mit `Query` (Level, `ns`, Muster, `since`, `limit`).
3. `logs_since`: `ReadSince` ab Cursor der letzten Antwort → neuer Cursor.

## Integration
- Konsumenten: `internal/mcpsrv/tools_logs.go` (MCP-Log-Tools), `tools/k3c-dev/app_logview.go` (Log-Ansicht), `internal/services/adopt.go` (`logs.Scan` der letzten Stunde zur Dienst-Übernahme).
- Abhängigkeiten: nur Standardbibliothek.
