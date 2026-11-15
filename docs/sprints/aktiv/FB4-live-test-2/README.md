# FB4 · CLI · Live-Test 2: Ruckeln, Xbox-Tasten, Menü-Tastatur, Ladescreen, Mauern, Bäume

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Live-Test 2 🧑 2026-11-15.

## Ziel

Flüssige Grafik, Xbox-Tasten mit Testseite, Tastatur im Menü, kein störender Hinweis, lückenlose Mauern, größere Bäume, schöner Ladescreen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/input/**`, `native/k3c-game/menu/**`, `native/k3c-game/devtools/**`, `native/k3c-game/hud/**`, `native/k3c-game/world/**`, `native/k3c-game/view/**`, `native/k3c-game/art/**`, `data/paper.json`, `public/grafik/**`
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Ruckeln gemessen und behoben (`B-943`).
- **AC-02** Xbox-Tasten und Testseite (`B-944`).
- **AC-03** Tastatur im Menü, Hinweis weg (`B-945`, `B-946`).
- **AC-04** Mauern lückenlos, Bäume größer, Ladescreen (`B-947`, `B-948`, `B-949`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| FB4.1 | `FB4.1-live-test-2.md` | Umsetzung | autonom | in Arbeit |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
