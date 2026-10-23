# KB3 · SRV, PLAT, REG · Endboss-Auslöser anzeigen, Dev-Overlay-Link, Wellenabstand unter Tage messen

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** SRV, PLAT, REG
- **Reife:** bereit
- **Tickets:** B-563, B-564, B-565
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Die Aktionsliste nennt den Bau-Auslöser des Endbosses nicht (B-563); die Entwicklerseite öffnet das Spiel ohne Debug-Overlay (B-564); der Wellenabstand unter Tage mit Adern ist nicht gemessen (B-565).

## Ziel

Auslöser des Endbosses sichtbar, Dev-Link mit `?dev=1`, Wellenabstand über 100 Seeds gemessen. Am Ende sichtbar: Overlay, Dev-Seite und Messbericht.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-563, B-564, B-565 › Anforderungen.

## Nicht-Ziele

Neue Boss-Mechanik.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Aktionsliste nennt den Bau-Auslöser des Endbosses, das Overlay zeigt ihn (B-563/AC).
- **AC-02** Die Entwicklerseite öffnet das Spiel mit `?dev=1` (B-564/AC).
- **AC-03** Der Wellenabstand ist über 100 Seeds über `sim_test` gemessen (B-565/AC).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB3.1 | `KB3.1-aktion-lair.md` | Umsetzung | autonom | fertig |
| KB3.2 | `KB3.2-dev-kachel.md` | Umsetzung | autonom | fertig |
| KB3.3 | `KB3.3-messung-wellenabstand.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-03 umgesetzt bzw. gemessen. Befund: Wellenabstand mit Adern über dem Korridor (Median 19,37 min), neues Ticket B-579 (Entscheidung 🧑). Annahme: `noticeUnits` 40.
