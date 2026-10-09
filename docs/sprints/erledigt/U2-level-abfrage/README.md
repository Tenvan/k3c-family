# U2 · SRV · Level-Abfrage per HTTP

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-091
- **Start-Commit:** a0ed852
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02 🧑 Chat („weiter mit U2 dann“; Revision 1)

## Ausgangslage

Der Level-Generator läuft nur in Go; ein Level sieht der Browser erst nach dem Beitritt zu einem Raum (B-091).

## Ziel

`GET /api/level?seed=…&biome=…` liefert das Level ohne Raum und ohne Simulation. Am Ende sichtbar: `curl` auf den Server liefert zweimal dieselbe JSON-Antwort, unbekanntes Biom ergibt 400.

## Beteiligte und Zielgruppen

Entwickler; der Level-Betrachter (U3) ist der erste Nutzer.

## Anforderungen

B-091 › Anforderungen.

## Nicht-Ziele

Simulation, Schreiben von Levels, Änderungen am WebSocket-Protokoll (B-091 › Nicht-Ziele). Der Betrachter selbst gehört zu U3 (PLAT).

## Regeln und Einschränkungen

Domäne SRV (`engine/net/`, README für den Aufruf). Schichtgrenzen: `engine/level` importiert nichts aus `engine/net`. Keine neue Abhängigkeit. `task check:go` muss grün sein.

## Beispiele

`GET /api/level?seed=test&biome=forest` → 200 mit 22 Abschnitten; `?biome=xyz` → 400.

## Ausnahme- und Fehlerfälle

Fehlender Seed → `k3c`, fehlendes Biom → `forest`; zu langer Seed oder unbekanntes Biom → 400; falsche Methode → 405 (B-091 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Antwort stimmt mit `engine/level` überein und ist wiederholbar (B-091/AC-01).
- **AC-02** 400 und 405 wie spezifiziert (B-091/AC-02).
- **AC-03** Kein Raum und keine Datei entstehen (B-091/AC-03).
- **AC-04** README nennt den Aufruf, `task check:go` grün (B-091/AC-04).

## Offene Fragen

keine. Entschieden 2026-10-02 durch 🧑 (Chat): Fehlt der Seed, gilt der Standardseed `k3c`; fehlt das Biom, gilt `forest`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U2.1 | `U2.1-endpunkt.md` | Umsetzung | autonom | fertig |
| U2.2 | `U2.2-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-02, Review U2.2 (Agent): AC-01 bis AC-04 belegt (U2.1 › Ergebnis, nachgeprüft in U2.2 › Ergebnis).
- AC-03 über `room.Manager.Rooms()` statt `/api/status` belegt: gleichwertig, beide lesen dieselbe Raumliste (`Manager.rooms`).
- Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine.
