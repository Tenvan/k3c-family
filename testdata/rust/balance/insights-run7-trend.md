## Kurzauswertung

- 4 von 13 Zielen verletzt, 2 knapp, 7 im Korridor: der Lauf besteht nicht.
- Die Burg hält in allen 5 Graden im Korridor.
- Am weitesten daneben: Hub-Stufe 2 erreicht vor Tag 5 mit 0,0 % statt 60–85 %; Hub-Stufe 3 erreicht vor Tag 10 mit 0,0 % statt 40–70 %.
- Größter Hebel: Stein-Kosten Hub-Stufe 2 (Schritt 1) senken (hub.json:levels.1.steps.0.cost.stone, je Schritt um 10 % ändern).
- Alle Kennzahlen liegen im oder nahe am Richtwert.
- Gegenüber der Baseline wechseln 1 Ziele den Status, 1 kippen.

## Ampeln

🟢 7 gut · 🟡 2 knapp · 🔴 4 verletzt · ⚪ 0 ungültig

So liest du die Tabelle: **Wert** ist das Messergebnis (bei Anteilen: in wie viel Prozent der Läufe das Ziel erreicht wurde, „Läufe“ sind die Seeds). **Korridor** ist der Soll-Bereich dafür. **Abstand** ist der Unterschied zur verletzten Grenze in Prozentpunkten (Pkt.) bzw. in der Einheit des Werts; im Korridor steht dort die Reserve bis zur nächsten Grenze. **Trend** vergleicht mit der Baseline (▲ näher am Korridor, ▼ weiter weg).

| Ampel | Ziel | Grad | Wert | Korridor | Abstand zum Korridor | Trend |
|---|---|---|---|---|---|---|
| 🟡 knapp | Burg hält Nacht 1–5 | normal | 80,0 % | 75–90 % | 5 Pkt. Reserve | ▲ 0 |
| 🟡 knapp | Burg hält Nacht 1–5 | dev | 100,0 % | 99–100 % | 1 Pkt. Reserve | ▲ 0 |
| 🟢 ok | Burg hält Nacht 1–5 | easy | 100,0 % | 90–100 % | 10 Pkt. Reserve | ▲ 0 |
| 🟢 ok | Burg hält Nacht 1–5 | hard | 60,0 % | 50–70 % | 10 Pkt. Reserve | ▲ 0 |
| 🟢 ok | Burg hält Nacht 1–5 | ultra | 35,0 % | 25–45 % | 10 Pkt. Reserve | ▲ 0 |
| 🔴 verletzt | Gold am Morgen je Spieler, Median | normal | 17,0 | 20–70 | 3 zu niedrig | ▼ -5 |
| 🟢 ok | Erste Mauer gebaut vor Ende der hellen Phase | normal | 100,0 % | 90–100 % | 10 Pkt. Reserve | ▲ 0 |
| 🟢 ok | Erster Turm gebaut vor Ende von Tag 2 | normal | 100,0 % | 70–100 % | 30 Pkt. Reserve | ▲ 0 |
| 🟢 ok | Holz am Tagesbeginn ≥ 100, ab Tag 3 | normal | 85,0 % | 70–100 % | 15 Pkt. Reserve | ▲ 0 |
| 🔴 verletzt | Hub-Stufe 2 erreicht vor Tag 5 | normal | 0,0 % | 60–85 % | 60 Pkt. zu niedrig | ▲ 0 |
| 🔴 verletzt | Hub-Stufe 3 erreicht vor Tag 10 | normal | 0,0 % | 40–70 % | 40 Pkt. zu niedrig | ▲ 0 |
| 🔴 verletzt | Zerstörte Gebäude je Welle 1–5, Median | normal | 2,0 | ≤ 1 | 1 Gebäude zu hoch | ▲ 0 |
| 🟢 ok | Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4 und Tag 6 ≥ 8 | normal | 80,0 % | 70–100 % | 10 Pkt. Reserve | ▲ 0 |

## Stärken

