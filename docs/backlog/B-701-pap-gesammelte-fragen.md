# B-701 · PAP: kritische Fragen und vorab getroffene Entscheidungen werden gesammelt von Ralf bestätigt

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** PS1
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: gesammelter Fragen-Task)

## Ausgangslage

🧑 hat am 2026-11-06 festgelegt: Bis zum Sichtprüfungs-Sprint PS1 entscheidet Claude offene Punkte im Projekt PAP mit einem Default und arbeitet weiter; kritische Fragen werden hier gesammelt statt einzeln gestellt.

## Ziel

🧑 bestätigt oder korrigiert alle vorab getroffenen Entscheidungen in einem Durchgang (PS1.1 oder früher auf Wunsch).

## Beteiligte und Zielgruppen

🧑 entscheidet; Agenten des Projekts PAP tragen neue Entscheidungen hier ein.

## Anforderungen

- Jede Entscheidung als Zeile `D-nn · Thema · Default · Alternative · betroffene Sprints · Status (offen|bestätigt|geändert)` im Abschnitt Notizen.
- Agenten, die einen Default wählen, tragen ihn hier ein und nennen ihn im Ergebnis ihrer Session.
- **Kritisch** (Arbeit stoppt, statt Default): Kosten, Lizenz/Rechte, Datenverlust, Änderung an `main` oder Veröffentlichung.

## Nicht-Ziele

Sichtprüfungen selbst (PS1.2–PS1.4).

## Regeln und Einschränkungen

Memory-Regel „Autonom: Specs selbst freigeben“ gilt; Änderungen nach 🧑-Entscheid laufen als neue Tickets im Projekt PAP.

## Beispiele

MG4 braucht einen Rig-Typ für die Spinne → Default `requisite` mit Keyframes, Zeile D-08, MG4 arbeitet weiter.

## Ausnahme- und Fehlerfälle

Eine spätere Korrektur durch 🧑 macht Arbeit ungültig → Folgeticket im Projekt PAP, kein Rückbau ohne Ticket.

## Akzeptanzkriterien

- **AC-01** Jede Zeile D-nn hat den Status `bestätigt` oder `geändert` (mit Folgeticket).

## Offene Fragen

Siehe Notizen; keine blockiert die Umsetzung (Defaults gelten bis PS1.1).

## Notizen

