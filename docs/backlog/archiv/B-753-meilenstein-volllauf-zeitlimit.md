# B-753 · Meilenstein-Volllauf über k3c-sim überschreitet das 60-min-Limit

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Meilenstein-Lauf (`sim_test` offline, `bots: coop2`, `world: standard`, `days: 40`, 101 Seeds) rechnet seit RS2 über den Kindprozess `bin/k3c-sim` (B-726). In B-745/AC-02 brach run-33 (2026-11-08) nach 1 h 0 min 2 s mit „rechnen: Zeitlimit erreicht“ ab, ohne Bericht. Vor RS2 dauerte derselbe Lauf 38 min 41 s (RS1.1, Last 68 %); 1 Seed lag in RS2.2 bei 48 s (vorher 52 s). Ursache ungeprüft; vermutet: Parallelität der Läufe in `bin/k3c-sim` (`engine/balance/parallel.go`, `workers = runtime.NumCPU()`) greift im Kindprozess nicht wie im Workbench-Prozess, oder Datenänderungen seit RS1.1 verlängern die Läufe.

## Ziel

Der Meilenstein-Volllauf läuft über `bin/k3c-sim` wieder innerhalb des Zeitlimits und höchstens 5 % langsamer als vor RS2 (B-726/AC-03).

## Beteiligte und Zielgruppen

Claude misst und behebt; Ralf (🧑) liest das Ergebnis.

## Anforderungen

- Ursache gemessen, nicht geraten (CPU-Auslastung des Kindprozesses, Läufe je Minute, Vergleich 1 Seed / 10 Seeds vor und nach RS2).
- Ergebnisse bleiben gleich (deterministisch, Bericht-Diff leer bis auf Zeitstempel).

## Nicht-Ziele

Balancing-Werte (B-698).

## Regeln und Einschränkungen

Läufe nur über `sim_test` (B-348), nie parallel zu anderen Läufen; Entscheidung 006.

## Beispiele

Meilenstein-Lauf 101 Seeds auf dem PC ohne parallele Agenten → fertig in ≤ 40 min 37 s (38 min 41 s + 5 %).

## Ausnahme- und Fehlerfälle

Lauf überschreitet das Zeitlimit → Bericht mit den bis dahin fertigen Seeds statt leerem Bericht (prüfen, ob gewollt).

## Akzeptanzkriterien

- **AC-01** `sim_test` Meilenstein-Lauf (coop2, standard, 40 Tage, 101 Seeds) endet ohne Zeitlimit, Dauer ≤ +5 % gegenüber 38 min 41 s (oder gegenüber einem Vergleichslauf mit dem Stand vor RS2 bei gleicher Last).
- **AC-02** Ursache in den Notizen benannt, mit Messwert.

## Offene Fragen

keine

## Notizen

Gefunden in B-745/AC-02 (run-33, 2026-11-08, CPU-Last vor dem Start gering). BAS-Standardlauf run-32 (101 Seeds) lief in 7 min 39 s durch.
