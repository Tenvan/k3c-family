# B-603 · Dev-Aktionen lösen Boss, Phasenwechsel und Events (Vollmond, Blutmond, Händler) sofort aus

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KB4
- **Projekt:** KMP
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Die Dev-Aktionen (`dev`: gold, material, timescale, pause, castleFall, `grade`) erlauben es nicht, einen Boss oder ein Event auszulösen. Zur Abnahme K5.4 (Ralf, 2026-10-24) müsste man bis zur Bosswelle spielen.

## Ziel

Im Dev-Mode lösen Aktionen einen Boss, dessen Phasenwechsel und die Events sofort aus.

## Beteiligte und Zielgruppen

Ralf und der Debug-Panel-Nutzer; Agent SRV (Protokoll, Sim) und CLI (Panel).

## Anforderungen

- Neue Dev-Aktionen: `boss` (Boss-Id aus `data/bosses.json`, erscheint am nächsten Portal/Ausgang), `bossPhase` (nächste Phase), `event` (`fullMoon`, `bloodMoon`, `merchantRaid`) und `night` (Nacht sofort beginnen).
- Nur im Dev-Mode (`forbidden` sonst), Info „Dev-Aktion“ im Log, deterministisch (RNG-Strom aus `engine/rng`).
- Debug-Panel (K5.3): eine Gruppe „Kampf“ mit den Aktionen.
- `docs/protocol.md` ergänzt (additive Aktionen, Version bleibt), Beispiel-Dateien `c2s-dev-*.json`.

## Nicht-Ziele

Neue Bosse oder Events; Balancing.

## Regeln und Einschränkungen

Protokoländerung additiv, SRV-Session vor der CLI-Session; Golden unverändert (Dev-Aktionen sind nicht Teil der Läufe); Tests wie `dev_test.go`.

## Beispiele

Panel › Kampf › Boss „Eisenherr“ → Boss-Leiste erscheint im Spiel nach wenigen Sekunden.

## Ausnahme- und Fehlerfälle

Unbekannte Boss-Id → `bad_request`. Ohne Dev-Mode → `forbidden`. Boss schon da → Aktion wird abgelehnt mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Die Aktionen `boss`, `bossPhase`, `event` und `night` wirken in der Simulation und sind im Dev-Mode erlaubt, sonst abgelehnt (Go-Test).
- **AC-02** `docs/protocol.md` und Beispiel-Dateien sind aktuell.
- **AC-03** Das Debug-Panel hat die Gruppe „Kampf“ (Vitest).

## Offene Fragen

keine

## Notizen

Aus K5.4, Entscheidung 2026-10-24. Spec: Entwurf. Noch keinem Sprint zugeordnet. Zusammenhang: B-107, B-602, KB2.
