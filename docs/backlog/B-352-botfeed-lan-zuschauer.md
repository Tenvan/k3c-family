# B-352 · `sim_test` hängt sich an Clients auf der Xbox an, ohne einen Platz im Raum zu belegen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`sim_test` mit `clients` (TR1.3) liest den Zustand für seine Bots über ein Beobachter-Gerät, das einen der 4 Plätze im Raum belegt und selbst als Bot mitspielt; das Protokoll kennt kein Gerät ohne Platz (`room.ValidSlots` lehnt leere `slots` ab). Daher gehen mit Clients nur `players` 1–3. Der Bot-Feed (`/bot/<lauf>/<n>` an k3c-dev) nimmt nur Loopback an; `attach` geht damit nur mit Clients auf demselben Rechner, nicht mit der Xbox.

## Ziel

Testläufe mit 4 lokalen Spielern und mit Clients auf der Xbox, ohne das Spiel durch einen zusätzlichen Monarchen zu verändern.

## Beteiligte und Zielgruppen

Agenten und 🧑 bei Testläufen; Protokoll betrifft Client und Server (eigene Session).

## Anforderungen

- Ein Zuschauer-Gerät (Beitritt ohne Platz) bekommt die Zustände wie ein Spieler, ohne Monarchen.
- Der Bot-Feed ist für Geräte im Heimnetz erreichbar, nur während eines Laufs und nur für den Raum des Laufs.

## Nicht-Ziele

Bot-Entscheidung im Client (B-349).

## Regeln und Einschränkungen

Protokoll-Änderung als eigene Session (docs/arbeitsweise.md › Grenzfälle). Firewall-Abfrage auf dem PC vermeiden (Workbench lauscht heute nur auf 127.0.0.1).

## Beispiele

`sim_test {action: start, mode: online, clients: 1, attach: ABCD}` mit der Xbox im Raum ABCD → Bots steuern deren Monarchen.

## Ausnahme- und Fehlerfälle

Feed von einer fremden Adresse ohne laufenden Lauf → abgelehnt.

## Akzeptanzkriterien

- **AC-01** Test: Ein Zuschauer tritt ohne Platz bei und bekommt Zustände; die Zahl der Monarchen bleibt gleich.
- **AC-02** Test: `sim_test` mit `clients` und `players: 4` läuft.

## Offene Fragen

Wie der Feed im LAN abgesichert wird (Token je Lauf?) und ob die Firewall-Abfrage hinnehmbar ist, entscheidet 🧑.

## Notizen

Gefunden in TR1.3 (2026-10-07).
