# KS5 · CLI · Anzeige im Spiel: Portal-HP, Krone, Münzverlust, Schild, Banner, Auslöser

- **Status:** aktiv
- **Projekt:** KSY
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

KS2–KS4 gemergt (#611, #613, #615): Portale mit HP, Krone und Gold, Gruppen und Banner; Zahlziele `CombatSpots` ohne Anzeige.

## Ziel

Das native Spiel zeigt Portal-HP, Krone, Münzverlust, Schild, Banner und Auslöser.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/**`, `data/paper.json`
- **Sperre:** –
- Grafik nur über `assets_*`; fehlt ein Asset, vorläufig gezeichnet und gemeldet.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Portal-HP, Krone (am Kopf, fliegend, liegend, getragen), Münzverlust, Schild, Banner und Auslöser sichtbar, 2 Spieler (`B-934/AC-01`, `AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS5.1 | `KS5.1-kampf-anzeige.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
