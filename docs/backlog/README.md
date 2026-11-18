# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-622](B-622-fackeln-als-spielobjekt.md) | SIM | Idee | niedrig | eingeplant | W15 | Fackeln gibt es als Spielobjekt mit Licht in der Nacht |
| [B-699](B-699-papierschnitt-grafik-mcp-review.md) | DEV | Idee | hoch | eingeplant | MG11 | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench |
| [B-720](B-720-bosse-neu.md) | PAP | Problem | hoch | eingeplant | MG10 | Bosse sind neu gestaltet, jeder mit eigener Silhouette |
| [B-775](B-775-native-umstellung-rust-bevy.md) | CLI | Idee | hoch | eingeplant | NG9 | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server |
| [B-780](B-780-rig-fuer-alle-figuren-gegnerzahl.md) | CLI | Frage | mittel | eingeplant | NG3 | Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen |
| [B-879](B-879-welten-editor-workbench.md) | DEV | Problem | hoch | eingeplant | WZ58 | Die Workbench hat den Welten-Editor aus dem alten Studio wieder |
| [B-880](B-880-asset-pakete-je-welt.md) | PAP | Idee | mittel | eingeplant | PT6 | Assets lassen sich zusammen mit einer Welt zu Paketen bündeln |
| [B-884](B-884-abnahme-mg11.md) | PAP | Schuld | mittel | offen | MG11 | Abnahme MG11: Sichtprüfung der Welt-Grafik und Gen-Läufe sind erledigt |
| [B-882](B-882-abnahme-laeufe-wz45.md) | DEV | Schuld | mittel | offen | WZ45 | Abnahme-Läufe WZ45 |
| [B-885](B-885-native-rig-export-bosse.md) | PAP | Schuld | mittel | eingeplant | NA1 | Nativer Rig-Export der Bosse ist nach MG10 veraltet |
| [B-886](B-886-abnahme-mg10.md) | PAP | Schuld | mittel | offen | MG10 | Abnahme MG10: Sichtprüfung der Bosse und Gen-Läufe sind erledigt |
| [B-888](B-888-nativ-cheat-dialog-und-tastenhilfe.md) | CLI | Idee | hoch | offen | CH1 | Das native Spiel hat wieder einen Cheat-Dialog (F9) und eine Tastenhilfe der UI-Tasten (F1) |
| [B-907](B-907-abnahme-wr1-gen-race.md) | SRV | Schuld | niedrig | offen | WR2 | Abnahme WR1: gen:paper samt Grafik-Golden und check:race laufen nach dem Rückbau grün |
| [B-889](B-889-nativ-debug-werte-und-logs.md) | CLI | Idee | hoch | offen | CH1 | Das native Spiel zeigt Debug-Werte der Sim (F10) und loggt Zustandswechsel und Zusammenfassungen |
| [B-890](B-890-nativ-hud-icons-statt-namen.md) | CLI | Idee | hoch | offen | CH1 | Das HUD des nativen Spiels zeigt Ressourcen als Icon mit Zahl statt als Namen |
| [B-899](B-899-abnahme-ch1.md) | CLI | Schuld | mittel | offen | CH1 | Abnahme CH1: Sichtprüfung der Entwickler-Tasten, Cheats, Debug-Werte und HUD-Icons und Gen-Läufe sind erledigt |
| [B-891](B-891-biom-eigenschaften-je-welt.md) | SIM | Idee | mittel | offen | BI1 | Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt |
| [B-895](B-895-workbench-webview-speicherleck.md) | DEV | Problem | hoch | offen | WZ60 | Die Oberfläche der Workbench wächst nicht unbegrenzt im Speicher und bleibt nach Stunden mit parallelen Prüfläufen bedienbar |
| [B-908](B-908-pages-bausatz.md) | PLAT | Idee | mittel | offen | GP1 | Die Präsentationsseite zeigt das Spiel als Bausatz: Einzelspieler und Couch-Koop, nativ, frei für nicht-kommerzielle Nutzung, eigene Welten mit dem MCP-Server |
| [B-910](B-910-abnahme-bi2.md) | CLI | Idee | mittel | eingeplant | BI2 | Abnahme BI2: Biom-Darstellung ist gesichtet, Grafik gerendert und Golden geprüft |
| [B-911](B-911-papierschnitt-tasten-icons.md) | PAP | Idee | mittel | eingeplant | NA1 | Tastenkappen und Xbox-Tasten gibt es als Papierschnitt-Icons der Gruppe icons |
| [B-912](B-912-e2e-flow-skillpunkt-rot.md) | CLI | Problem | hoch | eingeplant | NA1 | Der Ende-zu-Ende-Lauf TestFlow scheitert, weil Spieler 2 bis Takt 600 keinen Skill-Punkt findet |
| [B-913](B-913-abnahme-ep1.md) | CLI | Idee | mittel | offen | EP1 | Ralf nimmt EP1 ab: Eingabe-Profile und Tasten-Icons an TV und PC geprüft |
| [B-914](B-914-abnahme-st2.md) | CLI | Idee | mittel | offen | ST2 | Ralf nimmt ST2 ab: Testing-Menü und Stresstest-Bericht am PC geprüft |
| [B-935](B-935-klangbild-couch.md) | CLI | Problem | hoch | offen | SR1 | Klangbild für Couch-Abende festlegen und konsolige Piep-Sounds ersetzen |
| [B-936](B-936-musik-ruhiger.md) | CLI | Problem | hoch | offen | SR1 | Ruhigere, langsamere Musikstücke passend zur Atmosphäre finden oder erzeugen |
| [B-937](B-937-weltbilder-assets.md) | PAP | Idee | mittel | offen | WG1 | Jede Welt hat ein großes Titelbild mit Text und ein kleines Bild als Papierschnitt-Asset |
| [B-938](B-938-weltauswahl-menue.md) | CLI | Idee | mittel | offen | WG2 | Neues Spiel beginnt mit der Weltauswahl im großen Bild, Spielstände und Pausenmenü zeigen das kleine |
| [B-940](B-940-kampf-assets.md) | PAP | Idee | mittel | offen | KS6 | Papierschnitt-Assets für Krone, Banner, Schild, Zahlstellen, Portal-HP und Münzschauer ersetzen die gezeichneten Formen |
| [B-941](B-941-ausloeser-pfahl-vor-tor.md) | SIM | Problem | mittel | offen | KS7 | Der Angriffs-Auslöser steht als Horn auf einem Pfahl vor dem Tor statt schwebend dahinter |
| [B-942](B-942-schild-gestell-burgstufe.md) | SIM | Idee | mittel | offen | KS7 | Schilde gibt es erst ab höherer Burgstufe, gestapelt auf einem Gestell neben der Burg |
| [B-943](B-943-ruckeln-trotz-60fps.md) | CLI | Problem | hoch | offen | FB4 | Die Grafik ruckelt trotz stabiler 60 FPS, auch auf schnellen Rechnern |
| [B-944](B-944-xbox-tasten-testseite.md) | CLI | Problem | hoch | offen | FB4 | Xbox-Controller-Tasten stimmen wieder, mit Testseite in den Entwickleroptionen |
| [B-947](B-947-hintergrundmauern-luecken.md) | CLI | Problem | mittel | offen | FB4 | Die durchgezogenen Hintergrundmauern haben keine Lücken mehr |
| [B-948](B-948-baeume-groesser.md) | CLI | Problem | mittel | offen | FB4 | Bäume erscheinen in passender Größe statt zu klein |
| [B-949](B-949-ladescreen-huebsch.md) | CLI | Idee | niedrig | offen | FB4 | Der Ladescreen ist im Stil des Spiels gestaltet |
| [B-953](B-953-truhen-fontaene.md) | SIM | Problem | mittel | offen | WI1 | Truhen geben zehnmal mehr Gold und werfen es als Fontäne auf den Boden |
| [B-959](B-959-welten-editor-nachtraege.md) | DEV | Problem | mittel | offen | WE3 | Welten-Editor-Nachträge: Endboss je Insel-Thema, Hafen-Ablauf, Event- und Sites-Overrides, Gegnerarten je Welt |
| [B-960](B-960-welten-inhalte.md) | PAP | Idee | niedrig | offen | WE3 | Inhalte für die neuen Welten: Assets, Events und Gegnerarten für Sengoku, Nil und Britannien |
| [B-962](B-962-seekarte-im-spiel.md) | CLI | Idee | hoch | eingeplant | WE4 | Am Hafen wählen die Spieler die Zielinsel auf einer Seekarte |
| [B-964](B-964-seekarte-spiel-grafik.md) | PAP | Idee | mittel | eingeplant | GS2 | Die Seekarte im Spiel ist eine gestaltete Papierschnitt-Karte statt gezeichneter Formen |
| [B-965](B-965-geschichte-kunstsprache.md) | REG | Idee | mittel | eingeplant | GS1 | Jede Welt und Insel erzählt eine kurze Geschichte in Kunstsprache mit eingestreuten echten Wörtern |
| [B-966](B-966-chronik-der-reise.md) | REG | Idee | niedrig | eingeplant | GS3 | Abgeschlossene Inseln hinterlassen auf der Seekarte ein Andenken und eine Zeile der Geschichte |
| [B-967](B-967-fracht-auf-dem-schiff.md) | REG | Idee | mittel | eingeplant | SI1 | Vor der Abfahrt wählen die Spieler die Fracht des Schiffs |
| [B-968](B-968-koop-aktionen-zu-zweit.md) | REG | Idee | mittel | eingeplant | SI2 | Zwei Monarchen am selben Ort lösen gemeinsame Koop-Aktionen aus |
| [B-969](B-969-wetter-je-welt.md) | REG | Idee | mittel | eingeplant | SI3 | Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung |
| [B-970](B-970-reittiere-je-welt.md) | REG | Idee | mittel | eingeplant | SI4 | Jede Welt hat ein eigenes Reittier mit kleiner Eigenheit |
| [B-971](B-971-flaschenpost.md) | REG | Idee | niedrig | eingeplant | GS4 | Am Strand treibt eine Flaschenpost mit Geschichtszeile und Hinweis an |
| [B-972](B-972-einsiedler-mit-angebot.md) | REG | Idee | mittel | eingeplant | SI5 | Auf jeder Insel wohnt eine besondere Figur der Welt mit einem einmaligen Angebot |
| [B-973](B-973-fest-nach-dem-sieg.md) | REG | Idee | niedrig | eingeplant | SI6 | Nach dem Sieg über den Endgegner feiert das Volk ein Fest |
| [B-974](B-974-geschichte-einblendung-spiel.md) | CLI | Idee | mittel | eingeplant | GS1 | Das Spiel blendet die Geschichtszeilen an den Wendepunkten je Spielerhälfte ein |
| [B-975](B-975-geschichte-welten-werkzeuge.md) | DEV | Idee | mittel | eingeplant | GS1 | Die Welten-Werkzeuge schreiben und prüfen die Geschichte, „Welt anlegen“ erzeugt sie mit |
| [B-976](B-976-geschichten-vier-welten.md) | REG | Idee | mittel | eingeplant | GS1 | Die vier vorhandenen Welten haben Wortschatz und Geschichtszeilen |
| [B-977](B-977-seekarte-schiffsfahrt.md) | CLI | Idee | mittel | eingeplant | GS2 | Beim Inselwechsel fährt das Schiff im Vollbild über die Seekarte zur Zielinsel |
| [B-978](B-978-spielstand-auswahl-seekarte.md) | CLI | Idee | niedrig | eingeplant | GS3 | Die Spielstand-Auswahl zeigt die Seekarte mit den Trophäen als Vorschau |
| [B-979](B-979-sengoku-etappen-1-2-2.md) | SIM | Idee | mittel | eingeplant | WE5 | Sengoku nutzt die Etappen 1, 2, 2: Insel 3 bekommt einen Hafen |
| [B-980](B-980-flaschenpost-im-spiel.md) | CLI | Idee | niedrig | eingeplant | GS4 | Das Spiel zeigt Flaschenpost, Richtungspfeil und leuchtendes Ziel in beiden Spielerhälften |
| [B-981](B-981-endboss-je-insel.md) | SIM | Idee | mittel | eingeplant | WE5 | Jede Insel wählt ihren Endboss, der Bau liegt in ihrer tiefsten Stufe statt nur in der Kristallhöhle |
| [B-982](B-982-overrides-events.md) | SIM | Idee | mittel | eingeplant | WE5 | Eine Welt übersteuert die Events der Nacht über overrides › events |
| [B-983](B-983-overrides-hub-sites.md) | SIM | Idee | mittel | eingeplant | WE5 | Eine Welt übersteuert die Plätze hub › sites und islandSites, der Level-Generator liest den Wertesatz der Welt |
| [B-984](B-984-welt-skins-feld.md) | SIM | Idee | mittel | eingeplant | WE5 | Die Welt-Datei ordnet mit skins Gegnern und Objekten eigene Assets zu (Gegner-Varianten je Welt) |
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
| [B-997](B-997-fracht-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | SI1 | Ladeplätze am Steg und Fracht an Deck sind im Spiel sichtbar |
| [B-998](B-998-doppel-truhe-asset-anzeige.md) | CLI | Idee | mittel | eingeplant | SI2 | Die Doppel-Truhe hat ein eigenes Bild und zeigt, welcher Monarch fehlt |
| [B-999](B-999-wetter-vorboten-darstellung.md) | CLI | Idee | mittel | eingeplant | SI3 | Vorboten, Taifun und Flut sind im Spiel sichtbar und hörbar |
| [B-1000](B-1000-reittiere-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | SI4 | Gatter und Welt-Reittiere sind im Spiel sichtbar, der Monarch reitet sein gekauftes Tier |
| [B-1001](B-1001-einsiedler-assets-anzeige.md) | CLI | Idee | mittel | eingeplant | SI5 | Hütte, Figur und Bauwerk der Welt sind im Spiel sichtbar |
| [B-1002](B-1002-fest-darstellung-musik.md) | CLI | Idee | niedrig | eingeplant | SI6 | Das Fest zeigt Jubel, Himmels-Effekt je Welt und spielt Festmusik |
| [B-1003](B-1003-welten-editor-wetter-reittier-figur.md) | DEV | Idee | niedrig | eingeplant | SI7 | Der Welten-Editor wählt Wetter, Reittier und Figur einer Welt aus den Katalogen |
| [B-1004](B-1004-audio-tests-ohne-browser-quelle.md) | CLI | Schuld | hoch | eingeplant | NA1 | Die drei MatchBrowser-Tests in native/k3c-game/audio prüfen ohne die entfernten Browser-Quellen |
| [B-1005](B-1005-lint-qf1001-licensefile.md) | CLI | Schuld | niedrig | eingeplant | NA1 | golangci-lint meldet in native/k3c-game/menu/licensefile kein QF1001 mehr |
| [B-1006](B-1006-steg-gehaltenes-a-zahlt-nicht.md) | CLI | Problem | mittel | eingeplant | NA1 | Gehaltenes A am Steg zahlt nach der Zielwahl erst wieder nach dem Loslassen |
| [B-1007](B-1007-bin-alte-exe-aufraeumen.md) | INF | Schuld | niedrig | eingeplant | WZ61 | task ops:aufraeumen räumt die EXE entfernter Programme aus bin/ ab |
| [B-1008](B-1008-kronenraub-ueberlebt-laden-nicht.md) | SIM | Problem | hoch | offen | NA1 | Ein laufender Kronenraub bleibt nach Verlassen und Weiter bestehen |
| [B-1009](B-1009-anzeigename-geselle-troops.md) | SIM | Schuld | niedrig | offen | RG2 | Der Beruf craftsman heißt in data/troops.json Geselle statt Handwerker |
| [B-1010](B-1010-abnahme-we7-welten-assets.md) | PAP | Frage | mittel | offen | WE7 | Abnahme WE7: Welten-Assets sind gesichtet, Gen-Läufe und Texturbudget geprüft |
| [B-1011](B-1011-abnahme-gs2-seekarte.md) | CLI | Frage | mittel | offen | GS2 | Abnahme GS2: Seekarte als Papierschnitt und Schiffsfahrt sind am TV gesichtet, Gen-Läufe geprüft |
| [B-1012](B-1012-seekarte-nachtraege-review.md) | CLI | Schuld | mittel | offen | GS2 | Seekarten-Nachträge aus dem Review: Reihenfolge von completed abgesichert, gehaltenes A nach der Fahrt, Stufen der alten Insel im Wechsel-Frame |
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
| [B-1043](B-1043-gehlinie-ufer.md) | CLI | Idee | mittel | offen | BB2 | Gehlinie liegt bei 58 % der Ansicht, das Ufer ist 7 % hoch |
| [B-1044](B-1044-fluss-spiegelt-welt.md) | CLI | Idee | mittel | offen | BB2 | Der Fluss spiegelt die Welt über der Gehlinie |
| [B-1045](B-1045-mehr-welt-sichtbar.md) | CLI | Idee | mittel | offen | BB2 | Die Ansicht zeigt etwa 5 Monarchenhöhen Welt über der Gehlinie |
| [B-1046](B-1046-toenung-licht-dunst.md) | CLI | Idee | mittel | offen | BB2 | Tageszeit und Ereignis tönen das ganze Bild, Lichtquellen haben einen Lichthof |
| [B-1047](B-1047-wald-nach-rodung.md) | CLI | Idee | mittel | offen | BB2 | Der Wald wird mit dem Roden offener und heller |
| [B-1049](B-1049-typen-worlddef-island-fuer-ru7-5-vorziehen.md) | SIM | Schuld | mittel | offen | RU10 | Typen WorldDef und Island für RU7.5 vorziehen |
| [B-1050](B-1050-abnahme-ru8-sim-zustand-race-und-golden.md) | SIM | Schuld | mittel | offen | RU8 | Abnahme RU8: Race- und Golden-Läufe des Rust-Sim-Zustands stehen aus |
| [B-1052](B-1052-abnahme-laeufe-ru5.md) | CLI | Schuld | mittel | offen | RU5 | Abnahme-Läufe RU5 |
| [B-1053](B-1053-client-sim-naehte-ersetzen.md) | CLI | Schuld | mittel | eingeplant | RU30 | Sim-Nähte im Rust-Client sind durch die portierte Sim ersetzt |
| [B-1054](B-1054-verwaiste-sim-tests-zuordnen.md) | SIM | Schuld | mittel | eingeplant | RU22 | Jede Go-Testdatei aus engine/sim ist in Rust portiert oder begründet ausgelassen |
| [B-1059](B-1059-abnahme-ru13-welt-akteure.md) | CLI | Schuld | mittel | offen | RU13 | Abnahme RU13: Gen, Bildtests, Race und Sichtprüfung der Standbilder |
| [B-1055](B-1055-fluss-shader-rust-wgsl.md) | CLI | Schuld | niedrig | offen | RU13 | Fluss-Shader in Rust (WGSL) |
| [B-1056](B-1056-canvas-multiply-blend-lichtkarte.md) | CLI | Schuld | mittel | offen | RU13 | Canvas: Multiply-Blend für die Lichtkarte |
| [B-1058](B-1058-canvas-reihenfolge-textur.md) | CLI | Problem | mittel | offen | RU13 | Canvas sortiert innerhalb einer Ebene nach Textur statt nach Aufrufreihenfolge |
| [B-1057](B-1057-ru38-abnahme-sim-balance-gen.md) | SIM | Schuld | mittel | offen | RU38 | RU38 Abnahme: Fenster-Messung der Wiedergabe, Sim, Balance und Gen |
| [B-1063](B-1063-text-2px-tiefer-als-go.md) | CLI | Problem | niedrig | offen | RU16 | Rust setzt Text im Software-Rasterer rund 2 px tiefer als Go |
| [B-1060](B-1060-ton-atlas-ausschnitte-rust.md) | CLI | Schuld | mittel | eingeplant | RU30 | Der Rust-Client spielt Effekte aus Ausschnitten des Ton-Atlas |
| [B-1062](B-1062-ru17-abnahme-sim-balance-gen.md) | CLI | Schuld | mittel | offen | RU17 | RU17 Abnahme: Sim, Balance, Gen und Sichtprüfung Effekte und Ton |
| [B-1064](B-1064-ru16-abnahme-sim-balance-gen.md) | CLI | Schuld | mittel | offen | RU16 | RU16 Abnahme: Sim, Balance, Gen und Sichtprüfung Gebäude und Objekte |
| [B-1065](B-1065-rust-textmasse-zeilenhoehe-advance.md) | CLI | Problem | niedrig | offen | RU20 | Rust misst Textzeilen niedriger und kleine Schrift schmaler als Go |
| [B-1067](B-1067-rust-purse-skill-values-pending-draw-glyph-ausleihe.md) | CLI | Problem | niedrig | offen | RU20 | Hud: Spieler mit Skills stürzen in sim::values ab, draw_glyph mit Bibliotheksschrift vermutlich Doppelausleihe |
| [B-1068](B-1068-abnahme-ru20.md) | CLI | Schuld | mittel | offen | RU20 | Abnahme RU20: Sichtprüfung HUD und Gen-Läufe |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
