# U2 · SRV · Level-Abfrage per HTTP

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-091
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Zu langer Seed oder unbekanntes Biom → 400; falsche Methode → 405 (B-091 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Antwort stimmt mit `engine/level` überein und ist wiederholbar (B-091/AC-01).
- **AC-02** 400 und 405 wie spezifiziert (B-091/AC-02).
- **AC-03** Kein Raum und keine Datei entstehen (B-091/AC-03).
- **AC-04** README nennt den Aufruf, `task check:go` grün (B-091/AC-04).

## Offene Fragen

Fehlender Seed: Standardseed oder Fehler? (B-091 › Offene Fragen, entscheidet 🧑 vor der Freigabe.)

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- U2.1 Endpunkt `/api/level` mit Eingabeprüfung und Tests (AC-01, AC-02, AC-03).
- U2.2 README-Abschnitt, Smoke-Aufruf; Review (alle) (AC-04).

## Abnahme

–
