# WZ55 · DEV · assets_set ruft die Paper-CLI gesammelt auf

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-856, B-867, B-868
- **Start-Commit:** 21a4188c0
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Befund WZ48.2 (B-856): `assets_set` startet bun zwölfmal je Änderung.

## Ziel

Ein Sammelaufruf der Paper-CLI erledigt die Schritte eines `assets_set` in einem Prozess.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-856 › Anforderungen`.

## Nicht-Ziele

Render-Logik.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV). Nach WZ54 (Sperre exe).

- **Bereich:** `tools/paper/cli/**`, `tools/paper/bin/**`, `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`, `docs/standards/paper-mcp.md`
- **Abhängig von:** WZ54
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-856/AC-01` und `B-856/AC-02` grün.
- **AC-02** `B-867/AC-01` grün (gescheitertes Entfernen zurückgenommen).
- **AC-03** `B-868/AC-01` grün (Mindestversion von Task geprüft).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ55.1 | `WZ55.1-paper-batch.md` | Umsetzung | autonom | offen |
| WZ55.2 | `WZ55.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
