# Regelwerk: Spielstruktur, Stufen, Niederlage und Ziel

Beschlossen von 🧑 im Workshop R1.3 am 2026-10-02 (Grundlage: [`ist-abgleich.md`](ist-abgleich.md), Wirtschaft und Schwierigkeitsgrade: [`wirtschaft.md`](wirtschaft.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Zielkorridore prüft später der Balancing-Tester (B-099) mit dem
Standardszenario **Wald, 2 Spieler, Bot „sparsam“, Normal, je 100 Seeds**.

> **Wichtig, ändert die bisherige Planung und den Ist-Stand:** Ein Spielstand ist **kein** Hintereinander von Stufen, die alle gemeinsam
> betreten werden (wie in K2C), sondern: **1 Spielstand/Raum → n Inseln → pro Insel n Stufen.** Die Stufen einer Insel gehören zu
> **einem** Level und sind **pro Spieler frei begehbar**; jeder Spieler kann sich in einer anderen Stufe aufhalten.
> Im Code ist das heute anders (eine Stufe = eine Welt, alle reisen gemeinsam, `engine/sim/campaign.go`, `travel.go`).
> Entscheidung: [`../decisions/003-spielstruktur-inseln-stufen.md`](../decisions/003-spielstruktur-inseln-stufen.md).

## 1. Aufbau: Spielstand, Inseln, Stufen

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Ein Raum hat einen Spielstand, der Spielstand hat **n Inseln**, jede Insel hat **n Stufen** (Oberwelt/Wald, Höhle, Mine, später mehr). Die erste Ausbaustufe ist **1 Insel mit 3 Stufen**; weitere Inseln kommen später. | Struktur trägt Koop mit freier Wahl des Ortes; wächst über Inseln statt über beliebig viele Stufen. | `data/biomes/*.json` (Stufen), neue Insel-Daten (SIM legt sie an) | – |
| Die Stufen einer Insel bilden **ein Level** und sind **pro Spieler frei begehbar**: Jeder Spieler wechselt allein über den Tiefen-Eingang (2 s stehen) oder eine gebaute Treppe; es gibt keine gemeinsame Reise. | Spieler sollen verschiedene Aufgaben an verschiedenen Orten übernehmen können. | `data/hub.json` › `travel` | – |
| Jede Stufe hat **einen eigenen Hub**, der von Grund auf gebaut wird; alle Hubs bleiben bestehen. | Wie bisher. | `data/hub.json` | – |
| **Alle Stufen einer Insel laufen weiter**, auch wenn kein Spieler dort ist (eine gemeinsame Zeit, eigene Wellen je Stufe). Ein Hub ohne Verteidiger kann fallen. | Die Entscheidung „wo bin ich?“ ist Spielinhalt. Rechenlast wird mit B-099 und SP11 (Pi 3) gemessen. | – | Rechenzeit je Tick mit 3 Stufen aktiv im Ziel aus B-042 (p99 < 10 ms bei 2 Räumen × 3 Spielern) |
| **Inselwechsel:** Der Endboss der tiefsten Stufe macht den Weg zur nächsten Insel frei (Boot oder Portal). Der Wechsel erfolgt **gemeinsam**: alle lebenden Spieler stehen am Boot/Portal. Die neue Insel ist eine neue Welt mit eigenen Hubs. | Der Fortschritt bleibt ein gemeinsamer Meilenstein, Inseln folgen einander. | neue Insel-Daten | – |
| Die Wellenstärke einer Stufe skaliert mit der **Anzahl der Spieler der Insel** (Faktor `1 + 0,5 × (Spieler − 1)`), nicht mit den Spielern in genau dieser Stufe. | Einfach und gleichmäßig; ein Spieler allein in der Tiefe trifft die Wellen für alle. | `data/waves.json` | Korridore je Spieleranzahl wie in `wirtschaft.md` |
| Skalierung der Gegner: je Stufe multiplikativ wie heute (HP ×1,5, Schaden ×1,3, Tempo ×1,1 je Tiefe), und **je Insel eine eigene Tabelle** statt einer Formel. | Jede Insel lässt sich frei abstimmen. | `data/waves.json` › `depthScaling`, neue Insel-Tabellen | – |

## 2. Aggressionspool (unter Tage)

Unverändert (🧑 hat nicht gesondert bestätigt): Unter Tage statt Tag/Nacht: +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource; bei 100 % kommt eine Welle und der Pool geht auf 0. Der globale Tag/Nacht-Zyklus läuft weiter (Nacht-Gegner wie Skelette). Daten: `data/biomes/cave.json`, `mine.json`. Zielkorridor: Wellen unter Tage im Abstand von 4–12 Minuten Spielzeit (Normal, Mine, Ressourcen gesammelt) – Startziel, Messgröße mit B-099.

## 3. Bosse und Ziel der Kampagne

- **Minibosse:** je Stufe ein Miniboss. **Endboss:** in der tiefsten Stufe jeder Insel; sein Sieg macht den Weg zur nächsten Insel frei. Boss-Werte und Verhalten legt Regelwerk III fest (`gegner-truppen.md`).
- **Ziel der Kampagne (Standard):** Der **Endboss der letzten Insel** wird besiegt. In der ersten Ausbaustufe (1 Insel) ist das der Endboss der Mine.
- **Siegvarianten:** Das Ziel ist eine **Raum-Option** (siehe 5), **eine Variante je Raum**; Minibosse gibt es immer. Varianten:

| Variante | Sieg, wenn … | Startwert |
|---|---|---|
| Endboss (Standard) | der Endboss besiegt ist | – |
| Gold sammeln | die Spieler zusammen N Gold eingesammelt haben (Summe aller eingesammelten Münzen) | N = 1000 |
| Tage überleben | N Tage vergangen sind, ohne Spielende | N = 20 |
| Alles abbauen | alle Ressourcenknoten der Insel abgebaut sind | – |
| Alles ausbauen | alle Bauplätze aller Stufen der Insel gebaut sind | – |

Annahme (🧑 hat nicht gesondert bestätigt): Eine Variante gilt je Insel; das Erfüllen schaltet bei mehreren Inseln den Inselwechsel frei, der letzte Sieg ist der Kampagnensieg.

**Zielkorridore** (Insel 1, Normal, 2 Spieler, Bot „sparsam“, je 100 Seeds): Die Höhle wird vor Tag 6 erreicht in ≥ 80 %; der Miniboss je Stufe ist besiegt bis Tag 8 (Wald) in 60–85 %; der Endboss ist besiegt bis Tag 25 in 30–55 %.

## 4. Niederlage

Fällt die **Burg einer Stufe**, wirkt der **Niederlage-Modus** des Raums (Raum-Option, Standard nach Schwierigkeitsgrad, einzeln änderbar):

| Modus | Wirkung |
|---|---|
| **Gold/Material-Verlust** | Gold je Spieler und Material des Hubs je −50 %; Bauten und Truppen bleiben. Die Burg steht sofort wieder. |
| **Stufenverlust** | Hub der gefallenen Stufe wird zurückgesetzt (alle Bauplätze unbezahlt, alle Truppen weg außer Landstreichern, Material −50 %); Spieler dieser Stufe verlieren 50 % ihres Golds. Die Burg steht sofort wieder. |
| **Komplett verloren** | Game Over: der Raum endet, der Spielstand bleibt wie zuletzt gespeichert. |

Standard je Grad: **Dev und Leicht: Gold/Material-Verlust · Normal und Hart: Stufenverlust · Ultra: Komplett verloren.**
**Monarch tot:** Respawn an der Burg nach 5 s ohne Strafe (unverändert). Daten: neu `data/difficulty.json` (SIM legt Feld an); `engine/sim/world.go` › `castleFallen` ist der Ist-Stand von „Stufenverlust“. Zielkorridore: siehe `wirtschaft.md` § 4 (Burg hält Nacht 1–5).

## 5. Raum-Optionen

Beim Anlegen eines Raums (und im Dev-Mode im Debug-Panel) lassen sich einstellen; Standard nach Schwierigkeitsgrad, jede Option einzeln überschreibbar (Grad wirkt ab der nächsten Welle):

1. **Schwierigkeitsgrad** (Dev, Leicht, Normal, Hart, Ultra; `wirtschaft.md` § 4),
2. **Ziel** (Siegvariante, § 3),
3. **Niederlage-Modus** (§ 4).

Die Optionen stehen im Spielstand. **Dev** ist nur im Dev-Mode wählbar.

## 6. Steuerung

Die Taste X bleibt für Skills frei (`wirtschaft.md` § 6). **Das Skill-Menü liegt nicht auf View**, weil View + Menu gemeinsam „zurück zur Landingpage“ ist; die neue Taste legt Regelwerk II fest (Vorschlag: LB + RB).

## 7. Offen und Annahmen

- **Material je Hub oder je Insel?** Nicht besprochen: Heute gehört das Material dem Hub (je Stufe). Entscheidung mit der Umsetzung (SIM-Ticket) oder in Regelwerk III.
- **Anzahl und Reihenfolge der Inseln** über die erste Insel hinaus sind offen (Daten, später).
- **Kosten und Belastung:** Der Umbau von „Welt = Stufe“ zu „Level = Insel mit n Stufen“ ist groß (`engine/sim`, `engine/room`, Protokoll, Client-Kameras je Spieler). R1.4 legt dafür Tickets an; SP11 (Pi 3) und B-099 messen die Last mit allen Stufen aktiv.
- **Wolf bei Vollmond:** bleibt als Event-Idee für später (Regelwerk III), kein Ticket jetzt.
- Annahmen ohne gesonderte Bestätigung: Aggressionspool unverändert; ein Spieler wechselt die Stufe einzeln wie heute (2 s am Eingang/an der Treppe); Siegvarianten gelten je Insel.
