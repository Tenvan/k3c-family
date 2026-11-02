# B-690 · Studio-Welt: Testlücken Klick-Schwelle, Bühnen-Verdrahtung, Werkzeugleiste, Merge-Dialog

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ39
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Gefunden in WZ38.1: Verhalten ist im Code, aber nicht per Test belegt. (1) `CLICK_SLOP` in `stageLogic.ts:216` hat keinen Vitest (B-667/AC-02); Vorschlag `isClick(dx)` als reine Funktion auslagern. (2) Weltmodus-Bühne: `focusX`/`setMarks` in `LivePreview.tsx:97,99` ohne Attrappen-Test (B-666/AC-04); Verdrahtung `onSite` → `pickKeys` → `setSel`/`onPickSite` (B-667). (3) Lage der Werkzeugleiste (B-665/AC-01). (4) Reiner Merge-Dialog der Git-Seite (B-580/AC-04).

## Ziel

Die vier Verhaltensweisen sind per Vitest belegt.

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

- **AC-01** Test auf die Klick/Pan-Schwelle.
- **AC-02** Test mit Bühnen-Attrappe für `focusX`, `setMarks` und die `onSite`-Verdrahtung im Weltmodus.
- **AC-03** Test zur Lage der Werkzeugleiste.
- **AC-04** Test für den Merge-Dialog der Git-Seite.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
