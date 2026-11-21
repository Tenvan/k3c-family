# Stimmung: Abgleich Rust-Spiel gegen K2C-Screenshots (LA1.4, B-1164)

Stand 2026-11-21, Hauptrechner, `origin/main` f9cdf781, Release-EXE `dist/k3c-game/k3c-game.exe`. Status: **abgenommen von 🧑 am 2026-11-21** (AC-04 erfüllt).

## So lief die Vorbereitung (Schritt 1, Agent)

Standardprofil (ohne `K3C_LOOK`), Seed 1, 1920 × 1080, Hub der Oberwelt (Wald). Takte wie in `docs/messung/look-k2c.md`. Bilder liegen in `.work/stimmung/<szene>-<ansichten>/shot-01.png` (nicht eingecheckt).

```
k3c-game.exe --seed 1 --players 1 --input 0@0=- --tick <T> --shots .work/stimmung/<szene>-1 --count 1 --size 1920x1080
k3c-game.exe --seed 1 --players 2 --input 0@0=-,1@0=- --tick <T> --shots .work/stimmung/<szene>-2 --count 1 --size 1920x1080
```

| Szene | Takt | Bild eine Ansicht | Bild zwei Ansichten |
|---|---|---|---|
| Tag | 300 | `.work/stimmung/tag-1/shot-01.png` | `.work/stimmung/tag-2/shot-01.png` |
| Abend | 14000 | `.work/stimmung/abend-1/shot-01.png` | `.work/stimmung/abend-2/shot-01.png` |
| Nacht | 18000 | `.work/stimmung/nacht-1/shot-01.png` | `.work/stimmung/nacht-2/shot-01.png` |
| Morgengrauen | 23500 | `.work/stimmung/morgen-1/shot-01.png` | `.work/stimmung/morgen-2/shot-01.png` |

Nicht aufnehmbar ohne Geräteeingabe (Schalter fehlt, siehe `look-k2c.md`): Blutmond, dichter Wald mit Fackel, gerodeter Hub mit Leitmotiv, Lava-Stufe. Falls 🧑 diese Szenen braucht: am Gerät über das Cheat-Fenster (F4) und als Zeile unten ergänzen.

## Sichtung des Agenten (Vorschlag, nur an Tag 1 Hub und Nacht 1 Hub gesehen)

Das ist eine erste Lesart der Rust-Bilder, **keine** Aussage über die K2C-Screenshots; die liegen nur bei 🧑.

| Merkmal | Tag | Nacht |
|---|---|---|
| Farbe | Himmel hellblau und flach, Kulisse blaugrün und gleichmäßig, wenig Abstufung | fast schwarz, Welt außerhalb des Fackelkreises unsichtbar |
| Licht | kein Lichthof am Lagerfeuer, Monarch ohne eigenes Licht sichtbar | warmer Hof um den Monarchen sichtbar, Lagerfeuer ohne eigenen Hof erkennbar |
| Dunst | keiner (Kulisse hat gleiche Schärfe vorn und hinten) | keiner außer dunklem Verlauf |
| Tiefe | Kulisse in drei Ebenen (Berge, Farne), Vordergrund ohne Unterholz | Kulisse gar nicht sichtbar |
| Details | Boden kahl, wenige Grasbüschel, keine Bäume als Rahmen | Wasser mit Wellenstrichen, Mond ohne weiten Hof |
| Himmel | große leere Fläche oben (HUD-Bereich), keine Wolken | kein Sternenhimmel, Mond ohne Hof |
| Wasser | Spiegelung der Kulisse deutlich, aber Wellenlinien hart | Spiegelung schwach, Wellenstriche sehr hell gegen den Grund |
| HUD | „T <Takt>“-Anzeige rechts oben abgeschnitten (Messwerkzeug, siehe `abnahme-b-1129.md`), stört den Vergleich | gleiche Anzeige |

## Antworten von 🧑 (Dialog 2026-11-21)

Auswahl aus Vorschlägen des Agenten; „Wellenstriche“: bei Bedarf noch einmal in die K2C-Screenshots schauen.

| Szene | Merkmal | Was fehlt oder stört | Rang |
|---|---|---|---|
| Tag, Abend | Himmel | Verlauf und Wolken statt flacher Fläche | 3 |
| Tag, Abend | Tiefe, Dunst | Dunst zwischen den Kulissen-Ebenen | 3 |
| Tag, Abend | Hintergrund | Letzte (feste) Hintergrund-Ebene als monumentales Bild: großer Berg, Pyramide, Vulkan o. ä. | 3 |
| Tag | Wasser | Spiegelung weicher (Spiegel/Welt 74 % auf 60 bis 70 %) | 2 |
| Nacht, Hub | Licht | Fackeln an allen Häusern überblenden Gebäude und Figuren, sieht nicht schön aus; Licht soll nur die Gegend erhellen, ohne selbst sichtbar zu sein | 1 |
| Tag, Nacht | Licht | Lichthof Lagerfeuer; Fackel-Hof größer und weicher | 1 |
| Nacht | Licht | Nacht nicht ganz schwarz (Silhouetten der Kulisse leicht angehoben) | 2 |
| Abend | Farbe | Abendrot am Horizont | 2 |
| Nacht | Himmel | Sterne; Mond mit weitem Hof | 2 |
| Nacht | Wasser | Wellenstriche sehen nicht schön aus, abdunkeln (Vorbild in K2C-Screenshots prüfen) | 2 |

## Gesamt-Rangfolge

Rang 1 hat 🧑 festgelegt; Rang 2 und 3 hat 🧑 per Tausch der Vorschläge festgelegt (2026-11-21), 🧑 hat die Liste am 2026-11-21 abgenommen (AC-04).

| Rang | Merkmal | Szenen | Ziel für LA1.5 |
|---|---|---|---|
| 1 | Lichthöfe: Lagerfeuer, Fackel (größer, weicher), Licht erhellt die Umgebung, die Quelle überblendet Häuser und Figuren nicht | Nacht, Hub | Fackeln an Häusern nur als Hof, nicht als helle Fläche; Lagerfeuer-Hof |
| 2 | Nachthimmel (Sterne, Mondhof), Wellenstriche abdunkeln, Spiegelung weicher, Nacht nicht ganz schwarz, Abendrot | Abend, Nacht | Sterne, Mondhof, Wellenfarbe, Spiegel-Anteil |
| 3 | Himmel mit Verlauf und Wolken, Dunst zwischen Ebenen, monumentaler fester Hintergrund | Tag, Abend | Himmelsverlauf, Dunststufen je Ebene, Leitmotiv-Ebene prüfen (B-1148 bis B-1151) |

## Abnahme

🧑 hat die Rangliste am 2026-11-21 abgenommen: Rang 1 Lichthöfe, Rang 2 Nachthimmel (mit Wasser und Abendrot), Rang 3 Himmel mit Verlauf (mit Dunst und monumentalem Hintergrund). Weiter mit LA1.5.
