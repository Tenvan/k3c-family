# B-091 · Der Server liefert ein generiertes Level per HTTP, ohne einen Raum anzulegen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** U2
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Level-Generator läuft nur in Go (`engine/level`). Der Browser bekommt ein Level erst, wenn er einem Raum beitritt (WebSocket, Protokoll v2). Eine reine Level-Ansicht ohne Raum und ohne Simulation ist heute nicht möglich; `level_generate` in k3c-dev rechnet in-process für Agenten, nicht für den Browser.

## Ziel

`GET /api/level?seed=…&biome=…` liefert dasselbe Level, das ein Raum mit diesem Seed und Biom bekäme (Abschnitte, Objekte, Warnungen der Prüfung), als JSON. Nutzen: Level-Betrachter (B-092) und Skripte brauchen keinen Raum.

## Beteiligte und Zielgruppen

Entwickler und der Level-Betrachter; kein Spieler-Pfad.

## Anforderungen

- Nur lesend, deterministisch: gleicher Seed und gleiches Biom ergeben dieselbe Antwort wie `engine/level` und wie das Level im Raum.
- Kein Token nötig (reine Berechnung aus öffentlichen Daten), aber begrenzte Eingaben: Seed höchstens 64 Zeichen, Biom nur aus den Daten (`data/biomes`), Standardbiom `forest`.
- Die Antwort enthält die Warnungen der Level-Prüfung.
- Der Aufruf ändert keinen Zustand (kein Raum, keine Datei, kein Log-Eintrag über Info hinaus).

## Nicht-Ziele

Simulation (`sim_run` bleibt in k3c-dev), Schreiben oder Hochladen von Levels, Änderungen am WebSocket-Protokoll.

## Regeln und Einschränkungen

Domäne SRV (`engine/net/`); `engine/level` bleibt frei von Importen aus `engine/net` (Schichtgrenze). Keine neue Abhängigkeit. Antwortformat orientiert sich an dem Level-Teil des Protokolls v2 (`docs/protocol.md`), ohne es zu ändern. README beschreibt den Aufruf.

## Beispiele

`GET /api/level?seed=test&biome=forest` → 200, JSON mit Breite, Abschnitten und Objekten; zweimal aufgerufen identisch. `?biome=xyz` → 400 mit Meldung.

## Ausnahme- und Fehlerfälle

Fehlender Seed → Standardseed oder 400 (entscheidet die Spec-Freigabe); Seed zu lang oder Biom unbekannt → 400; falsche Methode → 405.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test (`httptest`) belegt: Antwort für Seed und Biom stimmt mit `engine/level` überein, zweimal identisch.
- **AC-02** Ein Go-Test belegt 400 für unbekanntes Biom und zu langen Seed, 405 für POST.
- **AC-03** Der Aufruf legt keinen Raum und keine Datei an (Test auf die Räume in `/api/status`).
- **AC-04** README nennt den Aufruf; `task check:go` ist grün.

## Offene Fragen

Fehlender Seed: Standardseed (`k3c`) oder Fehler? Entscheidet 🧑 mit der Freigabe.

## Notizen

Vorbild für das Format: `LevelInfo` in `src/online/clientProtocol.ts`.
