# B-460 · Der Welt-Betrachter im Asset-Studio zeigt Chunks maximiert und eine Live-Vorschau mit Hub-Stufen und schaltbaren Gebäuden

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ7
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Chat (Ralf: „Ja, freigegeben“)

## Ausgangslage

`leveltest.html` (Level-Betrachter, B-092; `src/tools/leveltest.ts`, reine Logik in `levelView.ts`, Abrufe in `levelApi.ts`) zeigt das Level zu Seed und Biom als Streifen aus Chunks mit Objekten, Hub-Linie, Legende und Warnungen der Spielbarkeits-Prüfung. Die Daten kommen vom laufenden Spielserver (`GET /api/level`, `engine/net/level.go`). „Im Spiel starten“ (nur Wald, `startTarget`) und „Voll ausgebaut starten“ (B-315, `POST /api/dev/fullsave`, nur im Dev-Mode) öffnen `game.html` über `openPage` der Shell. Erreichbar ist die Seite über die Kacheln in `src/tools/devTiles.ts` (Entwicklerseite) und `src/tools/testTiles.ts` (`testing.html`, Abschnitt Level). Der Streifen ist 12,5 rem hoch und nicht maximierbar; eine Ansicht mit Spielgrafik gibt es nur im Spiel.

Für eine Live-Vorschau fehlt ein `World`: Der Generator (`engine/level`) liefert nur das Layout, der Voll-Ausbau (`fullIsland` in `engine/room/fullsave.go`) ist nicht exportiert, und den Zustand für den Client baut `stateOf` in `engine/net/protocol.go` (ebenfalls nicht exportiert); der Client setzt Level und Zustand mit `createViewWorld` (`src/online/clientWorld.ts`) zusammen. Gezeichnet wird mit `StageView` (`src/scenes/stageView.ts`, Hintergrund-Schichten mit Scroll-Faktor) und `WorldRenderer` (`src/scenes/worldRenderer.ts`); die Nacht dunkelt `GameScene` über `daylight()` (`src/scenes/viewRules.ts`) ab.

Der Tageszyklus (`engine/sim/cycle.go`) kennt heute **drei Phasen**: `day`, `dusk` (Dämmerung) und `night` (Wald: 10/1/5 min, `data/biomes/forest.json` › `cycle`). `dawn` ist **keine Phase**, sondern ein Ereignis beim Beginn des Tages. Das Morgengrauen zeichnet der Client heute als **letzte 10 % der Nacht**: `daylight()` (`src/scenes/viewRules.ts`, Z. 30–34) liefert für `night` bei Fortschritt über 0,9 eine Helligkeit von 0 bis 1, jede unbekannte Phase gilt als Nacht. Die Regel Q65 (vier Phasen 3:1:2:1) ist noch nicht gebaut; das Ticket dafür ist B-470. Die Entwickler-Funktion `sim.DevSetPhase` (`engine/sim/dev.go`, Z. 46–48) springt nur bis kurz vor den Phasenwechsel und taugt daher nicht, um eine Tageszeit exakt zu setzen.

Voraussetzung: Das Asset-Studio aus WZ6 (B-459) mit Asset-Handler für `public/` und den Aliasen `@game` → `src/`, `@data` → `data/` im Studio-Frontend.

## Ziel

Der Welt-Betrachter läuft nur noch im Asset-Studio am PC und ersetzt `leveltest.html`: oben die Chunk-Vorschau, beim Öffnen maximiert; darunter eine Live-Vorschau mit der Spielgrafik, in der sich Hub-Stufe, einzelne Gebäude, Tageszeit, Hintergrund-Schichten umschalten lassen; die Vorschau zeigt Gelände, Natur-Objekte und Hintergrund-Schichten, keine Bürger und keine Gegner. Ein Knopf öffnet dasselbe Level im Spiel im Browser. Nutzen: Level, Hub-Stufen und Grafik in Sekunden beurteilen, ohne zu spielen und ohne laufenden Spielserver.

## Beteiligte und Zielgruppen

🧑 nutzt den Welt-Betrachter am PC, um Level, Hub-Stufen und Grafik zu prüfen (Grafik-Reviews, Hub-Ausbau B-458), entscheidet über die Spec und nimmt ab. Agenten setzen um, ein getrennter Reviewer prüft. Spieler sind nicht betroffen (nur Entwickler-Werkzeug). Chunk-Planung (B-461) und Gebäude-Planer (B-462) bauen später auf dieser Seite auf.

## Anforderungen

