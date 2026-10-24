# B-573 · Nach dem Fall der Burg spielt ein eigenes Niederlage-Stück

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** SO7
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-555 (S2, KR4.1): Variante C. Niederlage und Sieg bekommen getrennte Zustände mit eigener Musik. Heute löst nur `castleFallen` den Zustand „Ende“ aus und die Niederlage bleibt still (`MUSIC_SLOTS.ending = null` in `src/audio/music.ts`); im Slot `music:ende` liegt nur ein Siegstück (8-Bit Victory Loop, Wolfgang_, CC0).

## Ziel

Nach dem Fall der Burg spielt ein eigenes, kindgerechtes Niederlage-Stück; das Siegstück bleibt dem Sieg vorbehalten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Kandidatensuche für ein Niederlage-Stück (nur CC0 oder CC-BY, Q15; Auswahl durch 🧑, Lizenz von Claude geprüft).
- Neuer Slot `music:niederlage` in `data/assets.json`.
- Der Musik-Automat trennt `victory` und `castleFallen`; ein Sieg-Auslöser liefert die Simulation erst mit K2/K4.
- Test für den Automaten; Credits in `public/audio/README.md`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Bei `castleFallen` spielt das gewählte Niederlage-Stück; das Siegstück läuft nicht bei Niederlage.
- **AC-02** Credit-Eintrag und Lizenz stehen in `public/audio/README.md`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-555 (KR4.1, Alternative C). Spec noch Entwurf; Hörprobe am TV durch 🧑 (SO4.5).
