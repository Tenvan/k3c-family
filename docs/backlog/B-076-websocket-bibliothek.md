# B-076 · Der Go-Server spricht WebSocket über github.com/coder/websocket

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP07
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Protokoll v2 läuft über WebSocket `/ws` (Entscheidung 002). Die Go-Standardbibliothek hat keinen WebSocket-Server,
`go.mod` hat bisher keine Abhängigkeit. `docs/arbeitsweise.md` verlangt für jede neue Abhängigkeit ein Ticket und die
Zustimmung von 🧑.

## Ziel

Die Wahl der WebSocket-Bibliothek ist entschieden und festgehalten. Nutzen: SP07 kann `/ws` bauen, ohne RFC 6455
selbst umzusetzen.

## Beteiligte und Zielgruppen

🧑 entscheidet; SP07 setzt um.

## Anforderungen

- `github.com/coder/websocket` ist die einzige neue Abhängigkeit in `go.mod` (ohne transitive Abhängigkeiten).
- Server und Go-Tests nutzen dasselbe Paket (`Accept`, `Dial`); ein Lese-Limit begrenzt die Nachrichtengröße.

## Nicht-Ziele

Binärformat oder Kompression (erst nach Messung am Pi, SP11).

## Regeln und Einschränkungen

Entscheidung 001 (Standardbibliothek zuerst); verworfen: `gorilla/websocket` (ältere API ohne `context`),
eigene Umsetzung nach RFC 6455 (Randfälle und Sicherheit selbst verantworten).

## Beispiele

`go list -m all` nach SP07.2 → `k3c` und `github.com/coder/websocket`.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidung über eine Abhängigkeit.

## Akzeptanzkriterien

- **AC-01** Nach SP07 zeigt `go list -m all` im Repo-Wurzelmodul nur `k3c` und `github.com/coder/websocket`.

## Offene Fragen

keine. Entschieden von 🧑 (2026-10-01, Chat): `github.com/coder/websocket`.

## Notizen

Erledigt in SP07 (2026-10-01).