- **Welt-Betrachter-Seite** im Asset-Studio (`tools/k3c-studio`) mit Chunk-Vorschau oben und Live-Vorschau darunter; Seed, Biom und „Zufälliger Seed“ gelten für beide.
- **Chunk-Vorschau** mit dem Umfang von `leveltest.html`: Chunks mit Index und Art, Objekte nach Art mit Legende und Anzahl, Hub-Linie, Warnungen der Spielbarkeits-Prüfung, Zoom und Scrollen. Die Daten liefert ein Go-Binding des Studios direkt aus `engine/level` (`LoadBiome`, `Generate`, `Validate`), ohne laufenden Spielserver. **Quelle:** In WZ7 ist sie das Biom; sie ist aber so geschnitten, dass sie auch **Insel + Stufe** sein kann (Binding und Oberfläche tragen die Quelle als Wert, nicht als festes Biom). WZ8.5 (B-461) stellt sie später auf Insel + Stufe um, damit Chunk- und Live-Vorschau dieselbe Quelle haben; das braucht dann keine neue Seite.
- **Maximiert beim Öffnen:** Die Chunk-Vorschau füllt den Seitenbereich, die Live-Vorschau steht als eingeklappte Leiste darunter; ein Knopf (und eine Taste) verkleinert die Chunk-Vorschau, danach teilen sich beide die Höhe.
- **Snapshot aus Go (SRV):** Eine exportierte Go-Funktion mit **Options-Struktur** erzeugt Level und Zustand im Format der Nachrichten `level` und `snap` (Protokoll Version 3, unverändert). Die Optionen: Seed, Quelle (Biom bzw. Insel + Stufe), Hub-Stufe, Tageszeit als **Sim-Phase (`day`, `dusk`, `night`) + Fortschritt**, Gebäude-Auswahl. Der Snapshot enthält keine Bürger und keine Gegner. Die Funktion baut auf der Insel-Erzeugung von `fullIsland` auf, schreibt keinen Spielstand, braucht keinen Raum und setzt **keinen globalen Zustand** (kein `sim.UseData`, keine Paket-Variablen). **Die Struktur muss offen für Erweiterungen sein:** WZ8.4 (B-461) ergänzt die Option „geplante Chunks“, WZ9 (B-462) ergänzt in einer eigenen SRV-Session die Option „Kandidaten-`hub.json`“; beide ändern die bestehenden Felder und Aufrufer nicht (Nullwert eines neuen Feldes = heutiges Verhalten).
- **Tageszeit setzen (Go):** Eine eigene Setzfunktion (nicht `sim.DevSetPhase`, siehe Ausgangslage) setzt `w.Cycle` (Phase und Fortschritt) **direkt** und leitet `w.Time` passend daraus ab. Erlaubt sind die **drei Sim-Phasen** `day`, `dusk`, `night` und ein Fortschritt innerhalb der Phase im Bereich **[0, 1)** (0 eingeschlossen, 1 ausgeschlossen: Fortschritt 1 wäre schon der Beginn der nächsten Phase). Sie liegt im Snapshot-Code (`engine/room/`); `engine/sim` bleibt unverändert. Reicht dafür der exportierte Zugriff auf `w.Cycle` und `w.Time` nicht (ungeprüft), entscheidet 🧑 über eine kleine Ergänzung in `engine/sim`. Eine Phase `dawn` kennt die Sim nicht; die Setzfunktion lehnt sie wie jede unbekannte Phase ab.
- **Tageszeit wählen (Oberfläche):** Die Oberfläche bietet **vier Einträge**: Tag, Dämmerung, Nacht und Morgengrauen (keiner ausgegraut), je mit Fortschritt innerhalb des Eintrags (p in [0, 1)). Tag, Dämmerung und Nacht gehen unverändert an die Sim-Phasen. **Morgengrauen** bildet die Optionen-Logik des Studios ab auf die Phase `night` mit dem Fortschritt **0,9 + 0,1·p** (p = gewählter Fortschritt des Morgengrauens, p in [0, 1)); so zeichnet der Client das Morgengrauen heute (letzte 10 % der Nacht, Helligkeit `daylight()` = p). **p = 1 ist kein Wert des Morgengrauens:** Es entspräche dem Tagesbeginn, also der Phase `day` mit Fortschritt 0 (Helligkeit 1); die Oberfläche bietet p nur unter 1 an. Die Abbildung ist eine reine Funktion im Studio-Frontend, nicht in Go; die Vitest-Vergleiche der Fortschrittswerte (0,9 + 0,1·p, Helligkeit) laufen wegen Gleitkomma mit `toBeCloseTo`. Eine echte Phase Morgengrauen baut B-470 später; dann entfällt die Abbildung.
- **Hub-Stufe:** wählbar über alle Einträge aus `data/hub.json` › `levels` (heute 1–5, mehr mit B-458 ohne Code-Änderung); Standard ist die höchste. Bei Hub-Stufe n sind alle bis n freigeschalteten Bauplätze gebaut (`hubLevel`, Mauerlinie k ab Hub-Stufe k, Tor ab `gateHubLevel`). Die Material-Stufe der Gebäude läuft **reihum wie im Voll-Ausbau (B-315)**: je Art in der Reihenfolge der Bauplätze im Level (Reihenfolge der Plätze im Spielstand-Slice, der Zähler je Art beginnt bei Stufe 1) von Stufe 1 bis zur Obergrenze min(n, Anzahl Stufen der Art) (`nextLevel` in `engine/sim/hub_level.go`: Stufe m erst ab Hub-Stufe m), danach wieder von 1. Die Rotation läuft über alle gebauten Plätze und bleibt beim Abwählen einzelner Gebäude unverändert.
- **Gebäude ein/aus:** Jeder Bauplatz der Stufe ist einzeln schaltbar (Schlüssel `kind@x`); ein abgewählter Platz fehlt im Snapshot und wird nicht gezeichnet. Nur Darstellung: kein Spielstand, keine Datei wird geändert.
- **Live-Optionen:** Tageszeit (die vier Einträge oben) mit Fortschritt und mit der Helligkeit wie im Spiel (`daylight()`); Hintergrund-Schichten ein/aus mit Parallaxe beim Scrollen (Scroll-Faktoren der Schichten wie im Spiel). Bürger und Gegner gibt es in der Vorschau nicht.
- **Spielgrafik und Phaser:** Die Live-Vorschau zeichnet mit `StageView`/`WorldRenderer` aus `src/scenes/` über `@game`, unverändert; Grafiken aus `public/` über den Asset-Handler des Studios. **B-460 (WZ7.3) führt Phaser im Studio-Frontend ein** und baut die **wiederverwendbare Bühne** (Phaser-Szene samt Preload der Grafiken); WZ10 (B-463, B-464) nutzt sie für die Figuren-Seiten wieder, statt eine eigene zu bauen.
- **„Im Spiel öffnen“:** öffnet das Level im Standard-Browser über die Landingpage (`<Spiel-Adresse>/#game.html?…`, Shell und Vollbild bleiben gültig), wahlweise als neues Spiel mit dem Seed (Regeln von `startTarget`) oder als voll ausgebauter Spielstand (B-315, `POST /api/dev/fullsave`). Die Spiel-Adresse ist nur Loopback, Standard `http://127.0.0.1:5183`.
- **`leveltest.html` entfällt** samt Skript, Logik, Tests, Kacheln und nicht mehr benutzten Texten; die Logik lebt im Studio weiter.
- **Qualität:** deterministisch (gleiche Eingaben → gleicher Snapshot und gleiches Bild); Umschalten einer Option zeigt das neue Bild in unter 1 s (angenommen, am PC); keine Monarchen in der Vorschau, daher keine 2-Spieler-Anforderung; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Chunks planen oder festlegen (B-461); Freischaltung je Hub-Stufe bearbeiten (B-462); Hub-Ausbau in Feinstufen (B-458).
- Spielen in der Vorschau: keine Eingabe, keine Simulation über die Zeit, keine Wellen, keine Monarchen.
- Einen Spielstand aus der Vorschau-Auswahl erzeugen: Hub-Stufe, Gebäude-Auswahl und Tageszeit gehen nicht ins Spiel; „Im Spiel öffnen“ nutzt ein neues Spiel oder den Voll-Ausbau von B-315.
- Bedienung mit Controller auf dem Sofa: Der Welt-Betrachter ist ein PC-Werkzeug. **B-092 wird umgeplant** (Sprint HW1, Controller-Prüfung am TV); dieses Ticket ändert B-092 nicht.
- Änderungen an `src/scenes/`, `src/online/`, `src/model/` und `public/` (CLI): Das Studio nutzt sie unverändert; braucht die Vorschau dort eine Änderung, entsteht ein Ticket.
- Protokoll- oder Spielstand-Änderung (Protokoll Version 3 und `IslandSaveVersion` bleiben), Golden-Daten.
- Ein HTTP-Endpunkt für die Vorschau: Das Studio ruft die Go-Funktion direkt auf.
- Befunde zum Voll-Ausbau (B-425, B-426); mehrere Inseln (B-103).

