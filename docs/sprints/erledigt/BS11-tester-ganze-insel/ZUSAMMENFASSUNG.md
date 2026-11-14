# BS11 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Balancing-Tester spielt die ganze Insel bis zum Endboss
- **Projekt:** BAS

## Ziel

Ein Lauf `sim_test` mit `bots: coop2, world: standard` spielt Insel 1 von Wald bis Kristallhöhle und misst Ankunft je Tiefe, Bosse und den Endboss. **Am Ende sichtbar:** Bericht mit Zeile je Tiefe (Ankunftstag, Miniboss) und „Endboss besiegt Tag n“ bzw. die erreichte Tiefe; Referenzlauf mit den alten Werten liegt vor.

## Ergebnis

2026-10-26, Claude (AC-01 bis AC-06 laut Ergebnis BS11.1–BS11.5). Referenzlauf `run-1` rot (Endboss 0 %, Kristallhöhle Tag 37), Werte in B-642. Review: zwei mittlere Befunde behoben. `check:race` lokal nicht gelaufen (kein C-Compiler). Neue Tickets: B-643, B-644.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Insel-Lauf mit allen Tiefen der Welt `standard`, ohne `World` wie bisher (B-642/AC-01). |
| AC-02 | geprüft | Metriken je Stufe und Bosse im Bericht (B-642/AC-02). |
| AC-03 | geprüft | Bots steigen ab und bauen auf neuen Stufen auf (B-642/AC-03). |
| AC-04 | geprüft | Bots kämpfen in Bossnächten; Ziele `endbossDefeated`, `endbossDay` (B-642/AC-04). |
| AC-05 | geprüft | `sim_test` nimmt `world` und `days`; Referenzlauf mit alten Werten (B-642/AC-05). |
| AC-06 | geprüft | Entscheidung in `zielkorridore.md` § 6 (B-642/AC-06). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-642 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Der Balancing-Tester spielt nur eine Stufe, der Meilenstein-Lauf braucht die gan … |
