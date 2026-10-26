# B-642 · Der Balancing-Tester spielt nur eine Stufe, der Meilenstein-Lauf braucht die ganze Insel

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS11
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Balancing-Tester baut je Lauf eine Insel mit genau **einer** Stufe (`tools/k3c-workbench/internal/balance/run.go` › `newIsland`: `sim.CreateIsland(seed, []int{depth}, …)`). Bots wechseln keine Stufe und kämpfen nicht gezielt gegen Bosse. Der Meilenstein-Lauf von BAS (`docs/rules/zielkorridore.md` § 6) verlangt aber „Endboss der Insel 1 besiegt“, und die Endboss-Stufe ist die tiefste der Welt `standard` (Tiefen 0–4).

## Ziel

Der Balancing-Tester spielt die **ganze Insel** der Welt `standard` (Tiefen 0–4): Bots bauen die Hubs der Stufen auf, steigen über Tiefen-Eingang bzw. Treppe ab, besiegen Minibosse und den Endboss. Daraus misst der Meilenstein-Lauf „Endboss Insel 1 besiegt“.

## Beteiligte und Zielgruppen

🧑 entscheidet, Claude setzt um.

## Anforderungen

Entschieden: **Variante A** (🧑 2026-10-26, Chat). Eigener Sprint BS11 vor BS3.
- Insel-Lauf: `sim.CreateIslandIn(world, seed, depths, …)` mit allen Tiefen der Insel 1 aus `data/worlds/standard.json` statt einer Stufe (`balance/run.go` › `newIsland`); Szenario-Feld `World` (leer = bisheriger Ein-Stufen-Lauf, Ziele unverändert).
- Metriken je Stufe: Ankunftstag je Tiefe, Miniboss je Tiefe besiegt (Tag), Endboss besiegt (Tag), Ereignis `victory`, Burgfall je Stufe.
- Bot-Bausteine (in `coop2` und `wirtschaft`): Treppe bauen, zum Tiefen-Eingang laufen und absteigen, wenn die Stufe sicher steht (Regel im Bot: Hub-Stufe und Mauern der aktuellen Stufe), auf der neuen Stufe Hub aufbauen, in Bossnächten an der Burg kämpfen (`Attack`), Material der Insel nutzen.
- Kennzahlen als Ziele: `endbossDefeated` (Anteil), `endbossDay` (Median), Ankunft je Tiefe (Median), in `balance-targets.json`.
- `sim_test` offline nimmt `world` (z. B. `standard`) und `days` (bis 60); Beschreibung und `instructions.md` nennen den Meilenstein-Lauf.
- Laufzeit messen: ein Insel-Lauf mit Seed `k3c-dev` soll in wenigen Minuten fertig sein; sonst Maßnahme (z. B. Abbruch nach `victory`).

## Nicht-Ziele

Mehrere Inseln und Inselwechsel (BS5, BS9); Werteänderungen (BS3); neue Sim-Mechanik. Fehlt der Sim etwas für die Bots, wird es ein Ticket (SIM), keine Umgehung im Tester.

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-workbench/internal/balance`, `simrun`, `mcpsrv`), Ziele in `data/balance-targets.json`. Deterministisch (nur `engine/rng`). Bots spielen nur über `sim.PlayerCommand`. Alle Läufe über `sim_test`. ⚠️ Go-Änderungen an der Workbench → Neustart deutlich melden.

## Beispiele

`sim_test {action: start, bots: coop2, world: standard, days: 40}` → Bericht: „Wald T1, Höhle T6, Mine T11, Eisenstollen T16, Kristallhöhle T21, Endboss besiegt T27 ✅“.

## Ausnahme- und Fehlerfälle

Bots kommen nicht weiter (Stufe hält nicht, kein Material) → Lauf endet nach `days`, Bericht zeigt die erreichte Tiefe und den Grund (Burgfall, kein Abstieg). Burgfall auf einer Stufe mit Niederlage-Modus → wie Spiel, Bericht nennt Tag und Stufe.

## Akzeptanzkriterien

- **AC-01** Insel-Lauf mit allen Tiefen der Welt `standard`; Test: Szenario mit `World` baut 5 Stufen, ohne `World` wie bisher eine.
- **AC-02** Metriken je Stufe und Bosse im Bericht; Unit-Test mit festem Ereignisverlauf.
- **AC-03** Bots steigen ab und bauen auf neuen Stufen auf; Test: `coop2` erreicht in einem kurzen Lauf mindestens Tiefe 1.
- **AC-04** Bots kämpfen in Bossnächten; Ziel `endbossDefeated` und `endbossDay` in `balance-targets.json`, Schema-Test.
- **AC-05** `sim_test` nimmt `world` und `days`; Instructions ergänzt; Referenzlauf mit den alten Werten liegt als Bericht vor (Ergebnis darf rot sein).
- **AC-06** Die Entscheidung steht in `docs/rules/zielkorridore.md` § 6.

## Offene Fragen

- Absteige-Regel der Bots (Vorschlag: absteigen, wenn Hub-Stufe = Tiefe + 2 und beide Mauern der Linie 1 stehen) → Default Vorschlag, änderbar mit BS3.
- Tageslimit des Meilenstein-Laufs (Vorschlag 40 Tage) → Default 40.

## Notizen

Gefunden in BS1.1 (2026-10-26): Der Parameter `world` für `sim_test` ist deshalb noch nicht umgesetzt; er hängt an dieser Entscheidung.
