# Look k2c: Szenen und Pixelverhältnisse (RU64.5, B-1130 Teil 1)

2026-11-19, STIRA-PC, Second Worker, Commit eb9784dc. Profil `K3C_LOOK=k2c`, Seed 1, Größe 1920 × 1080, Rust-Release-EXE `dist/k3c-game/k3c-game.exe` (Aufnahmeweg `--shots`, nicht das GPU-Fenster). Messcode (Python, Kantensuche) lag in `.work/` und ist nicht eingecheckt.

Befehl je Szene (Beispiel Tag, eine Ansicht; zwei Ansichten: `--players 2 --input 0@0=-,1@0=-`):

```
K3C_LOOK=k2c k3c-game.exe --seed 1 --players 1 --input 0@0=- --tick <T> --shots .work/l/<szene> --count 1 --size 1920x1080
```

Takte aus einer Helligkeitsreihe (mittlere Bildhelligkeit): Tag 300, Abend 14000 (schon dunkel; Dämmerungsbeginn um 11000 bis 12000), Nacht 18000, Morgengrauen 23500 (Helligkeit steigt bis ca. 28000).

## Messung

Gehlinie = Oberkante des Grasstreifens, Ufer = Höhe des Bodenstreifens, Fluss = Rest bis unten, je Ansicht (bei zwei Ansichten je Hälfte, Anteile an der Hälfte). Spiegel/Welt = mittlere Helligkeit der Spiegelung (20 % der Ansicht unter dem Ufer) gegen die der Welt darüber.

| Szene | Ansichten | Gehlinie (Ziel 58 ± 1 %) | Ufer (Ziel 7 %) | Fluss (Rest, ca. 35 %) | Spiegel/Welt (Ziel 60 bis 70 %) |
|---|---|---|---|---|---|
| Hub Tag (Takt 300) | 1 | 57,8 % ok | 7,0 % ok | 35,1 % | 74 % (über Ziel) |
| Hub Tag | 2, oben | 57,7 % ok | 7,6 % | 34,7 % | 74 % (über Ziel) |
| Hub Tag | 2, unten | 57,7 % ok | 7,6 % | 34,7 % | 74 % (über Ziel) |
| Abend (Takt 14000) | 1 | 57,8 % ok | 7,2 % | 35,0 % | 183 % (Metrik unbrauchbar, s. u.) |
| Abend | 2 (oben / unten) | 57,7 % / 57,7 % ok | 7,4 % / 7,4 % | 34,9 % / 34,9 % | 194 % / 192 % |
| Nacht (Takt 18000) | 1 | 57,8 % ok | 7,3 % | 34,9 % | 193 % |
| Nacht | 2 (oben / unten) | 57,7 % / 57,7 % ok | 7,6 % / 7,6 % | 34,7 % / 34,7 % | 172 % / 205 % |
| Morgengrauen (Takt 23500) | 1 | 57,0 % (Kante schwach) | 8,1 % | 35,0 % | 111 % |
| Morgengrauen | 2 (oben / unten) | 52,1 % / 52,5 % (Kante unsicher) | 17,6 % / 12,8 % | 30,3 % / 34,7 % | 109 % / 107 % |

Lesart:

- **Geometrie Hub Tag, Abend, Nacht:** Gehlinie, Ufer und Fluss liegen in allen Ansichten im Zielbereich (57,7 bis 57,8 %; Ufer 7,0 bis 7,6 %; Fluss 34,7 bis 35,1 %). Gehlinie bei zwei Ansichten je Hälfte gleich wie bei einer.
- **Spiegel-Helligkeit:** Bei Tag 74 % statt 60 bis 70 %. In Dämmerung und Nacht liegt der Wert über 100 %; die Kennzahl ist dort nicht aussagekräftig, weil die dunkle Welt mit hellen Lichtern (Lagerfeuer, Mond) im Fluss gegen einen nahezu schwarzen Streifen verglichen wird. Ein Soll von 60 bis 70 % ist damit nur am Tag geprüft.
- **Morgengrauen:** Die Kantensuche findet bei schwachem Kontrast die Gehlinie nicht sicher (52 bis 57 %); Werte sind unsicher, nicht als Abweichung zu werten.
- **Nicht gemessen: M (Reiterhöhe 12 % bzw. 16 % je Hälfte).** Kein automatisierbarer Messweg; Sichtmaß Tag, eine Ansicht: Reiter von der Krone (y ≈ 465) bis zu den Hufen (y ≈ 628) = ca. 15 % der Bildhöhe, über dem Ziel 12 % (nur Sichtung, ungeprüft).

## Nicht aufgenommene Szenen

Blutmond, dichter Wald mit Fackel, gerodeter Hub mit Leitmotiv und Lava-Stufe: Die EXE hat keinen Schalter für Ereignis, Welt, Baumdichte oder Stufe (`--input` kennt nur Befehle und Menüaktionen; Ereignisse `bloodMoon`, `fullMoon` und Phasen sind Cheats im Dev-Menü, F4 oder `--devwin`). Ohne Geräteeingabe oder neuen Schalter nicht aufnehmbar; offen für den Hauptrechner oder 🧑 am Gerät.

## Sichtbefunde in allen Aufnahmen

Rechts oben steht eine große, am Rand abgeschnittene „T <Takt>“-Anzeige (siehe `docs/messung/abnahme-b-1129.md`); das Bild enthält sie ohne Zutun.
