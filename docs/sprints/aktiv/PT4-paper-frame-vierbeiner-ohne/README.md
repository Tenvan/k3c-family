# PT4 · PAP · Paper-CLI: --ohne blendet Beinpfade von Vierbeinern aus

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-851
- **Start-Commit:** 03a358518
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Befund aus PT3.2 (B-851): `--ohne oberbein-<b>` lässt den Beinpfad von Vierbeinern sichtbar.

## Ziel

`--ohne` blendet Beinpfade von Vierbeinern aus.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

`B-851 › Anforderungen`.

## Nicht-Ziele

Workbench-Teil.

## Regeln und Einschränkungen

Klasse A (Problem, PAP).

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/rig/**`, `tools/paper/bin/paper.test.ts`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-851/AC-01` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PT4.1 | `PT4.1-frame-ohne-vierbeiner.md` | Umsetzung | autonom | fertig |
| PT4.2 | `PT4.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
