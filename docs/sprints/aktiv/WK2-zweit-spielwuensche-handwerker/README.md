# WK2 · SIM, CLI · Spielwünsche aus dem Spieltest: Bürger und Handwerker

- **Status:** aktiv
- **Projekt:** NAC
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1245, B-1246, B-1252, B-1253, B-1254, B-1292
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-26, Ralf im Chat („Auch Spielwünsche“)

## Ausgangslage

Spieltest 2026-11-22: fünf Wünsche zu Bürgern und Handwerkern (B-1245, B-1246, B-1252, B-1253, B-1254).

## Ziel

Bürger und Handwerker verhalten sich wie im Spieltest gewünscht. Am Ende sichtbar: am PC im Spiel.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Balancing-Läufe, Neuerungen in der Go-Engine (Entscheidung 013), neue Grafik-Assets über das Tatsächlich Nötige hinaus.

## Regeln und Einschränkungen

Logik und Test in `rust/crates/k3c-sim`, das Spiel zeichnet nur; 2 Spieler gleichzeitig; Werte nach `data/`. Braucht eine Änderung `testdata/**` oder `engine/**` (Bereich RU41), wird die Session blockiert statt dort zu ändern.

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-game/**`, `data/**`, `docs/rules/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** In der Welt Standard kommen je Tag zwei neue Bürger, Test (`B-1246`).
- **AC-02** Handwerker arbeiten ab Sonnenaufgang und suchen nur in der Nacht oder bei Feindkontakt Schutz, Test (`B-1253`).
- **AC-03** Höchstens fünf Steine sind gleichzeitig markiert, eine markierte Erzader zählt mit, Test mit zwei Spielern (`B-1252`).
- **AC-04** Bürger halten nichts in der Hand, bis sie Werkzeug geholt haben (`B-1245`).
- **AC-05** Über Handwerkern zeigt ein kleines Symbol die Tätigkeit: warten, Holz fällen, Waffe holen und weitere vorhandene Zustände (`B-1254`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WK2.1 | `WK2.1-spawn-und-arbeitsbeginn.md` | Umsetzung | autonom | offen |
| WK2.2 | `WK2.2-stein-markierungen.md` | Umsetzung | autonom | offen |
| WK2.3 | `WK2.3-figuren-anzeige.md` | Umsetzung | autonom | offen |
| WK2.4 | `WK2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
