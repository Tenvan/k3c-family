# W9 · SIM · Welt spiegelt Lager-Maximum, Hub-Ausbau und Gefahr

- **Status:** geplant
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-323
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 1

## Ausgangslage

W5.1 ist blockiert: Lager-Maximum, Hub-Ausbau (Stufe, Kosten der nächsten Stufe, bezahltes Gold, Zustand) und der Wartegrund „Gefahr“ stehen nur in privaten Feldern und Funktionen von `engine/sim/`, W5.1 (SRV) darf sie nicht anlegen (B-323). Entscheidung 🧑 2026-10-06: eigene SIM-Session vor W5.1 (B-323 › Offene Fragen, Weg 1).

## Ziel

W5.1 kann alle Wirtschaftswerte ohne eigene Regel-Rechnung aus `*sim.World` übernehmen.

Am Ende sichtbar: ein Go-Test belegt die Werte im JSON der Welt, `task check:go` grün.

## Beteiligte und Zielgruppen

Entwickler SIM (Umsetzung), danach SRV (W5.1); 🧑 gibt die Spec frei.

## Anforderungen

B-323 › Anforderungen.

## Nicht-Ziele

Protokoll, Beispiele und Client-Typen (W5.1, B-153); Anzeige (W6); neue Regeln oder Werte.

## Regeln und Einschränkungen

Domäne SIM, nur `engine/sim/` und Golden-Daten. Die Regel (Lager-Maximum, Ausbau, Gefahr) bleibt an genau einer Stelle; das Spiegeln liest sie nur ab. Deterministisch, Golden-Daten nur nach dem Ablauf in `docs/arbeitsweise.md` › Golden aktualisieren. Einschiebbar, damit W5.1 nicht auf die SIM-Bahn warten muss.

## Beispiele

B-323 › Beispiele.

## Ausnahme- und Fehlerfälle

B-323 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test in `engine/sim/` belegt, dass `json.Marshal(world)` (oder eine exportierte Funktion) Lager-Maximum, Hub-Ausbau mit Kosten und den Wartegrund „Gefahr“ liefert (B-323/AC-01).
- **AC-02** Golden-Daten sind nur um die neuen Felder gewachsen (Begründung im Commit), `task check` und `task check:go` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W9.1 | `W9.1-welt-spiegeln.md` | Umsetzung | autonom | offen |
| W9.2 | `W9.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
