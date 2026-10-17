# WZ13 · INF, REG · Glossar und Regeln: Verweis für das Morgengrauen auf B-470

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** INF, REG
- **Reife:** bereit
- **Tickets:** B-466
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

`B-466 › Ausgangslage`: Glossar und Regel-Dokumente nennen für den Umbau des Tageszyklus (Phase Morgengrauen, Regel Q65) das Ticket B-213, doch B-213 ist ein anderes, erledigtes Ticket. Der Umbau ist B-470.

## Ziel

Glossar und Regel-Dokumente verweisen für den Umbau auf B-470; die Zielregel Q65 bleibt unverändert. Am Ende sichtbar: `git grep -n "B-213"` in `docs/glossar.md`, `docs/rules` und `docs/game-design.md` ist leer, `task check` grün.

## Beteiligte und Zielgruppen

Agenten und 🧑, die Glossar und Regeln als verbindliche Begriffe lesen; 🧑 entscheidet die offene Frage und gibt die Spec frei.

## Anforderungen

`B-466 › Anforderungen` (Stellen und Wortlaut dort). Sprint-eigen: zwei Domänen nacheinander, INF für das Glossar, dann REG für die Regel-Dokumente.

## Nicht-Ziele

`B-466 › Nicht-Ziele`: Tageszyklus bauen oder Werte ändern (B-470, B-155), Zielregel Q65 ändern, Tickets, Sprints und Sessions mit B-213 umschreiben (B-168, SO4, BR1.1).

## Regeln und Einschränkungen

Reine Doku-Änderung; Glossar ist verbindlich (`docs/arbeitsweise.md`); keine inhaltliche Regel- oder Wertänderung. Doku-Sprint: Die letzte Session schließt ihn ab (kein eigenes Review).

## Beispiele

`B-466 › Beispiele`: Glossar › Tag lautet danach „Startwert 6 min (heute Wald 10 min, bis B-470); …“.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Textkorrektur.

## Akzeptanzkriterien

- **AC-01** Das Glossar verweist für den Tageszyklus auf B-470 (`B-466/AC-01`).
- **AC-02** Die Zielregel Q65 steht unverändert in Glossar und `wirtschaft.md` (`B-466/AC-02`).
- **AC-03** `wirtschaft.md`, `zielkorridore.md` und `game-design.md` verweisen auf B-470 (`B-466/AC-03`).
- **AC-04** Die Einträge Morgengrauen und Tageszyklus beschreiben den heutigen Code richtig (`B-466/AC-04`).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): `docs/fragenkatalog.md` (Beschlussprotokoll Q65) bleibt unverändert, weil es die damals geplante Nummer B-213 dokumentiert (`B-466 › Offene Fragen`). Vor der Freigabe offen: nichts.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ13.1 | `WZ13.1-glossar-verweis.md` | Umsetzung | autonom | offen |
| WZ13.2 | `WZ13.2-regeln-verweis.md` | Umsetzung | autonom | offen |

- WZ13.1 (INF) `docs/glossar.md`: sieben Stellen auf B-470, Einträge Morgengrauen und Tageszyklus (AC-01, AC-02, AC-04).
- WZ13.2 (REG) `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md`, `docs/game-design.md` auf B-470; schließt den Sprint ab (AC-02, AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
