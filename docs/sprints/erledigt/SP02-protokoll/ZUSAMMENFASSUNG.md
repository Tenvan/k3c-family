# SP02 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Protokoll v2 & Raummodell
- **Projekt:** –

## Ziel

Protokoll und Raummodell für mehrere Räume, mehrere lokale Spieler pro Gerät und gemischten Couch-/Online-Koop sind beschlossen. Am Ende sichtbar: `docs/protocol.md`, Beispiel-Nachrichten in `testdata/protocol/`, `docs/decisions/002-protokoll-v2.md`.

## Ergebnis

**2026-09-30, SP02.3 (autonomer Agent).** Geprüft: 23 Dateien aus `git diff --stat 0fb32f8..origin/main` (`origin/main` 23b760c), jede vollständig gelesen: `docs/protocol.md`, die 15 Beispiele in `testdata/protocol/` (`s2c-level.json`, `s2c-snapshot-full.json`, `s2c-snapshot-delta.json` zusätzlich per Node nach Feldern ausgewertet), die vier Dateien dieses Sprints, `docs/sprints/README.md`, `docs/backlog/README.md`, B-059. Dazu gegen `docs/protocol.md` gelesen: SP07-/SP08-README, B-030, B-036, B-038, B-039, `src/online/protocol.ts`, `src/world/sim/campaign.ts`, `src/world/sim/travel.ts`, `serv …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/protocol.md` beschreibt Raum, Gerät und lokale Spieler, Raumliste und Code, Beitrete … |
| AC-02 | geprüft | Je Nachricht liegt ein gültiges JSON-Beispiel in `testdata/protocol/`; die Snapshot-Größe … |
| AC-03 | – | `docs/decisions/002-protokoll-v2.md` ist geschrieben; 🧑 beschließt sie mit der Abnahme des … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-030 | erledigt | – | Geräte verbinden sich nach Abbruch wieder |
| B-036 | erledigt | – | Mehrere Spiele laufen gleichzeitig |
| B-038 | erledigt | – | Lokale und Online-Spieler teilen sich einen Raum |
| B-039 | erledigt | – | Bewegungen laufen trotz Snapshots flüssig |
