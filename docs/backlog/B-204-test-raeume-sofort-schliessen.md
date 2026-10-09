# B-204 · Leere Test-Räume schließen sofort statt nach der Leer-Frist

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SV1
- **Projekt:** WRT
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein Raum mit Präfix `test-` (`engine/room/manager.go` › `TestPrefix`) bleibt wie jeder Raum nach dem letzten Gerät `EmptyFor` = 10 min offen und zählt so lange gegen die Grenze von 4 Räumen (`room.MaxRooms`). Das Lasttest-Werkzeug `cmd/k3c-load` (LT1.1) trennt am Ende alle Bots; ein zweiter Lauf mit 2 Räumen direkt danach scheitert deshalb an `too_many_rooms`, und `/api/status` listet die leeren Test-Räume noch 10 min.

## Ziel

Messläufe lassen sich direkt hintereinander starten, und nach einem Lauf ist sofort kein Test-Raum mehr offen.

## Beteiligte und Zielgruppen

🧑 und Agenten bei Messläufen (B-175), Testseite (B-086).

## Anforderungen

- Ein leerer Raum mit Präfix `test-` wird beim nächsten Sweep aufgeräumt (Spielstand gelöscht wie heute), nicht erst nach `EmptyFor`.
- Wartende Monarchen (Abbruch, 60 s) halten einen Test-Raum weiter offen, damit die Testseite ein Wiederverbinden prüfen kann.

## Nicht-Ziele

Andere Fristen für normale Räume; höhere Raum-Grenze.

## Regeln und Einschränkungen

Domäne SRV (`engine/room`), Komplexitäts-Budget, Protokoll unverändert.

## Beispiele

Zwei Läufe `k3c-load -rooms 2` nacheinander → der zweite startet ohne `too_many_rooms`.

## Ausnahme- und Fehlerfälle

Ein Test-Raum mit wartendem Gerät → bleibt bis zum Ende der Wartefrist offen.

## Akzeptanzkriterien

- **AC-01** Test: Ein Test-Raum ohne verbundenes oder wartendes Gerät ist nach dem nächsten Sweep weg, sein Spielstand gelöscht; ein normaler Raum bleibt `EmptyFor` offen.

## Offene Fragen

Braucht die Testseite (B-086) den leeren Test-Raum länger als bis zum nächsten Sweep? Entscheidet 🧑. Vermutet nein (2026-10-06); SV1.1 prüft es.

## Notizen

Anlass: LT1.1 (Sprint LT1). Der Test `cmd/k3c-load/load_test.go` überspringt die Frist heute mit der Uhr des Managers.