| Nr. | Thema | Default (Claude) | Alternative | Sprints | Status |
|---|---|---|---|---|---|
| D-01 | Neue Abhängigkeiten | `@resvg/resvg-js@2.6.2`, `ajv@8.17.1` | eigener Validator ohne ajv | PV1 | offen |
| D-02 | Arbeitsweise | Domäne PAP, Session-Felder `Lane` und `Modell`, max. 4 Agenten parallel | alles in DEV, seriell | PV1 | offen |
| D-03 | Laufzeit Renderer | bun (`bun tools/paper/bin/paper.ts`) | Node mit Build | PV1, RD1 | offen |
| D-04 | Ausgaben im Repo | PNGs + Manifeste committet, Index je Gruppe | nur Specs committen, Build erzeugt PNGs | PV1, MG*, US1 | offen |
| D-05 | Golden-Regel | exakte Hashes, bei Plattform-Abweichung Toleranz 0,1 % Pixel | nur Toleranz | PV1, RD1 | offen |
| D-06 | Altes Register | ab MC1 eingefroren, Entfernung in CU1 | parallel weiter pflegbar | MC1, CU1 | offen |
| D-07 | Neue Papier-Materialien | `smaragd`, `purpur`, `stein`, `holz` mit von Claude gewählten Tönen | 🧑 wählt Töne | PV1 | offen |
| D-08 | Rig für Spinne, Golem | `requisite` mit Keyframes | neuer Rig-Typ `vielbeiner` | MG4, MG5 | offen |
| D-09 | Schatten auf Boden/Hintergrund | Phaser-4-Filter, sonst abgedunkelte Sprite-Kopie | im PNG backen | US1 | offen |
| D-10 | Workbench während der Umsetzung | eine Workbench aus der Repo-Wurzel im Dev-Modus; Agenten bauen keine eigene; neue MCP-Tools gelten erst nach Merge | eigene Instanz je Worktree | alle | offen |
| D-11 | Datei-Watch | Server ignoriert `*.paper.json`, Neustart nur bei geänderten PNGs | alles beobachten | US1 | offen |
| D-12 | Chat und „In Claude öffnen“ | Studio-Chat entfällt, „In Claude öffnen“ bleibt und wird zusätzlich Prompt-Button | beides weg | WB1 | offen |
| D-13 | Spec-Felder über das 005-Beispiel hinaus | `art` Pflicht je Lage; `flaeche.auflage` (kleiner Schatten); `ausgabe.massstab` (Pferd 1.3 wie im Prototyp); Einzeltöne `linie`, `edelstein` | Renderer leitet Lagen-Art und Maßstab selbst ab | PV1, RD1, MG* | offen |
| D-14 | Proportions-Toleranz | absolut: Kopfhöhe/Rahmenhöhe = 0,286 ± 0,15 | relativ ± 15 % | PV1, RD1, MG* | offen |
| D-15 | Dateinamen der Gebäude-Stufen | `<animation>.s<n>.png` (Stufe 1 ohne Zusatz), CLI `frame --stufe` | eigener Ordner je Stufe | PV1, RD1, MG6, US1 | offen |
| D-16 | Typecheck `tools/paper` | im Programm `tests/tsconfig.json` (Node-Typen), nicht im Client-Programm `src/` | eigene `tools/paper/tsconfig.json` mit eigenem Task | PV1 | offen |
| D-17 | Fixture-Pferd und Purpur-Umhang | `pferd-galopp-0` ohne Reiter, Zügel **und** Umhang; Purpur setzt auch die Lichtkante `#a865b8` | Umhang am Pferd lassen | PV1 | offen |
| D-RD1-01 | Öffentliche Specs und Task | Monarch (`idle`, `lauf`, `schritt`, `trab`, `galopp`) unter `public/grafik/figuren/`, Pferd unter `public/grafik/reittiere/`; Mauer bleibt Fixture (`gen:gebaeude` liest `public/grafik/gebaeude/*.json`); `gen:paper` als Task `paper` in `tools/Taskfile.yml` | Mauer nach `public/grafik/gebaeude/`, eigenes `tools/paper/Taskfile.yml` | RD1, MG* | offen |
| D-RD1-02 | Zeit und Papier-Filter | Phase `n / frames`, Physik-Schritt `round(n · 120 / (freq · frames))`, 240 Schritte Einschwingen; Seed = SHA-256(id) `% 1000 + 1`; Struktur-`intercept` 0.86 fest; Transformationen als `rotate(…)`-Kette (2-Nachkommastellen-Regel ohne Verzerrung) | Zeit über `fps`; `matrix(…)` mit mehr Stellen | RD1 | offen |
| D-RD1-03 | Umhang-Saum | Hermelin-Saum des Prototyps entfällt (Spec kennt kein Saum-Material); Licht-Kante als `falz` | Feld `umhang.saum` (Vertrags-Nachtrag) | RD1, PV-Nachtrag | offen |
| D-RD1-04 | Reiten | Reittier im Ordner der Spec, dann in Nachbar-Gruppen; neue CLI-Regel `reittier-unbekannt`; Zeichenfolge Reittier bis vor `hals`, Umhang, Reiter ohne `oberschenkel-h`, `hals`, Zügel (`falz`, `leder`); ohne `gangart` reitet `schritt` | Reittier-Pfad in der Spec | RD1 | offen |
| D-RD1-05 | Reit-Rahmen (Vertrags-Lücke) | Hülle aus Reittier-Rahmen · `massstab` und Reiter-Rahmen + [−2, −54], Pixel wie der Reiter (Monarch 242 × 261); Manifest trägt `rahmen`, `frameW`, `frameH` je Reit-Animation | Reittier-Rahmen ohne Reiter (schneidet den Reiter ab) | RD1, PV-Nachtrag, US1 | offen |
| D-RD1-06 | renderKey beim Reiten (Vertrags-Lücke) | laut Vertrag nur Spec + Stil; Änderung am Reittier ändert den `renderKey` des Reiters nicht (Cache von MC1/RV1 betroffen, `gen:paper` nicht) | Reittier-Hash in den Schlüssel | RD1, MC1, RV1 | offen |
| D-RD1-07 | Zusätzliche CLI-Regeln | `animation-unbekannt` (figur ohne Keyframes kennt nur `idle`/`lauf`, vierbeiner braucht `gangart`), `variante-unbekannt`, `stufe-unbekannt`, `frame-unbekannt`, `pflicht-teil-fehlt` für Vierbeiner ohne Beinteile oder `bein`; Spec-Pfad ohne `.paper.json` → Exit 2; ohne `animationen` gilt `stand` mit 1 Frame | Regeln in `check.ts` (Vertrag) | RD1, PV-Nachtrag | offen |
| D-RD1-08 | Alte Pack-Tests | `src/tools/grafikPacks.test.ts` und `credits.test.ts` nehmen Ordner mit `index.paper.json` aus (eigene Grafik, keine Credits) | Papierschnitt-Gruppen außerhalb `public/grafik/` | RD1, CU1 | offen |
| D-18 | Typen und Prompt-Füllung der Asset-Oberfläche | WB1 besitzt `frontend/src/api/typesAssets.ts`, `assets/api.ts` und `assets/prompts/fill.ts`; RV1 importiert nur | RV1 typisiert in `assets/review/` selbst | WB1, RV1 | offen |
| D-19 | Sound und Quellen im Bereich Assets | reine Listen (Status, Hörprobe als Data-URL ≤ 10 MB, Lizenz, Treffer mit „Öffnen“); Zuordnen, Recherche, Import bleiben MCP | Zuordnen in der Oberfläche | WB1 | offen |
| D-20 | „Im Spiel ansehen“ | Engine in `internal/studioview` auf `127.0.0.1:0`, fester Seed `ansicht` (`voll-ansicht`), Spielstände unter `UserCacheDir`; im Web-Client öffnet der Link nur auf diesem Rechner | Seed-Eingabe, fester Port | WB1 | offen |
| D-21 | Teil-Auswahl per Klick | kleinste Box gewinnt, bei Gleichstand das spätere Teil in `Spec.teile` | Auswahl-Liste der Teile | WB1 | offen |
| D-22 | Prüfung von `tools/assetkit` | in `k3c-workbench:gotest`/`lint` und im CI-Job k3c-workbench (bisher Studio) | eigener Task `assetkit:*` | WB1 | offen |
| D-RV1-1 | `stil` im Review | je Asset: wahr, wenn ein genutztes Material/Einzelton geändert ist oder Papier/Licht (alle); Fixture verlangt monarch `stil: false` | global: jeder geänderte `stilHash` (Vertragstext paper-mcp.md › 4) | RV1 | offen |
| D-RV1-2 | Reihenfolge der Review-Einträge | `ungueltig`, `geaendert`, `neu`, `geloescht`, je Status nach `id` | Reihenfolge der Fixture | RV1 | offen |
| D-RV1-3 | Stil der Basis beim Rendern | Basis und Arbeitsstand als Schnappschuss-Baum im Cache (`<renderKey>/basis|stand/public/grafik/…`); echte CLI muss den Stil relativ zur Spec finden, sonst `frame --stil <datei>` im Vertrag | Vertrag sofort um `--stil` erweitern | RV1, RD1 | offen |
| D-RV1-4 | Fixture `ui/AssetsList.json` | pferd und mauer `geaendert: true` sind veraltet (gegen `HEAD` unverändert); Tests erwarten `false`, Fixture nachziehen | Vertrag „gegen HEAD“ ändern | RV1, PV1 | offen |
| D-RV1-5 | Assets ohne Animation | kein Frame: `Asset „x“ hat keine Animation`; `pixelAnteil` 0 | Vertrag: implizite Animation (z. B. `stand`, 1 Frame) für die CLI | RV1, RD1 | offen |
| D-RV1-6 | Leere Animation in `AssetsFrame` | = erste Animation der Spec (wie MCP) | Pflichtfeld | RV1 | offen |
| D-RV1-7 | Cache der Attrappe | eigene Renderer-Kennung (`+attrappe`), damit Stub-Bilder nach RD1 nicht aus `.cache/paper/` kommen | gleicher Schlüssel, Cache beim Wechsel leeren | RV1, RD1 | offen |
| D-RV1-8 | Frontend-Transport und Prompt-Füllung | eigener Transport `assets/review/api.ts` und `prompt.ts`, gleich benannt wie WB1 (`api/typesAssets.ts`, `assets/prompts/fill.ts`); nach Merge von WB1 auf `backend.assets*`/`fillPrompt` umstellen | RV1 auf WB1 rebasen | RV1, WB1 | offen |
| D-RV1-9 | `origin/develop` als Basis | Fetch höchstens einmal je Minute und Checkout, danach gilt das Ergebnis (aktuell/lokal) weiter | Fetch bei jedem Aufruf | RV1 | offen |
| D-MC1-01 | Checkout für lesende Asset-Tools | alle `assets_*` nehmen `checkout` (`takesCheckout` per Präfix), schreibende in `writeTools` | nur schreibende | MC1 | offen |
| D-MC1-02 | Register-Tools ersetzen | `assets_list/get/set` direkt ersetzt (MC1.2), `assets_catalog` in MC1.4 entfernt | Umbenennen bis MC1.4 | MC1 | offen |
| D-MC1-03 | Review-Quelle | `assets_review` und `assets_preview {basis}` über `assetsui.Dienst` (RV1); Tests mit Fixture-Quelle `fixtures/ui/` | eigene Review-Logik in MC1 | MC1, RV1 | offen |
| D-MC1-04 | Bilder im Review-Text | PNG aus den Data-URLs nach `.cache/paper/mcp/`, Namen bereinigt; Stil-Zeile ohne Stil-Pfad und vorher/nachher (Review-Struktur liefert sie nicht) | Struktur um Stil-Pfad/Bilder erweitern | MC1, RV1 | offen |
| D-MC1-05 | Atomares Schreiben | Temp-Ordner `.cache/paper/tmp/<zufall>/` mit gespiegelten Nachbar-Specs (Reittier-Suche der CLI), Rollback der Spec, Sperre je Asset | System-Temp | MC1, RD1 | offen |
| D-MC1-06 | Ablehnung | Fehler-Ergebnis (`isError`) mit `abgelehnt, nichts geändert: n Fehler` und `feld · regel · text` | normales Ergebnis | MC1 | offen |
| D-MC1-07 | `assets_style_set` | nur Hinweis „Lane: stil“; Prüfung in Go (Schema über jsonschema-go, `material-doppelt`, `material-unbekannt` je Spec); Render nur der betroffenen Assets als Job (Fixture) | alle Assets rendern (Vertragstext), Klärung B-706 | MC1 | offen |
| D-MC1-08 | MCP-Prompts | Liste beim Start aus `data/asset-prompts.json` der Repo-Wurzel, Text bei jedem Abruf neu | Prompts je Checkout | MC1 | offen |
| D-MC1-09 | Rig-Vorlagen | Go-Tabelle `paperassets/rigs.go`; Vierbeiner mit Beinteilen, `eltern` und `bein` aus der Fixture pferd (echte CLI verlangt sie), gegen Attrappe und RD1-CLI getestet | aus `check.ts` lesen | MC1, RD1 | offen |
| D-MC1-10 | Gruppen-Index | `paperassets` trägt bei create/set/render ein und bei delete aus (CLI schreibt den Index nur bei `render <ordner>`); `golden.json` offen (B-706) | `render <ordner>` je Änderung | MC1, US1 | offen |
| D-MC1-11 | bun in Go-Tests | Pflicht, kein Skip | `t.Skip` | MC1 | offen |
| D-MC1-12 | `assets_list` | `validate` je Asset parallel (8) mit Cache nach Datei-Stand von Spec und Stil | `validate` mehrerer Specs in der CLI | MC1, RD1 | offen |
| D-MC1-13 | Hashes in Antworten | `renderKey` und `stilHash` in voller Länge statt `9c…(64 Zeichen)` | gekürzt | MC1 | offen |
| D-MC1-14 | Zweite CLI-Anbindung | `paperassets.CLI` (Zeitlimit 2 min, `K3C_PAPER_CLI`) neben `review.CLI`; Zusammenführung B-706 | sofort gemeinsamer Runner | MC1, RV1 | offen |
| D-MC1-15 | Asset ohne Animation in `assets_preview` | Fehler mit leerer Animationsliste (RD1 kennt implizit `stand`, D-RD1-07) | `stand` als Standard | MC1, RD1 | offen |
| D-MG2-01 | Farben der Reittiere ohne eigenes Material | rappe/albtraum Körper `maehne` (dunkelbraun statt schwarz), Mähne `maehne` bzw. `purpur`, albtraum Auge `koenigsrot`; hoellenhund Körper `maehne`, Kamm, Schwanzspitze und Auge `koenigsrot`; schimmel/einhorn/pegasus/weisser-wolf `hermelin`, Mähne `stoff`/`purpur`; wolf `stoff`, Augen wolf `gold`, weisser-wolf `edelstein`; elefant `stein`; hirsch `fell-fuchs`, Geweih `holz`; Horn `gold` | neue Materialien `fell-schwarz`, `fell-grau` (Lane `stil`) | MG2 | offen |
| D-MG2-02 | Schweif im Galopp | Schweif ragte in `galopp` links aus dem Rahmen; bei den zehn neuen Reittieren `ausgabe.rahmen` links um 10 Einheiten erweitert (−78, Breite 155); `pferd` (MG1) hat das noch | Rahmen-Hülle aus der Pose in RD1 | MG2, MG1, RD1 | offen |
| D-MG2-03 | Reihenfolge nach `assets_set` | Patch-Schlüssel kommen sortiert an (Animationen `galopp, schritt, trab`, erste Animation = `galopp`); hingenommen, Nachzug in MC1-Folge (B-706) | Reihenfolge des Patches erhalten | MG2, MC1 | offen |
| D-MG2-04 | Antwort von `assets_set` | wiederholt jedes geänderte Feld vollständig (bei ganzen `teile` mehrere KB); hingenommen, kürzen in B-706 | nur Pfade und Anzahl | MG2, MC1 | offen |
| D-MG2-05 | `assets_create` für `vierbeiner` | Vorlage ohne Animation rendert nicht (`animation-unbekannt`), erst `assets_set` mit Gangarten; hingenommen | Vorlage mit `schritt` | MG2, MC1 | offen |
| D-MG2-06 | Zusatzteile und Ohren über dem Kopf | Horn (einhorn), Geweih (hirsch) und Spitzohren (wolf, weisser-wolf, hoellenhund) ragten im `schritt` oben aus dem Rahmen; `ausgabe.rahmen` oben erweitert (−128/132); dadurch Reittiere der Gruppe ~4 % unterschiedlich skaliert (für US1 beachten) | Teile kleiner zeichnen | MG2, US1 | offen |
| D-MG2-07 | Wackelnder Frontend-Test RV1 | `AnimationSync.test.tsx` (Cache, Pixel-Diff einmal) im Gesamtlauf `check:diff` zweimal rot, einzeln grün; nicht MG2, an RV1/B-704 | eigenes Ticket | RV1 | offen |
| D-MG2-08 | Wölfe, Höllenhund, Elefant im Rig `vierbeiner` | Pferderumpf und feste Beinlänge (Pivots, Beinpfad) mit eigenem Kopf, Schwanz und `bein`-Breiten; wirken pferdeartig, Elefant im Galopp unnatürlich; hingenommen, Wunsch an RD1: Beinlänge und Rumpf je Tier über die Spec | eigener Rig-Typ (Hund, Dickhäuter) | MG2, RD1 | offen |
| D-MG3-01 | Animationen der Truppen | `idle` und `lauf` als Rig-Posen, `angriff` mit Keyframes (8 Frames, fps 14) für alle sechs Truppen | Frame-Zahlen der alten Sheets (4–7) | MG3, US1 | offen |
| D-MG3-02 | Frame-Breite je Truppe | Rahmen nach Waffenreichweite: Landstreicher/Krieger/Elite-Krieger 150, Bogenschützen 160 (Pfeil), Bauer (Heugabel) 185; Höhe einheitlich 181 | ein Rahmen für alle (Waffe wird abgeschnitten) | MG3, US1 | offen |
| D-MG3-03 | Elite-Truppen | eigene Assets mit edleren Materialien (gold, koenigsrot), keine Varianten des Grundtyps; Elite-Bogenschütze zusätzlich mit roter Hutfeder und goldenem Bogen (Silhouette) | Variante mit Material-Tausch | MG3 | offen |
| D-MG3-04 | Fehlende Materialien | Leinen als `haar` (Bauer), grauer Stoff als `stoff` (Landstreicher), Eisen als `stein`, Strohhut als `gold`; kein neues Material | Lane `stil`: `leinen`, `eisen`, `stroh` | MG3, Lane stil | offen |
| D-MG3-05 | Golden-Test der Gruppen | `tools/paper/golden/public.test.ts` erwartet „enthält figuren und reittiere“ statt genau diese zwei (jede MG-Gruppe brach den Test) | Liste je Sprint erweitern | MG3, MG* | offen |
| D-MG3-06 | `task gen:paper` | schreibt alle Gruppen; fremde Gruppen (nur Zeilenenden) zurückgesetzt, nur `truppen` committet | `gen:paper <gruppe>` | MG3, MG* | offen |
| D-MG3-07 | Pfeil der Bogenschützen | Pfeil hängt an der Zughand und bleibt nach dem Lösen sichtbar (Teile lassen sich nicht ausblenden); beim Nachlegen dreht er einmal | Sichtbarkeit je Keyframe (Vertrags-Nachtrag) | MG3, RD1 | offen |
| D-MG3-08 | Kleinbefunde Review MG3 | zurückgestellt bis zur Sichtprüfung PS1: Helm-Nasal unter dem Querbalken (Krieger, Elite), Schild nur in lauf/angriff sichtbar, Waffe schwingt in `lauf` voll mit, Stabknauf vor dem Bart (Landstreicher), Strohhut so gelb wie die Krone | sofort nachbessern | MG3, PS1 | offen |
| D-MG5-01 | Größe der Bosse | `hoehe_px` 288 (1,5 × Truppen/Monarch 192), Rahmen wie die Grundform | je Boss eigene Größe | MG5, US1 | offen |
| D-MG5-02 | Farben der Bosse | goblin-anfuehrer Haut `smaragd`; troll-koenig Haut `stein`, Krone `gold`, Keule `holz`; ratten-koenigin Fell `stoff`, Robe `purpur`; lava-golem `stein` mit Glut `koenigsrot`/`gold`; splitter-titan `stein` mit Kristallen `purpur`/`edelstein`; Herz `purpur` | neue Materialien (Lava, Kristall) in Lane `stil` | MG5 | offen |
| D-MG5-03 | Rig der Golems | lava-golem und splitter-titan als `figur` (gehen, schlagen), kristallherz-waechter als `requisite` mit Keyframes (steht); weicht für Golems von D-08 („requisite“) ab | Golem als `requisite` | MG5 | offen |
| D-MG5-04 | Lesbarkeit der Bosse (Review MG5.3) | bleibt wie gerendert, Sichtprüfung in PS1 entscheidet: lava-golem-Glut nur als dünne Risse (schwer vom splitter-titan zu unterscheiden), Wangenpunkt der Figur-Vorlage bei beiden Golems, lava-golem-Arme in idle hinter dem Rumpf, heller dünner Schwanz der ratten-koenigin, Holzschwert des troll-koenig kontrastarm | Glut flächig in `koenigsrot`/`gold`, Wangenpunkt entfernen, Arme abspreizen, Schwanz mit ton `schatten` | MG5, PS1 | offen |
| D-MG7-01 | Varianten der Hintergründe | je Biom und Ebene zwei Varianten `v1`, `v2` (30 Assets); `v3`–`v5` des Pixel-Stands folgen erst nach der Sichtprüfung | alle fünf Varianten sofort (75 Assets) | MG7, PS1 | offen |
| D-MG7-02 | Namen und Größe der Hintergründe | IDs `hg-<biom>-<ebene>-v<n>` wie im Pixel-Stand (Biome wald, hoehle, mine, eisenstollen, kristall), Gruppe `hintergruende`, Rahmen `[0,-480,768,480]` (24 × 15 Kacheln), `hoehe_px` 480 | eigene Paper-IDs, höhere Auflösung | MG7, US1 | offen |
| D-MG7-03 | Kacheln der Parallax-Ebenen | Rig `hintergrund` kennt kein waagrechtes Kacheln: Profile sind periodisch (linker und rechter Rand gleich hoch), Einzelmotive halten Abstand zum Rand; Parallax, Wiederholung und Himmel macht der Client | Rig-Option `kachel` mit Prüfregel in Lane `render` | MG7, US1 | offen |
| D-MG7-04 | Biom-Paletten der Hintergründe | keine Biom-Töne in der Stil-Datei: Biom über vorhandene Materialien (wald `smaragd`/`holz`, hoehle `stein`, mine `leder`/`holz`/`stein`, eisenstollen `stein` mit Glut `koenigsrot`/`gold`, kristall `koenigsblau`/`purpur`/`edelstein`); Tiefe über Ton: fern nur `schatten`, mitte Grundton mit Schatten-Auflagen, nah mit Licht- und Schattenkanten; Abdunkeln und Nebel je Ebene im Client | eigene Hintergrund-Materialien je Biom in Lane `stil` | MG7, PS1 | offen |
