# RD2.5 · Review: Sprint RD2 abnehmen

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd2/5-review
- **Abhängig von:** RD2.1, RD2.2, RD2.3, RD2.4
- **Tickets:** B-712, B-713, B-714, B-715
- **Kriterien:** alle

## Ziel

Sprint RD2 abnehmen und nach `sprints/erledigt/` verschieben.

## Kontext

Sprint-README `docs/sprints/aktiv/RD2-renderer-stil-nachschaerfen/README.md`, Ergebnisse RD2.1 bis RD2.4. Offene manuelle Abnahme: Sichtprüfung Monarch idle 4 neben dem Prototyp (B-712/AC-01) liegt bei Ralf.

## Erlaubte Dateien

- Sprint-Ordner RD2, Tickets B-712 bis B-715 (über `plan_*`)
- Befund-Fixes unter 30 Zeilen in `tools/paper/**`

## Nicht-Ziele

Neue Funktionen; Asset-Änderungen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander; Diff `origin/develop...origin/sprint/rd2` lesen.
2. Je Kriterium AC-01 bis AC-04 den Nachweis aus den Session-Ergebnissen prüfen.
3. Abnahme (höchstens fünf Zeilen) in die README, Tickets und Sprint auf erledigt, PR mit Auto-Merge.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-04 mit Nachweis; AC-01 ausdrücklich als „Sichtprüfung offen bei Ralf“, falls nicht bestätigt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude (Review, autonom).

- `task check` grün (1 min 57 s), `task check:go` grün (3 min 2 s), `task check:race` läuft lokal ohne C-Compiler still durch (551 ms) und gilt als übersprungen; die CI fährt den Race-Detector.
- Diff `origin/develop...sprint/rd2` gelesen: Code in `tools/paper/**` (Stil-Umrechnung, renderKey, `sichtbar`, Proportionen), Specs, Tests, Standard. Vor dem Commit lief ein Reviewer bei RD2.4; sein Befund (Pfote schwebt) ist in RD2.4 behoben.
- AC-01 (B-712): umgesetzt und geprüft (RD2.1, RD2.2); **Sichtprüfung Monarch idle 4 neben dem Prototyp bleibt offen bei Ralf**. AC-02 (B-713), AC-03 (B-714), AC-04 (B-715): umgesetzt und mit Tests belegt (RD2.3, RD2.4).
- Neues Ticket: B-732 (Stroh, Lava, Geisterweiß, `fell-grau`, Eisen/Kristall bei Gegnern und Bossen).
- Offene Fragen mit Default entschieden: Pfeil-Abschuss bei t = 0,7; `biom-*` als Material statt Ton; Proportionen der Figur als Winkel-Faktoren, des Vierbeiners über Pivots; keine Rumpf-Achse je Tier.
