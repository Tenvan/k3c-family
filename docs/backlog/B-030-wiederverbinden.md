# B-030 · Geräte verbinden sich nach Abbruch wieder

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP07
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit SP02)

## Ausgangslage

Bricht die Verbindung ab, ist das Gerät heute aus dem Spiel; leere Räume bleiben bestehen.

## Ziel

Geräte verbinden sich nach Abbruch wieder. Nutzen: WLAN-Aussetzer am Handy dürfen kein Spiel beenden.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Ein Gerät kommt binnen 60 s nach Abbruch mit seiner Geräte-ID und denselben Spielern zurück; bis dahin stehen
  seine Monarchen still.
- Leere Räume werden nach 10 min aufgeräumt.
- Grenzen: höchstens 4 Monarchen pro Raum, 4 lokale Spieler pro Gerät, 4 Räume pro Server.

## Nicht-Ziele

Lobby (B-037).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Protokoll v2 (SP02, Entscheidung 002).

## Beispiele

Ein Handy verliert 10 s WLAN → es verbindet sich neu und steuert denselben Monarchen.

## Ausnahme- und Fehlerfälle

Ein Gerät kommt nach Ablauf der 60 s zurück → seine Plätze sind frei; es tritt neu bei und übernimmt freie Monarchen.

## Akzeptanzkriterien

- **AC-01** Test: ein Gerät verbindet sich nach Abbruch mit denselben Spielern neu.
- **AC-02** Test: leere Räume verschwinden nach der Frist.
- **AC-03** Test: die Grenzen für Räume und Geräte greifen.

## Offene Fragen

keine (entschieden von 🧑 am 2026-09-30, SP02)

## Notizen

Entwurf in SP02, Umsetzung in SP07.
