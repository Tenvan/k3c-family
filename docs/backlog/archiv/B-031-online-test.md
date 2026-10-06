# B-031 · Online-Verbindung ist automatisch getestet

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP07
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Der Online-Weg (WebSocket, Raum, Snapshot) ist in keinem automatischen Test geprüft.

## Ziel

Online-Verbindung ist automatisch getestet. Nutzen: Der Online-Weg ist heute nicht in der CI geprüft.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- Ein Test verbindet per WebSocket, tritt einem Raum bei und empfängt einen Snapshot.

## Nicht-Ziele

Lasttests.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

CI-Lauf → der Test tritt einem Raum bei und prüft den ersten Snapshot.

## Ausnahme- und Fehlerfälle

Kein Snapshot kommt → der Test scheitert mit Zeitlimit, statt zu hängen.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test deckt Beitreten und Snapshot ab und läuft in der CI.

## Offene Fragen

keine

## Notizen

Erledigt in SP07 (2026-10-01).
