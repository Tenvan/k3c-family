# KB4 · SRV, CLI · Dev-Aktionen für Boss, Phasenwechsel, Events und Nacht

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** SRV, CLI
- **Reife:** bereit
- **Tickets:** B-603
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Die Dev-Aktionen (`gold`, `material`, `timescale`, `pause`, `castleFall`, `grade`) lösen weder Boss noch Event aus; zur Abnahme K5.4 (Ralf, 2026-10-24) müsste man bis zur Bosswelle spielen (B-603).

## Ziel

Im Dev-Mode lösen Aktionen einen Boss, dessen Phasenwechsel, die Events Vollmond, Blutmond, Händler-Überfall und die Nacht sofort aus. Am Ende sichtbar: Debug-Panel › Kampf › Boss „Eisenherr“ → Boss-Leiste erscheint nach wenigen Sekunden.

## Beteiligte und Zielgruppen

Ralf und alle, die das Debug-Panel nutzen; Agenten SRV (Protokoll, Sim) und CLI (Panel); 🧑 nimmt am PC ab.

## Anforderungen

B-603 › Anforderungen.

## Nicht-Ziele

Neue Bosse oder Events; Balancing; Änderungen an den Golden-Läufen.

## Regeln und Einschränkungen

Protokolländerung additiv, Version bleibt; SRV-Session vor der CLI-Session; nur im Dev-Mode, sonst `forbidden`; deterministisch über `engine/rng`; Golden unverändert (Dev-Aktionen sind nicht Teil der Läufe); Tests wie `dev_test.go`; 2 Spieler.

## Beispiele

Panel › Kampf › Boss „Eisenherr“ → Boss erscheint am nächsten Portal/Ausgang. Panel › Kampf › Event „Blutmond“ → Blutmond-Banner und Faktor sofort.

## Ausnahme- und Fehlerfälle

Unbekannte Boss-Id → `bad_request`. Ohne Dev-Mode → `forbidden`. Boss schon da → abgelehnt mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Die Aktionen `boss`, `bossPhase`, `event` und `night` wirken in der Simulation und sind im Dev-Mode erlaubt, sonst abgelehnt (Go-Test) (B-603/AC-01).
- **AC-02** `docs/protocol.md` und Beispiel-Dateien `c2s-dev-*.json` sind aktuell (B-603/AC-02).
- **AC-03** Das Debug-Panel hat die Gruppe „Kampf“ (Vitest) (B-603/AC-03).
- **AC-04** 🧑 hat einen Boss und ein Event über das Panel am PC ausgelöst.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB4.1 | `KB4.1-dev-aktionen-server.md` | Umsetzung | autonom | fertig |
| KB4.2 | `KB4.2-debug-panel-kampf.md` | Umsetzung | autonom | fertig |
| KB4.3 | `KB4.3-review.md` | Review | autonom | offen |
| KB4.4 | `KB4.4-abnahme-pc.md` | Workshop | Mensch | offen |

- KB4.1 (SRV) Aktionen `boss`, `bossPhase`, `event`, `night` in Sim und Server, Go-Tests, `docs/protocol.md`, Beispiel-Dateien (AC-01, AC-02).
- KB4.2 (CLI) Debug-Panel Gruppe „Kampf“, Vitest (AC-03).
- KB4.3 Review: alle Kriterien.
- KB4.4 (Mensch) Abnahme am PC (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
