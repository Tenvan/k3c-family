# B-086 · Test-Spielstände der Testseite bleiben nicht liegen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („mach B-085 und B-086“) (Revision 2, Entscheidung zur Löschung 🧑 ebenda)

## Ausgangslage

Die Testseite (T1, B-081) startet jede Session mit einem neuen Spielstand `test-<Zeit>`. Der Raum speichert beim Beenden (`docs/protocol.md` › *Spielstand*),
daher bleibt je Klick eine Datei `saves/test-….json` liegen (`saves/` ist nicht eingecheckt). Nach dem Browser-Lauf zu SP08 und T1 lagen dort vier solcher Dateien.

## Ziel

Test-Spielstände sammeln sich nicht an. Nutzen: Die Spielstandliste bleibt übersichtlich, sobald Spielstände auswählbar sind (B-037/AC-04).

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 betreibt den Server.

## Anforderungen

- Das Präfix `test-` ist für Testläufe reserviert. Ein leerer Raum mit so benanntem Spielstand löscht ihn samt Sicherungen, wenn er nach 10 min aufgeräumt wird (`room.TestPrefix`, `Saves.Delete`).
- Beim Serverstart löscht der Server außerdem `test-*`-Spielstände, die seit mehr als 24 Stunden nicht geschrieben wurden (`Saves.Purge`), für Reste nach Absturz oder Neustart.

## Nicht-Ziele

Löschen anderer Spielstände; Änderungen am Protokoll.

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/store/`); Protokoll v2 unverändert; nie einen Spielstand löschen, der nicht mit `test-` beginnt.

## Beispiele

Ein Test-Raum wird verlassen und bleibt 10 min leer → seine Datei `saves/test-….json` ist weg.

## Ausnahme- und Fehlerfälle

Ein echter Spielstand heißt `test-familie` → er wird wie ein Test-Spielstand gelöscht. Das ist so entschieden: das Präfix ist reserviert.

## Akzeptanzkriterien

- **AC-01** Nach dem Aufräumen eines leeren Test-Raums liegt keine Datei `test-*.json` mehr in `saves/`, andere Spielstände bleiben (Go-Test).

## Offene Fragen

keine (🧑 2026-10-01: Präfix, Löschen beim Aufräumen des Raums plus Aufräumen beim Start nach 24 h).

## Notizen

Gefunden beim Review T1.2. Umgesetzt in `engine/room` (`TestPrefix`, `dropTestSave`), `engine/store` (`Delete`, `Purge`), `cmd/k3c-server`; Go-Tests in `room_test.go` und `store_test.go`. Das Präfix ist nur hier und im Code dokumentiert, `docs/protocol.md` bleibt unverändert (Protokoll hat eine eigene Session).
