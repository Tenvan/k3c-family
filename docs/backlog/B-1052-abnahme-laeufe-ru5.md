# B-1052 · Abnahme-Läufe RU5

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU5 (Rig in Rust) ist reviewt; Gen- und Race-Läufe sind nach Regel B-742 ausgelagert.

## Ziel

Die langen Läufe für RU5 grün nachweisen.

## Beteiligte und Zielgruppen

Claude arbeitet autonom ab (nachts).

## Anforderungen

- Läufe nach `docs/arbeitsweise.md` › Sim, Balance und Gen.

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Rot heißt neues Ticket (Typ Problem), nicht Nachbessern hier.

- **Bereich:** –
- **Abhängig von:** RU5
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

nicht relevant: Abnahme-Ticket.

## Akzeptanzkriterien

- **AC-01** Sim: nicht relevant, RU5 berührt keine Simulation.
- **AC-02** Balance: nicht relevant, keine Werte geändert.
- **AC-03** Gen: nicht relevant, keine Grafik-Specs geändert.
- **AC-04** Race: `check_run task:check:race`, grün (ohne C-Compiler übersprungen, im Ergebnis so nennen); zusätzlich `cargo test` für `k3c-rig` in `rust/`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Gen und Race sind auf dem Hauptrechner gesperrt).
