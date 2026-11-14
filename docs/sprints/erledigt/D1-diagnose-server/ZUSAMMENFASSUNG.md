# D1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Diagnose-Schnittstelle des Servers
- **Projekt:** –

## Ziel

Der Server schreibt ein JSON-Log, das k3c-dev liest, und bietet Speicher, Geräte, Log und zwei Aktionen hinter dem Token an. Am Ende sichtbar: `logs_query Heimnetz`-Einträge in k3c-dev und `curl`-Aufrufe der neuen Endpunkte mit Token.

## Ergebnis

Review 2026-10-01 (D1.4): `task check`, `task check:go`, `task check:dev` grün, Diff `7c4288c..main` geprüft, keine schweren Befunde. AC-01: D1.1 (JSON-Zeile, stderr bleibt, `services.json`); AC-02: D1.2 (`memory`, `devices`, Log-Endpunkt); AC-03: D1.3 (`disconnect`, `save`, Token-Schutz aller Wege). Nicht geprüft: Lauf gegen den echten Server und `-race` lokal. Neue Tickets: keine. B-066 und B-088 erledigt; SP10 (TUI, B-002) kann bereit gemacht werden.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Server schreibt sein Log zusätzlich als JSON nach `<Ordner>/k3c-server.jsonl`; ohne Or … |
| AC-02 | – | `/api/status` zeigt `memory`, `?room=` die `devices`, `/api/status/log` die Log-Zeilen ab … |
| AC-03 | – | `disconnect` und `save` wirken, alle Wege sind durch das Token geschützt (B-088/AC-03, B-0 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-066 | erledigt | AC-01 – | Der Go-Server schreibt sein Log als JSON nach logs/ |
| B-088 | erledigt | AC-02 –, AC-03 – | Der Server zeigt Speicher, Geräte und Log und führt Diagnose-Aktionen aus |
