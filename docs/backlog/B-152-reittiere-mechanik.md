# B-152 · Reittiere haben eine Mechanik, bisher gibt es nur die Grafiken

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Reittiere sind nur als Grafik vorhanden (Sattelpunkte in `data/sprites.json`, Referenz in `src/tools/spriteReference.ts`); in `engine/sim/` gibt es weder Aufsteigen noch Reiten noch Kosten. `docs/game-design.md` und `docs/roadmap.md` erwähnen sie ohne Regel.

## Ziel

Ein Monarch kann ein Reittier besitzen und reiten, mit beschlossenen Zahlen. Nutzen: Fortbewegung über die breiten Stufen bekommt eine Belohnung (Entscheidung durch 🧑, Q23).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); 🧑 entscheidet, ob und wie (`docs/fragenkatalog.md Q23`).

## Anforderungen

- Regel mit Zahlen in `docs/rules/` (Kosten, Geschwindigkeitsfaktor, Aufsteigen und Absteigen, Verhalten bei Treffer und beim Wechsel der Stufe).
- Daten nur in `data/`, Logik in `engine/sim/`; deterministisch, mit 2+ Spielern gleichzeitig.
- Reittier-Zustand im Spielstand (Versionssprung und Fixture nach Migrationsregel, B-137).

## Nicht-Ziele

Grafik-Anbindung (GR3, B-010), Protokoll-Felder (eigenes Ticket bei Planung), Ton.

## Regeln und Einschränkungen

`CLAUDE.md` › Regeln: keine Sprünge, nur `engine/rng`, Werte in `data/`. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Kein Sprint geplant: erst nach Q23 und nach Phase 3.

## Beispiele

Monarch mit Reittier reitet über eine Stufe → Bewegung mit dem Faktor aus den Daten; Treffer wirft ihn nach Regel ab.

## Ausnahme- und Fehlerfälle

Zwei Spieler wollen dasselbe Reittier → die Regel legt fest, wer es bekommt (offen, Q23).

## Akzeptanzkriterien

- **AC-01** `docs/rules/` enthält einen Abschnitt „Reittiere“ mit Zahlen für Kosten, Geschwindigkeitsfaktor und Abwurf (Beschluss von 🧑).
- **AC-02** Test in `engine/sim/`: Aufsteigen und Absteigen ändern die Geschwindigkeit laut `data/`; zweimal mit demselben Seed gleiches Ergebnis.
- **AC-03** Reittier-Zustand steht im Spielstand, ein alter Stand lädt (Fixture); Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Ob es Reittiere geben soll und mit welchen Werten: `docs/fragenkatalog.md Q23` (🧑).

## Notizen

Aus dem Plan `k3c-weiterentwicklung` (Reittiere ohne Mechanik). Bewusst ohne Sprint.
