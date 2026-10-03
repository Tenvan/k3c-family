# DBG1 · SRV · Dev-Aktionen: Gold, Material, Zeitraffer

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-178
- **Start-Commit:** f6bb3c3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-178 und die Vorschläge der Sessions (WebSocket-Nachricht nur im Dev-Mode, Domänen-Ausnahmen Client-Parser und engine/sim/dev.go)

## Ausgangslage

Der Dev-Mode kennt keine Aktionen im laufenden Raum (B-178 › Ausgangslage). Vorgezogen für Tests (🧑, 2026-10-03); steht in der SRV-Bahn vor F4.

## Ziel

Im Dev-Mode des Raums lassen sich Gold und Material droppen und die Zeit beschleunigen. Am Ende sichtbar: Go-Tests je Aktion, Protokoll-Beispiele, Log-Einträge.

## Beteiligte und Zielgruppen

🧑 und Entwickler beim Testen; Agenten setzen um; 🧑 nimmt am Gerät ab.

## Anforderungen

B-178 › Anforderungen.

## Nicht-Ziele

Bedienung im Client (DBG2), Stufenwechsel, Neustart (B-080), Wechsel des Grades (B-107).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/net/`, `docs/protocol.md`, `testdata/protocol/`). Die Protokolländerung enthält den Client-Parser (`src/online/protocol.ts`, `clientProtocol.ts`) als Domänen-Ausnahme dieser Spec, wie in `docs/arbeitsweise.md` › Protokoll vorgesehen.

## Beispiele

Dev-Raum, `dev gold 50` → Münzen am Spieler; `dev timescale 8` → der Raum läuft 8-fach.

## Ausnahme- und Fehlerfälle

Raum ohne Dev-Mode → `forbidden`; Zeitraffer über dem Tick-Budget → Server verlangsamt, rechnet korrekt weiter.

## Akzeptanzkriterien

- **AC-01** `dev` ohne Dev-Mode wird abgelehnt und im Log gewarnt (B-178/AC-01).
- **AC-02** Gold droppen funktioniert und ist aufhebbar (B-178/AC-02).
- **AC-03** Material erhöht den Insel-Vorrat, begrenzt durch das Lager-Maximum (B-178/AC-03).
- **AC-04** Der Zeitraffer ist deterministisch gleich N normalen Ticks (B-178/AC-04).
- **AC-05** Protokoll beschrieben, Beispiele vorhanden, beide Enden parsen (B-178/AC-05).
- **AC-06** Dev-Aktionen stehen im Log; `task check` und `task check:go` grün (B-178/AC-06).

## Offene Fragen

Siehe Ticket › Offene Fragen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| DBG1.1 | `DBG1.1-protokoll.md` | Umsetzung | autonom | fertig |
| DBG1.2 | `DBG1.2-gold-material.md` | Umsetzung | autonom | fertig |
| DBG1.3 | `DBG1.3-zeitraffer.md` | Umsetzung | autonom | offen |
| DBG1.4 | `DBG1.4-review.md` | Review | autonom | offen |

## Abnahme

–
