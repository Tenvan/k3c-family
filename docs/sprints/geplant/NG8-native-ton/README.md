# NG8 · CLI · Natives Spiel: Klänge und Musik

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-801
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach NG1 ist `native/k3c-game/audio` ein Platzhalter; der Ton-Dekoder ist in NG1 festgelegt. Vorbild: `src/audio/`, `public/audio/atlas.json`. Einzelheiten: B-801.

## Ziel

Klang-Effekte, Menü-Klänge und Musik spielen nativ wie im Browser. Am Ende sichtbar (hörbar): Münz-Klang beim Zahlen, Musikwechsel bei Nacht.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Hörprüfung in NG9).

## Anforderungen

`B-801 › Anforderungen`.

## Nicht-Ziele

`B-801 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

Lizenzen nach `public/audio/README.md`. Vertrag `view` fest.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/audio/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

`B-801 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-801 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Atlas und Busse (`B-801/AC-01`).
- **AC-02** Zuordnung Ereignis, Menü-Klang und Musik (`B-801/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG8.1 | `NG8.1-atlas-busse.md` | Umsetzung | autonom | offen |
| NG8.2 | `NG8.2-ereignisse-musik.md` | Umsetzung | autonom | offen |
| NG8.3 | `NG8.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
