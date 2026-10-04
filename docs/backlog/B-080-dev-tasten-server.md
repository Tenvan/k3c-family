# B-080 · Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** K4
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint K4

## Ausgangslage

Die Dev-Tasten in `src/scenes/GameScene.ts` (N/1/2/3 Neustart und Stufe, Gold geben) ändern die lokale Simulation. Mit SP08
rechnet der Browser nichts mehr, die Tasten entfallen.

## Ziel

Entwickler können Gold geben, Stufen und den Schwierigkeitsgrad wechseln und einen Raum neu starten, ohne den Browser rechnen zu lassen. Nutzen: schnelles
Ausprobieren neuer Mechaniken.

## Beteiligte und Zielgruppen

Entwickler und Agenten; das Debug-Panel (B-107, K5) nutzt den Gradwechsel.

## Anforderungen

- Dev-Aktionen laufen als geschützte Server-Aufrufe (Dev-Mode des Raums bzw. Status-Token), nicht als Protokoll-Nachricht für Spieler.
- Neue Dev-Aktion zum Wechsel des Schwierigkeitsgrads (Dev, Leicht, Normal, Hart, Ultra): wirkt ab der nächsten Welle (`SetGrade`, `engine/sim/island_options.go`), im laufenden Raum erlaubt, ohne Dev-Mode abgelehnt; der Zustand nennt den aktuellen Grad des Raums.

## Nicht-Ziele

Die Tasten im Browser zurückbringen.

## Regeln und Einschränkungen

Domäne SRV; die Spieler-Nachrichten bleiben unverändert, die Dev-Aktion steht in `docs/protocol.md` › Dev-Aktionen (Version mit K4); nur im Dev-Mode bzw. mit Token.

## Beispiele

MCP-Tool `room_give_gold(KRNZ, 0, 50)` → Monarch 0 hat 50 Gold mehr.

## Ausnahme- und Fehlerfälle

Ohne Token bzw. ohne Dev-Mode → abgelehnt; unbekannter Grad → `bad_request`.

## Akzeptanzkriterien

- **AC-01** Eine Dev-Aktion ändert den Zustand eines Raums und ist im nächsten `delta` sichtbar (Test).
- **AC-02** Der Gradwechsel per Dev-Aktion wirkt ab der nächsten Welle, ohne Dev-Mode wird er abgelehnt (Test).

## Offene Fragen

Geklärt (Spec-Prüfung 2026-10-04): Der Gradwechsel kommt in K4, nicht in M6 (B-047).

## Notizen

Entstanden beim Bereitmachen von SP08.

R1 (2026-10-02): Soll auch den Wechsel des Schwierigkeitsgrads im Dev-Mode tragen (B-107, Panel; wirkt ab der nächsten Welle).
