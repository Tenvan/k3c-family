# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-622](B-622-fackeln-als-spielobjekt.md) | SIM | Idee | niedrig | eingeplant | W15 | Fackeln gibt es als Spielobjekt mit Licht in der Nacht |
| [B-699](B-699-papierschnitt-grafik-mcp-review.md) | DEV | Idee | hoch | eingeplant | MG11 | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench |
| [B-720](B-720-bosse-neu.md) | PAP | Problem | hoch | eingeplant | MG10 | Bosse sind neu gestaltet, jeder mit eigener Silhouette |
| [B-775](B-775-native-umstellung-rust-bevy.md) | CLI | Idee | hoch | eingeplant | NG9 | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server |
| [B-879](B-879-welten-editor-workbench.md) | DEV | Problem | hoch | eingeplant | WZ58 | Die Workbench hat den Welten-Editor aus dem alten Studio wieder |
| [B-880](B-880-asset-pakete-je-welt.md) | PAP | Idee | mittel | eingeplant | PT6 | Assets lassen sich zusammen mit einer Welt zu Paketen bündeln |
| [B-884](B-884-abnahme-mg11.md) | PAP | Schuld | mittel | offen | MG11 | Abnahme MG11: Sichtprüfung der Welt-Grafik und Gen-Läufe sind erledigt |
| [B-882](B-882-abnahme-laeufe-wz45.md) | DEV | Schuld | mittel | offen | WZ45 | Abnahme-Läufe WZ45 |
| [B-885](B-885-native-rig-export-bosse.md) | PAP | Schuld | mittel | eingeplant | RU61 | Nativer Rig-Export der Bosse ist nach MG10 veraltet |
| [B-886](B-886-abnahme-mg10.md) | PAP | Schuld | mittel | offen | MG10 | Abnahme MG10: Sichtprüfung der Bosse und Gen-Läufe sind erledigt |
| [B-907](B-907-abnahme-wr1-gen-race.md) | SRV | Schuld | niedrig | offen | WR2 | Abnahme WR1: gen:paper samt Grafik-Golden und check:race laufen nach dem Rückbau grün |
| [B-889](B-889-nativ-debug-werte-und-logs.md) | CLI | Idee | hoch | offen | RU61 | Das native Spiel zeigt Debug-Werte der Sim (F10) und loggt Zustandswechsel und Zusammenfassungen |
| [B-890](B-890-nativ-hud-icons-statt-namen.md) | CLI | Idee | hoch | offen | RU61 | Das HUD des nativen Spiels zeigt Ressourcen als Icon mit Zahl statt als Namen |
| [B-891](B-891-biom-eigenschaften-je-welt.md) | SIM | Idee | mittel | offen | RU61 | Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt |
| [B-895](B-895-workbench-webview-speicherleck.md) | DEV | Problem | hoch | offen | WZ60 | Die Oberfläche der Workbench wächst nicht unbegrenzt im Speicher und bleibt nach Stunden mit parallelen Prüfläufen bedienbar |
| [B-908](B-908-pages-bausatz.md) | PLAT | Idee | mittel | offen | GP1 | Die Präsentationsseite zeigt das Spiel als Bausatz: Einzelspieler und Couch-Koop, nativ, frei für nicht-kommerzielle Nutzung, eigene Welten mit dem MCP-Server |
| [B-910](B-910-abnahme-bi2.md) | CLI | Idee | mittel | eingeplant | RU61 | Abnahme BI2: Biom-Darstellung ist gesichtet, Grafik gerendert und Golden geprüft |
| [B-911](B-911-papierschnitt-tasten-icons.md) | PAP | Idee | mittel | eingeplant | RU61 | Tastenkappen und Xbox-Tasten gibt es als Papierschnitt-Icons der Gruppe icons |
| [B-913](B-913-abnahme-ep1.md) | CLI | Idee | mittel | offen | EP1 | Ralf nimmt EP1 ab: Eingabe-Profile und Tasten-Icons an TV und PC geprüft |
| [B-914](B-914-abnahme-st2.md) | CLI | Idee | mittel | offen | ST2 | Ralf nimmt ST2 ab: Testing-Menü und Stresstest-Bericht am PC geprüft |
| [B-935](B-935-klangbild-couch.md) | CLI | Problem | hoch | offen | SR1 | Klangbild für Couch-Abende festlegen und konsolige Piep-Sounds ersetzen |
| [B-936](B-936-musik-ruhiger.md) | CLI | Problem | hoch | offen | SR1 | Ruhigere, langsamere Musikstücke passend zur Atmosphäre finden oder erzeugen |
| [B-938](B-938-weltauswahl-menue.md) | CLI | Idee | mittel | offen | RU61 | Neues Spiel beginnt mit der Weltauswahl im großen Bild, Spielstände und Pausenmenü zeigen das kleine |
| [B-940](B-940-kampf-assets.md) | PAP | Idee | mittel | offen | RU60 | Papierschnitt-Assets für Krone, Banner, Schild, Zahlstellen, Portal-HP und Münzschauer ersetzen die gezeichneten Formen |
| [B-941](B-941-ausloeser-pfahl-vor-tor.md) | SIM | Problem | mittel | offen | RU60 | Der Angriffs-Auslöser steht als Horn auf einem Pfahl vor dem Tor statt schwebend dahinter |
| [B-942](B-942-schild-gestell-burgstufe.md) | SIM | Idee | mittel | offen | RU60 | Schilde gibt es erst ab höherer Burgstufe, gestapelt auf einem Gestell neben der Burg |
| [B-944](B-944-xbox-tasten-testseite.md) | CLI | Problem | hoch | offen | RU61 | Xbox-Controller-Tasten stimmen wieder, mit Testseite in den Entwickleroptionen |
| [B-947](B-947-hintergrundmauern-luecken.md) | CLI | Problem | mittel | offen | LA1 | Die durchgezogenen Hintergrundmauern haben keine Lücken mehr |
| [B-948](B-948-baeume-groesser.md) | CLI | Problem | mittel | offen | RU61 | Bäume erscheinen in passender Größe statt zu klein |
| [B-949](B-949-ladescreen-huebsch.md) | CLI | Idee | niedrig | offen | RU61 | Der Ladescreen ist im Stil des Spiels gestaltet |
| [B-953](B-953-truhen-fontaene.md) | SIM | Problem | mittel | offen | RU60 | Truhen geben zehnmal mehr Gold und werfen es als Fontäne auf den Boden |
| [B-959](B-959-welten-editor-nachtraege.md) | DEV | Problem | mittel | offen | WE3 | Welten-Editor-Nachträge: Endboss je Insel-Thema, Hafen-Ablauf, Event- und Sites-Overrides, Gegnerarten je Welt |
| [B-960](B-960-welten-inhalte.md) | PAP | Idee | niedrig | offen | WE3 | Inhalte für die neuen Welten: Assets, Events und Gegnerarten für Sengoku, Nil und Britannien |
| [B-962](B-962-seekarte-im-spiel.md) | CLI | Idee | hoch | eingeplant | RU60 | Am Hafen wählen die Spieler die Zielinsel auf einer Seekarte |
| [B-964](B-964-seekarte-spiel-grafik.md) | PAP | Idee | mittel | eingeplant | RU60 | Die Seekarte im Spiel ist eine gestaltete Papierschnitt-Karte statt gezeichneter Formen |
| [B-965](B-965-geschichte-kunstsprache.md) | REG | Idee | mittel | eingeplant | RU60 | Jede Welt und Insel erzählt eine kurze Geschichte in Kunstsprache mit eingestreuten echten Wörtern |
| [B-966](B-966-chronik-der-reise.md) | REG | Idee | niedrig | eingeplant | GS3 | Abgeschlossene Inseln hinterlassen auf der Seekarte ein Andenken und eine Zeile der Geschichte |
| [B-967](B-967-fracht-auf-dem-schiff.md) | REG | Idee | mittel | eingeplant | RU62 | Vor der Abfahrt wählen die Spieler die Fracht des Schiffs |
| [B-968](B-968-koop-aktionen-zu-zweit.md) | REG | Idee | mittel | eingeplant | RU62 | Zwei Monarchen am selben Ort lösen gemeinsame Koop-Aktionen aus |
| [B-969](B-969-wetter-je-welt.md) | REG | Idee | mittel | eingeplant | RU62 | Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung |
| [B-970](B-970-reittiere-je-welt.md) | REG | Idee | mittel | eingeplant | RU62 | Jede Welt hat ein eigenes Reittier mit kleiner Eigenheit |
| [B-971](B-971-flaschenpost.md) | REG | Idee | niedrig | eingeplant | GS4 | Am Strand treibt eine Flaschenpost mit Geschichtszeile und Hinweis an |
| [B-972](B-972-einsiedler-mit-angebot.md) | REG | Idee | mittel | eingeplant | RU62 | Auf jeder Insel wohnt eine besondere Figur der Welt mit einem einmaligen Angebot |
| [B-973](B-973-fest-nach-dem-sieg.md) | REG | Idee | niedrig | eingeplant | RU62 | Nach dem Sieg über den Endgegner feiert das Volk ein Fest |
| [B-974](B-974-geschichte-einblendung-spiel.md) | CLI | Idee | mittel | eingeplant | RU60 | Das Spiel blendet die Geschichtszeilen an den Wendepunkten je Spielerhälfte ein |
| [B-975](B-975-geschichte-welten-werkzeuge.md) | DEV | Idee | mittel | eingeplant | GS1 | Die Welten-Werkzeuge schreiben und prüfen die Geschichte, „Welt anlegen“ erzeugt sie mit |
| [B-976](B-976-geschichten-vier-welten.md) | REG | Idee | mittel | eingeplant | GS1 | Die vier vorhandenen Welten haben Wortschatz und Geschichtszeilen |
| [B-977](B-977-seekarte-schiffsfahrt.md) | CLI | Idee | mittel | eingeplant | RU60 | Beim Inselwechsel fährt das Schiff im Vollbild über die Seekarte zur Zielinsel |
| [B-978](B-978-spielstand-auswahl-seekarte.md) | CLI | Idee | niedrig | eingeplant | GS3 | Die Spielstand-Auswahl zeigt die Seekarte mit den Trophäen als Vorschau |
| [B-979](B-979-sengoku-etappen-1-2-2.md) | SIM | Idee | mittel | eingeplant | WE5 | Sengoku nutzt die Etappen 1, 2, 2: Insel 3 bekommt einen Hafen |
| [B-980](B-980-flaschenpost-im-spiel.md) | CLI | Idee | niedrig | eingeplant | GS4 | Das Spiel zeigt Flaschenpost, Richtungspfeil und leuchtendes Ziel in beiden Spielerhälften |
| [B-981](B-981-endboss-je-insel.md) | SIM | Idee | mittel | eingeplant | RU60 | Jede Insel wählt ihren Endboss, der Bau liegt in ihrer tiefsten Stufe statt nur in der Kristallhöhle |
| [B-982](B-982-overrides-events.md) | SIM | Idee | mittel | eingeplant | RU60 | Eine Welt übersteuert die Events der Nacht über overrides › events |
| [B-983](B-983-overrides-hub-sites.md) | SIM | Idee | mittel | eingeplant | RU60 | Eine Welt übersteuert die Plätze hub › sites und islandSites, der Level-Generator liest den Wertesatz der Welt |
| [B-984](B-984-welt-skins-feld.md) | SIM | Idee | mittel | eingeplant | RU60 | Die Welt-Datei ordnet mit skins Gegnern und Objekten eigene Assets zu (Gegner-Varianten je Welt) |
| [B-987](B-987-welten-editor-neue-felder.md) | DEV | Idee | mittel | eingeplant | WE6 | Welten-Editor und Welten-MCP bearbeiten Endboss, Skins, Events und Plätze einer Welt |
| [B-988](B-988-assets-sengoku.md) | PAP | Idee | niedrig | eingeplant | WE7 | Sengoku bekommt die Assets Torii, Pagode, Ninja, Samurai und Samurai-Lager |
| [B-989](B-989-assets-nil.md) | PAP | Idee | niedrig | eingeplant | WE7 | Nil bekommt die Assets Palme, Barke, Obelisk, Skorpion und Mumie |
| [B-990](B-990-assets-britannien.md) | PAP | Idee | niedrig | eingeplant | WE7 | Britannien bekommt die Assets Steinkreis, Rundhaus, römischer Späher und Moorgeist |
| [B-991](B-991-event-ueberfall.md) | SIM | Idee | niedrig | eingeplant | WE8 | Das Event Überfall bringt jede n-te Nacht eine zusätzliche Gruppe einer Gegnerart (Ninja-Überfall in Sengoku) |
| [B-992](B-992-events-sandsturm-nebel.md) | SIM | Idee | niedrig | eingeplant | WE8 | Die Events Sandsturm und Nebel verändern für eine Nacht Tempo und Fernkampf-Reichweite an der Oberfläche |
| [B-993](B-993-spiel-endboss-bau-je-biom.md) | CLI | Idee | mittel | eingeplant | WE6 | Das Spiel zeigt Bau und Endboss in jedem Biom, ohne eigenes Asset mit dem der Basisart |
| [B-994](B-994-spiel-skins-je-welt.md) | CLI | Idee | mittel | eingeplant | WE6 | Das Spiel zeichnet Gegner und Objekte mit den Skins der Welt |
| [B-995](B-995-spiel-zeigt-welt-events.md) | CLI | Idee | niedrig | eingeplant | WE8 | Das Spiel kündigt Überfall, Sandsturm und Nebel an und zeigt sie in der Nacht |
| [B-996](B-996-welten-inhalte-zuordnen.md) | SIM | Idee | niedrig | eingeplant | WE8 | Sengoku, Nil und Britannien nutzen ihre Assets, Gegner-Varianten und Events |
| [B-997](B-997-fracht-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | RU62 | Ladeplätze am Steg und Fracht an Deck sind im Spiel sichtbar |
| [B-998](B-998-doppel-truhe-asset-anzeige.md) | CLI | Idee | mittel | eingeplant | RU62 | Die Doppel-Truhe hat ein eigenes Bild und zeigt, welcher Monarch fehlt |
| [B-999](B-999-wetter-vorboten-darstellung.md) | CLI | Idee | mittel | eingeplant | RU62 | Vorboten, Taifun und Flut sind im Spiel sichtbar und hörbar |
| [B-1000](B-1000-reittiere-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | RU62 | Gatter und Welt-Reittiere sind im Spiel sichtbar, der Monarch reitet sein gekauftes Tier |
| [B-1001](B-1001-einsiedler-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | RU62 | Hütte, Figur und Bauwerk der Welt sind im Spiel sichtbar |
| [B-1002](B-1002-fest-darstellung-musik.md) | CLI | Idee | niedrig | eingeplant | RU62 | Das Fest zeigt Jubel, Himmels-Effekt je Welt und spielt Festmusik |
| [B-1003](B-1003-welten-editor-wetter-reittier-figur.md) | DEV | Idee | niedrig | eingeplant | SI7 | Der Welten-Editor wählt Wetter, Reittier und Figur einer Welt aus den Katalogen |
| [B-1006](B-1006-steg-gehaltenes-a-zahlt-nicht.md) | CLI | Problem | mittel | eingeplant | RU61 | Gehaltenes A am Steg zahlt nach der Zielwahl erst wieder nach dem Loslassen |
| [B-1007](B-1007-bin-alte-exe-aufraeumen.md) | INF | Schuld | niedrig | eingeplant | WZ61 | task ops:aufraeumen räumt die EXE entfernter Programme aus bin/ ab |
| [B-1008](B-1008-kronenraub-ueberlebt-laden-nicht.md) | SIM | Problem | hoch | offen | RU60 | Ein laufender Kronenraub bleibt nach Verlassen und Weiter bestehen |
| [B-1009](B-1009-anzeigename-geselle-troops.md) | SIM | Schuld | niedrig | offen | RG2 | Der Beruf craftsman heißt in data/troops.json Geselle statt Handwerker |
| [B-1010](B-1010-abnahme-we7-welten-assets.md) | PAP | Frage | mittel | offen | WE7 | Abnahme WE7: Welten-Assets sind gesichtet, Gen-Läufe und Texturbudget geprüft |
| [B-1011](B-1011-abnahme-gs2-seekarte.md) | CLI | Frage | mittel | offen | RU60 | Abnahme GS2: Seekarte als Papierschnitt und Schiffsfahrt sind am TV gesichtet, Gen-Läufe geprüft |
| [B-1012](B-1012-seekarte-nachtraege-review.md) | CLI | Schuld | mittel | offen | RU60 | Seekarten-Nachträge aus dem Review: Reihenfolge von completed abgesichert, gehaltenes A nach der Fahrt, Stufen der alten Insel im Wechsel-Frame |
| [B-1013](B-1013-abnahme-laeufe-we5.md) | SIM | Schuld | mittel | eingeplant | WE5 | Abnahme-Läufe WE5 |
| [B-1014](B-1014-rust-umstellung-analyse.md) | INF | Idee | mittel | eingeplant | RU1 | Spiel und Simulation laufen komplett in Rust (Analyse, Empfehlung, Gates) |
| [B-1015](B-1015-rust-umstellung-wellen-kosten.md) | INF | Idee | mittel | eingeplant | RU1 | Wellen- und Kostenplan der Rust-Umstellung |
| [B-1016](B-1016-rust-client-spike-g1.md) | CLI | Frage | mittel | eingeplant | RU2 | Ein schlanker Rust-Client hält die Messwerte der Ebitengine (Spike, Gate G1) |
| [B-1017](B-1017-laeufe-im-game-client.md) | CLI | Idee | mittel | eingeplant | RU33 | Sim und Balancing laufen im Game-Client, headless oder mit Fenster per Parameter |
| [B-1018](B-1018-rust-port-leitfaden.md) | INF | Idee | mittel | eingeplant | RU1 | Port-Leitfaden: Regeln, nach denen jede Port-Session von Go nach Rust arbeitet |
| [B-1021](B-1021-abnahme-ru1-rust-workspace.md) | INF | Idee | mittel | offen | RU1 | Abnahme RU1: Rust-Workspace, check:race und CI-Lauf prüfen |
| [B-1022](B-1022-abnahme-wz61-race.md) | DEV | Schuld | niedrig | offen | WZ61 | Abnahme WZ61: check:race über die geänderten Workbench- und Cleanup-Pakete läuft grün |
| [B-1028](B-1028-ru4-abnahme-gen-race.md) | CLI | Schuld | mittel | offen | RU4 | Abnahme RU4: Gen- und Race-Läufe sowie Hardware-Prüfung der Rust-Plattform stehen aus |
| [B-1023](B-1023-go-exp-haengt-an-fma.md) | SIM | Problem | mittel | offen | RU22 | Go-math.Exp rechnet auf CPUs ohne AVX/FMA andere Bits als die Golden |
| [B-1024](B-1024-rust-datenpruefungen-mit-fremden-tabellen.md) | SIM | Schuld | mittel | offen | RU25 | Rust-Datenprüfungen, die Tabellen aus Balance oder Sim brauchen, fehlen noch in k3c-data |
| [B-1027](B-1027-abnahme-laeufe-ru3.md) | SIM | Schuld | mittel | offen | RU3 | Abnahme-Läufe RU3 |
| [B-1033](B-1033-abnahme-si4.md) | PAP | Schuld | mittel | offen | SI4 | Abnahme-Läufe Sprint SI4 Reittiere je Welt sind grün |
| [B-1034](B-1034-si4-engine-anzeige-offen.md) | SIM | Frage | hoch | offen | SI4 | Engine und Anzeige der Reittiere je Welt sind nach Verwerfen von SI4.3 und SI4.4 ohne Ersatz |
| [B-1029](B-1029-abnahme-laeufe-gs1.md) | REG | Schuld | mittel | offen | GS1 | Abnahme-Läufe GS1: Gen samt Golden und Race sind grün |
| [B-1031](B-1031-abnahme-si3.md) | PAP | Schuld | mittel | offen | SI3 | Abnahme-Läufe SI3 Wetter je Welt sind grün |
| [B-1032](B-1032-abnahme-laeufe-si2.md) | INF | Schuld | mittel | offen | SI2 | Abnahme-Läufe SI2: Sim, Balance, Gen samt Golden und Race sind grün |
| [B-1036](B-1036-abnahme-laeufe-si5.md) | INF | Schuld | mittel | offen | SI5 | Abnahme-Läufe SI5: Sim, Balance, Gen samt Golden und Race sind grün |
| [B-1037](B-1037-abnahme-laeufe-si6.md) | INF | Schuld | mittel | offen | SI6 | Abnahme-Läufe SI6: Sim, Balance, Gen samt Golden und Race sind grün |
| [B-1048](B-1048-abnahme-laeufe-ru7.md) | CLI | Schuld | mittel | offen | RU7 | Abnahme-Läufe RU7: Grafik laden samt gerenderter Grafik und Race sind grün |
| [B-1035](B-1035-abnahme-laeufe-si1.md) | INF | Schuld | mittel | offen | SI1 | Abnahme-Läufe SI1: Sim, Balance, Gen samt Golden und Race sind grün |
| [B-1038](B-1038-buerger-truppen-masstab.md) | PAP | Idee | mittel | offen | BB1 | Bürger und Truppen sind 0,55–0,65 Monarchenhöhen groß |
| [B-1039](B-1039-schiff-masstab.md) | PAP | Idee | mittel | offen | BB1 | Schiff ist mit Segel etwa 3 Monarchenhöhen hoch und 4–5 lang |
| [B-1040](B-1040-hub-gebaeude-masstab.md) | PAP | Idee | mittel | offen | BB1 | Hub-Gebäude folgen den Monarchenhöhen, die Burg ist das größte Gebäude |
| [B-1041](B-1041-waldwand-baumvarianten.md) | PAP | Idee | mittel | offen | BB1 | Waldbäume in mehreren Höhen und Tönen ergeben eine geschlossene Waldwand |
| [B-1042](B-1042-leitmotiv-je-welt.md) | PAP | Idee | mittel | offen | BB1 | Jede Welt hat ein großes Leitmotiv in der Ferne des Hubs |
| [B-1043](B-1043-gehlinie-ufer.md) | CLI | Idee | mittel | eingeplant | RU55 | Gehlinie liegt bei 58 % der Ansicht, das Ufer ist 7 % hoch |
| [B-1044](B-1044-fluss-spiegelt-welt.md) | CLI | Idee | mittel | eingeplant | RU55 | Der Fluss spiegelt die Welt über der Gehlinie |
| [B-1045](B-1045-mehr-welt-sichtbar.md) | CLI | Idee | mittel | eingeplant | RU55 | Die Ansicht zeigt etwa 5 Monarchenhöhen Welt über der Gehlinie |
| [B-1046](B-1046-toenung-licht-dunst.md) | CLI | Idee | mittel | eingeplant | RU55 | Tageszeit und Ereignis tönen das ganze Bild, Lichtquellen haben einen Lichthof |
| [B-1047](B-1047-wald-nach-rodung.md) | CLI | Idee | mittel | eingeplant | RU55 | Der Wald wird mit dem Roden offener und heller |
| [B-1049](B-1049-typen-worlddef-island-fuer-ru7-5-vorziehen.md) | SIM | Schuld | mittel | offen | RU10 | Typen WorldDef und Island für RU7.5 vorziehen |
| [B-1050](B-1050-abnahme-ru8-sim-zustand-race-und-golden.md) | SIM | Schuld | mittel | offen | RU8 | Abnahme RU8: Race- und Golden-Läufe des Rust-Sim-Zustands stehen aus |
| [B-1052](B-1052-abnahme-laeufe-ru5.md) | CLI | Schuld | mittel | offen | RU5 | Abnahme-Läufe RU5 |
| [B-1053](B-1053-client-sim-naehte-ersetzen.md) | CLI | Schuld | mittel | eingeplant | RU30 | Sim-Nähte im Rust-Client sind durch die portierte Sim ersetzt |
| [B-1054](B-1054-verwaiste-sim-tests-zuordnen.md) | SIM | Schuld | mittel | eingeplant | RU22 | Jede Go-Testdatei aus engine/sim ist in Rust portiert oder begründet ausgelassen |
| [B-1059](B-1059-abnahme-ru13-welt-akteure.md) | CLI | Schuld | mittel | offen | RU13 | Abnahme RU13: Gen, Bildtests, Race und Sichtprüfung der Standbilder |
| [B-1057](B-1057-ru38-abnahme-sim-balance-gen.md) | SIM | Schuld | mittel | offen | RU38 | RU38 Abnahme: Fenster-Messung der Wiedergabe, Sim, Balance und Gen |
| [B-1063](B-1063-text-2px-tiefer-als-go.md) | CLI | Problem | niedrig | offen | RU16 | Rust setzt Text im Software-Rasterer rund 2 px tiefer als Go |
| [B-1060](B-1060-ton-atlas-ausschnitte-rust.md) | CLI | Schuld | mittel | eingeplant | RU30 | Der Rust-Client spielt Effekte aus Ausschnitten des Ton-Atlas |
| [B-1062](B-1062-ru17-abnahme-sim-balance-gen.md) | CLI | Schuld | mittel | offen | RU17 | RU17 Abnahme: Sim, Balance, Gen und Sichtprüfung Effekte und Ton |
| [B-1064](B-1064-ru16-abnahme-sim-balance-gen.md) | CLI | Schuld | mittel | offen | RU16 | RU16 Abnahme: Sim, Balance, Gen und Sichtprüfung Gebäude und Objekte |
| [B-1067](B-1067-rust-purse-skill-values-pending-draw-glyph-ausleihe.md) | CLI | Problem | niedrig | offen | RU20 | Hud: Spieler mit Skills stürzen in sim::values ab, draw_glyph mit Bibliotheksschrift vermutlich Doppelausleihe |
| [B-1068](B-1068-abnahme-ru20.md) | CLI | Schuld | mittel | offen | RU20 | Abnahme RU20: Sichtprüfung HUD und Gen-Läufe |
| [B-1069](B-1069-rust-fenstergroesse-aus-config.md) | CLI | Schuld | niedrig | eingeplant | RU44 | k3c-platform öffnet das Fenster fest mit 1280x720, --size der Wiedergabe wirkt nicht |
| [B-1074](B-1074-gate-messung-g2-ruhiger-pc-mini-pc.md) | CLI | Schuld | mittel | offen | RU36 | Gate-Messung G2 am ruhigen PC und Mini-PC samt Sichtprüfung (aus RU39.2) |
| [B-1076](B-1076-stab-sim-panics-in-result-umbauen.md) | CLI | Schuld | niedrig | eingeplant | RU43 | Sim-Panics (Go-Nil-Dereferenz, Ereignis, Tag/Nacht-Zyklus) und Test-Fixtures in Result umbauen |
| [B-1077](B-1077-perf-allokationen-im-replay-takt.md) | CLI | Schuld | mittel | eingeplant | RU44 | PERF: Replay-Takt (step) allokiert bis zu 788 mal je Bild, Ziel 0 |
| [B-1078](B-1078-perf-allokationen-im-draw-pfad.md) | CLI | Schuld | mittel | eingeplant | RU44 | PERF: draw allokiert bis zu 29 mal je Bild, Ziel 0 |
| [B-1081](B-1081-abnahme-ru11-sim-einheiten-gegner.md) | SIM | Schuld | mittel | offen | RU11 | Abnahme RU11: Sim, Balance und Gen für Einheiten und Gegner in Rust |
| [B-1082](B-1082-islandref-grade-klon-je-stufenschritt.md) | SIM | Schuld | niedrig | offen | RU52 | Island::stage_ref klont den Grad (Grade mit String) je Stufenschritt und Tick |
| [B-1086](B-1086-abnahme-laeufe-sim-insel.md) | SIM | Schuld | mittel | offen | RU10 | Abnahme-Läufe Sim Insel |
| [B-1087](B-1087-protected-night-platzhalter-apply-damage.md) | SIM | Problem | mittel | offen | RU52 | protected_night in state/common.rs ruft den Platzhalter mit unimplemented |
| [B-1084](B-1084-abnahme-laeufe-ru15-sim-kampf-bosse.md) | SIM | Schuld | mittel | offen | RU15 | Abnahme-Läufe RU15: Sim, Balance, Gen und Race für Kampf, Bosse, Krone, Portale in Rust |
| [B-1090](B-1090-replay-blend-auf-game-blend.md) | CLI | Schuld | niedrig | offen | RU26 | replay/blend.rs auf game/blend.rs umstellen |
| [B-1091](B-1091-importregel-ausnahmen-fachmodule.md) | CLI | Schuld | niedrig | offen | RU26 | Importregel der Fachmodule hat fünf bekannte Ausnahmen |
| [B-1092](B-1092-ru19-decode-world-story-vorziehen.md) | SIM | Schuld | mittel | offen | RU19 | decode_world_story (RU19) blockiert 91 Tests, Reihenfolge der Sprints prüfen; events_moon_stages_test zu RU19 |
| [B-1093](B-1093-ru14-abnahme-laeufe.md) | SIM | Schuld | mittel | offen | RU51 | Abnahme-Läufe Sim Welt-Schritt, Kampagne, Schiff und Vorschau |
| [B-1094](B-1094-plattform-fenster-api.md) | PLAT | Schuld | niedrig | offen | RU26 | Die Plattform bekommt eine Fenster-API für Größe, Lage und Rahmen |
| [B-1096](B-1096-abnahme-laeufe-ru26-spielschleife.md) | CLI | Schuld | mittel | offen | RU51 | Abnahme-Läufe RU26: Sim, Balance, Gen und Race für die Rust-Spielschleife |
| [B-1099](B-1099-port-rumpf-tests-ru18-ausfuellen.md) | SIM | Schuld | mittel | offen | RU52 | Rumpf-Tests der Rust-Sim (leer oder unimplemented) mit echten Rümpfen ausfüllen |
| [B-1098](B-1098-abnahme-ru21-store-room.md) | SRV | Schuld | mittel | offen | RU51 | Abnahme-Läufe RU21: Sim, Balance, Gen und Race für Rust-Store und Raum (Lane zweit) |
| [B-1100](B-1100-leere-ruempfe-ignorierte-tests.md) | SIM | Schuld | mittel | offen | RU52 | 21 ignorierte k3c-sim-Tests haben leere Rümpfe und zählen als portiert, ohne es zu sein |
| [B-1101](B-1101-abnahme-ru18-sim-wirtschaft-bau-monarch.md) | SIM | Schuld | mittel | offen | RU51 | Abnahme RU18: Sim, Balance, Gen, Race und Golden-Abgleich der neuen Rust-Sim-Teile |
| [B-1102](B-1102-spiel-story-an-sim-anbinden.md) | SIM | Schuld | mittel | offen | RU19 | Das Spiel zeigt die Geschichte der Sim, Campaign trägt die WorldDef |
| [B-1103](B-1103-ignorierte-sim-tests-nach-ru19.md) | SIM | Schuld | mittel | offen | RU52 | 76 ignorierte Rust-Sim-Tests nach RU19 abarbeiten (Testhelfer, Ursache prüfen) |
| [B-1104](B-1104-abnahme-laeufe-ru19.md) | SIM | Schuld | mittel | offen | RU51 | Abnahme-Läufe RU19 (Sim, Balance, Gen, Race) auf dem Second Worker |
| [B-1107](B-1107-rust-client-sim-sicht-ohne-insel.md) | CLI | Problem | mittel | offen | RU63 | Rust-Client: Anzeige rechnet ohne IslandRef (Lager-Maximum, Bau des Endbosses) und ohne Geschichte |
| [B-1108](B-1108-rust-abnahme-bildvergleich-ignore-marken.md) | CLI | Schuld | mittel | offen | RU51 | Abnahme Smoke-Version: Bildvergleich mit Grafik rot, Speicher-Rundlauf und veraltete Ignore-Marken (Lane zweit) |
| [B-1109](B-1109-rust-start-grafik-aller-biome-im-speicher.md) | CLI | Schuld | mittel | offen | RU30 | Rust-Start lädt die Grafik aller Biome auf einmal (ohne Loader und Ladeanzeige je Stufe) |
| [B-1105](B-1105-abnahme-laeufe-ru22-durchsatz.md) | SIM | Schuld | mittel | offen | RU51 | Abnahme-Läufe RU22: Durchsatz Rust gegen Go, Sim und Race (Lane: zweit) |
| [B-1110](B-1110-native-json-bruchwerte.md) | CLI | Schuld | niedrig | offen | RU52 | Rust liest data/native.json mit i64-Feldern, Go mit float64 (Chronik, Wert 0) |
| [B-1111](B-1111-dev-session-rolle-im-view.md) | CLI | Schuld | mittel | offen | RU52 | view::Session liefert keine Entwickler-Rolle (DevSession), Cheats im Rust-Spiel wirkungslos |
| [B-1112](B-1112-abnahme-laeufe-ru29.md) | CLI | Schuld | niedrig | offen | RU51 | Abnahme-Läufe RU29 (Race, Gen) und Prüfliste für Ralf, Lane zweit |
| [B-1113](B-1113-rust-welt-bibliothek-je-stufe.md) | CLI | Schuld | mittel | offen | RU30 | Rust: Welt-Grafik wird beim Start für alle Biome geladen, nicht je Stufe |
| [B-1114](B-1114-aufnahme-sichtpruefung-grafik.md) | CLI | Schuld | mittel | offen | RU30 | Aufnahme-Modus im Rust-Spiel mit echter Grafik gegen Go prüfen und Speicher von ShotCanvas messen |
| [B-1115](B-1115-rust-f12-pixel-und-maus.md) | CLI | Schuld | niedrig | offen | RU30 | Rust-Spiel: F12 zeigt keine Sprites und die Werte-Seite hat keine Maus-Auswahl |
| [B-1116](B-1116-replay-report-auf-game-measure.md) | CLI | Schuld | niedrig | offen | RU63 | replay/report.rs nutzt Report, FrameStats und FrameLog aus game/measure.rs |
| [B-1117](B-1117-menu-stress-starter-verdrahten.md) | CLI | Schuld | hoch | offen | RU63 | Menü bekommt den Stress-Starter des Spiels (MenuSet und Verdrahtung) |
| [B-1118](B-1118-rust-spiel-icon-manifest-ressource.md) | CLI | Schuld | mittel | offen | RU30 | Rust-EXE trägt Spiel-Icon und Manifest (Ressourcen-Crate braucht Freigabe) |
| [B-1119](B-1119-ru30-abnahme-messen-stresstest-aufnahme.md) | CLI | Schuld | mittel | offen | RU51 | RU30 Abnahme: Messläufe, Stresstest und Aufnahmevergleich des Rust-Spiels |
| [B-1120](B-1120-ru32-abnahme-aufnahme-bildvergleich-bericht.md) | CLI | Schuld | mittel | offen | RU64 | RU32 Abnahme (Lane zweit): Aufnahme beider EXE, Bildvergleich Go gegen Rust, Bericht, Gen |
| [B-1125](B-1125-rust-hud-insel-fuer-wirtschaft-werte.md) | CLI | Problem | mittel | offen | RU63 | Rust-HUD rechnet Beutel-Obergrenze und Tausch-Hinweis ohne Insel (Abweichung von Go) |
| [B-1126](B-1126-ru53-abnahme-skills-inselwahl.md) | CLI | Schuld | mittel | offen | RU64 | RU53 Abnahme: Skills lernen und Inselwahl im Rust-Spiel (Sim, Gen, Race, Spieltest) |
| [B-1123](B-1123-abnahme-ru35-paket-und-ci.md) | INF | Schuld | mittel | eingeplant | RU64 | Abnahme RU35: Rust-Paket, CI-Lauf und schwere Läufe auf dem Second Worker |
| [B-1124](B-1124-ci-main-rot-drei-jobs.md) | INF | Problem | mittel | eingeplant | RU58 | Drei CI-Jobs sind auf main rot: Natives Spiel, Go arm64 und k3c-workbench |
| [B-1139](B-1139-menue-lizenzliste-nennt-go-bibliotheken.md) | CLI | Problem | mittel | offen | RU58 | Die Lizenz-Seite des Rust-Spiels nennt Go-Bibliotheken statt der Rust-Crates |
| [B-1141](B-1141-rust-paket-auf-anderem-pc-abnehmen.md) | INF | Schuld | mittel | offen | RU64 | Abnahme: Das Rust-Paket dist/k3c-game startet auf einem anderen PC und die Lizenzdatei genügt |
| [B-1130](B-1130-ru55-abnahme-look-aufnahmen-referenz.md) | CLI | Schuld | mittel | eingeplant | RU64 | RU55 Abnahme: Aufnahmen gegen K2C-Referenz, Bildvergleich und Frame-Zeit (Lane zweit) |
| [B-1131](B-1131-cheatfenster-protokoll-client-fenster.md) | CLI | Idee | mittel | eingeplant | RU56 | Cheat-Fenster: Protokoll zwischen Spiel und Fenster als eigenes Crate k3c-devproto |
| [B-1132](B-1132-cheatfenster-win32-fenster-dark-layout.md) | CLI | Idee | mittel | eingeplant | RU56 | Cheat-Fenster: Win32-Fenster mit Dark-Look, Layout und Pipe-Server als Crate k3c-devwin |
| [B-1133](B-1133-cheatfenster-spielseite-pipe-zustand-befehle.md) | CLI | Idee | mittel | eingeplant | RU56 | Cheat-Fenster: Spielseite mit Pipe-Client, Zustandsabzug und Befehlen über die Cheat-Pfade |
| [B-1134](B-1134-cheatfenster-controls-je-bereich.md) | CLI | Idee | mittel | eingeplant | RU56 | Cheat-Fenster: Controls je Bereich (Welt und Zeit, Wirtschaft, Spieler und Skills, Gegner und Welle, Anzeige) |
| [B-1135](B-1135-cheatfenster-start-paket-ci.md) | CLI | Idee | mittel | eingeplant | RU56 | Cheat-Fenster: Start per F4 und Schalter, Paket, Build und CI |
| [B-1136](B-1136-cheatmenues-im-spiel-entfernen.md) | CLI | Schuld | niedrig | eingeplant | RU56 | In-Game-Cheat-Menü (F9) entfernen, sobald das Cheat-Fenster abgenommen ist |
| [B-1142](B-1142-cheatfenster-pipe-haerten.md) | CLI | Schuld | niedrig | offen | RU56 | Cheat-Fenster-Pipe härten: Zugriffsrechte, Pipe-Squatting, blockierendes Schreiben |
| [B-1144](B-1144-cheatfenster-restpunkte-rev2.md) | CLI | Problem | mittel | offen | RU56 | Cheat-Fenster: Zielspieler und Schalterzustand bleiben erhalten, goto, Fokus und Skillpunkte fehlen |
| [B-1145](B-1145-abnahme-laeufe-bb1.md) | PAP | Schuld | mittel | offen | BB1 | Abnahme-Läufe BB1: gen:paper und Grafik-Golden |
| [B-1146](B-1146-waldwand-baeume-leitmotive-im-spiel.md) | CLI | Idee | mittel | offen | LA1 | Waldwand, acht neue Bäume und Leitmotive im Rust-Spiel anbinden |
| [B-1147](B-1147-schiff-geruest-bauplatz-breiten.md) | CLI | Problem | mittel | offen | LA1 | Schiff-Baustelle mit eigenem Gerüst, Bauplatz-Breiten für Schiff und Burg |
| [B-1148](B-1148-schiffsrumpf-4-bis-5-m.md) | PAP | Problem | niedrig | offen | LA1 | Schiffsrumpf auf 4 bis 5 M Länge zeichnen |
| [B-1149](B-1149-plantage-wachstumsstufen-pappel.md) | CLI | Problem | mittel | offen | LA1 | Plantage-Wachstumsstufen wieder unterscheidbar machen |
| [B-1150](B-1150-leitmotive-luftperspektive.md) | PAP | Problem | niedrig | offen | LA1 | Leitmotive mit Luftperspektive: weniger Kontrast |
| [B-1151](B-1151-texturbudget-baeume.md) | PAP | Schuld | niedrig | offen | LA1 | Texturbudget der Bäume um 15 MiB senken |
| [B-1152](B-1152-rig-export-drift-ausserhalb-bosse.md) | PAP | Schuld | niedrig | eingeplant | LA1 | Rig-Export unter public/rig weicht außerhalb der Bosse von den Specs ab, gen:rig:check ist rot |
| [B-1153](B-1153-kristallherz-waechter-ohne-rig-export.md) | PAP | Problem | niedrig | eingeplant | LA1 | kristallherz-waechter hat keinen Rig-Export, gen:rig schreibt ihn nicht |
| [B-1156](B-1156-kritische-fragen-ru59-ru62.md) | INF | Frage | mittel | offen | KR6 | Kritische Entscheidungen aus RU59 bis RU62 (selbst entschieden, zur Durchsicht) |
| [B-1154](B-1154-abnahme-ru62-sim-balance-gen.md) | SIM | Schuld | mittel | offen | RU62 | Abnahme-Läufe RU62: Sim, Balance, Gen samt Golden und Race |
| [B-1155](B-1155-welt-schalter-nach-ru41-entfernen.md) | SIM | Schuld | niedrig | offen | RU62 | Welt-Schalter der Features nach RU41 als Felder der Welt-Datei führen |
| [B-1160](B-1160-hub-platzhalter-rechteck.md) | CLI | Problem | mittel | offen | LA1 | Im Hub steht neben dem Bogner-Haus kein einfarbiges Platzhalter-Rechteck mehr |
| [B-1161](B-1161-fackeln-alle-gebaeude-monarch-licht.md) | CLI | Idee | mittel | offen | LA1 | Nachts tragen alle Gebäude Fackeln und der Monarch hat ein folgendes Licht (Glühwürmchen oder kleiner Stern) |
| [B-1162](B-1162-messluecken-gate-g4.md) | CLI | Schuld | mittel | offen | RU36 | Die Gate-Messung ist vollständig: Lauf mit 300 s und VSync, FPS ohne Deckel, RAM von Rust gegen Go |
| [B-1163](B-1163-groessenstufen-monarch-mount.md) | CLI | Problem | hoch | offen | LA1 | Alle Figuren und Gebäude folgen definierten Größenstufen, Bezugswert ist der Monarch auf dem Mount, abgeglichen an den K2C-Screenshots |
| [B-1164](B-1164-stimmung-naeher-an-k2c.md) | CLI | Problem | hoch | offen | LA1 | Die Stimmung des Rust-Spiels (Farbe, Licht, Dunst, Tiefe) kommt dem K2C-Vorbild deutlich näher |
| [B-1165](B-1165-weltwahl-ohne-entwicklermodus.md) | CLI | Problem | mittel | offen | RU36 | Die Weltwahl zeigt alle Welten aus data/worlds, nicht nur die Standard-Welt |
| [B-1166](B-1166-wald-biom-dicht-standard-seed.md) | SIM | Problem | mittel | offen | RU36 | Wald-Biome sind im Standard-Seed immer dicht, Lichtungen sind selten, man rodet sich anfangs durch |
| [B-1167](B-1167-hub-kompass-zeigt-biome.md) | CLI | Idee | mittel | offen | LA1 | Der Kompass im Hub zeigt die Biome der Insel an |
| [B-1168](B-1168-studio-feste-welten.md) | DEV | Problem | mittel | offen | WZ62 | Das Studio listet die vier festen Welten (standard, sengoku, nil, britannien), nicht nur die Custom World Nordmeer |
| [B-1169](B-1169-studio-welt-filter-vererbung.md) | DEV | Idee | mittel | offen | WZ62 | Katalog und Sound filtern nach Welt: alle Assets einer Welt (mit geerbten der Basis) oder nur die exklusiven |
| [B-1170](B-1170-studio-groessenvergleich-lineal.md) | DEV | Idee | mittel | offen | WZ62 | Der Größenvergleich im Studio ist eine fensterfüllende Ansicht mit dem Größenstufen-Lineal statt vier Zoomstufen |
| [B-1171](B-1171-welten-editor-link-katalog.md) | DEV | Idee | niedrig | offen | WZ62 | Der Welten-Editor hat einen Schnell-Link in den Katalog mit den Assets dieser Welt (exklusiv oder gesamt) |
| [B-1172](B-1172-welten-editor-welt-loeschen.md) | DEV | Idee | mittel | offen | WZ62 | Im Welten-Editor lässt sich eine Welt löschen (mit Bestätigung, feste Welten geschützt) |
| [B-1173](B-1173-welten-editor-werte-tabelle.md) | DEV | Problem | mittel | offen | WZ62 | Die Werte-Tabelle im Welten-Editor ist übersichtlich und bequem zu bedienen |
| [B-1174](B-1174-balancing-rechenmodell-startwerte.md) | SIM | Idee | mittel | offen | RU31 | Das Balancing berechnet Startwerte vorab, kalibriert sie in der Simulation und tunt über die Empfindlichkeit je Hebel |
| [B-1175](B-1175-mauer-burgbereich-luecke.md) | CLI | Problem | mittel | offen | LA1 | Im Burgbereich schließen Mauern die Lücke zwischen den inneren Mauerstücken |
| [B-1176](B-1176-mauer-stufe-1-zaun.md) | PAP | Idee | mittel | offen | LA1 | Die Mauer der Stufe 1 sieht aus wie ein Zaun oder eine Bretterwand |
| [B-1177](B-1177-werte-tabelle-einheit-min-max.md) | DEV | Idee | niedrig | offen | WZ62 | Das Backend liefert je Welt-Wert Einheit und Min/Max, damit die Werte-Tabelle Regler und Bereichsprüfung beim Tippen bietet |
| [B-1183](B-1183-stimmung-reste-wolken-monumental-wellen.md) | CLI | Idee | mittel | offen | LA2 | Stimmung: Wolken, monumentaler Hintergrund und Wellenstriche fehlen noch |
| [B-1179](B-1179-cheat-fenster-panel-umgebungseffekte.md) | CLI | Idee | mittel | offen | LA2 | Cheat-Fenster braucht ein Panel, das die Umgebungseffekte einzeln ein- und ausschaltet |
| [B-1180](B-1180-fps-anzeige-zu-gross-null-fps.md) | CLI | Problem | mittel | offen | LA1 | Die FPS-Anzeige oben rechts zeigt 0 FPS, ist zu gross und am Rand abgeschnitten |
| [B-1181](B-1181-licht-assets-fackeln-lampen-feuerschalen.md) | PAP | Idee | mittel | offen | LA2 | Lichtquellen als Assets: animierte Flammen, Fackeln, Lampen und Feuerschalen mit Ankerpunkten an Gebäuden |
| [B-1182](B-1182-bauplaetze-abstand-nach-gebaeudegroesse.md) | SIM | Problem | mittel | offen | LA3 | Die festen Bauplätze im Hub überlappen sich, ihr Abstand soll der Gebäudegröße folgen |
| [B-1178](B-1178-abnahme-wz62-studio-welten.md) | DEV | Schuld | mittel | offen | WZ62 | Abnahme WZ62: Studio feste Welten, Welt-Filter, Größenvergleich, Welten-Editor (PC-Abnahme, Sim, Balance, Gen, Race) |
| [B-1184](B-1184-abnahme-la1-stimmung.md) | CLI | Schuld | mittel | offen | LA1 | Abnahme LA1: Stimmung und Look-Anbindung (FPS-Messung, Gen, Race) |
| [B-1185](B-1185-abnahme-la2-look-nachtraege.md) | CLI | Schuld | mittel | offen | LA2 | Abnahme LA2: Cheat-Panel, Wolken, Wellenstriche, Licht-Assets, Leitmotive (PC-Abnahme, Gen, Race) |
| [B-1186](B-1186-hud-baustellen-gebaeudenamen-haendler.md) | CLI | Idee | mittel | offen | HB1 | HUD: Baustellen-Texte kleiner mit Icons und x/n, Gebäudename über dem Monarchen, Händler und Stände sichtbar |
| [B-1187](B-1187-biome-dicht-radar-landstreicher-taverne.md) | SIM | Idee | mittel | offen | HB1 | Biome: dichter Bewuchs, Biome am Radar, Landstreicher-Lager zwischen Bäumen und Taverne an der Lagerstelle |
| [B-1188](B-1188-hafen-start-biome-bewuchs.md) | SIM | Idee | mittel | offen | HB1 | Hafen als Startpunkt neuer Spiele, ein bis zwei Biome vom Lager, vom Wald umgeben und freizuräumen |
| [B-1190](B-1190-abnahme-hb1-hud-biome-hafen.md) | DEV | Schuld | mittel | offen | HB1 | Abnahme HB1: HUD-Baustellen, Biome, Landstreicher-Lager, Hafen (PC-Abnahme, Sim, Gen, Race) |
| [B-1189](B-1189-abnahme-la3-bauplaetze.md) | SIM | Schuld | mittel | offen | LA3 | Abnahme LA3: Bauplätze nach Gebäudegröße (PC-Sichtprüfung, Race, Rust-Aufzeichnung) |
| [B-1191](B-1191-siegel-assets-schwierigkeit.md) | PAP | Idee | mittel | offen | ECO1 | Je Schwierigkeitsgrad ein Wachssiegel als Papierschnitt-Symbol |
| [B-1192](B-1192-siegel-anzeige-schwierigkeit.md) | CLI | Idee | mittel | offen | ECO1 | Das Schwierigkeits-Siegel steht in der Auswahl, bei den Spielständen und im HUD bei der Welt |
| [B-1193](B-1193-werkzeugguertel-werkzeugmacher.md) | SIM | Idee | mittel | offen | ECO1 | Der Werkzeugmacher stellt einen Werkzeuggürtel mit Hammer, Spitzhacke und Axt her |
| [B-1194](B-1194-handwerker-alle-bauarbeiten.md) | SIM | Idee | mittel | offen | ECO1 | Ein Bürger mit Werkzeuggürtel ist Handwerker und macht alle Bauarbeiten: roden, Felsen hauen, bauen, reparieren |
| [B-1195](B-1195-reparatur-kostet-material-und-gold.md) | SIM | Idee | mittel | offen | ECO1 | Gebäude reparieren kostet ein Grundmaterial der Ausbaustufe und zwei Gold |
| [B-1196](B-1196-gebaeude-nach-tiefe-einschraenken.md) | SIM | Idee | mittel | offen | ECO1 | In den unteren Ebenen stehen nur Verteidigungsanlagen, produzierende Gebäude nur ganz oben |
| [B-1197](B-1197-ausruestung-von-oben-holen.md) | SIM | Idee | mittel | offen | ECO1 | Bürger und Soldaten in den unteren Ebenen holen Ausrüstung von ganz oben oder von fallen gelassenen Stücken |
| [B-1198](B-1198-erzadern-freiraeumen.md) | SIM | Idee | mittel | offen | ECO1 | Erzadern in den Minen müssen erst freigeräumt werden: keine Steine oder Felsen bis zum Hub |
| [B-1199](B-1199-buerger-umbenennung-citizen.md) | SIM | Idee | mittel | offen | FG1 | Landstreicher und Bauer werden zum Bürger (citizen) |
| [B-1200](B-1200-meister-gebaeude-ausruestung-kombinationen.md) | SIM | Idee | mittel | offen | FG1 | Waffenschmied, Rüstungsschmied und Bogner fertigen Ausrüstung; Soldat und Elite aus Bürger |
| [B-1201](B-1201-figuren-grafik-texte-standard.md) | CLI | Idee | mittel | offen | FG1 | Grafik und Texte der Grundfiguren und Meister-Gebäude im Standard |
| [B-1202](B-1202-abnahme-eco1-siegel-eco-plan.md) | DEV | Schuld | mittel | eingeplant | ECO1 | ECO1 ist am PC abgenommen: Siegel, Werkzeuggürtel, tiefe Ebenen, Minen |
| [B-1203](B-1203-abnahme-fg1-grundfiguren.md) | DEV | Schuld | mittel | eingeplant | FG1 | FG1 ist am PC abgenommen: Bürger, Meister-Gebäude, Kombinationen, Grafik |
| [B-1204](B-1204-wald-baumwuchs-rodungsregeln.md) | SIM | Idee | mittel | offen | WD1 | Oberfläche hat vollen Baumwuchs mit Lichtungen; gerodet wird von den Rändern der Waldstücke |
| [B-1205](B-1205-ruestung-abholen.md) | SIM | Problem | mittel | eingeplant | EQ1 | Kämpfer holen die Rüstung am Gebäude ab, statt sie aus der Ferne zu bekommen |
| [B-1206](B-1206-ausruestungswerte-nach-data.md) | SIM | Schuld | mittel | eingeplant | EQ1 | Preise, Regalgrößen und Herstellzeiten der Ausrüstung stehen in data/buildings.json |
| [B-1207](B-1207-gegenstands-icons-im-hud.md) | CLI | Idee | mittel | eingeplant | EQ1 | Gegenstands-Icons erscheinen am Regal und im Holen-Hinweis |
| [B-1208](B-1208-bogner-oben-gemeinsames-limit.md) | SIM | Problem | mittel | eingeplant | EQ1 | Bogner steht nur ganz oben, das Truppen-Limit gilt für die ganze Insel |
| [B-1209](B-1209-besorger-sichtbarer-weg.md) | SIM | Idee | mittel | eingeplant | EQ1 | Besorger laufen sichtbar über die Treppe in die obere Stufe und zurück |
| [B-1210](B-1210-welten-im-dist-und-repo-welten-vorrang.md) | CLI | Problem | mittel | offen | WL1 | Welten liegen im dist-Paket; beim Start aus dem Repo-dist gelten die Repo-Welten (Dev-Symbol) |
| [B-1211](B-1211-welten-editor-weitere-welten-waehlbar.md) | DEV | Problem | mittel | offen | WL1 | Im Welten-Editor lassen sich alle Welten anwählen, nicht nur die erste |
| [B-1212](B-1212-gamepad-hotplug-und-hud-beitreten-meldung.md) | CLI | Problem | mittel | offen | WL1 | Gamepad funktioniert auch, wenn es nach dem Spielstart eingeschaltet wird; die Beitreten-Meldung im HUD entfällt |
| [B-1213](B-1213-zaun-und-mauern-lueckenlos-hoehe-nach-stufe.md) | PAP | Idee | mittel | offen | WL1 | Mauern: Jägerzaun auf Holz-Level, lückenloser Anschluss und Höhe nach Ausbaustufe |
| [B-1214](B-1214-workbench-spielstart-dev-release-dienste.md) | DEV | Idee | mittel | offen | WL1 | Workbench: Spielstart (Dev und Release) als Dienste eintragen und loggen |
| [B-1217](B-1217-welt-art-ableger-oder-neu.md) | DEV | Idee | mittel | eingeplant | WL2 | Eine Welt hat eine Art (Ableger oder komplett neu); die Statistik erwartet danach eigene Assets, Einheiten und Werte |
| [B-1218](B-1218-spieltest-seite-ablage-aufraeumen.md) | DEV | Idee | mittel | offen | WZ64 | Workbench-Seite Spieltest listet Aufnahmen in playtests/ und räumt sie per Knopf auf |
| [B-1219](B-1219-spieltest-aufnahme-ffmpeg.md) | DEV | Idee | mittel | offen | WZ64 | Spieltest-Start nimmt Raummikro und Spielbild mit ffmpeg auf und startet das Spiel im Spieltest-Modus |
| [B-1220](B-1220-spieltest-flag-rec-pad-marker.md) | CLI | Idee | mittel | offen | WZ66 | Spiel mit --playtest zeigt REC und schreibt Pad-Marker mit Spielkontext |
| [B-1221](B-1221-spieltest-ruckler-marker-spielstand.md) | CLI | Idee | mittel | offen | WZ66 | Spieltest-Modus setzt bei Rucklern selbst Marker und legt je Marker einen Spielstand ab |
| [B-1222](B-1222-spieltest-fragebogen-spielende.md) | CLI | Idee | niedrig | offen | WZ66 | Spieltest-Modus fragt am Spielende drei Fragen je Spieler per Pad ab |
| [B-1223](B-1223-spieltest-transkript-ffmpeg-whisper.md) | DEV | Idee | mittel | offen | WZ65 | Knopf Transkribieren erzeugt per ffmpeg-whisper ein Transkript mit Zeitstempeln und ein Bild je Marker |
| [B-1224](B-1224-spieltest-befunde-mcp-tickets.md) | DEV | Idee | mittel | offen | WZ65 | MCP-Tools playtest_* und Prompt-Knöpfe machen aus angehakten Befunden Tickets |
| [B-1225](B-1225-abnahme-spieltest-aufnahme.md) | DEV | Frage | mittel | offen | WZ65 | Abnahme Spieltest-Aufnahme (WZ64–WZ66): Sofa-Test mit zwei Pads und Frame-Messung bei laufender Aufnahme |
| [B-1226](B-1226-abnahme-wz63.md) | DEV | Schuld | mittel | offen | WZ63 | Abnahme WZ63: Studio startet allein und bearbeitet eine Welt |
| [B-1227](B-1227-studio-dev-worktree-b925.md) | DEV | Schuld | mittel | offen | WZ63 | Studio rechnet die Engine: Dev-Worktree-Instanz und Regel B-925 für das Studio |
| [B-1228](B-1228-mount-ausdauer.md) | SIM | Idee | mittel | offen | AU1 | Reittiere haben Ausdauer: Sprint verbraucht sie, Pausen laden sie auf |
| [B-1229](B-1229-abnahme-spieltest-modus.md) | CLI | Frage | mittel | offen | WZ66 | Abnahme-Läufe WZ66: Sim, Balance, Gen samt Golden und Race laufen grün |
| [B-1231](B-1231-abnahme-wl1-sim-balance-gen-race.md) | CLI | Frage | mittel | offen | WL1 | Abnahme-Läufe WL1: Sim, Balance, Gen samt Golden, Race |
| [B-1233](B-1233-spieltest-monitor-null-absturz.md) | DEV | Problem | hoch | eingeplant | WZ65 | Workbench-Fenster wird beim Start der Spieltest-Aufnahme schwarz (Monitor bekommt null-Listen) |
| [B-1234](B-1234-spieltest-player-mikro-session.md) | DEV | Idee | hoch | eingeplant | WZ65 | Spieltest: Player im Fenster, Mikro-Session ohne Spiel, bessere Transkription |
| [B-1235](B-1235-spieltest-aufnahme-mit-dem-spiel.md) | DEV | Idee | hoch | eingeplant | WZ65 | Spieltest: Aufnahme beginnt mit dem Spiel im Vollbild, Pegel statt Datenrate beim Ton, Whisper 30 s |
| [B-1236](B-1236-spieltest-liste-modell-ton-mitlauf.md) | DEV | Idee | mittel | eingeplant | WZ65 | Spieltest: large-v3 als Standard, Liste nach Uhrzeit, Modelle bei Neu laden, Ton läuft mit dem Bild |
| [B-1237](B-1237-spieltest-befunde-lesbar.md) | DEV | Idee | niedrig | eingeplant | WZ65 | Spieltest: Spielkontext, Bild und Vergleichsticket eines Befunds sind groß und gut lesbar |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
