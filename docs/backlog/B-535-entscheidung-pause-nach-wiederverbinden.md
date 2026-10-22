# B-535 · Entscheidung: Pause nach Wiederverbinden erneut senden, Schutz beginnt neu

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD7.3 bindet die Raum-Pause im Client an: Die Optionen-Szene sendet `pause` beim Öffnen, `resume` beim Schließen. Bricht die Verbindung ab, beendet der Server die Pause (`docs/protocol.md` v8, `engine/room/pause.go`).

## Ziel

🧑 bestätigt oder ändert, was nach dem Wiederverbinden mit noch offenem Menü gilt.

## Beteiligte und Zielgruppen

Spielende online; 🧑 entscheidet.

## Anforderungen

Default (autonom, umgesetzt in BD7.3): Der Client sendet nach dem Wiederbeitritt mit offenem Menü erneut `pause`; im Couch-Raum steht der Raum wieder, online beginnt der 60-s-Schutz neu. Während der Trennung ist der Monarch ohnehin geschützt (§ 3, B-530), ein neuer Schutz ist also kein zusätzlicher Vorteil, außer bei mehreren Abbrüchen hintereinander.

Alternative: nach dem Wiederbeitritt kein `pause`; das Menü bleibt offen, die Monarchen stehen (Leerlauf-Eingaben), sind aber verwundbar, und der Couch-Raum läuft weiter, bis das Menü neu geöffnet wird.

## Nicht-Ziele

Protokolländerung.

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 1 und § 3.

## Beispiele

Menü offen, WLAN bricht 5 s ab, Gerät kommt zurück → Default: Raum pausiert wieder (Couch) bzw. 60 s Schutz (online).

## Ausnahme- und Fehlerfälle

nicht relevant (Entscheidung).

## Akzeptanzkriterien

- **AC-01** 🧑 hat Default oder Alternative bestätigt; bei Alternative ist ein Umsetzungs-Ticket angelegt.

## Offene Fragen

Default oder Alternative (🧑).

## Notizen

Angelegt in BD7 (autonom abarbeiten), Default umgesetzt in BD7.3 (`src/online/clientPause.ts`).
