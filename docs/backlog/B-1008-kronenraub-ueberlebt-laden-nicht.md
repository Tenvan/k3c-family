# B-1008 · Ein laufender Kronenraub bleibt nach Verlassen und Weiter bestehen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beobachtet in NA1.2 (Diagnoselauf e2e, 2026-11-15): Trägt ein Gegner beim „Spiel verlassen“ gerade eine Krone (`crownTaken` im Takt 15608), ist sie nach „Weiter“ wieder beim Monarchen: Spieler 2 tritt mit Krone und 0 Gold neu bei, nach dem Laden (Takt 15917) gibt es keine gefallene oder getragene Krone mehr. Fundort vermutet: `engine/sim/island_save*.go` und das Wiedereintreten in `engine/room`.

## Ziel

Ein Kronenraub lässt sich nicht per Verlassen und Weiter abbrechen: Lage der Krone (am Boden, getragen, geraubt) übersteht Speichern und Laden.

## Beteiligte und Zielgruppen

Spieler (Koop 1–2). Entscheidet: 🧑.

## Anforderungen

- Spielstand trägt die Krone eines Monarchen in jedem Zustand nach `docs/rules/kampf.md` § 6.
- Trägt ein Gegner die Krone beim Speichern, liegt sie nach dem Laden am Ort des Trägers am Boden (Gegner sind flüchtig, B-814); der Monarch bleibt ohne Krone.
- Mit 2 Spielern, deterministisch.

## Nicht-Ziele

Gegner im Spielstand speichern.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Pendants in `rust/crates/k3c-sim` (Insel-Speicherung, Krone) und `rust/crates/k3c-room` (Entscheidung 013); die Go-Engine bekommt keine Änderung mehr.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** spielstand

## Beispiele

Gegner trägt die Krone von Spieler 2, Spiel verlassen, Weiter → Krone liegt am Boden, Spieler 2 muss sie aufheben.

## Ausnahme- und Fehlerfälle

Krone schon im Portal (geraubt) → Niederlage steht fest, Laden ändert nichts.

## Akzeptanzkriterien

- **AC-01** Test in `engine/sim` bzw. `engine/room`: Speichern mit getragener Krone, Laden → Krone am Boden, Monarch ohne Krone.
- **AC-02** e2e `TestFlow` bleibt grün.

## Offene Fragen

Krone nach dem Laden am Boden (Default) oder Träger samt Krone speichern? Entscheidet 🧑.

## Notizen

Gemeldet vom Agenten NA1.2; Verhalten nicht von Claude nachgestellt (ungeprüft).
