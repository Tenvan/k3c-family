# Balancing-Tester

Bots spielen eine Szenario-Matrix headless gegen `engine/sim` (B-099, BAL1), `task balance` prüft die Zielkorridore
(B-157, BAL2). Aufbau: `codemap.md`. Befehle: `task balance`, `task balance:run`, `task balance:baseline`,
`task balance:sensitivity` (Sensitivität und Grad-Kurven, z. B. `-- --seeds 10 --vary economy.json:purse.startGold`).

## Profile und Grad-Kurven

**Beschlossen von 🧑 am 2026-10-05** (Workshop BAL3.1 im Chat; Pflicht-Profile aus Q19 vom 2026-10-03).

### Profile

Alle Profile spielen nur über `sim.PlayerCommand` und stehen nachts an der Burg.

| Name | Profil | Verhalten |
|---|---|---|
| `passive` | passiv | Steht am Hub und zahlt nichts (BAL1). |
| `saver` | sparsam | Erst die nächste bezahlbare Mauer, dann Landstreicher, nur mit genug Gold (BAL1). |
| `walls` | Mauern zuerst | Zahlt nur Mauern, auch deren Ausbau, und wirbt erst an, wenn keine Mauer mehr offen ist. |
| `economy` | Wirtschaft zuerst | Wirbt erst Landstreicher an (Arbeiter), Mauern erst ab Tag 2. |
| `coop2` | Koop 2 Spieler | Rollen geteilt: Monarch 1 spielt „Mauern zuerst“, Monarch 2 „Wirtschaft zuerst“; braucht 2 Spieler. |
| `coop4` | Koop 4 Spieler | Wie Koop 2, die Rollen wiederholen sich (1 und 3 Mauern, 2 und 4 Wirtschaft); braucht 4 Spieler. |

Ein Profil, das mehr Spieler braucht, als das Szenario hat, ist ein Fehler beim Start. Ein „Kind-Bot“ kommt später (Q19).

### Grad-Kurven

Je Schwierigkeitsgrad aus `data/difficulty.json` (`dev`, `easy`, `normal`, `hard`, `ultra`) eine Tabelle über die
Nächte 1 bis 5 im Standardszenario (Wald, 2 Spieler, `saver`):

- Anteil der Seeds, in denen die Burg die Nacht hält,
- Median der zerstörten Gebäude in der Welle der Nacht,
- Median des Golds aller Monarchen zur Dämmerung.

### Sensitivität

Variation um −25 %, −10 %, +10 %, +25 %; standardmäßig diese vier Werte, weitere Pfade per Flag:

- `economy.json` › `purse.startGold`
- `economy.json` › `dawnGoldPerPlayer`
- `buildings.json` › `wall.cost.gold`
- `waves.json` › `perExtraPlayer`

Der Lauf ändert keine Datei in `data/`; die Sim lädt die variierten Daten nur im Speicher (`sim.UseData`, Beschluss
🧑 2026-10-05). Ein unbekannter Pfad ist ein Fehler mit Pfad. Pfad-Syntax `datei.json:a.b.c` (Listen über den Index); „gekippt“ heißt:
die Bewertung eines Ziels aus `data/balance-targets.json` ändert sich gegenüber dem unveränderten Lauf.
