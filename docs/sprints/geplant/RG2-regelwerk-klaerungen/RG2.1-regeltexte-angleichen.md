# RG2.1 · Regeltexte angleichen: Verlust-Korridor, Tier-Gating, Geselle nur Werkstatt

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** haiku
- **Umgebung:** offline
- **Branch:** sprint/rg2
- **Abhängig von:** –
- **Tickets:** B-185, B-219, B-295, B-203
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Die Regeltexte widersprechen sich in drei Punkten nicht mehr: Verlust-Korridor je Welle (B-185), Tier-Gating 2/4/6 (B-219), Geselle nur an der Werkstatt (B-295). Die entfallene Kennzahl „erste Gold-Schwelle“ (B-203) ist gegengeprüft.

## Kontext

Reine Doku-Session, kein Code, keine Werte in `data/`, keine Simulation (Sim und Balancing sind gesperrt).

- **B-185:** `docs/rules/wirtschaft.md` § 3 (Zeile 37) sagt „Verluste je Welle: Median höchstens die Hälfte der Truppen (Normal)“. Gültig ist „Median höchstens 25 % der Kämpfer“ (`docs/rules/buerger.md` § 3, `docs/rules/kampf.md` § 4, `docs/rules/zielkorridore.md` § 4 Zeile 53; 🧑 2026-10-03). In `zielkorridore.md` Zeile 53 steht ein Nebensatz über den abweichenden Wert und B-185, der entfällt.
- **B-219:** `docs/game-design.md` Zeile 57 nennt „Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten“. Gültig ist 2/4/6 gelernte Skills der Linie (`docs/rules/monarch.md` § 3 Zeile 30, `data/monarch.json` › `tierPoints`). `monarch.md` Zeile 57 („Offen“) nennt den Widerspruch noch. `engine/sim/events.go` Zeile 12 ist schon richtig und wird nicht angefasst.
- **B-295:** Seit B-950 ist der Handwerker der Bauer mit Hammer; der Beruf `craftsman` heißt Geselle (`docs/glossar.md` Zeile 105, 115). `docs/rules/buerger.md` Zeile 21 (§ 1), Zeile 38 und 41 (§ 2) und `docs/glossar.md` Zeile 105 nennen den Gesellen an Werkstatt, Schmiede und Rüstkammer. Entschieden (B-627, 2026-10-31): nur Werkstatt; `data/buildings.json` hat `craftsman` nur unter `workshop.offers`. Zeile 61 („Offen: Handwerker-Anzahl je Gebäude …“) prüfen und anpassen.
- **B-203:** Kennzahl entfällt; nur prüfen, dass kein Regeltext sie nennt.

Begriffe nach `docs/glossar.md`. Zeilennummern Stand 2026-11-15, vor dem Ändern per `grep` bestätigen.

## Erlaubte Dateien

- `docs/rules/wirtschaft.md`
- `docs/rules/zielkorridore.md`
- `docs/rules/buerger.md`
- `docs/rules/monarch.md`
- `docs/game-design.md`

## Nicht-Ziele

Werte in `data/*.json`; Code (auch `engine/sim/events.go`); Simulation und Balancing; andere veraltete Stellen der Doku (Ticket anlegen); Glossar (Sammeldatei, ändert RG2.2 im letzten Commit).

## Schritte

1. Stellen per `grep` bestätigen („Hälfte der Truppen“, „ab 5, Tier 3 ab 10“, „5/10/15“, „Geselle“, „Gold-Schwelle“).
2. `wirtschaft.md` § 3: Korridor „Median höchstens 25 % der Kämpfer (`zielkorridore.md` § 4)“; in `zielkorridore.md` § 4 den Nebensatz zum abweichenden Wert streichen.
3. `game-design.md`: Tier-Gating 2/4/6 gelernte Skills der Linie, Verweis `rules/monarch.md` § 3; in `monarch.md` § „Offen“ den Widerspruch streichen.
4. `buerger.md` § 1 und § 2: Geselle nur an der Werkstatt (höchstens 2); Herstellungszeiten: Elite-Upgrade, Rüstungsstufe und Bogen ohne Gesellen-Bonus; § 4 „Offen“ anpassen.
5. B-203: `grep -rn "Gold-Schwelle" docs/rules docs/game-design.md` ohne Treffer im Ergebnis festhalten.
6. `task check:diff`.

## Fertig, wenn

- [ ] AC-01: `grep -n "Geselle" docs/rules/buerger.md` nennt als Ort nur die Werkstatt; `git diff --stat data/` leer (B-295/AC-01, Glossar folgt in RG2.2).
- [ ] AC-02: `grep -n "Hälfte der Truppen" docs/rules/wirtschaft.md docs/rules/zielkorridore.md` ohne Treffer; `wirtschaft.md` § 3 nennt 25 % der Kämpfer (B-185/AC-01).
- [ ] AC-03: `grep -rn "Gold-Schwelle" docs/rules docs/game-design.md` ohne Treffer (B-203/AC-01).
- [ ] AC-04: `grep -n "ab 5, Tier 3 ab 10" docs/game-design.md` und `grep -n "5/10/15" docs/rules/monarch.md` ohne Treffer (B-219/AC-01).
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
