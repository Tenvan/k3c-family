# B-803 · Ein Raum bietet seine Absturzbehandlung und den Grund abgelehnter Skill-Befehle nach außen an

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das native Spiel taktet einen Raum im selben Prozess (`native/k3c-game/local`, NG1.2). Die Absturzbehandlung des Raums (`safeTick`, `crash` in `engine/room`) ist nicht exportiert; `local` fängt einen Panic im Takt deshalb selbst ab und markiert den Raum nicht als geschlossen. Eine schon laufende Hintergrund-Speicherung mit einem Stand von vor dem Absturz kann danach noch fertig schreiben (vermutet harmlos). `Room.Learn` und `Room.Respec` liefern nur `ErrBadRequest`, der Grund aus der Simulation geht verloren; `local` meldet deshalb allgemeine Texte.

## Ziel

Ein Aufrufer im selben Prozess nutzt dieselbe Absturzbehandlung wie der Server und kann dem Spieler den Grund eines abgelehnten Skill-Befehls nennen.

## Beteiligte und Zielgruppen

Claude (Umsetzung), Spieler des nativen Spiels (Meldungen).

## Anforderungen

- Exportierter Weg für den abgesicherten Takt (z. B. `Room.SafeTick` oder `Manager.Crash`).
- `Learn` und `Respec` geben den Grund der Ablehnung zurück.
- `local` nutzt beides.

## Nicht-Ziele

Änderung am Protokoll oder am Verhalten des Servers.

## Regeln und Einschränkungen

Schichtgrenzen (`engine/room` kennt kein Netz), Golden-Dateien bleiben unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `engine/room/**`, `native/k3c-game/local/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Skill ohne Lernpunkt lernen → Meldung „kein Lernpunkt“ statt „Befehl abgelehnt“.

## Ausnahme- und Fehlerfälle

Panic im Takt → Raum gilt als geschlossen, kein späteres Speichern eines alten Stands.

## Akzeptanzkriterien

- **AC-01** Test: Panic im Takt über den exportierten Weg schließt den Raum, danach schreibt keine Speicherung mehr.
- **AC-02** Test: abgelehntes `Learn` nennt den Grund.

## Offene Fragen

keine

## Notizen

Gefunden in NG1.2 und NG1.6 (2026-11-11).