- Burg hält Nacht 1–5 (easy): 100,0 % im Korridor 90–100 %.
- Burg hält Nacht 1–5 (hard): 60,0 % im Korridor 50–70 %.
- Burg hält Nacht 1–5 (ultra): 35,0 % im Korridor 25–45 %.
- Erste Mauer gebaut vor Ende der hellen Phase: 100,0 % im Korridor 90–100 %.
- Erster Turm gebaut vor Ende von Tag 2: 100,0 % im Korridor 70–100 %.
- Holz am Tagesbeginn ≥ 100, ab Tag 3: 85,0 % im Korridor 70–100 %.
- Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4 und Tag 6 ≥ 8: 80,0 % im Korridor 70–100 %.
- Erste Mauer 836,5 Ticks im Richtwert.

## Schwächen

- Hub-Stufe 2 erreicht vor Tag 5: 0,0 % statt 60–85 %, 20 Seeds betroffen.
- Hub-Stufe 3 erreicht vor Tag 10: 0,0 % statt 40–70 %, 20 Seeds betroffen.
- Zerstörte Gebäude je Welle 1–5, Median: 2,0 statt ≤ 1, 20 Seeds betroffen.
- Gold am Morgen je Spieler, Median: 17,0 statt 20–70, 20 Seeds betroffen.
- Gekippt gegenüber der Baseline: Gold am Morgen je Spieler, Median (ok → verletzt).

## Tuning-Hinweise


### Stellschrauben nach Wirkung

Derselbe Wert in data/ taucht bei mehreren Verletzungen auf; hier je Wert einmal, mit den Verletzungen, die er verbessern soll.

- Morgen-Gold je Spieler erhöhen `economy.json:dawnGoldPerPlayer` – soll 2 Verletzung(en) verbessern: Hub-Stufe 3 erreicht vor Tag 10; Gold am Morgen je Spieler, Median. Aktuell 2. Probierbereich 2 … 4 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 3.
- Stein-Kosten Hub-Stufe 2 (Schritt 1) senken `hub.json:levels.1.steps.0.cost.stone` – soll 1 Verletzung(en) verbessern: Hub-Stufe 2 erreicht vor Tag 5. Aktuell 25. Probierbereich 17 … 25 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 22.
- Stein-Kosten Hub-Stufe 2 (Schritt 3) senken `hub.json:levels.1.steps.2.cost.stone` – soll 1 Verletzung(en) verbessern: Hub-Stufe 2 erreicht vor Tag 5. Aktuell 50. Probierbereich 35 … 50 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 45.
- Gold-Kosten Hub-Stufe 3 (Schritt 3) senken `hub.json:levels.2.steps.2.cost.gold` – soll 1 Verletzung(en) verbessern: Hub-Stufe 3 erreicht vor Tag 10. Aktuell 10. Probierbereich 7 … 10 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 9.
- Mauer-HP Stufe 1 erhöhen `buildings.json:wall.levels.0.hp` – soll 1 Verletzung(en) verbessern: Zerstörte Gebäude je Welle 1–5, Median. Aktuell 300. Probierbereich 300 … 480 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 360.
- Gegner je Welle 1–5 senken `waves.json:table.0.standard.1` – soll 1 Verletzung(en) verbessern: Zerstörte Gebäude je Welle 1–5, Median. Aktuell 10. Probierbereich 7 … 10 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 9.
- Startgold erhöhen `economy.json:purse.startGold` – soll 1 Verletzung(en) verbessern: Gold am Morgen je Spieler, Median. Aktuell 14. Probierbereich 14 … 19 (1 bis 3 Schritte à 10 % rauf). Startvorschlag: 16.

### Hinweise je Ziel

