# B-272 · Die Rotation in reports/ erfasst auch die Spielmetrik-Reports

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** ST1
- **Projekt:** LST
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Rotation aus F4.2 (`engine/store/reports.go`, `prune` und `Count`) arbeitet nur auf `reportGlob = "gamepad-*.json"`. Die geplanten Spielmetrik-Reports `session-*.json` (B-150, S2.3) würden auf der SD-Karte des Pi unbegrenzt wachsen. Aufgefallen bei Schritt 1 von S2.3 (2026-10-04).

## Ziel

`reports/` bleibt auch mit Spielmetrik-Reports in der Größe begrenzt.

## Beteiligte und Zielgruppen

Betreiber des Pi, Entwickler (Server).

## Anforderungen

- Ältere `session-*.json` werden wie Gamepad-Berichte begrenzt (eigene Obergrenze oder gemeinsame), Gamepad-Berichte verdrängen keine Sessions und umgekehrt.

## Nicht-Ziele

Backup außerhalb des Pi (B-142).

## Regeln und Einschränkungen

Nur `engine/store/`; Löschen nur nach Zeitstempel im Dateinamen, andere Dateien bleiben unberührt.

## Beispiele

101 Raumläufe → höchstens die Obergrenze an `session-*.json` liegt in `reports/`, die neuesten bleiben.

## Ausnahme- und Fehlerfälle

Löschen scheitert → Log, der neue Report wird trotzdem geschrieben.

## Akzeptanzkriterien

- **AC-01** Ein Test in `engine/store` belegt die Grenze für `session-*.json`, ohne Gamepad-Berichte zu löschen.

## Offene Fragen

keine

## Notizen

–
