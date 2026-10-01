# B-066 · Der Go-Server schreibt sein Log als JSON nach logs/

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** D1
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („D1 freigegeben“), Revision 2

## Ausgangslage

SP03 plant `log/slog` für den Go-Server, ohne Ziel und Format festzulegen. `k3c-dev` (B-046, B-064) liest jede
`logs/*.jsonl` im Format seines eigenen Logs.

## Ziel

Der Go-Server schreibt sein Log als JSON nach `logs/`. Nutzen: Server-Fehler erscheinen ohne weiteren Aufwand in
`k3c-dev` (Logs-Seite und `logs_*`-Tools).

## Beteiligte und Zielgruppen

Entwickler und Agenten am Entwickler-PC; im Docker und auf dem Pi bleibt stdout.

## Anforderungen

- `k3c-server` schreibt mit dem JSON-Handler von `log/slog` nach `<Ordner>/k3c-server.jsonl`. Ordner: `K3C_LOG_DIR`, sonst
  `logs/`, wenn dieser Ordner im Arbeitsverzeichnis schon existiert (Start aus dem Repo, auch per k3c-dev). Ohne beides
  (Docker, Pi) bleibt das Text-Log auf stderr, wie bisher.
- Die Datei kommt zusätzlich zum Text-Log auf stderr, nicht statt dessen.
- Der Dienst „Heimnetz“ in `tools/k3c-dev/services.json` bekommt `"log": "k3c-server"`, damit `svc_status` und `logs_*` ihn sehen.
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
- **AC-02** Ohne `K3C_LOG_DIR` und ohne Ordner `logs/` entsteht keine Datei, das Log bleibt auf stderr (Test).
- **AC-03** `services.json` führt den Dienst „Heimnetz“ mit `"log": "k3c-server"`, k3c-dev lädt sie (Test).

## Offene Fragen

keine

## Notizen

Vorgesehen für SP03.2 (dort steht `log/slog` schon); beim Bereitmachen von SP03 einplanen.
