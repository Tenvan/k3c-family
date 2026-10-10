# B-386 · Netz-Budget des Kampf-Zustands: Q08 gilt für alles oder nur für Ereignisse?

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-154/AC-05 (Sprint K4, Abnahme K4.3) verlangt „Bytes je Tick in einer Bosswelle mit 4 Spielern … höchstens 200 Byte je Tick und Client (Q08)“. Gemessen in `docs/protocol.md` › Snapshot-Größe (`engine/net/kampf_bytes_test.go`): `snap` Ø 10,2 KB, `delta` Ø 913 B (p99 1 534 B), `events` Ø 2,9 B. K4.2 hat die Grenze auf das Ereignis-Budget (`TestEventsBudgetJeTick`) eingeschränkt, weil nur es beschlossen sei. Andere Tickets lesen Q08 als Netz-Budget je Tick und Client: B-208 („≤ 200 Byte je Tick und Client im Mittel“), B-378 („das Netz-Budget“).

## Ziel

🧑 entscheidet, was Q08 umfasst. Danach ist klar, ob der Kampf-Zustand (`snap`, `delta`) das Budget einhalten muss oder nur die Ereignisse.

## Beteiligte und Zielgruppen

🧑 entscheidet. Entwickler (Server, Client) setzen die Entscheidung um.

## Anforderungen

- Eine Entscheidung zu Q08 (`docs/fragenkatalog.md`), die AC-05 in B-154 eindeutig macht.

## Nicht-Ziele

Keine Änderung am Protokoll oder am Code, bevor 🧑 entschieden hat. Kein Umschreiben von AC-05 ohne neue Freigabe.

## Regeln und Einschränkungen

Spec-Änderungen nach `docs/arbeitsweise.md` › SDD: neue Revision, zurück auf `Entwurf`, neue Freigabe durch 🧑.

## Beispiele

Variante A: Q08 gilt für den ganzen Zustand → `delta` muss im Mittel unter 200 Byte je Tick und Client liegen (eigenes Ticket für die Kompression oder das Senden seltener Felder).
Variante B: Q08 gilt nur für Ereignisse → AC-05 in B-154 wird auf „Ereignisse ≤ 200 Byte je Tick und Client (Q08)“ geändert, der Zustand wird nur gemessen.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** 🧑 hat Q08 für den Kampf-Zustand entschieden (Variante A oder B), die Entscheidung steht in `docs/fragenkatalog.md`.

## Offene Fragen

- Gilt das Budget aus Q08 (≤ 200 Byte je Tick und Client) für den ganzen Zustand (`snap`, `delta`) oder nur für Ereignisse? Entscheidet 🧑.

## Notizen

- Messung K4.2 (Intel Core Ultra 7 165H, 2026-10-09): Bosswelle, 4 Spieler, Endboss Phase 2, 1800 Ticks; `delta` ≈ 27 KB/s je Client bei 30 Hz.
- Herkunft: Abnahme K4.3 (Sprint K4), Verweis aus B-154/AC-05.
