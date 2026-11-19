# Regelwerk: Koop-Aktionen

Beschlossen von 🧑 in der Ideenrunde k3c-07 am 2026-11-15 (Ticket B-968, Sprint SI2; Rahmen: [`wirtschaft.md`](wirtschaft.md) § 2, [`monarch.md`](monarch.md), [`welten.md`](welten.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Alle Werte sind **vorläufig** (Defaults, von 🧑 bestätigt; Balancing später).
**Grundsatz:** Basiswerte entsprechen Solo; der Koop-Bonus läuft nur über Multiplikator-Felder, nie über getrennte Koop-Werte.

## 1. Grundsatz Koop-Aktionen

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Eine **Koop-Aktion** ist etwas, das nur mit zwei gesteuerten Monarchen derselben Stufe gemeinsam geht und dafür mehr bringt. | Das Zusammenspiel auf der Couch wird belohnt. | – | – |
| **Solo verliert nichts:** Ohne zweiten gesteuerten Monarchen gibt es die normale Variante der Aktion mit normalem Ertrag. | Solo bleibt vollwertig; Basiswerte = Solo. | – | – |
| Der Koop-Bonus ist ein **Multiplikator** auf den Solo-Wert (Feld in `data/coop.json`). | Koop-Werte nur über Multiplikator-Felder. | `data/coop.json` | – |
| **Zwei Monarchen genügen**, auch bei mehr als zwei Spielern. Ein Monarch ohne Krone zählt mit, solange er gesteuert wird. | Einfach und fair; Kronenverlust bestraft schon genug. | – | – |
| „**Gleichzeitig davor**“: zwei gesteuerte Monarchen derselben Stufe stehen im selben Tick im Aufheb-Radius des Ortes. Stehen sie in verschiedenen Stufen, bleibt der Ort zu. | Eindeutig prüfbar, Takt der Simulation. | – | – |
| Die Aktion berührt weder Level-Generator noch Spielstand; Kennzeichen werden berechnet, nicht gespeichert. | Keine Golden-/Spielstand-Änderung. | – | – |

## 2. Doppel-Truhe

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Die **Doppel-Truhe** ist eine Truhe mit zwei Schlössern. Sie öffnet sich nur, wenn **beide Monarchen gleichzeitig davorstehen**. | Erste Koop-Aktion; zwingt zum gemeinsamen Gang. | – | – |
| **Kennzeichen:** Welche Truhe doppelt ist, berechnet die Simulation deterministisch aus Insel-Seed, Stufe und Position der Truhe (`engine/rng`, kein `math/rand`), im Mittel jede **`everyNth`-te** Truhe. Der Level-Generator bleibt unverändert, das Kennzeichen wird nicht gespeichert. | Gleicher Seed, gleiche Doppel-Truhen; keine Änderung an Golden und Spielstand. | `data/coop.json` › `doubleChest.everyNth` (vorläufig 4) | Anteil der Doppel-Truhen etwa 1/`everyNth` über viele Seeds |
| Das Kennzeichen **wirkt nur bei mindestens zwei gesteuerten Monarchen**. Sonst verhält sich die Truhe wie eine normale: Öffnen allein, normales Gold. | Solo verliert nichts. | – | – |
| **Steht nur ein Monarch davor**, bleibt die Truhe zu und zeigt, dass der zweite fehlt; der Zustand nennt je wartender Doppel-Truhe den Monarchen, der davorsteht (Grundlage für das leuchtende Schloss, B-998). | Der Spieler versteht, worauf die Truhe wartet. | Zustand `World::coop.chests` (je wirksame Doppel-Truhe `id`, `x`, `double`, `waiting` = Index des Wartenden) | – |
| **Beute:** Gold = normales Truhen-Gold × **`goldFactor`**, als **Fontäne** ausgeworfen (wie B-953); beide sammeln, was sie erwischen. | Deutlich mehr Gold als eine normale Truhe; gemeinsames Sammeln. | `data/economy.json` › `chestGold` (Basis), `data/coop.json` › `doubleChest.goldFactor` (vorläufig 3), Fontäne `engine/sim/economy_fountain.go` | Gold je Doppel-Truhe = `goldFactor` × normale Truhe |
| **Zusatz für beide:** Was eine normale Truhe dem Öffner zusätzlich gibt (Skill-Punkt, `chestSkillPoint`), bekommen **beide** Monarchen (zwei Aufrufe von `chest_skill_point`, die Doppel-Truhe zählt also als zwei geöffnete Truhen für `chestEvery`). | Beide haben die Truhe geöffnet. | `data/economy.json` › `chestSkillPoint` | – |
| **Schwankende Spielerzahl:** Geht ein Spieler, während Doppel-Truhen ungeöffnet sind, verhalten sie sich wie normale Truhen und öffnen sich für den verbliebenen Monarchen allein; kommt wieder ein zweiter Spieler, sind sie wieder Doppel-Truhen. | Kein Spieler wird durch das Gehen des anderen ausgesperrt. | – | – |
| **Stufe ohne Truhe** hat keine Doppel-Truhe. | Kennzeichen hängt an einer Truhe. | – | – |

## 3. Offen und Annahmen

- **Vorläufige Defaults (🧑 2026-11-15 bestätigt):** `doubleChest.goldFactor` 3, `doubleChest.everyNth` 4 (RU62.3); Balancing später. Die Felder stehen in `data/coop.json` (mit `worlds`: nur dort wirkt die Doppel-Truhe, `standard` nicht), weil `data/economy.json` für die Go-Engine unverändert bleibt.
- Zwei Monarchen genügen bei mehr als zwei Spielern; Monarch ohne Krone zählt mit; Kennzeichen bleibt bei Spielerwechsel, wirkt nur bei zwei gesteuerten Monarchen.
- Anzeige (Asset, leuchtendes Schloss) steht in B-998, nicht hier.
- **Spätere Kandidaten, nicht beschlossen:** gemeinsam bezahlen und bauen, gemeinsamer Angriff, besondere Beute (Ausrüstung, Material), Truppe als zweiter Schlüssel.
- Kennzahlen fehlen (Anteil Doppel-Truhen, Öffnungen je Lauf); eine Messung bleibt dem Balancing vorbehalten.