## Regeln und Einschränkungen

- **Domänen nacheinander:** zuerst SIM (Session WZ7.8: neue Datei `engine/sim/preview.go` mit `PreviewCycle` für die Tageszeit und `PreviewHubLevel` für Hub-Stufe und Material-Stufen reihum nach den Regeln der Simulation, dazu Tests; bestehende Funktionen bleiben unverändert), dann SRV für den Snapshot (neue Datei in `engine/room/` mit der Insel-Erzeugung aus `fullsave.go`, exportierte Funktion in `engine/net/` über `stateOf`, dazu Tests; Session WZ7.1), danach DEV für das Studio (`tools/k3c-studio/**`; Sessions WZ7.2–WZ7.7). Schichtgrenze: `engine/room` importiert nichts aus `engine/net`; `engine/level` bleibt unverändert, `engine/sim` nur um die neue Datei aus WZ7.8.
- **Snapshot ohne globalen Zustand:** Die Funktion ruft kein `sim.UseData` und setzt keine Paket-Variablen, damit sich Aufrufe nicht beeinflussen. Die Options-Struktur bleibt rückwärtskompatibel erweiterbar (WZ8.4: geplante Chunks, WZ9: Kandidaten-`hub.json`).
- **Tageszeit:** nicht `sim.DevSetPhase` (springt nur bis kurz vor den Phasenwechsel, `engine/sim/dev.go` Z. 46–48), sondern eine eigene Setzfunktion mit Phase und Fortschritt. Die Sim kennt drei Phasen (`day`, `dusk`, `night`); die Setzfunktion setzt nur diese. Die vier Einträge der Oberfläche (Tag, Dämmerung, Nacht, Morgengrauen) bildet das Studio-Frontend ab: Morgengrauen = Phase `night` mit Fortschritt 0,9 + 0,1·p. `engine/sim/cycle.go` und `src/scenes/viewRules.ts` bleiben unverändert. Eine echte Phase Morgengrauen baut B-470; dann entfällt die Abbildung (Anpassung des Welt-Betrachters dort).
- **Grenzfall DEV** (wie WZ6.6 bei `assetvorschau.html`, B-299): Das Entfernen von `leveltest.html` (sonst PLAT) samt `src/tools/leveltest.ts`, `levelView.ts`, `levelApi.ts` und Tests, Kacheln in `src/tools/devTiles.ts` und `testTiles.ts`, betroffenen Tests (`devTiles.test.ts`, `testTiles.test.ts`, `textRule.test.ts`), Texten `level.*` in `src/tools/texts.*.ts`, `docs/sprints/README.md` (Übersicht) und Codemaps (`src/tools/codemap.md`, `src/model/codemap.md`) gehört zur DEV-Session. `level.httpError` benutzt auch `src/tools/dmApi.ts` und bleibt. `.codemap/state.json` erzeugt das Codemap-Werkzeug; nicht von Hand ändern.
- **Nur lesend:** `src/scenes/`, `src/online/clientWorld.ts`, `src/model/` (außer der Codemap, siehe Grenzfall), `src/core/`, `public/` und `data/` über `@game`/`@data` bzw. den Asset-Handler aus WZ6.
- **Go-Anbindung:** Das Studio importiert `k3c/engine/...` über `replace` auf das Wurzel-Modul wie `tools/k3c-dev`; kein `go.work`, kein neues Modul.
- **Determinismus:** Snapshot nur aus Seed und Eingaben, Zufall nur über `engine/rng`; kein `Math.random()`. Der Zufalls-Seed im Studio kommt wie heute aus `crypto.getRandomValues` (keine Spiel-Logik).
- **Firewall:** Das Studio öffnet keinen Port; Anfragen an den Spielserver nur auf `127.0.0.1` und nur nach Klick auf „Im Spiel öffnen“; EXE mit festem Pfad (B-459).
- **Seiten-Regeln:** „Im Spiel öffnen“ geht über die Landingpage (`#game.html?…`, `src/landing/landing.ts`), nie direkt auf `game.html`, damit Shell, Home-Kombi und Vollbild gelten.
- **Abhängigkeiten:** keine neue außer `phaser` für das Studio-Frontend in der Version der Wurzel; B-460 (WZ7.3) führt sie ein und baut die wiederverwendbare Bühne, die WZ10 nutzt (siehe Offene Fragen). Komplexitäts-Budget: Datei ≤ 400, Funktion ≤ 60 Zeilen, Session ≤ ~400 Code-Zeilen.
- **Prüfungen im Fenster oder Browser** nur mit Freigabe von 🧑 für den Lauf.

