# Balancing-Bericht

Gesamt: **Fail**, 20 Seeds, Ziele nach `data/balance-targets.json`.

| Ziel | Szenario | Wert | Grenzen | Bewertung | Seeds zum Nachspielen |
|---|---|---|---|---|---|
| Burg hält Nacht 1–5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 80,0 % | 75–90 % | Pass (knapp) | – |
| Burg hält Nacht 1–5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 100,0 % | 99–100 % | Pass (knapp) | – |
| Burg hält Nacht 1–5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 100,0 % | 90–100 % | Pass (ok) | – |
| Burg hält Nacht 1–5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 60,0 % | 50–70 % | Pass (ok) | – |
| Burg hält Nacht 1–5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 35,0 % | 25–45 % | Pass (ok) | – |
| Gold am Morgen je Spieler, Median | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 17,0 | 20–70 | Fail (verletzt) | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 |
| Erste Mauer gebaut vor Ende der hellen Phase | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 100,0 % | 90–100 % | Pass (ok) | – |
| Erster Turm gebaut vor Ende von Tag 2 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 100,0 % | 70–100 % | Pass (ok) | – |
| Holz am Tagesbeginn ≥ 100, ab Tag 3 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 85,0 % | 70–100 % | Pass (ok) | – |
| Hub-Stufe 2 erreicht vor Tag 5 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 0,0 % | 60–85 % | Fail (verletzt) | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 |
| Hub-Stufe 3 erreicht vor Tag 10 | 2 Spieler, wirtschaft, Tiefe 0, 9 Tage | 0,0 % | 40–70 % | Fail (verletzt) | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 |
| Zerstörte Gebäude je Welle 1–5, Median | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 2,0 | ≤ 1 | Fail (verletzt) | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 |
| Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4 und Tag 6 ≥ 8 | 2 Spieler, wirtschaft, Tiefe 0, 5 Tage | 80,0 % | 70–100 % | Pass (ok) | – |

## Vergleich mit der Baseline

Keine Baseline vorhanden (`testdata/balance/baseline.json`); `task balance:baseline` legt sie an.
