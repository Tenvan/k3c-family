# BB1.1 · Maßstab Figuren und Schiff: Bürger und Truppen 0,6 M, Schiff 3 M (B-1038, B-1039)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bb1
- **Abhängig von:** –
- **Tickets:** B-1038, B-1039
- **Kriterien:** AC-01, AC-02

## Ziel

Bürger und Truppen sind 0,55–0,65 M hoch (Default 0,6 M), die Gier 0,4 M; das Schiff ist mit Segel 2,8–3,2 M hoch (Default 3 M). Ein Test sichert die Maße ab.

## Kontext

- M = berittener Monarch = 121,5 Spiel-px (`data/paper.json` › `figuren.spieler[0].hoehe`). Standard `docs/standards/bildaufbau.md` § 2 (Richtwerte) und § 7 (Abgleich).
- Figuren: `data/paper.json` › `figuren.truppen.<rolle>.hoehe` ist die Zielhöhe in Spiel-px (heute vagrant 102,5, peasant 102,5, archer 99, warrior 102,5, eliteArcher 99, eliteWarrior 101,25 = 0,81–0,84 M). Default-Werte bei Faktor 0,711 (peasant 0,6 M): vagrant 72,9, peasant 72,9, archer 70,4, warrior 72,9, eliteArcher 70,4, eliteWarrior 72,0 (alle 0,58–0,60 M). Gegner: `figuren.gegner.greed.hoehe` 81 (0,67 M) auf 48,6 (0,4 M, K2C „Gier“); die übrigen Gegner, Schwärme (Ratten) und Bosse behalten ihre Größen (Ausnahme im Ticket).
- Schiff: `public/grafik/gebaeude-paper/schiff.paper.json`, `ausgabe.hoehe_px` 164 (1 : 1 in Spiel-px, 1,35 M), vier Stufen im selben Rahmen. Anzeige-Maßstab je Asset: `data/paper.json` › `skalierung` (`gruppe/id` → Faktor, Fußpunkt bleibt; Leser `rust/crates/k3c-game/src/art/scales.rs`). Default: `"gebaeude-paper/schiff": 2.2` → 361 px = 2,97 M, alle Stufen gleich; ohne Umzeichnen der Spec.
- Länge: Rahmenbreite 200 px × 2,2 = 440 px = 3,6 M (Richtwert 4–5 M, nicht Teil von AC-01): Maß im Ergebnis nennen; Verlängern des Rumpfs ist ein Ticket-Vorschlag, kein Umzeichnen hier.
- Assets nur über `assets_*` (Entscheidung 005): `assets_get`, `assets_set`, `assets_preview`, `assets_review` (alle mit `checkout`). Ein einzelnes Asset mit `assets_render` rendern ist erlaubt (Feature-Arbeit, B-742); kein `gen:paper`, kein Golden (Abnahme-Ticket der Review).
- Marken über Figuren (Krone, Balken) sollen der neuen Höhe folgen: die Höhe über Figuren entsteht im Spiel (Suche `hoehe` in `rust/crates/k3c-game/src/props/` und `hud/`); ändern darf diese Session nur Daten und Assets, ein fester Pixelwert ist ein Ticket-Vorschlag.
- Lesbarkeit (Werkzeug, Bogen, Schild erkennbar) prüft `assets_preview` je Truppe in 0,6 M; ist etwas unlesbar: die Spec in `public/grafik/truppen/` vereinfachen statt größer zu bleiben.

## Erlaubte Dateien

- `data/paper.json`
- `public/grafik/gebaeude-paper/schiff.paper.json`
- `public/grafik/truppen/**`
- `tests/bildaufbauMasstab.test.ts`

## Nicht-Ziele

Monarch, Reittiere und Trefferzonen ändern, Rumpf verlängern, `geruest-6` und Bauplatz-Breite umbauen (Ticket-Vorschlag), `gen:paper`, Golden, Sichtprüfung am TV (🧑).

## Schritte

1. Test zuerst: `tests/bildaufbauMasstab.test.ts` (Vitest, liest `data/paper.json` und `public/grafik/gebaeude-paper/schiff.paper.json`): jede `figuren.truppen.*.hoehe` ÷ `figuren.spieler[0].hoehe` liegt in 0,55–0,65; `figuren.gegner.greed.hoehe` ÷ M liegt in 0,35–0,45; Schiff: `ausgabe.hoehe_px` × `skalierung["gebaeude-paper/schiff"]` ÷ M liegt in 2,8–3,2. Rot vor der Änderung (`check_run` mit Ziel `task:test`, Filter `bildaufbauMasstab`).
2. `data/paper.json`: Truppen- und Gier-Höhen auf die Default-Werte, `skalierung` um `gebaeude-paper/schiff` ergänzen.
3. `assets_preview` je Truppe (Bauer, Bogenschütze, Krieger, Elite-Krieger) bei 0,6 M ansehen; unlesbare Specs vereinfachen (`assets_set`, `assets_review`; `k3c-grafik` prüft).
4. Schiff: Ladeplätze (B-997, Stufe 4) bleiben abzählbar; Ergebnis nennt Höhe, Breite und Länge in M; Bauplatz-Breite und `geruest-6` als Befund (Überlappung ja/nein) ins Ergebnis.
5. `check_run` mit Ziel `task:check:diff`: grün.

## Fertig, wenn

- [ ] AC-01: Test `bildaufbauMasstab` grün: Bürger und Truppen 0,55–0,65 M, Gier 0,35–0,45 M; `k3c-grafik` ohne schweren Befund.
- [ ] AC-02: Test grün: `schiff` 2,8–3,2 M mit Segel, alle Stufen im selben Faktor; Maße (Höhe, Länge) und Befund zu Steg und Ladeplätzen im Ergebnis (Sichtprüfung am TV bleibt 🧑).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
