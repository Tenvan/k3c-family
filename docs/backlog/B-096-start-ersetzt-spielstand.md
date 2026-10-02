# B-096 · „Im Spiel starten“ ersetzt einen gleichnamigen Spielstand, statt abgewiesen zu werden

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Level-Betrachter (B-092, `src/tools/leveltest.ts`) öffnet `game.html?autostart=1&fresh=1&save=<Seed>` und schreibt auf der Seite, der Start ersetze einen gleichnamigen Spielstand (die Sicherung bleibt). Der Server weist `create` mit `fresh` aber ab, sobald es den Spielstand gibt oder ein Raum mit dem Namen offen ist (`engine/room/manager.go`: `open` und `create` → `ErrSaveExists`, Code `save_exists`; `engine/net/ws_test.go` prüft das). Jeder Raum speichert beim Aufräumen (`lockedSweep` → `save()`), daher scheitert schon der zweite Start mit demselben Seed (z. B. die Vorgabe `k3c`): `game.html` bleibt in der Lobby, „Spielen“ sendet erneut `fresh` und scheitert wieder. Gefunden im Review U3.4.

## Ziel

„Im Spiel starten“ startet auch beim zweiten Mal mit demselben Seed ein neues Spiel, wie in B-092 entschieden (🧑, 2026-10-02): Der Start ersetzt einen gleichnamigen Spielstand, der bisherige bleibt als Sicherung erhalten. Der Hinweis auf der Seite stimmt dann.

## Beteiligte und Zielgruppen

Entwickler, 🧑 beim Balancing mit dem Level-Betrachter; 🧑 entscheidet über die Protokoll-Änderung.

## Anforderungen

- Ein neues Spiel über einen vorhandenen, nicht offenen Spielstand gleichen Namens ersetzt diesen; der bisherige Stand liegt danach in den Sicherungen (`engine/store`, B-028).
- Ist ein Raum mit dem Namen offen, bleibt die Abweisung oder es gibt eine klare Meldung (entscheidet 🧑).
- Der Hinweistext in `src/tools/leveltest.ts` stimmt mit dem Verhalten überein.

## Nicht-Ziele

Start mit beliebigem Seed oder in Höhle und Mine (B-095); Löschen von Spielständen ohne Sicherung.

## Regeln und Einschränkungen

`save_exists` gehört zum Protokoll v2 (`docs/protocol.md`): Eine Änderung der Bedeutung läuft als eigene Session mit beiden Enden (Arbeitsweise › Protokoll). Spielstände nie ohne Sicherung überschreiben.

## Beispiele

Seed `k3c`, Wald: „Im Spiel starten“, spielen, zur Übersicht, warten bis der Raum aufgeräumt ist, erneut „Im Spiel starten“ → neues Spiel; der vorige Stand `k3c` ist als Sicherung aufgeführt (`GET /api/save/backups?slot=k3c`).

## Ausnahme- und Fehlerfälle

Raum mit dem Namen noch offen → Verhalten nach Entscheidung von 🧑 (abweisen mit Meldung oder beitreten), nie ein stiller Datenverlust.

## Akzeptanzkriterien

- **AC-01** Go-Test: `create` mit `fresh` über einen vorhandenen Spielstand startet einen Raum; nach dem Speichern liegt der alte Stand in den Sicherungen.
- **AC-02** Der Hinweis auf `leveltest.html` beschreibt das tatsächliche Verhalten.

## Offene Fragen

Server-Änderung (Protokoll-Semantik von `fresh`) oder Lösung auf der Seite (ohne `fresh` starten: vorhandenen Stand gleichen Namens fortsetzen, sonst neues Spiel über den Wiederholungsversuch in `src/scenes/lobbyLogic.ts`)? Entscheidet 🧑.

## Notizen

Die Szenarien der Testseite umgehen das Problem mit eindeutigen Namen (`scenarioUrl`, Uhrzeit als Kennung).
