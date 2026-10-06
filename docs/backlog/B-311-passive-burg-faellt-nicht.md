# B-311 · Mit der Verlust-Kaskade fällt die Burg bei passivem Spiel nie

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit W4.3a (Verlust-Kaskade, Q67) lässt ein getroffener Bogenschütze den Bogen fallen, wird Bauer und hebt den Bogen
gleich wieder auf (Abholauftrag wie aus dem Regal). Die zwei Start-Bogenschützen fallen dadurch nie aus. Messung mit dem
Balancing-Tester (`tools/k3c-dev/internal/balance`, Bot `passive`, 1 Spieler): Vor W4.3a fiel die Burg in Nacht 1
(Seed `2`, Tick 25040), danach in **keinem** Lauf (Tiefe 0–3, 1/2/4 Tage, Seeds 1–3). Der Test
`TestReplayRunGleicherHashUndBurgfall` (k3c-dev) brauchte deshalb einen Lauf mit Burgfall und wurde angepasst.

## Ziel

🧑 entscheidet, ob ein Spieler ohne jede Handlung die Burg verlieren soll (Zielkorridor „Burg hält Nacht 1–5“ je Grad,
`docs/rules/zielkorridore.md`).

## Beteiligte und Zielgruppen

🧑 (Regel), REG (Balancing), SIM (Umsetzung).

## Anforderungen

- Entscheidung, ob das Wiederaufheben fallengelassener Ausrüstung begrenzt wird (z. B. Wartezeit, Kosten, Gegner
  tragen Ausrüstung weg – `equipmentTaken`, K1) oder ob die Burg bei passivem Spiel halten darf.

## Nicht-Ziele

Umsetzung (folgt als SIM-Ticket nach der Entscheidung).

## Regeln und Einschränkungen

Q67, Q68 (`docs/fragenkatalog.md`); Zielkorridore F1.

## Beispiele

Passiver Spieler, Seed `2`, Nacht 1: Goblins treffen die Bogenschützen, diese heben den Bogen sofort wieder auf und
schießen weiter; die Burg verliert 15 HP und hält.

## Ausnahme- und Fehlerfälle

nicht relevant (Frage)

## Akzeptanzkriterien

- **AC-01** Entscheidung von 🧑 steht in `docs/fragenkatalog.md`, Folge-Ticket angelegt oder Frage als verworfen archiviert.

## Offene Fragen

Soll die Burg bei passivem Spiel fallen können (🧑)?

## Notizen

Gefunden beim CI-Fix für PR #164 (Sprint W4). Mit K1 (`equipmentTaken`: Gegner tragen Ausrüstung weg) könnte sich das
von selbst ändern.