## Beispiele

- Ralf öffnet im Asset-Studio den Welt-Betrachter → Seed `k3c`, Biom Wald; die Chunk-Vorschau füllt die Seite, darunter die eingeklappte Leiste „Live-Vorschau“.
- Ralf verkleinert die Chunk-Vorschau → oben der Chunk-Streifen, darunter die Live-Vorschau mit Burg und Hub in Hub-Stufe 5 (Kristall), Mauerlinien 1–5 gebaut.
- Hub-Stufe 5 → 3 → Burg der Stufe 3, Rüstkammer (`hubLevel` 4) fehlt, Mauerlinien 4 und 5 fehlen; Mauern und Türme zeigen reihum die Stufen 1 bis 3, Stufe 4 und 5 kommen nicht vor.
- Schmiede aus → die Schmiede wird nicht gezeichnet, alles andere bleibt; `saves/` und `data/` unverändert.
- Tageszeit Nacht, Fortschritt Mitte → die Oberfläche setzt Phase `night`, Fortschritt 0,5; die Setzfunktion setzt `w.Cycle` direkt und leitet `w.Time` ab; das Bild ist so dunkel wie mitten in der Nacht im Spiel.
- Tageszeit Morgengrauen, Fortschritt Anfang (p = 0) → die Oberfläche setzt Phase `night`, Fortschritt 0,9; die Helligkeit ist 0 wie bei Beginn des Morgengrauens im Client. p = 0,5 → Phase `night`, Fortschritt 0,95 (Gleitkomma, im Test `toBeCloseTo`), Helligkeit 0,5. Die Option ist wählbar, nicht ausgegraut; die Sim bekommt nie eine Phase `dawn`. Der Tagesbeginn (p = 1) ist kein Wert des Morgengrauens, sondern Phase `day` mit Fortschritt 0 und Helligkeit 1.
- Hintergrund an, Ralf scrollt nach rechts → hintere Schichten wandern langsamer als vordere; Hintergrund aus → nur Himmel und Boden.
- Keine Bürger und keine Gegner in der Vorschau, auch nicht die Startbesatzung aus `data/hub.json` › `startTroops` (Entscheidung 11).
- Dieselben Eingaben zweimal → identischer Snapshot (JSON byte-gleich) und gleiches Bild.
- Wald, „Im Spiel öffnen“ als neues Spiel → Browser öffnet `http://127.0.0.1:5183/#game.html?autostart=1&save=k3c`, das Spiel startet in der Shell.
- „Im Spiel öffnen“ voll ausgebaut → Studio legt `voll-k3c` über `POST /api/dev/fullsave` an und öffnet `…/#game.html?autostart=1&save=voll-k3c`.

