# B-695 · Zwei Tests flackern: simrun-TempDir-Cleanup und Abbruch eines wartenden Laufs

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

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
