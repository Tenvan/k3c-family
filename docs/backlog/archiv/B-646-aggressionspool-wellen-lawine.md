# B-646 · Unter Tage löst jede Welle die nächste aus: Der Aggressionspool läuft in eine Lawine

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1

## Ausgangslage

Gefunden vom Stufen-Test für ein komplettes Spiel mit einem Bot (BS3.4, `task balance:fullgame STUFE=4`, `K3C_FULLGAME=voll`): Unter Tage (Aggressionspool, `world_cycle.go`) füllt jeder Kill +5 % (`percentPerKill`, `data/biomes/mine.json` u. a.). Eine Welle hat mehr als 20 Gegner; tötet der Hub sie, ist der Pool sofort wieder über 100 %, und die nächste Welle startet (Reset und `startWave`). Die Beschwörung der Ratten-Königin und die Rattenschwärme verstärken das. Im Test stand die Mine (Stufe 2) am Tag 18 bei Welle 499, am Tag 41 bei Welle 25 461 mit 854 Gegnern gleichzeitig; ein 40-Tage-Lauf dauerte 947 s statt etwa 1 min.

## Ziel

Unter Tage kommt eine Welle höchstens im Abstand der Regel (4–12 min Spielzeit, `zielkorridore.md` § 1), egal wie viele Gegner der Hub tötet; die Simulation bleibt über 40 Tage schnell.

## Beteiligte und Zielgruppen

Claude (Simulation), 🧑 entscheidet über die Regel.

## Anforderungen

- Entscheidung nötig (Vorschlag, ungeprüft): Der Pool wächst nicht, solange Gegner der laufenden Welle leben oder in der Warteschlange stehen; zusätzlich Mindestabstand zwischen zwei Wellen nach Regel.
- Alternativen: `percentPerKill` stark senken (Wert, `data/biomes/*.json`), oder Kills von Beschwörungen und Schwärmen zählen nicht.
- Golden-Läufe mit Aggressionspool ändern sich (cave-aggression u. a.), Begründung in den Commit.

## Nicht-Ziele

Balance der Wellenstärke; Oberwelt (Tag und Nacht).

## Regeln und Einschränkungen

Domäne SIM (`engine/sim/world_cycle.go`); deterministisch; Golden über `task go:golden` mit Begründung; Regelwerk `stufen.md` § 2 und `gegner.md` ändern.

## Beispiele

Mine, Hub 5, 9 Kämpfer: heute Welle 499 am Tag 18; erwartet höchstens eine Welle je 4 min Spielzeit.

## Ausnahme- und Fehlerfälle

Der Spieler sammelt viele Ressourcen schnell (+1 % je Sammeln): Der Pool füllt sich, die Welle kommt, aber nicht schneller als der Mindestabstand.

## Akzeptanzkriterien

- **AC-01** Sim-Test: Mine mit 20 Kämpfern, 10 Spielminuten Dauerkämpfe: höchstens 3 Wellen.
- **AC-02** `task balance:fullgame STUFE=4` (voll) läuft in unter 3 min bis zum Ende.

## Offene Fragen

Welche Regel (Vorschlag oben, Alternative Wert)? 🧑 entscheidet; Default: Pool friert ein, solange Gegner der Welle leben, plus Mindestabstand 4 min.

## Notizen

Reproduktion: `K3C_FULLGAME=voll task balance:fullgame STUFE=4` (Log je Tag zeigt Pool, Welle, Gegner je Stufe). Gefunden zusammen mit: Der Endboss `crystalHeartWarden` (10 024 HP) wird vom einzelnen Bot nie getroffen; dort ist eine zweite Entscheidung nötig (B-645).
