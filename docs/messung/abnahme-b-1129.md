# Abnahme B-1129: Farbe und Pause im Rust-Fenster gegen die Referenz (RU64.4)

2026-11-19, STIRA-PC, Second Worker, Commit f2013716. Referenz: `testdata/rust/go-referenz/farben.json` und `pause.png` (Tag `go-referenz-final`).

**Abweichung vom Plan:** Eine Fensteraufnahme des GPU-Fensters (Bildschirmaufnahme) ist in dieser Session nicht gelaufen (kein Aufnahmeweg ohne Computer-Use-Freigabe). Gemessen ist der Aufnahmeweg der EXE (`--shots`, 1028x720 wie die Go-Referenz, nicht das GPU-Fenster). Der Fensterteil steht in der Prüfliste für Ralf.

## Befehle

`task buildall` (aktuell, Paket `dist/k3c-game`), dann je Szene:

- `dist/k3c-game/k3c-game.exe --seed k3c --players 2 --tick 300 --shots .work/f/tag --count 1 --size 1028x720`
- `... --menu main --tick 1 --shots .work/f/main --count 1 --size 1028x720`
- `... --menu pause --seed k3c --players 2 --tick 300 --shots .work/f/pause --count 1 --size 1028x720`

Pixel an `x*Breite`, `y*Höhe` laut `farben.json`, Schwelle ±2 je Kanal. Aufnahmen danach gelöscht.

## Farbwerte (AC-01)

| Stelle | Bild | Rust | Go | Abweichung | Ergebnis |
|---|---|---|---|---|---|
| himmel_oben (257,14) | tag | 142,202,230 | 142,202,230 | 0,0,0 | ok |
| hud_flaeche_beutel (61,21) | tag | 64,91,104 | 64,91,104 | 0,0,0 | ok |
| boden_streifen (514,241) | tag | 48,86,88 | 88,129,87 | −40,−43,+1 | **Abweichung** |
| wasser (514,324) | tag | 34,57,75 | 72,131,179 | −38,−74,−104 | **Abweichung** |
| hauptmenue_himmel (514,36) | main | 11,16,38 | 11,16,38 | 0,0,0 | ok |
| hauptmenue_papier (514,396) | main | 224,210,176 | 224,210,176 | 0,0,0 | ok |
| pause_himmel (205,36) | pause | 11,16,38 | 11,16,38 | 0,0,0 | ok |
| pause_boden (514,612) | pause | 59,58,107 | 59,58,107 | 0,0,0 | ok |

6 von 8 Stellen gleich. Bei `boden_streifen` und `wasser` trifft die Messstelle in Rust eine andere Bildzone: In der Rust-Aufnahme liegt der Grasstreifen bei y≈210–232 (30 % der Höhe), die Spiegelung/Wasser darunter; die Go-Stellen (y=0,335 und 0,45) setzen einen anderen Horizont/Ansichtsaufteilung voraus. Ob das ein Layoutunterschied oder eine andere Szene ist (Go-Fenster lief mit 13 FPS, Takt 300 evtl. nicht gleich), ist ungeprüft.

## Pause (AC-02)

Das Pausenbild entspricht in den gemessenen Stellen (Himmel, Boden) der Referenz `pause.png` (Nachthimmel mit eigenem Bildschirm, kein Overlay, wie im Hinweis der Referenz). Ganzbildvergleich gegen `pause.png`: mittel 3,745, Anteil 0,0596 (Schwellen 0,5 / 0,005), also über der Schwelle; Sichtung beider Bilder: Die Referenz zeigt rechts oben „13 FPS“ (Go-Zähler), Rust an derselben Stelle eine große Anzeige „T 3…“ (am Rand abgeschnitten); in Rust fehlen die kleinen Figur-Symbole vor „Fortsetzen“ und „Spieler abmelden“ (Go hat sie). Menüaufbau, Farben und Texte sonst gleich.

## Abgleich mit B-1120 (AC-03)

`docs/messung/rust-bildvergleich.md`: Menüstationen (`hauptmenue`, `verlassen`, `ende`) liegen mit Mittel über 11 über der Schwelle, obwohl die Farben (Himmel, Papier) gleich sind. Das passt zu den Layout-/Overlay-Abweichungen hier (große „T“-Anzeige statt FPS-Zähler in den Rust-Aufnahmen, Paar 03 in B-1120), nicht zu einem Farbfehler.

## Für den Hauptrechner

- Farben stimmen (±0) für Himmel, HUD-Fläche, Papier, Pause; kein Farbbefund (B-1128 insoweit belegt).
- Befund: In den Rust-Aufnahmen steht rechts oben eine große „T <Takt>“-Anzeige, abgeschnitten am Bildrand (Go-Referenz: „13 FPS“); Ursache vermutet `--tick`/Aufnahme-Overlay, ungeprüft. Sie erklärt einen Teil der Abweichung in B-1120 und der Pause. Zweiter Befund Pause: Figur-Symbole bei „Fortsetzen“ und „Spieler abmelden“ fehlen in Rust.
- `boden_streifen`/`wasser`: Horizont in Rust höher als in der Go-Referenz oder andere Zonen; Entscheidung, ob die Referenzstellen neu gesetzt werden.
- Prüfliste Ralf (GPU-Fenster, Pad): Farbwerte im echten Fenster und Pause über der Welt (B-1127) per Sicht.
