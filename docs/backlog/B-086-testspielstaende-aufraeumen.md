# B-086 · Test-Spielstände der Testseite bleiben nicht liegen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Testseite (T1, B-081) startet jede Session mit einem neuen Spielstand `test-<Zeit>`. Der Raum speichert beim Beenden (`docs/protocol.md` › *Spielstand*),
daher bleibt je Klick eine Datei `saves/test-….json` liegen (`saves/` ist nicht eingecheckt). Nach dem Browser-Lauf zu SP08 und T1 lagen dort vier solcher Dateien.

## Ziel

Test-Spielstände sammeln sich nicht an. Nutzen: Die Spielstandliste bleibt übersichtlich, sobald Spielstände auswählbar sind (B-037/AC-04).

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 betreibt den Server.

## Anforderungen

- Spielstände mit dem Präfix `test-` werden nicht gespeichert, oder beim Aufräumen des Raums (10 min leer) gelöscht, oder beim Serverstart nach X Tagen entfernt (Entscheidung offen).

## Nicht-Ziele

Löschen anderer Spielstände; Änderungen am Protokoll.

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/store/`); Protokoll v2 unverändert; nie einen Spielstand löschen, der nicht mit `test-` beginnt.

## Beispiele

Ein Test-Raum wird verlassen und bleibt 10 min leer → seine Datei `saves/test-….json` ist weg.

## Ausnahme- und Fehlerfälle

Ein echter Spielstand heißt zufällig `test-familie` → er wäre betroffen; deshalb Präfix nur für Räume, die per Testseite entstehen? (Entscheidung offen)

## Akzeptanzkriterien

- **AC-01** Nach dem Aufräumen eines leeren Test-Raums liegt keine Datei `test-*.json` mehr in `saves/`, andere Spielstände bleiben (Go-Test).

## Offene Fragen

Wie wird ein Test-Spielstand erkannt (Präfix, eigenes Flag im Protokoll)? Wann wird gelöscht? (🧑)

## Notizen

Gefunden beim Review T1.2.