## Ausnahme- und Fehlerfälle

- Seed leer oder länger als 64 Zeichen, Biom unbekannt → Hinweis an der Eingabe, die letzte gültige Ansicht bleibt stehen.
- Generator oder Snapshot meldet einen Fehler → Hinweis mit der Meldung, kein leeres Fenster, keine Ausnahme im Frontend.
- Mehrere Anfragen schnell hintereinander (Tippen, Umschalten) → nur die letzte wird gezeichnet (wie `request` in `leveltest.ts`).
- Hub-Stufe außerhalb von 1 … Anzahl `levels` → die Go-Funktion lehnt ab; die Oberfläche bietet nur gültige Stufen an.
- Tageszeit mit einer Phase außerhalb der drei Sim-Phasen `day`, `dusk`, `night` (auch `dawn`) oder Fortschritt außerhalb von [0, 1) (auch genau 1) → die Go-Funktion lehnt ab; die Oberfläche bietet nur die vier Einträge mit gültigem Fortschritt (p in [0, 1)) an und bildet Morgengrauen vorher auf `night` ab.
- Gebäude-Auswahl mit unbekanntem `kind@x` (z. B. nach Seed-Wechsel, weil die Linien 2–5 streuen) → wird ignoriert, der Platz erscheint wieder; eine Hub-Stufe ohne den Platz blendet seinen Schalter aus.
- Gebäudeart mit weniger Ausbaustufen als die erlaubte Höchststufe → die Rotation läuft über die vorhandenen Stufen der Art.
- Hub-Gebäude (Burg) mit weniger Ausbaustufen als die Hub-Stufe → höchste vorhandene Stufe.
- Biom ohne Hintergrund-Schichten → Schalter ausgegraut, Darstellung wie im Spiel (Himmel bzw. Platzhalter).
- Grafik fehlt in `public/` → Platzhalter wie im Spiel.
- „Im Spiel öffnen“: Spielserver nicht erreichbar → „Spielserver läuft nicht“, kein Browser; kein Dev-Mode (403) → Hinweis wie heute (`level.fullForbidden`); Spielstand in einem Raum mit verbundenem Gerät offen (409) → Hinweis, kein Browser.
- Neues Spiel mit einem anderen Biom als Wald oder Seed außerhalb des Namensformats → Knopf gesperrt, Grund daneben (B-095, `startTarget`).
- Seed passt nicht ins Format des Voll-Ausbaus (`^[a-z0-9-]{1,27}$`) → Auswahl „voll ausgebaut“ gesperrt mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Welt-Betrachter im Asset-Studio zeigt zu Seed und Quelle (Biom oder Insel + Stufe) Chunks (Index, Art), Objekte mit Legende und Anzahl, Hub-Linie und Warnungen wie `leveltest.html`, ohne laufenden Spielserver; in WZ7 ist die Quelle das Biom, die Schnittstelle trägt Insel + Stufe bereits (WZ8.5 stellt um). Nachweis: Go-Test des Studio-Bindings liefert für Seed `k3c` und jedes Biom dasselbe Layout und dieselben Warnungen wie `level.Generate`/`level.Validate`; die umgezogenen Tests aus `levelView.test.ts` laufen im Prüfziel des Studios grün.
- **AC-02** Die Chunk-Vorschau ist beim Öffnen maximiert und lässt sich per Knopf und Taste verkleinern und wieder maximieren; Zoom und Scrollen wie bisher. Nachweis: Frontend-Test des Ansichtszustands (Start `maximiert`), Beobachtung durch 🧑 im Studio.
- **AC-03** Eine exportierte Go-Funktion (SRV) mit Options-Struktur liefert zu Seed, Quelle (Biom bzw. Insel + Stufe), Hub-Stufe, Gebäude-Auswahl und Tageszeit (Sim-Phase `day`/`dusk`/`night` + Fortschritt in [0, 1)) Level und Zustand ohne Bürger und Gegner im Format von `level` und `snap`, ohne globalen Zustand (kein `sim.UseData`). Nachweis Go-Tests: gleiche Eingaben → byte-gleiches JSON; für jede Hub-Stufe aus `data/hub.json` › `levels` sind genau die bis dahin freigeschalteten Bauplätze gebaut; die Material-Stufe läuft je Art in Platzreihenfolge reihum von 1 bis zur bei dieser Hub-Stufe erlaubten Höchststufe (kein Platz über der Regel von `nextLevel`); abgewählte Plätze fehlen, die Stufen der übrigen ändern sich dadurch nicht; ungültige Hub-Stufe, eine Phase außerhalb der drei Sim-Phasen (auch `dawn`) und Fortschritt außerhalb von [0, 1) (auch genau 1) → Fehler; ein Aufruf verändert keinen Paket-Zustand (zwei Aufrufe in beliebiger Reihenfolge liefern dasselbe JSON); `saves/` bleibt unberührt. Die Options-Struktur lässt neue Felder zu, ohne bestehende Aufrufer zu ändern (Vorgabe für WZ8.4 und WZ9).
- **AC-04** Die Live-Vorschau zeichnet den Snapshot mit `StageView`/`WorldRenderer` aus `src/scenes/` (unverändert, über `@game`) auf einer wiederverwendbaren Phaser-Bühne (Szene samt Preload), die WZ10 ohne Kopie nutzen kann; Hub-Stufe über alle Einträge von `levels` umschaltbar (ein zusätzlicher Eintrag in einer Test-Fixture erscheint ohne Code-Änderung in der Auswahl), Gebäude einzeln ein- und ausschaltbar; gleiche Eingaben → gleiches Bild. Nachweis: Frontend-Tests (Auswahl aus `levels`, Gebäude-Liste aus dem Snapshot, der Snapshot enthält keine Bürger, Truppen und Gegner und die Szene erzeugt keine), `git diff` ohne Änderung unter `src/scenes/`, Beobachtung durch 🧑.
- **AC-05** Tageszeit mit den vier Einträgen Tag, Dämmerung, Nacht und Morgengrauen (alle wählbar, mit Fortschritt p in [0, 1) innerhalb des Eintrags) mit der Helligkeit aus `daylight()`, Hintergrund-Schichten ein/aus mit Parallaxe beim Scrollen wirken in der Live-Vorschau; Morgengrauen wird im Studio auf die Sim-Phase `night` mit Fortschritt 0,9 + 0,1·p abgebildet (p = 1 gehört nicht dazu, es wäre der Tagesbeginn: Phase `day`, Fortschritt 0). Nachweis: Go-Test der Setzfunktion für die drei Sim-Phasen `day`, `dusk`, `night`: Sie setzt `w.Cycle` (Phase und Fortschritt) direkt, `w.Time` wird passend abgeleitet, Phase und Fortschritt stehen im Snapshot; Fortschritt 1 wird abgelehnt; Frontend-Test der Abbildung der vier Einträge auf Phase und Fortschritt (Morgengrauen mit p = 0, 0,5 und 0,9 → `night` mit 0,9, 0,95 und 0,99; Helligkeit nach `daylight()` gleich p; alle Gleitkomma-Vergleiche mit `toBeCloseTo`); Tests der übrigen Optionen-Logik; Beobachtung durch 🧑.
- **AC-06** „Im Spiel öffnen“ öffnet im Standard-Browser `<Spiel-Adresse>/#game.html?autostart=1&save=<seed>` (neues Spiel) bzw. nach `POST /api/dev/fullsave` `…&save=voll-<seed>`; Adresse nur Loopback (Standard `http://127.0.0.1:5183`); Fehlerfälle als Hinweis laut Ausnahme- und Fehlerfälle. Nachweis: Tests der URL- und Fehlerlogik, Beobachtung durch 🧑.
- **AC-07** `leveltest.html`, `src/tools/leveltest.ts`, `levelView.ts`, `levelApi.ts` und ihre Tests sind entfernt, ebenso die Kacheln in `devTiles.ts` und `testTiles.ts` und die nicht mehr benutzten Texte `level.*`; Codemaps und die Sprint-Übersicht sind nachgezogen (Neuerzeugung bzw. von Hand). `git grep -n "leveltest" -- ':!.codemap/state.json' ':!docs/backlog' ':!docs/sprints/erledigt' ':!docs/sprints/*/WZ[67]-*'` ist leer, `task check` grün. Ausnahmen mit Grund: `.codemap/state.json` ist erzeugt; `docs/backlog` und `docs/sprints/erledigt` sind Historie; die Sprint-Ordner WZ6 und WZ7 nennen den Namen als Auftrag bzw. Ergebnis (nach der Erledigung liegen sie unter `docs/sprints/erledigt`).
- **AC-08** Go- und Frontend-Tests des Welt-Betrachters und der Vorschau-Funktion laufen in `task check:all` und in der CI mit.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):
1. **`leveltest.html` wird entfernt:** Der Welt-Betrachter läuft nur im Asset-Studio am PC (`tools/k3c-studio`, Wails, Frontend mit den Aliasen `@game`/`@data` aus WZ6). B-092 (Level-Betrachter auf dem Sofa, Sprint HW1) und B-315 sind berücksichtigt: B-092 wird umgeplant (Nicht-Ziel), B-315 lebt als „Im Spiel öffnen“ voll ausgebaut weiter (AC-06).
2. **Live-Vorschau:** Go erzeugt den Snapshot (neue exportierte Funktion in SRV auf Basis von `engine/room/fullsave.go`; Eingaben Seed, Biom/Stufe, Hub-Stufe, Gebäude-Auswahl), das Studio zeichnet mit `worldRenderer`; deterministisch.
3. **Gebäude ein/aus** ist nur Darstellung; die Hub-Stufe geht über alle Stufen aus `data/hub.json` › `levels` (heute 1–5, erweiterbar für B-458).
4. **Live-Optionen zusätzlich:** Tag/Nacht (Dämmerung, Nacht, Morgengrauen), Hintergrund-Schichten ein/aus mit Parallaxe beim Scrollen, ~~Truppen/Bürger am Hub ein/aus~~ (ersetzt durch Entscheidung 11: entfällt), Knopf „Im Spiel öffnen“ startet das Spiel mit diesem Seed bzw. Spielstand im Browser.
5. **Chunk-Vorschau** ist beim Öffnen maximiert.