- 🔴 **Hub-Stufe 2 erreicht vor Tag 5** – Zu niedrig (Wert unter dem Korridor). Eine dieser Stellschrauben probieren: Stein-Kosten Hub-Stufe 2 (Schritt 1) senken oder Stein-Kosten Hub-Stufe 2 (Schritt 3) senken (Zahlen darunter). In 0 von 20 Läufen traf „Hub-Stufe 2 erreicht vor Tag 5“ zu (0 %). Sollwert: 60–85 % der Läufe. Damit 60 Prozentpunkte unter dem Korridor, also zu niedrig. Unverändert gegenüber der Baseline (0 %).
  - Stein-Kosten Hub-Stufe 2 (Schritt 1) senken `hub.json:levels.1.steps.0.cost.stone`
    Aktuell 25. Probierbereich 17 … 25 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 22. Wirkung: „Hub-Stufe 2 erreicht vor Tag 5“ soll von 0,0 % Richtung Korridor 60–85 % steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Stein-Kosten Hub-Stufe 2 (Schritt 3) senken `hub.json:levels.1.steps.2.cost.stone`
    Aktuell 50. Probierbereich 35 … 50 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 45. Wirkung: „Hub-Stufe 2 erreicht vor Tag 5“ soll von 0,0 % Richtung Korridor 60–85 % steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
- 🔴 **Hub-Stufe 3 erreicht vor Tag 10** – Zu niedrig (Wert unter dem Korridor). Eine dieser Stellschrauben probieren: Gold-Kosten Hub-Stufe 3 (Schritt 3) senken oder Morgen-Gold je Spieler erhöhen (Zahlen darunter). In 0 von 20 Läufen traf „Hub-Stufe 3 erreicht vor Tag 10“ zu (0 %). Sollwert: 40–70 % der Läufe. Damit 40 Prozentpunkte unter dem Korridor, also zu niedrig. Unverändert gegenüber der Baseline (0 %).
  - Gold-Kosten Hub-Stufe 3 (Schritt 3) senken `hub.json:levels.2.steps.2.cost.gold`
    Aktuell 10. Probierbereich 7 … 10 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 9. Wirkung: „Hub-Stufe 3 erreicht vor Tag 10“ soll von 0,0 % Richtung Korridor 40–70 % steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Morgen-Gold je Spieler erhöhen `economy.json:dawnGoldPerPlayer`
    Aktuell 2. Probierbereich 2 … 4 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 3. Wirkung: „Hub-Stufe 3 erreicht vor Tag 10“ soll von 0,0 % Richtung Korridor 40–70 % steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
- 🔴 **Zerstörte Gebäude je Welle 1–5, Median** – Zu schwer (Wert über dem Korridor). Eine dieser Stellschrauben probieren: Mauer-HP Stufe 1 erhöhen oder Gegner je Welle 1–5 senken (Zahlen darunter). Median von „Zerstörte Gebäude je Welle 1–5, Median“: 2 Gebäude. Sollwert: höchstens 1. Damit 1 Gebäude über dem Korridor, also zu schwer. Unverändert gegenüber der Baseline (2).
  - Mauer-HP Stufe 1 erhöhen `buildings.json:wall.levels.0.hp`
    Aktuell 300. Probierbereich 300 … 480 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 360. Wirkung: „Zerstörte Gebäude je Welle 1–5, Median“ soll von 2,0 Richtung Korridor ≤ 1 sinken. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Gegner je Welle 1–5 senken `waves.json:table.0.standard.1`
    Aktuell 10. Probierbereich 7 … 10 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 9. Wirkung: „Zerstörte Gebäude je Welle 1–5, Median“ soll von 2,0 Richtung Korridor ≤ 1 sinken. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
- 🔴 **Gold am Morgen je Spieler, Median** – Zu niedrig (Wert unter dem Korridor). Eine dieser Stellschrauben probieren: Morgen-Gold je Spieler erhöhen oder Startgold erhöhen (Zahlen darunter). Median von „Gold am Morgen je Spieler, Median“: 17. Sollwert: 20–70. Damit 3 unter dem Korridor, also zu niedrig. Gegenüber der Baseline (22) weiter weg vom Korridor: -5.
  - Morgen-Gold je Spieler erhöhen `economy.json:dawnGoldPerPlayer`
    Aktuell 2. Probierbereich 2 … 4 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 3. Wirkung: „Gold am Morgen je Spieler, Median“ soll von 17,0 Richtung Korridor 20–70 steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Startgold erhöhen `economy.json:purse.startGold`
    Aktuell 14. Probierbereich 14 … 19 (1 bis 3 Schritte à 10 % rauf). Startvorschlag: 16. Wirkung: „Gold am Morgen je Spieler, Median“ soll von 17,0 Richtung Korridor 20–70 steigen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
