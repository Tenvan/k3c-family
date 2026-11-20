# Standard: Bildaufbau, Maßstab und Stimmung

Stand: 2026-11-17 · Domäne PAP (Assets), Umsetzung im Spiel CLI · Ergänzt [`paper-spec.md`](paper-spec.md) (Form der
Assets) um das **Bild**: wie Himmel, Land und Fluss aufgeteilt sind, wie groß Dinge zueinander sind und wie Licht und
Farbe die Stimmung tragen. Vorbild ist Kingdom Two Crowns (K2C), ausgewertet an 16 Screenshots (Ralf, 2026-11-17).
Wir übernehmen Aufbau, Maßstab und Licht, **nicht** den Pixel-Stil: Unsere Assets bleiben Papierschnitt (Entscheidung
[`005`](../decisions/005-papierschnitt-assets-mcp.md)). Begriffe: [`glossar.md`](../glossar.md) (Gehlinie, Monarchenhöhe).

Die Regeln sind **Richtwerte für neue und überarbeitete Assets**. Wo das Spiel heute abweicht (Abschnitt 7), entsteht
ein Ticket; bestehende Assets werden nicht stillschweigend umgebaut.

## 1. Drei Bänder: Himmel und Land, Ufer, Fluss

Jede Ansicht (auch jede Hälfte im Split-Screen) hat dieselben drei Bänder, gemessen an der Höhe der Ansicht:

| Band | K2C (gemessen) | Regel |
|---|---|---|
| Himmel, Kulisse, Welt bis zur Gehlinie | 57–59 % (offener Hub mit viel Himmel: 68 %, Winter-Nebel: 74 %) | **58 %**, Ausnahmen nur als Stimmung (Abschnitt 5) |
| Ufer unter der Gehlinie | 6–8 % (Erde mit Rissen und Wurzeln oder Steinkai) | **7 %**, Material je Welt |
| Fluss mit Spiegelung | 33–35 % | **Rest (≈ 35 %)** |

- Die **Gehlinie** ist die Linie, auf der Figuren, Gebäude und Bäume stehen. Sie liegt in allen Welten und Stufen gleich
  hoch; nur die Kulisse darüber wechselt.
- Das Ufer ist ein eigenes, dunkles Band und hat Charakter: Risse, Steine, Wurzeln, Grasbüschel, die über die Kante ins
  Wasser hängen. Es trennt Welt und Spiegelung klar.
- Der Fluss ist **kein** Wellenmuster, sondern ein **Spiegel** der Welt darüber (Abschnitt 4).

## 2. Maßstab in Monarchenhöhen (M)

Alle Höhen beziehen sich auf den berittenen Monarchen (**M**, heute 121,5 Spiel-px). In K2C ist M ≈ 12 % der Bildhöhe;
die Welt über der Gehlinie ist also **≈ 5 M hoch** und das Bild **≈ 15 M breit**. Die Welt ist groß, der König klein.

| Ding | K2C (gemessen) | Richtwert |
|---|---|---|
| Bürger, Bauer, Soldat | 0,5–0,6 M | 0,55–0,65 M |
| Gegner klein (Gier) / groß (Brecher) | 0,4 M / 1,1–1,4 M | wie K2C |
| Grasbüschel, Busch | 0,2–0,3 M / 0,4–0,8 M | wie K2C |
| Laubbaum, Birke | 2,5–3,5 M | 2,5–3,5 M |
| Nadelbaum im Wald | ≥ 5 M, ragt oben aus dem Bild | 4–6 M, darf oben anschneiden |
| Zelt, Stand, Werkstatt | 1–1,4 M | 1–1,4 M |
| Mauer, Palisade | 0,8–1,2 M | 0,8–1,2 M |
| Turm mit Schützen | 2–2,5 M | 2–2,5 M |
| Burg, Haupthaus der Welt | 3,5–4,5 M | 3,5–4,5 M, Mitte des Hubs |
| Schiff (Rumpf / mit Segel) | 0,7 M / 3 M, Länge 4–5 M | wie K2C |
| Leitmotiv im Hintergrund (Berg, Statue, Schlange) | 3–5 M, oft angeschnitten | 3–5 M |

