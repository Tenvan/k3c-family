# BS11 · DEV · Balancing-Tester spielt die ganze Insel bis zum Endboss

- **Status:** erledigt
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-642
- **Start-Commit:** 3e95ba48
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1, mit den Defaults der offenen Fragen

## Ausgangslage

Der Balancing-Tester baut je Lauf eine Insel mit nur einer Stufe (`tools/k3c-workbench/internal/balance/run.go` › `newIsland`); Bots wechseln keine Stufe. Der Meilenstein-Lauf von BAS verlangt „Endboss Insel 1 besiegt“ (`docs/rules/zielkorridore.md` § 6). Entscheidung 🧑 2026-10-26: Variante A, der Tester spielt die ganze Insel (B-642).

## Ziel

Ein Lauf `sim_test` mit `bots: coop2, world: standard` spielt Insel 1 von Wald bis Kristallhöhle und misst Ankunft je Tiefe, Bosse und den Endboss.

**Am Ende sichtbar:** Bericht mit Zeile je Tiefe (Ankunftstag, Miniboss) und „Endboss besiegt Tag n“ bzw. die erreichte Tiefe; Referenzlauf mit den alten Werten liegt vor.

## Beteiligte und Zielgruppen

Claude (Meilenstein-Läufe in BS3, BS8, BS9, BS6), 🧑 gibt frei.

## Anforderungen

B-642 › Anforderungen.

## Nicht-Ziele

Mehrere Inseln und Inselwechsel (BS5, BS9); Werteänderungen (BS3); neue Sim-Mechanik (fehlt etwas: Ticket SIM).

## Regeln und Einschränkungen

Abhängig von BS1.1 (Seed-Modus, `sim_test` offline mit `bots`). BS3 wartet auf BS11. Domäne DEV; Bots nur über `sim.PlayerCommand`; deterministisch. Alle Läufe über `sim_test`.

**⚠️ Workbench-Neustart:** BS11.1 bis BS11.4 ändern Go-Code der Workbench → Neustart deutlich melden (Ergebnis Zeile 1, `notify_ui` warn, Chat); Läufe über `sim_test` erst danach (BS11.4 prüft `workbench_status`).

## Beispiele

B-642 › Beispiele.

## Ausnahme- und Fehlerfälle

B-642 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Insel-Lauf mit allen Tiefen der Welt `standard`, ohne `World` wie bisher (B-642/AC-01).
- **AC-02** Metriken je Stufe und Bosse im Bericht (B-642/AC-02).
- **AC-03** Bots steigen ab und bauen auf neuen Stufen auf (B-642/AC-03).
- **AC-04** Bots kämpfen in Bossnächten; Ziele `endbossDefeated`, `endbossDay` (B-642/AC-04).
- **AC-05** `sim_test` nimmt `world` und `days`; Referenzlauf mit alten Werten (B-642/AC-05).
- **AC-06** Entscheidung in `zielkorridore.md` § 6 (B-642/AC-06).

## Offene Fragen

Aus B-642, je mit Default (nicht blockierend): Absteige-Regel der Bots (Hub-Stufe = Tiefe + 2 und Linie 1 steht), Tageslimit 40.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS11.1 | `BS11.1-insel-lauf-metriken.md` | Umsetzung | autonom | fertig |
| BS11.2 | `BS11.2-bots-abstieg.md` | Umsetzung | autonom | fertig |
| BS11.3 | `BS11.3-bots-bosskampf-ziele.md` | Umsetzung | autonom | fertig |
| BS11.4 | `BS11.4-sim-test-world-referenz.md` | Umsetzung | autonom | fertig |
| BS11.5 | `BS11.5-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-26, Claude (AC-01 bis AC-06 laut Ergebnis BS11.1–BS11.5). Referenzlauf `run-1` rot (Endboss 0 %, Kristallhöhle Tag 37), Werte in B-642. Review: zwei mittlere Befunde behoben. `check:race` lokal nicht gelaufen (kein C-Compiler). Neue Tickets: B-643, B-644.
