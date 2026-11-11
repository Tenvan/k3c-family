# B-754 · Tick p99 online bei 4 Geräten liegt über dem RS2.3-Wert

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-745/AC-01 verlangt bei `sim_test` online `perf` (1 Raum × 4 Geräte, 5 min) Tick p99 nicht schlechter als RS2.3 (run-28: 2,5 ms) und Abbruch bei > 5 ms. Am 2026-11-08 mit RS2-Stand (frisch gebautes `bin/k3c-sim`, frisch gestarteter Spielserver, CPU-Last vorher gering): run-29 Tick p99 max 6,8 ms, Wiederholung run-30 3,2 ms. Mit 2 Geräten, 10 min (run-31): 1,6 ms. Keine Trennungen, keine Fehler. Ungeprüft, ob echte Regression oder Messrauschen: der Wert ist das Maximum über 60 Proben und reagiert auf einzelne OS-Ausreißer (vgl. RS1.3).

## Ziel

Klarheit, ob der Raum-Zustand nach RS2 (B-731, `engine/net/state.go`) den Tick p99 verschlechtert; falls ja, wieder auf dem RS2.3-Niveau.

## Beteiligte und Zielgruppen

Claude misst; Ralf (🧑) entscheidet, ob ein Max-über-Proben-Wert das richtige Abnahmemaß ist.

## Anforderungen

- Vergleich mit gleicher Last: Stand RS2.3 gegen aktuellen Stand, je ein Lauf (höchstens drei Läufe insgesamt).
- Neben dem Maximum auch Median der Proben-p99 nennen.

## Nicht-Ziele

Pi-Messung (RS2.5).

## Regeln und Einschränkungen

Läufe nur über `sim_test` (B-348), nie parallel zu anderen Läufen.

## Beispiele

online perf 1 × 4, 5 min → Tick p99 max ≤ 2,5 ms (oder begründet als Rauschen eingeordnet).

## Ausnahme- und Fehlerfälle

Einzelner Ausreißer durch das OS → im Bericht als solcher erkennbar (Median vs. Maximum).

## Akzeptanzkriterien

- **AC-01** Vergleichslauf RS2.3-Stand gegen aktuellen Stand bei gleicher Last, Werte in den Notizen; Regression ja/nein begründet.
- **AC-02** Bei Regression: Tick p99 max ≤ 2,5 ms im online perf-Lauf 1 × 4, 5 min.

## Offene Fragen

Soll `sim_test` neben dem Maximum den Median der Proben-p99 ausgeben? (🧑)

## Notizen

Gefunden in B-745/AC-01: run-29 6,8 ms, run-30 3,2 ms, run-31 (2 Geräte, 10 min) 1,6 ms; Referenz run-28 2,5 ms.
