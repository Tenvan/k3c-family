# B-312 · Sofortiges Wiederaufheben fallengelassener Ausrüstung macht die Burg bei passivem Spiel unverwundbar

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W7
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit W4.3a (Verlust-Kaskade, Q67, Q68) lässt ein getroffener Bogenschütze den Bogen fallen, wird Bauer und hebt ihn
sofort wieder auf. Bei passivem Spiel fällt die Burg deshalb in keinem Lauf mehr (Messung in B-311). 🧑 hat am
2026-10-06 entschieden: Die Burg kann bei passivem Spiel fallen (Q72).

## Ziel

Ein Spieler ohne jede Handlung kann die Burg wieder verlieren, im Zielkorridor „Burg hält Nacht 1–5“ je Grad
(`docs/rules/zielkorridore.md`).

## Beteiligte und Zielgruppen

Spieler (Koop); 🧑 entscheidet den Mechanismus; REG stimmt die Werte ab (BR1).

## Anforderungen

- Fallengelassene Ausrüstung lässt sich nicht beliebig oft ohne Folgen wieder aufheben (Mechanismus: Offene Fragen).
- Deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig.

## Nicht-Ziele

Balancing der Werte (BR1, B-099); Gegner tragen Ausrüstung weg (`equipmentTaken`, K1), sofern nicht als Mechanismus gewählt.

## Regeln und Einschränkungen

Q67, Q68, Q72 (`docs/fragenkatalog.md`); Werte nur in `data/`.

## Beispiele

Passiver Spieler, Seed `2`: Die Burg fällt wieder in einer frühen Nacht (vor W4.3a in Nacht 1, Tick 25040).

## Ausnahme- und Fehlerfälle

nicht relevant: Mechanismus noch offen.

## Akzeptanzkriterien

- **AC-01** Bei passivem Spiel (Bot `passive`, 1 Spieler) fällt die Burg in mindestens einem der Seeds 1–3 (Test).
- **AC-02** `task check:go` grün, Golden-Daten aktualisiert.

## Offene Fragen

Entschieden 2026-10-06 (🧑, Chat): Gegner tragen fallengelassene Ausrüstung weg (Ereignis `equipmentTaken`, in
`engine/sim/events.go` bisher nur angelegt). Keine Wartezeit vor dem Wiederaufheben. Die nötigen Werte kommen mit
vorläufigen Zahlen als neue Felder nach `data/`, REG legt sie später fest.

## Notizen

Folge-Ticket aus B-311 (Q72).