- 🟡 **Burg hält Nacht 1–5** – Knapp an der Grenze: beim Tunen anderer Ziele im Blick behalten, erst bei Rot drehen. In 16 von 20 Läufen hielt die Burg Nacht 1 bis 5 (80 %). Sollwert: 75–90 % der Läufe. Im Korridor, aber knapp: nur 5 Prozentpunkte bis zur unteren Grenze. Unverändert gegenüber der Baseline (80 %).
  - Mauer-HP Stufe 1 erhöhen `buildings.json:wall.levels.0.hp`
    Aktuell 300. Probierbereich 300 … 480 (1 bis 3 Schritte à 20 % rauf). Startvorschlag: 360. Wirkung: „Burg hält Nacht 1–5“ (80,0 %) soll sich von der knappen Grenze entfernen, ohne den Korridor 75–90 % zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Gegner je Welle 1–5 senken `waves.json:table.0.standard.1`
    Aktuell 10. Probierbereich 7 … 10 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 9. Wirkung: „Burg hält Nacht 1–5“ (80,0 %) soll sich von der knappen Grenze entfernen, ohne den Korridor 75–90 % zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Schaden der Bogenschützen erhöhen `troops.json:archer.damage`
    Aktuell 15. Probierbereich 15 … 20 (1 bis 3 Schritte à 10 % rauf). Startvorschlag: 17. Wirkung: „Burg hält Nacht 1–5“ (80,0 %) soll sich von der knappen Grenze entfernen, ohne den Korridor 75–90 % zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
- 🟡 **Burg hält Nacht 1–5 (dev)** – Knapp an der Grenze: beim Tunen anderer Ziele im Blick behalten, erst bei Rot drehen. In 20 von 20 Läufen hielt die Burg Nacht 1 bis 5 (100 %). Sollwert: 99–100 % der Läufe. Im Korridor, aber knapp: nur 1 Prozentpunkt bis zur unteren Grenze. Unverändert gegenüber der Baseline (100 %).
  - Gegner je Welle 1–5 erhöhen `waves.json:table.0.standard.1`
    Aktuell 10. Probierbereich 10 … 13 (1 bis 3 Schritte à 10 % rauf). Startvorschlag: 11. Wirkung: „Burg hält Nacht 1–5“ (100,0 %) soll sich von der knappen Grenze entfernen, ohne den Korridor 99–100 % zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.
  - Mauer-HP Stufe 1 senken `buildings.json:wall.levels.0.hp`
    Aktuell 300. Probierbereich 210 … 300 (1 bis 3 Schritte à 10 % runter). Startvorschlag: 270. Wirkung: „Burg hält Nacht 1–5“ (100,0 %) soll sich von der knappen Grenze entfernen, ohne den Korridor 99–100 % zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.

## Nächster Lauf

1. Seeds 1, 2, 3 im Replay nachspielen: Hub-Stufe 2 erreicht vor Tag 5 verletzt.
2. Hebel abstimmen: hub.json:levels.1.steps.0.cost.stone=17..25, hub.json:levels.2.steps.2.cost.gold=7..10 (Suche über höchstens drei Schritte).
3. Nach jeder Änderung in data/ den vollen Lauf wiederholen und mit diesem Bericht vergleichen.

Alle Empfehlungen als ein Block (in den Chat an Claude):

