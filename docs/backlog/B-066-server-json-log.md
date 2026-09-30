# B-066 · Der Go-Server schreibt sein Log als JSON nach logs/

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SP03 plant `log/slog` für den Go-Server, ohne Ziel und Format festzulegen. `k3c-dev` (B-046, B-064) liest jede
`logs/*.jsonl` im Format seines eigenen Logs.

## Ziel

Der Go-Server schreibt sein Log als JSON nach `logs/`. Nutzen: Server-Fehler erscheinen ohne weiteren Aufwand in
`k3c-dev` (Logs-Seite und `logs_*`-Tools).

## Beteiligte und Zielgruppen

Entwickler und Agenten am Entwickler-PC; im Docker und auf dem Pi bleibt stdout.

## Anforderungen

- `k3c-server` schreibt mit dem JSON-Handler von `log/slog` nach `logs/k3c-server.jsonl`, wenn `K3C_LOG_DIR` gesetzt ist
  oder der Server aus dem Repo startet; sonst nach stdout.
- Format wie B-046 › Eigenes Log: `time`, `level`, `msg`, optional `ns` (z. B. `http`, `room`, `store`).

## Nicht-Ziele

Log-Rotation; zentrale Log-Sammlung.

## Regeln und Einschränkungen

Nur Standardbibliothek. Keine Spielstände oder Geheimnisse (Token) im Log.

## Beispiele

Kaputte Spielstand-Datei → `{"level":"ERROR","ns":"store","msg":"Spielstand nicht lesbar","slot":"autosave"}`.

## Ausnahme- und Fehlerfälle

Ordner nicht schreibbar → Log nach stdout, eine Warnung.

## Akzeptanzkriterien

- **AC-01** Der Server schreibt im Repo nach `logs/k3c-server.jsonl` im Format aus B-046 (Test).
- **AC-02** Ohne Repo bzw. im Docker geht das Log nach stdout (Test).

## Offene Fragen

keine

## Notizen

Vorgesehen für SP03.2 (dort steht `log/slog` schon); beim Bereitmachen von SP03 einplanen.