Querschnitts-Entscheidungen für WZ7–WZ10 (Planung, 2026-10-17; gelten mit der Freigabe der Spec als bestätigt):
6. **Snapshot (X1):** exportierte Go-Funktion mit Options-Struktur, ohne globalen Zustand (kein `sim.UseData`); offen für WZ8.4 (Option „geplante Chunks“) und WZ9 (Option „Kandidaten-`hub.json`“, eigene SRV-Session).
7. **Tageszeit (X2):** eigene Setzfunktion mit Phase und Fortschritt statt `sim.DevSetPhase`. Die Sim kennt drei Phasen (`day`, `dusk`, `night`); `dawn` ist nur ein Ereignis beim Tagesbeginn. Die Setzfunktion setzt diese drei Phasen, die Oberfläche bietet vier Einträge, Morgengrauen wird im Studio als Phase `night` mit Fortschritt 0,9 + 0,1·p abgebildet (so zeichnet der Client es heute, `viewRules.ts`) und ist nicht ausgegraut. Eine echte Phase Morgengrauen (Q65) baut B-470, dann entfällt die Abbildung. Das Glossar nennt für das Morgengrauen noch ein falsches Ticket (B-213); das behebt B-466 (Verweis auf B-470).
8. **Phaser (X3):** B-460 (WZ7.3) führt Phaser im Studio-Frontend ein, in der Version der Wurzel-`package.json`, und baut die wiederverwendbare Bühne (Szene + Preload); WZ10 nutzt sie wieder. Die Auflösung über die Wurzel-`node_modules` oder einen Eintrag in `tools/k3c-studio/frontend/package.json` legt WZ7.3 fest (ungeprüft); eine weitere Abhängigkeit entsteht nicht.
9. **Chunk-Quelle (X4):** Biom oder Insel + Stufe; WZ8.5 stellt die Chunk-Vorschau auf Insel + Stufe um (gleiche Quelle wie die Live-Vorschau).

