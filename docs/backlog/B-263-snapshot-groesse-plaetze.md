# B-263 · Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NT1
- **Projekt:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit W0 (B-206) legt jede Stufe 30 Linien-Plätze (5 Linien × Mauer, Turm, Tor je Seite) und sieben neue Hub-Plätze an. Der volle Welt-Snapshot wächst dadurch spürbar (Messung W0.3): `sim-forest-tag` im Mittel 9011 → 12096 Bytes (+34 %, 13 → 39 Plätze), frische Welt Seed 7 mit 1 Spieler 7641 → 11289 Bytes. Die meisten Plätze sind `unpaid` und ändern sich selten.

## Ziel

Volle Zustände und Deltas bleiben auch mit vielen Plätzen und mehreren Stufen im Netz- und Pi-Budget (LT1).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen, mehrere Räume), Betrieb auf dem Pi; SRV setzt um, 🧑 entscheidet bei Bedarf.

## Anforderungen

- Messung mit `task load` (LT1) vor und nach W0: Bytes je voller Zustand und je Delta, CPU je Tick.
- Falls über Budget: Plätze ohne Änderung nicht in jedem vollen Zustand senden (z. B. mit B-208 festlegen).

## Nicht-Ziele

Protokoll-Felder für Plätze selbst (B-208); Client-Anzeige (B-207, B-209).

## Regeln und Einschränkungen

Protokoll v3 nach `docs/protocol.md`; Änderungen am Protokoll nur mit Version oder als optionales Zusatzfeld.

## Beispiele

Raum mit 3 Spielern auf einer Insel mit 3 Stufen: Bytes je Sekunde gegen die LT1-Messung vom 2026-10-03.

## Ausnahme- und Fehlerfälle

nicht relevant: Messaufgabe.

## Akzeptanzkriterien

- **AC-01** Eine `task load`-Messung mit dem Stand nach W0 ist im Ticket vermerkt und mit LT1 verglichen; liegt sie über Budget, gibt es ein Folge-Ticket mit Maßnahme.

## Offene Fragen

Budget je Snapshot (W5/LT1): 🧑.

## Notizen

Angelegt in der Review-Session W0.4 (2026-10-04) aus dem Ergebnis von W0.3.
