# PT3 · PAP · Paper-CLI: --ohne mit Reittier-Teilen, Bein-Box mit Wulst

- **Status:** erledigt
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
| PT3.1 | `PT3.1-frame-ohne-reittier.md` | Umsetzung | autonom | fertig |
| PT3.2 | `PT3.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

- 2026-11-13, Claude (autonom, Klasse A): AC-01 erfüllt (PT3.1 › Ergebnis: `B-845/AC-01`, `B-845/AC-02` grün).
- `task check` und `go:test` grün; k3c-reviewer ohne Befunde, keine Fixes nötig.
- Neue Tickets: eins vorgeschlagen (PT3.2 › Ergebnis, `--ohne oberbein-<b>` bei Vierbeiner-Animationen), noch anzulegen.
- Abnahme-Ticket Sim, Balance, Gen: keins (Grafik und Golden unverändert).
