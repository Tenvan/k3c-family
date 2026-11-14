# WZ55 · DEV · assets_set ruft die Paper-CLI gesammelt auf

- **Status:** erledigt
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
| WZ55.1 | `WZ55.1-paper-batch.md` | Umsetzung | autonom | fertig |
| WZ55.2 | `WZ55.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14, Claude (autonom). AC-01 bis AC-03 geprüft (WZ55.1, WZ55.2); `task check`, `task check:go`, `go:test`, `task check:dev` grün; `assets_set` startet bun einmal statt zwölfmal.
Behoben im Review: Zeitlimit des Sammelaufrufs, Test ohne Skip, gocyclo in `apply`.
Neue Tickets: keine angelegt (Vorschläge im Bericht).
Abnahme-Ticket: vorgeschlagen, `gen:paper` samt Grafik-Golden (CLI geändert, Render-Logik nicht); vom Koordinator anzulegen.
