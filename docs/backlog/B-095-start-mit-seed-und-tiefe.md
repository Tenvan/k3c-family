# B-095 · Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Protokoll v2 kennt in `create` nur `save` (Name des Spielstands, `^[a-z0-9-]{1,32}$`), `fresh`, `depth` und `slots`. Der Seed eines neuen Spiels ist der Name des Spielstands (`engine/room/manager.go`), die Startstufe ist beim Autostart immer 0 (`src/scenes/lobbyLogic.ts`: `depth: 0`); `game.html` kennt `?depth=` nicht. Ein Level mit beliebigem Seed (Großbuchstaben, Sonderzeichen, mehr als 32 Zeichen) oder in Höhle und Mine ist deshalb nicht direkt startbar. Betrifft auch die toten Kacheln „Höhle“ und „Mine“ der Landingpage (B-079).

## Ziel

Ein Aufruf wie `game.html?autostart=1&fresh=1&save=NAME&seed=SEED&depth=N` startet ein neues Spiel mit diesem Seed in dieser Tiefe. Nutzen: Levels aus dem Level-Betrachter (B-092) und aus Fehlermeldungen lassen sich genau so im Spiel nachstellen.

## Beteiligte und Zielgruppen

Entwickler und Tester; 🧑 entscheidet über die Protokoll-Änderung.

## Anforderungen

- `create` bekommt ein optionales Feld `seed` (Text, höchstens 64 Zeichen); ohne `seed` bleibt der Name der Seed (rückwärtskompatibel).
- Der Client wertet `?depth=` und `?seed=` aus und reicht beides an `create` weiter.
- Die Änderung am Protokoll läuft als eigene Session mit beiden Enden und den Beispielen in `testdata/protocol/` (Arbeitsweise › Protokoll).

## Nicht-Ziele

Änderungen am Spielstand-Format oder an der Simulation; neue Tasten oder Menüs.

## Regeln und Einschränkungen

Protokoll-Änderung nur in einer eigenen Session (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`, `testdata/protocol/`); `docs/protocol.md` nennt die Version und die Rückwärtskompatibilität.

## Beispiele

`game.html?autostart=1&fresh=1&save=probe&seed=Familie-2026&depth=1` → neuer Raum in der Höhle mit dem Level zu Seed `Familie-2026`.

## Ausnahme- und Fehlerfälle

Ungültige Tiefe (nicht 0–2) oder zu langer Seed → Fehlermeldung des Servers (wie bisher bei ungültigem `create`), kein Raum.

## Akzeptanzkriterien

- **AC-01** Go-Test: `create` mit `seed` und `depth` erzeugt ein Level, das dem von `GET /api/level` für dieselben Werte entspricht.
- **AC-02** Der Client reicht `?seed=` und `?depth=` an `create` weiter (Vitest für `parseStartParams`).
- **AC-03** Protokoll-Beispiele und `docs/protocol.md` sind aktualisiert, alte Clients ohne `seed` funktionieren weiter.

## Offene Fragen

Soll der Seed ohne weiteres vom Namen des Spielstands getrennt werden (neues Feld) oder reicht der Name mit erweitertem Format? Entscheidet 🧑.

## Notizen

Entstanden bei der Vorbereitung von U3 (Level-Betrachter): Dort ist „Im Spiel starten“ vorerst auf Wald und Seeds im Namensformat beschränkt.
