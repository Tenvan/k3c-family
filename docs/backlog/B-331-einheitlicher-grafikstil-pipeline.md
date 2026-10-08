# B-331 · Alle Grafiken laufen durch eine Pipeline mit Ziel-Palette und gleicher Pixeldichte

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GRA
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Unter `public/grafik/` liegen über 40 Packs mit eigener Palette, eigener Pixeldichte (16 px, 32 px, grob hochskaliert),
eigenem Outline-Stil und eigener Lichtrichtung. Der Stilbeschluss Q13 (16 px, ×2) und der Palettentausch von Hand in
`grafik/k3c-paletten` (GR2.3) decken nur einzelne Objekte ab. Neue Assets, auch aus Bild-KIs wie PixelLab oder
Retro Diffusion, würden die Mischung weiter vergrößern.

Ein Prototyp (Go, nur Stdlib, nicht im Repo) hat acht Assets aus acht Packs auf DB32 und Endesga 32 abgebildet
(nächste Farbe im OKLab-Farbraum, ohne Dithering, hartes Alpha). Befund: Die Palette allein gleicht Farben und
Kontraste spürbar an. Größter verbleibender Bruch ist die Pixeldichte, z. B. `plants-and-flowers-pixel-art/greentree.png`
mit groben Blockpixeln neben feinen 32-px-Packs.

## Ziel

Jede Grafik im Spiel wirkt wie aus einem Set: gleiche Ziel-Palette, gleiche Pixeldichte, gleiche Outline- und
Lichtregel. Neue Assets (Pack, eigenes Zeichenraster oder Bild-KI) werden durch dieselbe Pipeline passend gemacht,
statt einzeln von Hand.

## Beteiligte und Zielgruppen

- 🧑 entscheidet Ziel-Palette, Pixeldichte und Stilregeln und nimmt den Look am TV ab.
- Spieler sehen ein einheitliches Bild; Entwickler bekommen einen festen Weg für neue Assets.

## Anforderungen

- Eine Stil-Bibel in `docs/assets/` legt Ziel-Palette, Pixeldichte, Outline-Regel und Lichtrichtung fest.
- Ziel-Palette ist **DB32** (DawnBringer 32), Pixeldichte **16 px** (eine Quell-Pixel-Kante = 2 Bildschirm-Pixel bei ×2),
  beides Entscheidung 🧑 vom 2026-10-06.
- Ein `task` rechnet Quell-Assets deterministisch in Spiel-Assets um: erkennt eine vorhandene Hochskalierung
  (größter gemeinsamer Teiler der Lauflängen), verkleinert auf 16-px-Dichte (32-px-Packs halbiert, je Block die häufigste
  Farbe), bildet auf DB32 ab (nächste Farbe in OKLab, ohne Dithering) und macht Alpha hart; gleiche Eingabe ergibt
  byte-gleiche Ausgabe.
- Quelle und Ergebnis liegen getrennt; Lizenzen und `CREDITS.md` bleiben je Pack gültig.
- Eine Vorher/Nachher-Testseite zeigt alle umgerechneten Assets nebeneinander auf gemeinsamem Boden.
- Assets aus Bild-KIs laufen ohne Sonderweg durch dieselbe Pipeline.

## Nicht-Ziele

- Color-Grading-Shader je Biom und Tageszeit (eigenes Ticket, falls die Pipeline nicht reicht).
- Neue Figuren oder Animationen zeichnen (B-329, B-162).
- Anbindung einer Bild-KI per API oder MCP (eigenes Ticket nach Auswahl durch 🧑).

## Regeln und Einschränkungen

- Stilbeschluss Q13 (16 px, ×2) gilt, bis 🧑 ihn mit der Stil-Bibel ändert.
- Befehle nur über `task` (`Taskfile.yml`); Testseite nach der Seiten-Regel in `CLAUDE.md` (`installPageChrome()`, `src/landing/pages.ts`).
- Pipeline in Go (Stdlib reicht) oder TS ohne neue Abhängigkeit; kein `Math.random()`.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- `gothicvania-town/props-einzeln/house-a.png` und `opp2017-village-and-room/kandidaten/village-fountain.png`
  → nach dem Lauf nur noch Farben der Ziel-Palette, beide in derselben Pixeldichte.
- Ein neues PixelLab-Sprite in 64 px → Pipeline bringt es auf Ziel-Dichte und Ziel-Palette, ohne Handarbeit.

## Ausnahme- und Fehlerfälle

- Asset mit weichem Alpha (Schatten, Glühen) → Alpha wird hart; Ausnahmen nur über eine Liste in der Stil-Bibel.
- Asset, das beim Verkleinern Details verliert → bleibt in Quell-Dichte und steht als Lücke in der Zuordnung.
- Kaputte oder fehlende Quelldatei → Lauf bricht mit Dateinamen ab, schreibt keine halben Ergebnisse.

## Akzeptanzkriterien

- **AC-01** `docs/assets/stil-bibel.md` nennt Ziel-Palette (DB32, Hex-Werte), Pixeldichte (16 px), Outline-Regel und Lichtrichtung; 🧑 hat sie freigegeben.
- **AC-02** Ein `task`-Befehl rechnet alle zugeordneten Assets um; ein Test prüft, dass jedes Ergebnis-Pixel eine Farbe der Ziel-Palette hat und zwei Läufe byte-gleich sind.
- **AC-03** Eine Testseite zeigt alle umgerechneten Assets vorher und nachher auf gemeinsamem Boden und ist von der Landingpage erreichbar.
- **AC-04** Das Spiel lädt die umgerechneten Assets; 🧑 nimmt den Look am TV ab.

## Offene Fragen

- Outline-Regel und Lichtrichtung: welche Vorgabe gilt? Entscheidet 🧑 mit der Stil-Bibel.
- Assets, die nach dem Angleichen zu klein sind (z. B. `greentree.png` als 16-px-Topfpflanze statt Baum): Lücke oder neu zeichnen bzw. per Bild-KI erzeugen? Entscheidet 🧑.
- Wird ein Bild-KI-Dienst (PixelLab, Retro Diffusion) fester Teil des Wegs? Entscheidet 🧑.

## Notizen

- Prototyp-Seite (privat): https://claude.ai/artifact/XsRcUYtxfkBakAgGucXdyo
- Entschieden 2026-10-06 (🧑, Chat): DB32 statt Endesga 32, Pixeldichte 16 px statt 32 px.
- Prototyp-Befund mit DB32 + 16 px: Die 32-px-Packs (`wooden-castle`, `village-fountain`) behalten nach dem Halbieren
  ihre Weltgröße und werden gröber, passen aber. `greentree.png` ist 10-fach hochskaliert und schrumpft auf 16 px.
  Das dunkle Violett von Gothicvania (Dach, Schatten) fällt in DB32 teils auf Grau-Braun, die Schatten von `house-a`
  verlieren Kontrast. Die Stil-Bibel braucht dafür eine Prüfung, ggf. Feinabstimmung je Asset.
