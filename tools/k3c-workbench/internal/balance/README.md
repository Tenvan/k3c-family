# Balancing-Tester

Bots spielen eine Szenario-Matrix headless gegen `engine/sim` (B-099, BAL1), `task balance` prüft die Zielkorridore
(B-157, BAL2). Aufbau: `codemap.md`. Befehle: `task balance`, `task balance:run`, `task balance:baseline`,
`task balance:sensitivity` (Sensitivität und Grad-Kurven, z. B. `-- --seeds 10 --vary economy.json:purse.startGold`).
Laufzeit: Seit BAL6 prüft `task balance` sechs Szenarien (Bot `wirtschaft` je Grad, Hub-Stufe 3 über 9 Tage), bei 100 Seeds geschätzt
35–40 min (nicht gemessen); zum Entwickeln `task balance -- --seeds 10`.

## Profile und Grad-Kurven

**Beschlossen von 🧑 am 2026-10-05** (Workshop BAL3.1 im Chat; Pflicht-Profile aus Q19 vom 2026-10-03).

### Profile

Alle Profile spielen nur über `sim.PlayerCommand` und stehen nachts an der Burg.

| Name | Profil | Verhalten |
|---|---|---|
| `passive` | passiv | Steht am Hub und zahlt nichts (BAL1). |
| `saver` | sparsam | Erst die nächste bezahlbare Mauer, dann Landstreicher, nur mit genug Gold (BAL1). |
| `walls` | Mauern zuerst | Zahlt erst Mauern (offene Linie) samt Ausbau, dann Türme und den Hub-Ausbau (schaltet die nächste Mauer-Stufe frei); wirbt erst an, wenn davon nichts offen ist. |
| `economy` | Wirtschaft zuerst | Wirbt erst Landstreicher an (Arbeiter), Mauern erst ab Tag 2. |
| `wirtschaft` | Wirtschaft (Standard) | Bot des Standardszenarios (BAL6): Mauern, Turm, Baumschule, Werkstatt (nur mit Material im Vorrat), Hub-Ausbau (Material markieren oder beim Händler kaufen, sonst Gold sparen), Holz bis 150 nachmarkieren, Bögen bis 8 Kämpfer, Landstreicher bis 4 Bauern (mehr als 5 bricht in Nacht 5 der Münzskala zusammen, B-645). |
| `coop2` | Koop 2 Spieler | Rollen geteilt: Monarch 1 spielt „Mauern zuerst“, Monarch 2 „Wirtschaft zuerst“; braucht 2 Spieler. |
| `coop4` | Koop 4 Spieler | Wie Koop 2, die Rollen wiederholen sich (1 und 3 Mauern, 2 und 4 Wirtschaft); braucht 4 Spieler. |

Eine **Mischung je Spieler** (B-611) verbindet Profile mit `+`: `saver+passive` = Monarch 1 `saver`, Monarch 2 `passive`;
genau ein Profil je Spieler, ohne `coop2`/`coop4`. Ein Profil, das mehr Spieler braucht, als das Szenario hat, ist ein Fehler beim Start. Ein „Kind-Bot“ kommt später (Q19).

**Beschluss B-300 (BAL6.3, autonom, Claude, 2026-10-25; Default der Spec):** `walls` zahlt auch Türme und den
Hub-Ausbau und wartet nur noch auf Mauern einer offenen Linie; so spielt es messbar anders als `saver` (baut an Tag 1
einen Turm, `saver` nie). Belegt in `profiles_test.go` (Turm, Mauer auf Stufe 2 nach Hub-Stufe 2). `saver` bleibt
unverändert (BR1-Messung).

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
🧑 2026-10-05). Ein unbekannter Pfad ist ein Fehler mit Pfad, ebenso ein Pfad in einer Datei, die `sim.UseData` nicht
neu lädt (z. B. `difficulty.json`, B-301): Die Variation hätte keine Wirkung. Pfad-Syntax `datei.json:a.b.c` (Listen über den Index); „gekippt“ heißt:
die Bewertung eines Ziels aus `data/balance-targets.json` ändert sich gegenüber dem unveränderten Lauf.
