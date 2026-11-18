# B-1061 · Figuren und Gefechtsmarken tönen mit gerader statt vormultiplizierter Farbe

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU16
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Canvas-Shader erwartet eine gerade Tönung (rgb nicht mit alpha multipliziert). RU16.5 hat das in `props` korrigiert. In `actors` (RU13) wird an `rust/crates/k3c-game/src/actors/draw.rs:195` und `actors/combatmarks.rs:103` `[al, al, al, al]` als Tönung durchgereicht – dieselbe Verwechslung, Figuren und Marken werden bei Deckkraft < 1 zu dunkel (vermutet, Reviewer-Hinweis RU16.5).

## Ziel

Halbtransparente Figuren und Gefechtsmarken haben dieselbe Helligkeit wie im Go-Spiel.

## Beteiligte und Zielgruppen

RU16.6 setzt um, RU16.7 prüft.

## Anforderungen

Tönung an beiden Stellen als `[1, 1, 1, al]` (gerade Farbe) bzw. wie in `props/paint.rs`; Test mit `NullCanvas`/Aufzeichnung der Tönung.

## Nicht-Ziele

Änderungen am Shader.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/actors/draw.rs`, `rust/crates/k3c-game/src/actors/combatmarks.rs`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Figur mit Deckkraft 0,5 auf weißem Grund: Go-Pixel ≈ Rust-Pixel (Bildvergleich RU13.6).

## Ausnahme- und Fehlerfälle

–

## Akzeptanzkriterien

- **AC-01** Ein Test belegt die gerade Tönung an beiden Stellen; der Bildvergleich aus RU13.6 bleibt in seinen Schwellen.

## Offene Fragen

keine

## Notizen

Gefunden in RU16.5 (Commit 326256289). Klasse A (Problem, CLI, ohne Spielregel).
