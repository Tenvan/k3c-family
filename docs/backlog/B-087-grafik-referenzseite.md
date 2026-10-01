# B-087 · Eine Referenzseite zeigt die gewählten CC0-Grafik-Packs für Gebäude, Ressourcen und Hintergründe

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** G1
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („Auswahl B-010: …“ und „Ja, alle zehn“, danach „einbauen“ in der Referenzseite wie die Spielfiguren)

## Ausgangslage

B-010 verlangt Pixel-Art für gebaute Gebäude, Ressourcen und Hintergründe (nur CC0). Die Suche (Explorer-Recherche, `docs/funde/b010-grafik-funde.html`) hat Kandidaten
von OpenGameArt geliefert; 🧑 hat zehn Packs gewählt. Für die Figuren gibt es Referenzseiten (`figuren.html`, `aufstellung.html`), für diese Grafiken noch nichts.

## Ziel

Die zehn gewählten Packs liegen mit Lizenz und Quelle im Repo (`public/grafik/`) und sind auf einer Referenzseite `grafiken.html` ansehbar. Nutzen: 🧑 kann entscheiden,
was davon wofür ins Spiel kommt (B-010), ohne die Packs einzeln zu öffnen.

## Beteiligte und Zielgruppen

🧑 entscheidet über die Verwendung; Entwickler und Agenten binden später im Spiel ein (B-010).

## Anforderungen

- Gewählte Packs (Auswahl B-010): `gothicvania-town`, `gothicvania-church-pack`, `various-stones-and-oregem-veins-16x16`, `portals-32-x-48`,
  `16x16-small-and-medium-coin-animation`, `gold-treasure-icons-16x16`, `sunnyland-tall-forest-environment`, `gotthicvania-swamp`, `forest-background`, `blue-cave-background`.
- Nur Umgebungs-Grafiken kommen ins Repo (Ebenen, Tilesets, Props, Häuser, Objekte). Nicht übernommen: Musik (nicht CC0, Namensnennung Pflicht), Code, PSD/Aseprite-Quellen, GIFs, `__MACOSX`, Figuren-Sprites.
- Je Pack: `LICENSE.txt` des Packs, Credits in `public/grafik/CREDITS.md` und Eintrag auf `lizenzen.html`; beides stimmt mit der Seite überein (Test).
- Die Seite `grafiken.html` zeigt je Pack Name, Urheber, Lizenz, Quelle, Hinweis wofür es taugt und die Bilder pixelscharf in wählbarer Vergrößerung; Hintergrund-Ebenen zusätzlich als gestapelte Vorschau.

## Nicht-Ziele

Einbau ins Spiel und Ersatz der Platzhalter-Formen (B-010); Zuschneiden zu Sprites; Lücken schließen (Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen).

## Regeln und Einschränkungen

Seiten-Regeln aus `CLAUDE.md` (`installPageChrome()`, Eintrag in `src/landing/pages.ts`, oben ca. 70 px frei, kein `Math.random()`); Domäne PLAT mit `public/grafik/` als Fremdmaterial (wie `public/sprites/`);
Lizenz je Pack CC0 laut OpenGameArt-Seite und Datei im Pack.

## Beispiele

Kachel „Alle Grafiken“ → Seite → Abschnitt „Gothicvania Town“: Häuser, Props und Ebenen in 2×, mit Quelle und „CC0 1.0 · ansimuz“.

## Ausnahme- und Fehlerfälle

Ein Bild lässt sich nicht laden → die Karte zeigt „Bild fehlt“ statt einer leeren Stelle.

## Akzeptanzkriterien

- **AC-01** Alle zehn Packs liegen unter `public/grafik/<pack>/` mit `LICENSE.txt`, ohne Musik, Code und Quelldateien (Test).
- **AC-02** `public/grafik/CREDITS.md` und `lizenzen.html` nennen jedes Pack mit Quelle, Urheber und Lizenz (Test).
- **AC-03** `grafiken.html` ist von der Landingpage erreichbar und zeigt alle Packs mit allen ihren Bildern (Test der Seitendaten, Seite ruft `installPageChrome()`).
- **AC-04** 🧑 hat die Seite angesehen und die Packs abgenommen (Beobachtung durch 🧑).

## Offene Fragen

keine

## Notizen

Entstanden aus der Recherche zu B-010. Urheber lt. Pack-Lizenzdateien: ansimuz (Luis Zuno) für Gothicvania Town/Church/Swamp, Forest Background und Tall Forest; Gold-Icons: Bonsaiheldin; die übrigen Packs nennen den OGA-Uploader (siehe CREDITS).
