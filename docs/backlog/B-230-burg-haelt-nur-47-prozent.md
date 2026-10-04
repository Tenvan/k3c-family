# B-230 · Burg hält Nacht 1–5 nur in 47 % der Seeds, Ziel 75–90 %

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der erste volle `task balance`-Lauf (BAL2.2, 100 Seeds, 2 Spieler, Bot `saver`, Wald, 5 Tage) ergibt für „Burg hält Nacht 1–5“ 47 % statt 75–90 % (`docs/rules/zielkorridore.md` § 1). Die anderen beiden bewerteten Ziele bestehen. Ob Werte in `data/` oder der Bot `saver` (spielt nur sparsam, BAL1) die Ursache ist, ist ungeprüft.

## Ziel

Klären, ob der Bot oder die Balance den Korridor reißt, und beides (Bot-Profil, Werte oder Korridor) mit Beschluss von 🧑 angleichen.

## Beteiligte und Zielgruppen

🧑 beschließt Zahlen (REG); Review-Sessions der Balancing-Runden B-155 und B-156 lesen den Bericht.

## Anforderungen

- Ursache mit dem Replay eines verletzten Seeds (z. B. Seed 3) und `task balance` belegen.
- Danach liegt der Wert im Korridor oder der Korridor ist mit Begründung geändert (Revision + 1).

## Nicht-Ziele

Neue Bot-Profile als Selbstzweck (BAL3).

## Regeln und Einschränkungen

Werte in `data/` ändert nur REG mit Beschluss; Baseline nur mit Begründung im Commit aktualisieren.

## Beispiele

`task balance` → „Burg hält Nacht 1–5: 47 %, Ziel 75–90 %: Fail, Seeds 3, 4, 6 …“.

## Ausnahme- und Fehlerfälle

nicht relevant (Auswertung eines Messlaufs).

## Akzeptanzkriterien

- **AC-01** `task balance` zeigt für „Burg hält Nacht 1–5“ Pass oder der Korridor ist mit Beschluss angepasst.

## Offene Fragen

Bot-Schwäche oder Balance? Entscheidet 🧑 nach der Analyse.

## Notizen

Messung BAL2.2: 100 Läufe in 1 min 37 s (Windows-Entwicklungsrechner), Bericht `reports/balance-<Zeit>.md`.