- **Mehr Welt als Figur:** Bäume und Leitmotive dürfen den oberen Bildrand verlassen; Figuren nie.
- **Split-Screen:** jede Hälfte mit eigener Gehlinie bei 58 % der Hälfte; M wird dabei etwa 0,7-fach kleiner, nicht
  halb so groß, damit Figuren lesbar bleiben (K2C: ≈ 16 % der Hälften-Höhe). Trenner schmal (≈ 2 % der Höhe) und
  verziert.

### Größenstufen (LA1.1, B-1163)

Die Richtwerte oben stehen als Tabelle in [`data/sizes.json`](../../data/sizes.json) und gelten im Look-Profil `k2c`
(`look.json › sizes`; Profil `go` behält die alten Höhen). **1 M = `bezugPx` = 121,5 Spiel-px**, der berittene Monarch;
er und seine Reittiere bleiben unverändert und sind der Bezugswert.

- `stufen`: Name → sichtbare Höhe in M (z. B. `buerger` 0,6, `gegner-riese` 1,2, `boss` 1,3, `gebaeude-mittel` 1,1, `turm`
  2,2, `burg` 4,0, `baum-hoch` 3,25, `nadel-hoch` 5,6).
- `assets`: `gruppe/id` → Stufe und `sicht` (sichtbare Höhe ÷ Rahmenhöhe des Bildes, erstes Bild der Ruhe-Animation).
  Das Spiel zeichnet ein Asset so, dass seine **sichtbare** Höhe Stufe × M ist: Figuren über die Rahmenhöhe im Katalog
  (`actors/sizes.rs`), Gebäude und Objekte über `units_per_px` (`art/scales.rs`; der Faktor aus `paper.json › skalierung`
  entfällt für Assets mit Stufe).
- Ohne Eintrag bleibt ein Asset, wie es ist: Kleinkram (Truhen, Felsen, Münzen), Schiff, Mounts, Hintergründe.
- Ändern: eine Stufe ändern verschiebt alle ihre Assets; ein neu gezeichnetes Asset braucht ein neues `sicht`
  (der Test `sicht_stimmt_mit_der_grafik` meldet es, nach `task gen:paper`).

## 3. Tiefe: Ebenen und Luftperspektive

- 3–4 Parallax-Ebenen über der Gehlinie: **Vordergrund** (Büsche, Gras an der Gehlinie), **Spielebene** (Figuren,
  Gebäude, Bäume), **Mittelgrund** (Waldwand, Hügel), **Ferne** (Berg, Leitmotiv, Himmel mit Wolken).
- **Luftperspektive:** Je weiter hinten, desto heller, blasser und näher an der Himmelsfarbe; weniger Kontrast, weniger
  Falz-Details. Die Spielebene hat den stärksten Kontrast.
- Für Papierschnitt heißt das: hintere Ebenen nur mit Ton `licht` und wenig Lagen, Spielebene mit voller Schattierung;
  ein leichter Dunst (Himmelsfarbe, halb transparent) zwischen Ebenen setzt das Spiel, nicht das Asset.

## 4. Der Fluss spiegelt die Welt

- Gespiegelt wird alles über der Gehlinie (Kulisse, Bäume, Gebäude, Figuren, Mond, Licht), an der Unterkante des Ufers
  nach unten geklappt.
- Die Spiegelung ist **dunkler** (≈ 60–70 % Helligkeit), leicht in Wasserfarbe getönt und waagerecht gestaucht verzerrt.
- Kurze **helle Striche** (Glanz) laufen waagerecht und flackern; sie sind das einzige Muster im Wasser.
- **Lichtquellen** (Feuer, Fackel, Mond, Portal, Kristall) bleiben in der Spiegelung hell: Das Wasser verdoppelt das Licht.
- Assets brauchen dafür nichts Eigenes; das Spiel spiegelt die fertige Ansicht.

## 5. Stimmung: dichter Wald gegen gerodetes Land