Beantwortet von 🧑 am 2026-10-17 (Chat, zu den früheren Annahmen 10–13; die Freigabe der Spec steht noch aus):
10. **Hub-Stufe n im Bild:** Stufen reihum wie im Voll-Ausbau (B-315), nicht „alle bis n in der höchsten Stufe“.
11. **Keine Bürger, keine Gegner, dazu die Umgebung:** Die Vorschau zeigt keine Bürger und keine Gegner, aber die Umgebung (Environment). Der Schalter „Bürger ein/aus“ aus Entscheidung 4 entfällt. „Umgebung“ heißt Gelände, Natur-Objekte und Hintergrund-Schichten (von 🧑 bestätigt am 2026-10-17).
12. **Kein HTTP-Endpunkt:** wie es technisch am einfachsten ist. Festgelegt: das Studio ruft die Go-Funktion direkt als Wails-Binding auf (Annahme bleibt), ein Endpunkt entfällt.
13. **Morgengrauen-Abbildung / Nacht:** Nacht und Morgengrauen sollen generell geändert werden: deutlich dunkler, dafür geben Fackeln und Lagerfeuer Licht. Das ist eine Darstellungsänderung im Spiel (CLI) und gehört nicht in WZ7: eigenes Ticket B-472. Die Abbildung des Morgengrauens im Studio-Frontend (Annahme 13 bisher) bleibt vorerst, bis 🧑 etwas anderes sagt; B-472 und B-470 müssen sie später mit anpassen.

