# B-203 · Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-099 verlangt die Kennzahl „Tick der ersten Gold-Schwelle“, nennt aber keine Schwelle. Der Balancing-Tester (BAL1.1, `tools/k3c-dev/internal/balance/`) misst sie als ersten Tick, an dem das Gold aller Monarchen zusammen mindestens `--gold-threshold` erreicht (0 = nicht messen). Jeder Monarch startet mit 100 Gold, das zugleich das Maximum ist (`data/economy.json` › `purse`); jede Schwelle bis 100 × Spieler ist also schon in Tick 1 erreicht.

## Ziel

Die Kennzahl sagt etwas über den Spielverlauf aus und hat einen festen Standardwert für den Bericht (BAL2).

## Beteiligte und Zielgruppen

🧑 entscheidet; Agenten setzen im Tester um.

## Anforderungen

- Bedeutung der Schwelle festlegen (z. B. Gold je Spieler oder zusammen, erst nach dem ersten Ausgeben, Gold plus Material).
- Standardwert in `docs/rules/zielkorridore.md` oder `data/`.

## Nicht-Ziele

Zielkorridor-Prüfung selbst (BAL2, B-157).

## Regeln und Einschränkungen

Werte nur über REG-Beschluss; Messung im Tester (SIM-Sprint der BAL-Schiene).

## Beispiele

„Erster Tick nach Tag 1, an dem ein Monarch wieder 50 Gold hat“ → Zahl im Bericht je Seed.

## Ausnahme- und Fehlerfälle

Schwelle nie erreicht → `null` (heute schon so).

## Akzeptanzkriterien

- **AC-01** Bedeutung und Standardwert der Gold-Schwelle stehen in `docs/rules/` und der Tester misst danach.

## Offene Fragen

- Welche Bedeutung und welcher Wert? Entscheidet 🧑.

## Notizen

–
