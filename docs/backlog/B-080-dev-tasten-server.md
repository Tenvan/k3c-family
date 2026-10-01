# B-080 · Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Dev-Tasten in `src/scenes/GameScene.ts` (N/1/2/3 Neustart und Stufe, Gold geben) ändern die lokale Simulation. Mit SP08
rechnet der Browser nichts mehr, die Tasten entfallen.

## Ziel

Entwickler können Gold geben, Stufen wechseln und einen Raum neu starten, ohne den Browser rechnen zu lassen. Nutzen: schnelles
Ausprobieren neuer Mechaniken.

## Beteiligte und Zielgruppen

Entwickler und Agenten; M6 (MCP-Tools, B-047) ist der nächstliegende Weg.

## Anforderungen

- Dev-Aktionen laufen als geschützte Server-Aufrufe (Status-Token), nicht als Protokoll-Nachricht für Spieler.

## Nicht-Ziele

Die Tasten im Browser zurückbringen.

## Regeln und Einschränkungen

Domäne SRV; Protokoll v2 bleibt unverändert; nur mit Token.

## Beispiele

MCP-Tool `room_give_gold(KRNZ, 0, 50)` → Monarch 0 hat 50 Gold mehr.

## Ausnahme- und Fehlerfälle

Ohne Token → abgelehnt.

## Akzeptanzkriterien

- **AC-01** Eine Dev-Aktion ändert den Zustand eines Raums und ist im nächsten `delta` sichtbar (Test).

## Offene Fragen

Gehört das in M6 (B-047)? (🧑)

## Notizen

Entstanden beim Bereitmachen von SP08.
