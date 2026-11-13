# PT3 · PAP · Paper-CLI: --ohne mit Reittier-Teilen, Bein-Box mit Wulst

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-845
- **Start-Commit:** 30dcdee51
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13 (Ralf: autonom abarbeitbare Tickets abarbeiten)

## Ausgangslage

Review PT2.3: `paper frame --ohne` kennt die Teile des Reittiers nicht, die Bein-Box lässt den Wulst der Vorderbeine weg (B-845).

## Ziel

`--ohne` nimmt bei Reit-Animationen auch `reittier:<teil>`, und die Bein-Boxen umfassen den Wulst.

## Beteiligte und Zielgruppen

Claude (Umsetzung, autonom, Klasse A); Agenten, die über `assets_preview` Teile ausblenden.

## Anforderungen

`B-845 › Anforderungen`.

## Nicht-Ziele

Workbench-Teil (WZ43).

## Regeln und Einschränkungen

Klasse A (Problem, PAP). Vertrag `docs/standards/paper-spec.md` › 8 nachziehen.

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/rig/**`, `tools/paper/bin/paper.test.ts`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-845/AC-01` und `B-845/AC-02` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PT3.1 | `PT3.1-frame-ohne-reittier.md` | Umsetzung | autonom | offen |
| PT3.2 | `PT3.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
