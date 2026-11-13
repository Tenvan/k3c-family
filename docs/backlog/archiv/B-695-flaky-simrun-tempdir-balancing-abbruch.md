# B-695 · Zwei Tests flackern: simrun-TempDir-Cleanup und Abbruch eines wartenden Laufs

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ42
- **Projekt:** WZG
- **Erstellt:** 2026-11-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Beobachtet am 2026-11-02 bei `task check:diff` unter Last (parallele Agents):
1. `tools/k3c-workbench/internal/simrun` › `TestLaufIDNachNeustartHinterHoechstemOrdner`: `TempDir RemoveAll cleanup: …\reports: The directory is not empty.` (zweimal, einzeln grün). Vermutet: ein Kindprozess oder Schreiber hält `reports/` beim Cleanup noch offen.
2. Workbench-Frontend `src/balancing/BalancingPage.test.tsx` › „bricht einen wartenden Lauf ab“: kein Lauf im Zustand „wartet“ gefunden (einmal, danach grün). Vermutet: Timing, der Lauf startet schon.
3. Studio `ChunkCount.test.tsx` fiel unter Last einmal (B-690/WZ37.3 gemeldet).

## Ziel

Die Tests laufen auch unter Last stabil.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `TestLaufIDNachNeustartHinterHoechstemOrdner` (`internal/simrun`) räumt sein TempDir unter Windows sicher auf (Schreiber beendet bzw. abgewartet).
- Der Frontend-Test „bricht einen wartenden Lauf ab“ (`frontend/src/balancing/BalancingPage.test.tsx`) stellt den Zustand „wartet“ deterministisch her.
- Punkt 3 (Studio `ChunkCount`) entfällt: das Asset-Studio ist entfernt (Entscheidung 005, WB1).
- Kein Sleep als Behebung.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Domäne DEV, Klasse A (Problem). Nur Tests und Test-Hilfen; Sim-Läufe bleiben gesperrt.

- **Bereich:** `tools/k3c-workbench/internal/simrun/**`, `tools/k3c-workbench/frontend/src/balancing/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Je Test Ursache geklärt und behoben (Warten auf Ende der Schreiber, deterministische Reihenfolge), 20 Wiederholungen grün.
- **AC-02** `task check:diff` grün.

## Offene Fragen

keine

## Notizen

Weiter beobachtet 2026-11-02: Studio `src/world/WorldDraft.test.tsx:141` fiel einmal in `check:diff`, die ganze Studio-Suite danach grün (876/876).