```
# Empfehlungen aller Stellschrauben (Startvorschläge) als Wert-Varianten, ein Lauf mit allen zugleich:
economy.json:dawnGoldPerPlayer=3  # Morgen-Gold je Spieler erhöhen; aktuell 2, Bereich 2 … 4; verbessert: Hub-Stufe 3 erreicht vor Tag 10; Gold am Morgen je Spieler, Median
hub.json:levels.1.steps.0.cost.stone=22  # Stein-Kosten Hub-Stufe 2 (Schritt 1) senken; aktuell 25, Bereich 17 … 25; verbessert: Hub-Stufe 2 erreicht vor Tag 5
hub.json:levels.1.steps.2.cost.stone=45  # Stein-Kosten Hub-Stufe 2 (Schritt 3) senken; aktuell 50, Bereich 35 … 50; verbessert: Hub-Stufe 2 erreicht vor Tag 5
hub.json:levels.2.steps.2.cost.gold=9  # Gold-Kosten Hub-Stufe 3 (Schritt 3) senken; aktuell 10, Bereich 7 … 10; verbessert: Hub-Stufe 3 erreicht vor Tag 10
buildings.json:wall.levels.0.hp=360  # Mauer-HP Stufe 1 erhöhen; aktuell 300, Bereich 300 … 480; verbessert: Zerstörte Gebäude je Welle 1–5, Median
waves.json:table.0.standard.1=9  # Gegner je Welle 1–5 senken; aktuell 10, Bereich 7 … 10; verbessert: Zerstörte Gebäude je Welle 1–5, Median
economy.json:purse.startGold=16  # Startgold erhöhen; aktuell 14, Bereich 14 … 19; verbessert: Gold am Morgen je Spieler, Median
```

1. Replays schreiben (im Terminal des Checkouts):

```
task balance:targets -- --seeds 20
```

2. Seed 1 nachspielen (in den Chat an Claude):

```
Spiele das Replay mit replay_run {"path":"reports/balance-replays/1-p2-wirtschaft-d0.replay.json"} ab und erkläre, warum Hub-Stufe 2 erreicht vor Tag 5 verletzt ist.
```

Abstimmen auf der Balancing-Seite (Workbench › Balancing-Seite › Abstimmen):

```
Abstimmen: Modus search · Seeds 20 · Parameter hub.json:levels.1.steps.0.cost.stone=17..25 und hub.json:levels.2.steps.2.cost.gold=7..10
```

Abstimmen lassen (in den Chat an Claude):

```
Stimme mit dem Balancing-Tester (--tune search) hub.json:levels.1.steps.0.cost.stone=17..25 und hub.json:levels.2.steps.2.cost.gold=7..10 ab und schlage den besten Wert als Änderung in data/ vor.
```

Abstimmen in der Shell (im Terminal des Checkouts):

```
task balance:run -- --targets --tune search --seeds 20 --param hub.json:levels.1.steps.0.cost.stone=17..25 --param hub.json:levels.2.steps.2.cost.gold=7..10
```

Voller Lauf zum Vergleich (in den Chat an Claude):

```
Starte sim_test {"action":"start","mode":"offline","seeds":20} und vergleiche den neuen Bericht mit simtest-run-7.
```

## Kennzahlen

Kennzahlen aus 120 gültigen Läufen; Richtwerte sind Startwerte (data/balance-hints.json).

- 🟢 Erste Mauer: 836,5 Ticks (Median; Richtwert grün ≤ 1500, gelb ≤ 3000)
- 🟡 Erste Werkstatt: 9943 Ticks (Median; Richtwert grün ≤ 6000, gelb ≤ 12000)
- 🟡 Leerlauf der Bauern: 26,9 % (Richtwert grün ≤ 25, gelb ≤ 35)
- 🟡 Überlebensabfall je Nacht: 10 Pkt. (Richtwert grün ≤ 5, gelb ≤ 15)
- 🟢 Gold-Bilanz je Tag ab Tag 2: 20 Gold (Richtwert grün -20 bis 60, gelb -60 bis 120)
- 🟢 Holz-Bilanz je Tag: 23,3 Holz (Richtwert grün 0 bis 150, gelb -30 bis 300)
- 🟢 Stein-Bilanz je Tag: 0,9 Stein (Richtwert grün 0 bis 80, gelb -20 bis 160)
- 🟡 Leerlauf der Bauern (dev): 27,8 % (Richtwert grün ≤ 25, gelb ≤ 35)
- 🟡 Leerlauf der Bauern (easy): 28,8 % (Richtwert grün ≤ 25, gelb ≤ 35)
- 🟡 Leerlauf der Bauern (hard): 29,6 % (Richtwert grün ≤ 25, gelb ≤ 35)
- 🟡 Leerlauf der Bauern (ultra): 25,9 % (Richtwert grün ≤ 25, gelb ≤ 35)

