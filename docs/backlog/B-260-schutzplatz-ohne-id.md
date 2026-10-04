# B-260 · Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei Gefahr läuft eine Truppe ohne Auftrag zu `w.HubX + t.ID%5 - 2` (`engine/sim/units.go`, `stepTroop`). Jede neue Entity vor den Truppen (z. B. ein neuer Bauplatz in `data/hub.json`) verschiebt alle IDs und damit die Schutzplätze. In W0.1 hat das sieben neue Plätze zu Positions-, Kampf- und Münz-Änderungen in neun `sim-*.json` gemacht, obwohl sich keine Regel für Truppen geändert hat.

## Ziel

Neue Bauplätze und andere Entities ändern Golden-Läufe nur über ihre eigene Wirkung, nicht über verschobene IDs; Golden-Diffs der Sprints W0 bis W4 bleiben lesbar.

## Beteiligte und Zielgruppen

SIM (Golden-Prüfung in Review-Sessions); Spieler merken nichts.

## Anforderungen

- Schutzplatz deterministisch aus einer Größe, die nicht von der globalen ID-Vergabe abhängt (z. B. Index unter den Truppen), weiterhin über 5 Plätze gestreut.

## Nicht-Ziele

Andere Truppen-Verhalten, Posten der Bogenschützen.

## Regeln und Einschränkungen

Deterministisch, nur `engine/rng`; Golden nach `docs/arbeitsweise.md` › „Golden aktualisieren“ (einmalige Änderung mit Begründung).

## Beispiele

Ein neuer Bauplatz ohne Wirkung → `sim-*.json` ändern sich nur in IDs und dem neuen Platz.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Ableitung einer Position.

## Akzeptanzkriterien

- **AC-01** Ein zusätzlicher unbezahlter Eintrag in `hub.json` › `sites` ändert in den Golden-Läufen nur IDs und den neuen Platz (Test oder Nachweis im Diff).

## Offene Fragen

keine; ungeprüft, ob weitere ID-abhängige Stellen (Sortierung nach ID) dieselbe Wirkung haben.

## Notizen

Gefunden in W0.1 beim Abgleich der Golden-Daten ohne IDs und ohne neue Plätze.
