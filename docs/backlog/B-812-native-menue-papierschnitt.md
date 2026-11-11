# B-812 · Das Menü des nativen Spiels erscheint im Papierschnitt-Stil mit Illustrationen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Menü des nativen Spiels ist schlichter Text. Der Browser-Client zeigt Kacheln mit Illustrationen und Papier-Hintergrund (`pauseHub.ts:62-82`, `hubArt.ts`, `paperBackdrop.ts`).

## Ziel

Das native Menü wirkt wie der Papierschnitt-Stil des Spiels.

## Beteiligte und Zielgruppen

Spieler, Ralf (Geschmack, Freigabe der Assets), Claude (Umsetzung).

## Anforderungen

- Kacheln mit Illustrationen im Hauptmenü und Pause-Menü.
- Papier-Hintergrund.
- Assets nur über `assets_*`.

## Nicht-Ziele

Neue Menüseiten (B-807, B-808, B-809).

## Regeln und Einschränkungen

Entscheidungen 005 und 008. Größe L: Das Ticket wird vor der Freigabe in Tickets je Teil zerlegt (Hintergrund, Kacheln Hauptmenü, Kacheln Pause).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** L
- **Sperre:** –

## Beispiele

Hauptmenü zeigt je Eintrag eine Kachel mit Bild.

## Ausnahme- und Fehlerfälle

Bild fehlt → Kachel mit Text, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Nach der Zerlegung stehen die Kriterien in den Teil-Tickets; dieses Ticket wird dann verworfen mit Verweis.

## Offene Fragen

Zerlegung und Umfang: entscheidet Ralf bei der Freigabe.

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11). Prio niedrig.