| Grad | Nacht | Läufe | Überleben % | Gold Tag | Gold Nacht | Holz | Stein | Leerlauf % |
|---|---|---|---|---|---|---|---|---|
| normal | 1 | 40 | 90 | -150,7 | -4,4 | 44,2 | 5,7 | 33,1 |
| normal | 2 | 40 | 90 | -18,8 | -2,6 | 122,2 | -0,2 | 12 |
| normal | 3 | 40 | 90 | 40 | 2,8 | 16,8 | 2,5 | 34,3 |
| normal | 4 | 40 | 90 | 49,1 | 0 | -6,1 | -0,1 | 31,3 |
| normal | 5 | 40 | 80 | 44 | -9,5 | -20,6 | -1,5 | 38,5 |
| dev | 1 | 20 | 100 | -150,7 | 0,4 | 43,8 | 6 | 34,4 |
| dev | 2 | 20 | 100 | -33,4 | 0 | 151,5 | 2 | 12,1 |
| dev | 3 | 20 | 100 | -2,1 | 0 | 4,8 | 3,2 | 22,8 |
| dev | 4 | 20 | 100 | 7,6 | 0 | 0 | -2,3 | 32,2 |
| dev | 5 | 20 | 100 | 25,3 | 0 | 0 | 0 | 37,4 |
| easy | 1 | 20 | 100 | -150,7 | 0,4 | 43,3 | 6 | 34,3 |
| easy | 2 | 20 | 100 | -27,3 | 0 | 151,8 | 1,2 | 21,1 |
| easy | 3 | 20 | 100 | -7,7 | 0,8 | 5 | 2 | 21,8 |
| easy | 4 | 20 | 100 | 9 | 0 | 0 | -0,3 | 28,5 |
| easy | 5 | 20 | 100 | 30,4 | 0 | 0 | 0 | 38,2 |
| hard | 1 | 20 | 75 | -150,7 | -14,1 | 40,3 | 4,7 | 30,9 |
| hard | 2 | 20 | 75 | -16,5 | -3 | 112,5 | 0,6 | 11 |
| hard | 3 | 20 | 75 | 34,3 | -0,3 | 6,6 | 1,2 | 36 |
| hard | 4 | 20 | 75 | 48 | 0 | -20,5 | -0,6 | 24,7 |
| hard | 5 | 20 | 60 | 27,6 | -13 | -34 | -0,7 | 45,3 |
| ultra | 1 | 20 | 55 | -150,7 | -16,3 | 22 | 4,3 | 26,5 |
| ultra | 2 | 20 | 55 | -18,5 | -3 | 89,3 | 0,6 | 11,9 |
| ultra | 3 | 20 | 55 | 19,8 | -0,1 | -3,5 | 0,8 | 33,5 |
| ultra | 4 | 20 | 55 | 43,3 | 0 | -29,6 | -1 | 25,5 |
| ultra | 5 | 20 | 35 | 29,6 | -20 | -35,8 | -0,7 | 32,1 |
| normal | 6 | 20 | 90 | 23,4 | 0 | -8,3 | -0,1 | 16,1 |
| normal | 7 | 20 | 90 | 6,7 | 0 | 26,1 | -0,1 | 20 |
| normal | 8 | 20 | 90 | 0 | 0 | -1,2 | 0 | 20,6 |
| normal | 9 | 20 | 90 | 0 | 0 | -3,1 | 0 | 20,8 |
