# B-642 · Der Balancing-Tester spielt nur eine Stufe, der Meilenstein-Lauf braucht die ganze Insel

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Balancing-Tester baut je Lauf eine Insel mit genau **einer** Stufe (`tools/k3c-workbench/internal/balance/run.go` › `newIsland`: `sim.CreateIsland(seed, []int{depth}, …)`). Bots wechseln keine Stufe und kämpfen nicht gezielt gegen Bosse. Der Meilenstein-Lauf von BAS (`docs/rules/zielkorridore.md` § 6) verlangt aber „Endboss der Insel 1 besiegt“, und die Endboss-Stufe ist die tiefste der Welt `standard` (Tiefen 0–4).

## Ziel

Klären, woran Meilenstein 1 gemessen wird, bevor BS3.4 läuft.

## Beteiligte und Zielgruppen

🧑 entscheidet, Claude setzt um.

## Anforderungen

Entscheidung zwischen:
- **A** Tester spielt die ganze Insel der Welt `standard` (alle Stufen, Bots steigen ab, Bosskampf). Großer Umbau (Bots, Metriken, Laufzeit), eigener Sprint vor BS3.
- **B** Meilenstein 1 = Wald-Lauf `coop2` über N Tage (Vorschlag 10) ohne Burgfall, Miniboss Wald besiegt, Pflicht-Korridore im Bericht; die ganze Insel kommt später (eigenes Ticket, vor M3).
- **C** Wie B, aber zusätzlich je Tiefe ein Einzel-Lauf (Tiefen 0–4 nacheinander, Startvorrat je Tiefe), Endboss-Tiefe muss halten.

## Nicht-Ziele

Umsetzung vor der Entscheidung.

## Regeln und Einschränkungen

Läufe nur über `sim_test`.

## Beispiele

nicht relevant, Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant, Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** Die Entscheidung steht in `docs/rules/zielkorridore.md` § 6, BS3.4 und B-635 sind angepasst.

## Offene Fragen

- A, B oder C? → 🧑. Vorschlag Claude: **C** (realistisch in BS3, misst jede Stufe mit den neuen Werten; A als eigener Sprint vor M3).

## Notizen

Gefunden in BS1.1 (2026-10-26): Der Parameter `world` für `sim_test` ist deshalb noch nicht umgesetzt; er hängt an dieser Entscheidung.
