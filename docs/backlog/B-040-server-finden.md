# B-040 · Geräte finden den Server im Heimnetz

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** BT1
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Handys müssen heute die IP-Adresse des Servers eintippen.

## Ziel

Geräte finden den Server im Heimnetz. Nutzen: Handys sollen ohne IP-Eintippen beitreten können.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Adresse oder QR-Code auf der Landingpage oder in der TUI.

## Nicht-Ziele

Zugriff von außerhalb des Heimnetzes.

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

QR-Code mit dem Handy scannen → das Spiel öffnet sich.

## Ausnahme- und Fehlerfälle

Server hat mehrere Netzwerkadressen → siehe Offene Fragen.

## Akzeptanzkriterien

- **AC-01** Ein QR-Code führt direkt ins Spiel.

## Offene Fragen

mDNS ja oder nein; QR auf der Landingpage oder in der TUI; welche Adresse bei mehreren Netzen? (🧑)

## Notizen

–
