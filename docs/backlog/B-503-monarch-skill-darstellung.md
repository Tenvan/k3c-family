# B-503 · Die 24 Monarch-Skills haben eine Darstellung im Spiel

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SK2
- **Projekt:** SKL
- **Erstellt:** 2026-10-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Lücke 5 der Monarch-Seite (WZ10.5, 🧑 am 2026-10-19): Die 24 Skills aus `data/monarch.json` (Tank 9, Zauberer 8, Heiler 7) haben im Spiel keine Darstellung; nur `noTarget` bei `castFailed` kommt vor. Die Monarch-Seite des Studios markiert jede Zeile als Lücke.

## Ziel

Spielende sehen am TV, wenn ein Monarch einen Skill wirkt oder einen passiven Skill lernt.

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; 🧑 nimmt am TV ab.

## Anforderungen

- Default (autonom, Claude, 2026-10-23; Bestätigung 🧑 über KR5): ein Ring in der Farbe der Linie an der Figur, je Klasse gemeinsam.
- Aktive Skills beim Wirken (Abklingzeit des Slots steigt zwischen zwei Zuständen), Passive beim Lernen (neuer Eintrag in `skills`). Reiner Vergleich, keine neuen Ereignisse, kein Spielzustand im Client.
- 2 Spieler: je Spieler eigene Erkennung; begrenzt durch `MAX_LIVE_EFFECTS`.

## Nicht-Ziele

Effekt je einzelnem Skill, Monarch-Animationen (GR9, B-505), Sounds je Klasse (B-558), Studio-Seite (eigenes Ticket).

## Regeln und Einschränkungen

CLI zeichnet nur Snapshot und Ereignisse; Farben je Linie in `src/scenes/effects/effects.ts`.

## Beispiele

Zauberer wirkt Fireball → violetter Ring am Monarchen. Spieler lernt Thick Skin → Tank-Ring.

## Ausnahme- und Fehlerfälle

Skill ohne Linie in den Daten → kein Ring. Neues Spiel oder Respec → Vergleich beginnt neu, kein Ring.

## Akzeptanzkriterien

- **AC-01** Vitest: Anstieg der Abklingzeit eines Slots ergibt einen Ring der Linie des Skills; ein neuer passiver Skill ebenso; jede Linie aus `data/monarch.json` mit Skills hat eine Farbe.
- **AC-02** Am TV: Ring beim Wirken sichtbar (🧑).

## Offene Fragen

Darstellung je Skill oder je Klasse: Default je Klasse, 🧑 bestätigt über KR5.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
