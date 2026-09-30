# B-030 · Geräte verbinden sich nach Abbruch wieder

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP07
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bricht die Verbindung ab, ist das Gerät heute aus dem Spiel; leere Räume bleiben bestehen.

## Ziel

Geräte verbinden sich nach Abbruch wieder. Nutzen: WLAN-Aussetzer am Handy dürfen kein Spiel beenden.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Ein Gerät kommt nach Abbruch mit denselben Spielern zurück.
- Leere Räume werden nach einer Frist aufgeräumt.
- Grenzen für Räume und Geräte.

## Nicht-Ziele

Lobby (B-037).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Protokoll v2 (SP02, Entscheidung 002).

## Beispiele

Ein Handy verliert 10 s WLAN → es verbindet sich neu und steuert denselben Monarchen.

## Ausnahme- und Fehlerfälle

Ein Gerät kommt nach Ablauf der Frist zurück → siehe Offene Fragen.

## Akzeptanzkriterien

- **AC-01** Test: ein Gerät verbindet sich nach Abbruch mit denselben Spielern neu.
- **AC-02** Test: leere Räume verschwinden nach der Frist.
- **AC-03** Test: die Grenzen für Räume und Geräte greifen.

## Offene Fragen

Frist, Grenzwerte und Verhalten nach Ablauf der Frist (SP02, 🧑).

## Notizen

Entwurf in SP02, Umsetzung in SP07.
