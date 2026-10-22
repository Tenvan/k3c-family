# B-378 · Eine gehaltene Skill-Taste ohne Ziel meldet castFailed einmal je Tastendruck

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BD2
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-22, Ralf im Chat (Revision 1)

## Ausgangslage

Seit S9.1a (B-321) sendet `castSkill` (`engine/sim/skills.go`) `castFailed {from, slot, x}`, wenn ein bereiter Skill kein Ziel findet, ohne Abklingzeit. Hält ein Spieler die Taste, entsteht das Ereignis in jedem Tick (30 Hz), also bis zu 30 Ereignisse je Sekunde und Spieler im Netz. Der Client stapelt seit dem Review S9.4 keine Texte mehr (`src/scenes/effects/effectsView.ts` › `label`), das Netz-Budget (Q08, ≤ 200 Byte je Tick und Client) belastet es trotzdem. Ob `TestEventsBudgetJeTick` gehaltene Tasten abdeckt, ist ungeprüft.

## Ziel

Ein Druck auf eine Skill-Taste ohne Ziel ergibt genau ein `castFailed`, solange die Taste gehalten wird; ein neuer Druck meldet wieder.

## Beteiligte und Zielgruppen

Alle Spieler (Rückmeldung B-318), Netz-Budget; Agent baut in `engine/sim/`.

## Anforderungen

- `castFailed` nur bei der Flanke „Taste neu gedrückt“ (Slot ≠ Slot des Vor-Ticks), je Spieler und Slot; die Sim merkt den gehaltenen Slot in `Player` (nicht im Spielstand), weil `PlayerCommand.Skill` ein gehaltener Wert ohne Flanke ist.
- Nur das Ereignis wird gedrosselt; der Wirkungsversuch läuft weiter jeden Tick, findet der Skill beim Halten ein Ziel, wirkt er wie heute.
- Kein Einfluss auf Skills mit Ziel und auf die Abklingzeit.
- Deterministisch, mit 2+ Spielern getrennt.

## Nicht-Ziele

Abklingzeit für fehlgeschlagene Skills (B-321 › Nicht-Ziele), Client-Darstellung (S9.3), Protokoll.

## Regeln und Einschränkungen

SIM; Werte nur in `data/`; Datei ≤ 400 Zeilen, Funktion ≤ 60. Golden-Läufe ändern sich höchstens um weggefallene `castFailed`-Wiederholungen (Begründung im Ergebnis).

## Beispiele

Feuerball-Taste 1 s gehalten, kein Gegner in Reichweite → ein `castFailed`; Taste los und wieder gedrückt → ein zweites.

## Ausnahme- und Fehlerfälle

Findet der Skill während des Haltens ein Ziel, wirkt er wie heute; danach gilt die Abklingzeit.

## Akzeptanzkriterien

- **AC-01** Sim-Test: Skill-Taste 30 Ticks gehalten ohne Ziel → genau ein `castFailed`; nach Loslassen und neuem Druck ein zweites; zwei Spieler unabhängig.
- **AC-02** `TestEventsBudgetJeTick` oder ein neuer Test deckt gehaltene Skill-Tasten ohne Ziel ab; `task check:go` grün.

## Offene Fragen

keine (entschieden 2026-10-22 beim Bereitmachen von BD2: `PlayerCommand` kennt keine Flanke, die Sim merkt den Slot des Vor-Ticks; Ralf kann das bei der Freigabe ändern)

## Notizen

Aus dem Review S9.4 (2026-10-09, Befund gehaltene Taste), auf Wunsch von 🧑 angelegt. Nummer von Hand (k3c-dev-Build lehnt Worktree-Planung ab, B-372).