| Ort | So sieht er aus |
|---|---|
| **Dichter Wald** (Rand der Insel, ungerodet) | Kronendach schließt oben, Himmel nur in Lücken; viele senkrechte Stämme hintereinander; dunkles Oliv- und Tannengrün, wenig Sättigung; Unterholz bis zur Gehlinie; Wild halb versteckt. Licht kommt **nur** von der Fackel des Monarchen: ein warmer Lichthof, der Büsche und Boden um ihn aufhellt. |
| **Gerodet, Hub** | Freie Sicht bis zum Horizont; das Leitmotiv der Welt steht groß in der Ferne (Berg, Statuen, Tempel); Wolken, heller und gesättigter; Bäume nur noch als Rahmen links und rechts. Das Lagerfeuer in der Mitte ist das wärmste Licht. |
| **Übergang** | Je weiter der Spieler roden lässt, desto offener und heller das Bild: **Fortschritt sieht man als Licht und Weite.** |

Für Assets: Waldbäume dicht gestaffelt in mehreren Höhen und Tönen anlegen (Varianten), damit eine Waldwand ohne Lücken
entsteht; Hub-Kulissen mit viel Himmel und einem klaren Leitmotiv je Welt.

## 6. Licht, Tageszeit und Farbe

- **Tageszeit und Ereignis tönen das ganze Bild** gleich: Himmel, Kulisse, Figuren und Spiegelung. Assets sind
  deshalb **neutral bei Tageslicht** angelegt; keine Nacht, kein Abendrot im Asset.
  - Tag: klar, blau bis türkis. Abend: lila Himmel, warme Ränder. Nacht: Dunkelblau bis Violett, Mond mit weitem Hof,
    Sterne.
  - Blutmond: alles rot, Mond groß mit rotem Hof. Vulkan, Lava: Orange mit Rauchsäulen. Winter, Moor: Grau-Blau,
    Bodennebel, Schneeflocken.
- **Lichtquellen** (Lagerfeuer, Fackel, Laterne, Kohlebecken, Kristall) haben einen weichen, warmen Lichthof, der Boden
  und Nachbarn aufhellt. Nachts sind sie die Orientierung. Im Asset nur die Quelle; den Hof setzt das Spiel.
- **Farb-Hierarchie:** Die Welt bleibt gedämpft. Gold (Krone, Münzen, Schilde der Spieler) ist die Spielerfarbe;
  **Magenta** gehört allein den Gegnern (Portale, Gier, Brecher) und ist die stärkste Sättigung im Bild. Kein anderes
  Asset nutzt Magenta.
- **Lesbarkeit:** Figuren heben sich über Helligkeit vom Grund ab (helles Reittier, Fackel, heller Schild), nicht über
  Umrisse; keine Konturen (Stil-Datei).

## 7. Abgleich mit dem Spiel (Stand 2026-11-17)

| Punkt | Spiel heute | Richtwert | Folge |
|---|---|---|---|
| Gehlinie (`view.WorldShare`) | 66,7 % | 58 % | Ticket (Spiel) |
| Ufer (`world.bankUnits`) | 20 px bei 32 px/Unit (≈ 3 %) | 7 % der Ansicht | Ticket (Spiel) |
| Fluss | Wellen-Reihen bzw. Shader mit Glanzstrichen, keine Spiegelung der Welt | Spiegel nach Abschnitt 4 | Ticket (Spiel) |
| Sichtbare Welthöhe (`ViewUnits` 420/32) | ≈ 3,5 M | ≈ 5 M | Ticket (Spiel), mit Ralf am TV abwägen |
| Bürger (`peasant` 102,5 px) | 0,84 M | 0,55–0,65 M | Ticket (Grafik) |
| Hub-Gebäude (Schmiede, Taverne, Dojo) | 1,2–1,4 M | Burg 3,5–4,5 M, Werkstätten 1–1,4 M | prüfen je Gebäude |
| Schiff (`schiff` 164 px) | 1,35 M mit Segel | 3 M mit Segel | Ticket (Grafik) |
| Tageszeit-Tönung, Lichthof, Dunst | teilweise (Nacht) | Abschnitte 3 und 6 | Ticket (Spiel) |

Das Spiel wird nach Rust portiert (Go-Stopp); Änderungen am Bildaufbau gehören deshalb in die Rust-Sprints, Assets
können vorher entstehen.
