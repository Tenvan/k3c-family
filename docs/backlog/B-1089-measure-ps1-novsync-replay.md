# B-1089 · measure.ps1 gibt -Vsync an die Rust-Wiedergabe nicht weiter

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU45
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`native/measure.ps1` (Zeile ~117 und ~188-194) hängt `--novsync` nur an, wenn `-Vsync` fehlt, und überschreibt für `-Engine replay` die Argumentliste ganz (`replay <trace> --window --measure … --size`), ohne `--novsync`. Dazu schreibt Zeile 117 „Die Rust-Wiedergabe misst immer mit VSync“. Seit RU44.4 (B-1069 AC-02) versteht `k3c-game replay` `--novsync`; die Weitergabe aus dem Skript fehlt, also lässt sich die Rust-Wiedergabe über das Skript nicht ohne VSync messen (Bildrate-Gate G2).

## Ziel

`native/measure.ps1 -Engine replay` misst ohne `-Vsync` mit `--novsync` und mit `-Vsync` ohne diesen Schalter, wie Go (`game`).

## Beteiligte und Zielgruppen

Der Second Worker setzt um; Ralf misst damit (Gate G2).

## Anforderungen

- Für `replay` `--novsync` anhängen, wenn `-Vsync` nicht gesetzt ist.
- Hinweis „misst immer mit VSync“ (Zeile 117) und den Kommentar im Kopf (Zeile 14) und im Block `# Wiedergabe: …` anpassen.

## Nicht-Ziele

Änderungen an der Rust-Wiedergabe, `--size`, Frame-Auswertung (`Get-FrameStats`), die Go-Läufe.

## Regeln und Einschränkungen

PowerShell 5.1 und pwsh 7 müssen laufen (Kopfkommentar). Keine Messung ausführen (Sim, Balance, Gen gesperrt bzw. Ralfs Gate).

- **Bereich:** `native/measure.ps1`
- **Abhängig von:** RU44
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`-Engine replay` (ohne `-Vsync`): ArgumentList endet mit `--size 1920x1080 --novsync`. `-Engine replay -Vsync`: kein `--novsync`.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Skript-Änderung; Syntax wird mit dem PowerShell-Parser geprüft (`[System.Management.Automation.Language.Parser]::ParseFile`).

## Akzeptanzkriterien

- **AC-01** `native/measure.ps1` hängt für `replay` `--novsync` an, wenn `-Vsync` fehlt; mit `-Vsync` nicht.
- **AC-02** Der Hinweis und die Kommentare nennen nicht mehr „immer mit VSync“; das Skript parst fehlerfrei in PowerShell 5.1 und pwsh 7.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus RU44 (B-1069/AC-02 deckt nur Rust ab).
