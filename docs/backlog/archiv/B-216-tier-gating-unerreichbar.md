# B-216 · Tier-Gating 5/10/15 je Linie ist mit einem Punkt je Skill unerreichbar

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** S1
- **Projekt:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S1.1 (`docs/sprints/aktiv/S1-monarch-schlag-skills/S1.1-schlag-pool.md`) legt fest: Ein Punkt kauft einen Skill oder ein Passiv, jeden höchstens einmal; „Punkte in der Linie“ = Anzahl gelernter Skills der Linie; Tier n verlangt `tierPoints[n-1]` = 0/5/10/15 gelernte Skills der Linie (`docs/rules/monarch.md` § 3). Der Katalog aus `docs/game-design.md` hat aber je Linie nur: Tank 2 × Tier 1, 5 Passive (Tier 2), je 1 × Tier 3 und 4 (9 Skills); Zauberer 2 + 4 + 1 + 1 (8); Heiler 2 + 3 + 1 + 1 (7). Vor Tier 2 sind höchstens 2 Skills der Linie lernbar (< 5), vor Tier 3 höchstens 7 (< 10). Damit sind Passive, Tier 3 und Tier 4 in keiner Linie erreichbar, und Test (c) der Session („Tier 3 mit 10 gelernten Skills der Linie erlaubt“) lässt sich nicht ehrlich schreiben.

## Ziel

Eine Gating-Regel, mit der jeder Tier jeder Linie erreichbar ist und die Spezialisierung trotzdem belohnt.

## Beteiligte und Zielgruppen

🧑 entscheidet (Regelwerk REG); Umsetzung in S1.1 (SIM).

## Anforderungen

- Jeder Tier jeder Linie (Tank, Zauberer, Heiler) ist mit dem Katalog erreichbar.
- Die Regel steht in `docs/rules/monarch.md` § 3 und in der Session S1.1 (Test (c)).

## Nicht-Ziele

Wirkungen der Skills (S1.2a bis S1.2c), Dieb-Linie.

## Regeln und Einschränkungen

`docs/rules/monarch.md` § 3; Werte nur in `data/monarch.json` (`tierPoints`).

## Beispiele

Mögliche Auflösungen (Vorschläge des Agenten, nicht beschlossen):
1. Gating zählt alle gelernten Skills des Spielers (linienübergreifend) statt nur der Linie.
2. Schwellen passend zum Katalog, z. B. `tierPoints: [0, 2, 5, 7]` je Linie (Tank: 2 Tier-1 → Passive; 2 + 3 Passive → Tier 3 …), je Linie verschieden oder als kleinste gemeinsame Schwelle.
3. Skills haben Ränge (mehrere Punkte je Skill), „Punkte in der Linie“ = ausgegebene Punkte.

## Ausnahme- und Fehlerfälle

nicht relevant (Regelfrage).

## Akzeptanzkriterien

- **AC-01** `docs/rules/monarch.md` § 3 und S1.1 (Test (c)) nennen eine Gating-Regel, unter der jeder Tier jeder Linie mit dem Katalog erreichbar ist.

## Offene Fragen

Welche der Auflösungen oben (oder eine andere)? Entscheidet 🧑; danach kann S1.1 weiterlaufen.

## Notizen

Gefunden in S1.1 vor der Umsetzung (Session `Status: blockiert`).

**Ergebnis (2026-10-04):** 🧑 hat im Chat entschieden: Gating je Linie gezählt, `tierPoints: [0, 2, 4, 6]` (gelernte Skills der Linie, jeder Skill weiterhin nur einmal). Damit erreichbar: Tank 2 Tier 1 → Passive → Tier 3 ab 4, Tier 4 ab 6; Zauberer und Heiler ebenso (2 + Passive + Tier 3). AC-01 umgesetzt: `docs/rules/monarch.md` § 3 und S1.1 Test (c) nennen die Regel; Sprint S1 Revision 2.
