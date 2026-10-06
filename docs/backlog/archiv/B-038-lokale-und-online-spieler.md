# B-038 · Lokale und Online-Spieler teilen sich einen Raum

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP07
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit SP02); 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Heute steuert jedes Gerät genau einen Monarchen (`src/online/`).

## Ziel

Lokale und Online-Spieler teilen sich einen Raum. Nutzen: Kern-Spielprinzip laut Entscheidung 001.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Ein Gerät tritt mit N lokalen Spielern bei (z. B. Xbox mit 2 Controllern), andere Geräte online.

## Nicht-Ziele

Layout für mehr als 2 lokale Spieler (B-016).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Kern-Spielprinzip laut Entscheidung 001; Protokoll v2.

## Beispiele

Xbox mit 2 Controllern und ein Handy im selben Raum → drei Monarchen, jeder eigen gesteuert.

## Ausnahme- und Fehlerfälle

Ein lokaler Spieler verlässt das Spiel → die anderen Spieler desselben Geräts bleiben.

## Akzeptanzkriterien

- **AC-01** Test: Gerät mit 2 lokalen Spielern und Gerät mit 1 Spieler im selben Raum.
- **AC-02** Jeder steuert im Test seinen eigenen Monarchen.

## Offene Fragen

keine

## Notizen

Entwurf SP02, Server SP07, Client SP08.

Erledigt in SP07 (2026-10-01).
