# B-061 · Die Spec von SP08 deckt alle Client-Regeln aus Protokoll v2 ab

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP08
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Review SP02 (SP02.3) hat `docs/sprints/geplant/SP08-client/README.md` gegen `docs/protocol.md` geprüft. SP08
nennt als Fehlerfall nur „Verbindung weg → Hinweis und Wiederverbinden“. `docs/protocol.md` verlangt vom Client mehr:
Geräte-ID im `localStorage`, `hello` als erste Nachricht, nach `replaced` **kein** automatisches Neuverbinden, nach
`room_closed` zurück zur Raumliste, nach `version` Hinweis „Seite neu laden“, Anzeige der übrigen Fehler-Codes,
`input` bei Änderung und mindestens alle 500 ms (höchstens eine pro Tick), `seq` je Verbindung ab 1, Level aus
`level` statt aus dem Seed, Delta `{set, del}` anwenden.

## Ziel

SP08 setzt die Client-Seite von Protokoll v2 vollständig um, ohne dass eine Regel beim Bereitmachen vergessen wird.

## Beteiligte und Zielgruppen

Wer SP08 bereit macht (Planung); 🧑 gibt die Spec frei.

## Anforderungen

- SP08-README nennt `docs/protocol.md` und `testdata/protocol/` als Vertrag und hat Kriterien für die oben genannten
  Regeln (mindestens: Geräte-ID, Fehler-Codes mit Verhalten, Eingabe-Takt, Level und Delta).

## Nicht-Ziele

Umsetzung (SP08); Server-Seite (B-060); Lobby-Oberfläche über B-037 hinaus.

## Regeln und Einschränkungen

Entscheidung 001 und 002, `docs/protocol.md`; Seiten-Regeln aus `CLAUDE.md`; Spec-Änderung nur mit neuer Revision
und Freigabe durch 🧑.

## Beispiele

Zweiter Tab im selben Browser tritt demselben Raum bei → der erste Tab zeigt „an anderer Stelle geöffnet“ und
verbindet sich nicht neu.

## Ausnahme- und Fehlerfälle

nicht relevant: Planungs-Ticket.

## Akzeptanzkriterien

- **AC-01** Jede Regel aus der Ausgangslage hat in der SP08-README ein Kriterium oder steht dort begründet unter
  Nicht-Ziele.

## Offene Fragen

keine

## Notizen

Eingearbeitet in SP08 Revision 3: Handschlag AC-06, Eingabe-Takt AC-07, Level und Delta AC-08, Fehler-Codes AC-09, Wiederverbinden AC-10, Slots AC-11.
