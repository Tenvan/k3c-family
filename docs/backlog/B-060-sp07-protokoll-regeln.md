# B-060 · Die Spec von SP07 deckt alle Server-Regeln aus Protokoll v2 ab

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP07
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Das Review SP02 (SP02.3) hat `docs/sprints/geplant/SP07-raeume/README.md` und B-030 gegen `docs/protocol.md`
geprüft. Die Kriterien von SP07 decken nur Räume parallel, lokale Spieler, Wiederverbinden, Aufräumen, Grenzen,
WebSocket und `/api/status` ab. Es fehlen Regeln, die `docs/protocol.md` festlegt: der Raum speichert selbst
(Beschluss 4: Stufenwechsel, leer, Aufräumen, Beenden), Handschlag `hello`/`welcome` mit `version`, die Codes
`save_exists`, `save_not_found`, `room_closed`, `replaced`, `bad_request`, die Prüf-Reihenfolge bei `create`,
Fristen nach Wanduhr auch im pausierten Raum, Delta `{set, del}`, `seq`/`ack` je Verbindung und das Schließen bei
vollem Sendepuffer. B-030 › Anforderungen verlangt „mit denselben Spielern“ zurück; `docs/protocol.md` erlaubt beim
Wiederverbinden weniger oder mehr Slots.

## Ziel

SP07 setzt Protokoll v2 vollständig um, ohne dass eine Regel beim Bereitmachen des Sprints vergessen wird.

## Beteiligte und Zielgruppen

Wer SP07 bereit macht (Planung); 🧑 gibt die Spec frei.

## Anforderungen

- SP07-README nennt `docs/protocol.md` und `testdata/protocol/` als Vertrag und hat Kriterien für die oben genannten
  Regeln (mindestens: Speichern pro Raum, Fehler-Codes, Handschlag, Fristen, Delta).
- B-030 › Anforderungen passt zu `docs/protocol.md` › *Wiederverbinden* (geänderte Slots).

## Nicht-Ziele

Umsetzung (SP07); Client-Seite (B-061); Stufenwechsel mit freien Monarchen (B-059).

## Regeln und Einschränkungen

Entscheidung 001 und 002, `docs/protocol.md`; Spec-Änderung nur mit neuer Revision und Freigabe durch 🧑.

## Beispiele

Ein Raum wird nach 10 min leer aufgeräumt → ein Test in SP07 prüft, dass der Spielstand vorher gespeichert wurde.

## Ausnahme- und Fehlerfälle

nicht relevant: Planungs-Ticket.

## Akzeptanzkriterien

- **AC-01** Jede Regel aus der Ausgangslage hat in der SP07-README ein Kriterium oder steht dort begründet unter
  Nicht-Ziele.
- **AC-02** B-030 › Anforderungen widerspricht `docs/protocol.md` › *Wiederverbinden* nicht.

## Offene Fragen

Gehört das Speichern pro Raum (`engine/store`) in SP07 oder schon in SP03 (Go-Server Basis)? Entscheidet 🧑 beim Planen.

## Notizen

Erledigt mit der Planung von SP07 (2026-10-01): AC-01 durch SP07-README AC-01 bis AC-08 (Speichern pro Raum AC-04, Handschlag und Codes AC-05, Fristen AC-03, Delta und `seq`/`ack` AC-07, Sendepuffer AC-07), AC-02 durch B-030 Revision 3. Speichern pro Raum gehört in SP07 (🧑).