## Notizen

- Abhängigkeit B-472 (Nacht dunkler, Licht von Fackeln und Lagerfeuern): ändert `daylight()` und die Darstellung der Nacht. WZ7.4 testet gegen `daylight()` (AC-05); wer von beiden später landet, zieht Test und Live-Vorschau nach.
- Verwandt: B-092 (Level-Betrachter, HW1: Controller-Prüfung am TV; wird umgeplant), B-095 (Start mit Seed und Tiefe), B-315 (Voll-Ausbau, `engine/room/fullsave.go`, `engine/net/dev.go`), B-458 (mehr Hub-Stufen), B-461 und B-462 (bauen auf der Seite auf), B-466 (Glossar: Morgengrauen-Verweis), B-470 (Morgengrauen als echte Phase; entfernt die Abbildung im Welt-Betrachter), B-103 (mehrere Inseln).
- B-426 Punkte 1 (Biom-Wahl beim Voll-Ausbau) und 4 (Taste V mit Strg) betreffen `leveltest.ts`; mit dem Entfernen entscheidet 🧑 in der Abnahme, ob sie für das Studio gelten oder entfallen. B-425 (Ausbauregeln des Voll-Ausbaus) bleibt unabhängig.
- Glossar: Asset-Studio, Welt-Betrachter, Chunk-Vorschau und Live-Vorschau trägt WZ6.1 ein; Level-Betrachter bleibt als Name von B-092 (Historie).
- **Chunk-Vorschau später auf Insel + Stufe (X4):** WZ7 liefert sie je Biom. WZ8.5 (B-461) stellt sie auf Insel + Stufe um, damit sie dieselbe Quelle wie die Live-Vorschau hat und geplante Chunks zeigt; deshalb trägt die Schnittstelle von Anfang an eine Quelle als Wert (AC-01).
- **Erweiterungen der Snapshot-Funktion (X1):** WZ8.4 ergänzt „geplante Chunks“, WZ9 („Kandidaten-`hub.json`“) eine eigene SRV-Session. Beides sind neue Felder der Options-Struktur; ein Aufruf ohne sie verhält sich wie in WZ7.1.
- **Phaser-Bühne (X3):** Die Bühne aus WZ7.3 ist die einzige im Studio; WZ10 (Monarch- und Gegner-Seite) baut keine zweite.
- **Aufräumen in AC-07:** Die Ausnahmen der Suche nach `leveltest` sind Historie bzw. erzeugte Dateien. Treffer in anderen Docs (z. B. `docs/sprints/README.md`) und in den Codemaps `src/tools/codemap.md` und `src/model/codemap.md` zieht die DEV-Session nach.
- **Tageszeit (Korrektur 2026-10-17):** Eine frühere Fassung ging davon aus, `cycle.go` sende `dawn` als Phase; das stimmt nicht (drei Phasen, `dawn` ist ein Ereignis beim Tagesbeginn). Spec, WZ7-Sessions und AC-03/AC-05 sind darauf angepasst.
- Korrekturen vor der Freigabe (2026-10-17): Snapshot-Optionen, Tageszeit mit Morgengrauen-Abbildung, Chunk-Quelle, Phaser-Bühne und AC-07 überarbeitet. Frühere Fassung: Revision 1 mit Spec nach den Entscheidungen von 🧑 (AC-03 Go-Funktion, AC-04 Live-Vorschau, AC-07 Entfernen von `leveltest.html`, AC-05, AC-06 und AC-08 neu).
