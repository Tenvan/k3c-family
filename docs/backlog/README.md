# Backlog

Alle Tickets, eine Zeile pro Ticket. Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md).
Jedes Ticket ist eine Spec (SDD, siehe [`../arbeitsweise.md`](../arbeitsweise.md)). Neues Ticket: nächste freie Nummer,
Datei `B-NNN-kurzname.md` aus der Vorlage, `Spec: Entwurf`, Zeile hier ergänzen. `task test` prüft beides.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-008](B-008-spieleabend.md) | REG | Frage | niedrig | eingeplant | P1 | Familie hat einen Spieleabend gespielt und Feedback gegeben |
| [B-015](B-015-gebaeude-werte.md) | REG | Problem | niedrig | eingeplant | BR1 | Gebäude-HP und -Kosten sind gebalanced |
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | eingeplant | PB1 | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | eingeplant | RG2 | Tiefe 3 und 4 sind beschrieben |
| [B-041](B-041-wails-starter.md) | DEV | Idee | niedrig | eingeplant | BT1 | Wails-Starter für Windows existiert |
| [B-042](B-042-pi-leistungsziel.md) | SRV | Frage | hoch | eingeplant | HW1 | Pi-Modell und Leistungsziel sind festgelegt |
| [B-090](B-090-radar.md) | CLI | Idee | mittel | eingeplant | HW1 | Ein Radar im HUD zeigt Burg, Portale, Ausgang, Mitspieler und Gegner |
| [B-099](B-099-balancing-tester.md) | SIM | Idee | mittel | eingeplant | BAL6 | Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele |
| [B-117](B-117-anzeige-bau-lager.md) | CLI | Idee | mittel | eingeplant | W6 | Der Client zeigt Wartezeit, Lagerstand, Hub-Stufe, Adern und Plantage |
| [B-126](B-126-buerger-ui.md) | CLI | Idee | mittel | eingeplant | W6 | Der Client zeigt Berufe, Ausbildung, Händler, Truppen-Limit und Heilung |
| [B-132](B-132-anzeige-bosse-events.md) | CLI | Idee | mittel | eingeplant | K5 | Der Client zeigt Gegner-Fähigkeiten, Bosse, Phasen und Events |
| [B-151](B-151-spieleabend-fragebogen.md) | REG | Idee | niedrig | eingeplant | P1 | Der Spieleabend hat einen kindgerechten Fragebogen und eine Playtest-Vorlage |
| [B-155](B-155-balancing-runde-wirtschaft.md) | REG | Idee | niedrig | eingeplant | BR1 | Die Wirtschaft ist in einer Balancing-Runde gegen die Zielkorridore abgestimmt |
| [B-156](B-156-balancing-runde-kampf-bosse.md) | REG | Idee | niedrig | eingeplant | BR2 | Kampf, Gegner und Bosse sind in einer Balancing-Runde gegen die Zielkorridore abgestimmt |
| [B-160](B-160-abgleich-spielmetrik-simulator.md) | REG | Idee | mittel | eingeplant | BAL4 | Spielmetrik echter Abende und Simulatorwerte sind abgeglichen |
| [B-272](B-272-rotation-session-reports.md) | SRV | Schuld | mittel | eingeplant | ST1 | Die Rotation in reports/ erfasst auch die Spielmetrik-Reports |
| [B-263](B-263-snapshot-groesse-plaetze.md) | SRV | Problem | niedrig | eingeplant | NT1 | Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget |
| [B-184](B-184-pages-screenshots.md) | PLAT | Idee | niedrig | eingeplant | PG1 | Die Präsentationsseite zeigt echte Bilder aus dem Spiel |
| [B-185](B-185-verluste-je-welle-angleichen.md) | REG | Schuld | niedrig | eingeplant | RG2 | Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger |
| [B-186](B-186-autospeichern-takt.md) | SRV | Idee | mittel | eingeplant | ST1 | Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ |
| [B-187](B-187-speichern-windows-rename.md) | SRV | Problem | mittel | eingeplant | ST1 | Speichern übersteht unter Windows eine kurz gesperrte Zieldatei |
| [B-190](B-190-events-dropped-im-protokoll.md) | SRV | Problem | niedrig | eingeplant | NT1 | Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden |
| [B-191](B-191-debug-overlay-links-unten.md) | CLI | Problem | mittel | eingeplant | U5 | Debug-Overlay und Aktionsliste verdecken das HUD nicht |
| [B-194](B-194-splitscreen-ruckelt-xbox.md) | CLI | Problem | hoch | eingeplant | PF1 | Der Split-Screen läuft auf der Xbox flüssig |
| [B-195](B-195-debug-overlay-xbox.md) | PLAT | Problem | mittel | eingeplant | PL1 | Das Debug-Overlay lässt sich auf der Xbox öffnen |
| [B-157](archiv/B-157-zielkorridor-pruefung.md) | SIM | Idee | mittel | erledigt | BAL2 | Der Balancing-Tester prüft Zielkorridore und meldet Pass oder Fail je Kennzahl |
| [B-159](archiv/B-159-replay-repro-format.md) | SIM | Idee | mittel | erledigt | BAL1 | Ein Lauf ist als Datei aus Seed und Eingaben wiederholbar |
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-207](B-207-bauplaetze-anzeige.md) | CLI | Idee | mittel | eingeplant | W11 | Der Client zeigt freie und gesperrte Bauplätze mit Grund (ab Hub-Stufe n, Linie fehlt) |
| [B-208](B-208-protokoll-bauplaetze.md) | SRV | Idee | mittel | eingeplant | W11 | Das Protokoll trägt die Bauplätze des Layouts sowie Platz- und Hub-Stufe zum Client |
| [B-230](B-230-burg-haelt-nur-47-prozent.md) | REG | Problem | mittel | eingeplant | RG1 | Burg hält Nacht 1–5 nur in 47 % der Seeds, Ziel 75–90 % |
| [B-219](B-219-doku-gating-und-schlag.md) | REG | Schuld | niedrig | eingeplant | RG2 | Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen |
| [B-274](B-274-testrestore-flackert-windows.md) | SRV | Problem | mittel | eingeplant | NT1 | TestRestore läuft unter Windows auch in task check:all stabil grün |
| [B-280](B-280-warteschlange-nicht-zustaende.md) | SRV | Problem | niedrig | eingeplant | NT1 | Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwischen Zuständen stehen |
| [B-284](B-284-lasttest-eingaben-flake.md) | DEV | Problem | niedrig | eingeplant | NT1 | TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läuft |
| [B-286](B-286-lasttest-tick-reihe-wackelt.md) | SRV | Problem | niedrig | eingeplant | NT1 | TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl |
| [B-287](B-287-hub-ausbau-beutel-maximum.md) | REG | Problem | mittel | eingeplant | RG1 | Hub-Stufe 4 und 5 sind mit dem Beutel-Maximum bezahlbar |
| [B-289](B-289-aggressionspool-mit-adern.md) | REG | Problem | mittel | eingeplant | RG1 | Der Aggressionspool steigt mit Adern nicht zu schnell |
| [B-293](B-293-spiel-im-menue-verlassen.md) | CLI | Idee | hoch | eingeplant | S8 | Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ |
| [B-294](B-294-hinweis-glyph-muenze-nacht.md) | CLI | Frage | mittel | eingeplant | S8 | Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph |
| [B-295](B-295-handwerker-schmiede-ruestkammer.md) | REG | Frage | mittel | eingeplant | RG2 | Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden |
| [B-300](B-300-mauern-zuerst-wie-sparsam.md) | SIM | Frage | mittel | eingeplant | BAL6 | Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ |
| [B-301](B-301-vary-ohne-wirkung.md) | SIM | Problem | niedrig | eingeplant | BAL6 | Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler |
| [B-314](B-314-controller-pruefungen-zurueckgestellt.md) | PLAT | Schuld | niedrig | eingeplant | HW1 | Alle Controller-Prüfungen sind gesammelt nachgeholt |
| [B-315](B-315-spielstand-voll-ausgebaut.md) | SRV | Idee | hoch | eingeplant | SV1 | Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut |
| [B-317](B-317-cheat-dialog-fokus-tastatur.md) | CLI | Problem | hoch | eingeplant | U5 | Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller bedienen |
| [B-324](B-324-client-typen-gegnerdaten.md) | CLI | Schuld | niedrig | eingeplant | KB2 | Die Client-Typen der Gegner- und Wellendaten passen zu den JSON-Dateien |
| [B-334](B-334-performance-modus.md) | CLI | Idee | hoch | eingeplant | PM1 | Der Client misst Leistung in einem Performance-Modus automatisch und überträgt die Werte an den Server |
| [B-335](B-335-landingpage-spieler-entwicklung.md) | PLAT | Idee | hoch | eingeplant | HW1 | Die Landingpage zeigt nur Spieler-Kacheln, Entwicklungs-, Performance- und Balancing-Aufrufe liegen auf einer eigenen Entwicklerseite |
| [B-341](B-341-header-ursache-live-messen.md) | DEV | Problem | mittel | eingeplant | WZ2 | Die Ursache für den fehlenden Header `X-K3C-Root` aus Worktrees ist live gemessen |
| [B-344](B-344-raum-speichert-nicht-nach-game-over.md) | SRV | Idee | hoch | eingeplant | KB2 | Der Raum speichert nach „Komplett verloren“ nicht mehr |
| [B-346](B-346-miniboss-wald-kippt-nacht-5.md) | REG | Problem | hoch | eingeplant | RG1 | Der Miniboss Wald kippt Nacht 5 in fast jedem Seed |
| [B-347](B-347-tester-misst-wirtschaft.md) | SIM | Schuld | hoch | eingeplant | BAL6 | Der Balancing-Tester misst die Wirtschafts-Kennzahlen |
| [B-351](B-351-client-diagnose-zeile.md) | CLI | Idee | hoch | eingeplant | PM1 | Der Client schreibt FPS, Latenz und Puffer regelmäßig als Diagnose-Zeile ins Client-Log |
| [B-352](B-352-botfeed-lan-zuschauer.md) | SRV | Idee | niedrig | eingeplant | TR4 | `sim_test` hängt sich an Clients auf der Xbox an, ohne einen Platz im Raum zu belegen |
| [B-354](B-354-bot-eingabe-impulse-frist.md) | PLAT | Problem | mittel | eingeplant | TR4 | Die Bot-Eingabe verliert keine kurzen Drücke und hält bei stummem Feed an |
| [B-360](B-360-plan-set-fehlendes-feld.md) | DEV | Problem | niedrig | eingeplant | WZ2 | plan_set lehnt ein Feld ab, das in der Datei fehlt, statt es still zu übergehen |
| [B-363](B-363-planung-flags-bearbeiten.md) | DEV | Idee | mittel | eingeplant | WZ2 | Die Planungsseite ändert Prio, Umgebung, Agent und Projekt direkt im Detail |
| [B-366](B-366-fahrplan-spalte-projekt.md) | DEV | Schuld | mittel | eingeplant | WZ2 | k3c-workbench füllt im Fahrplan die Spalte Projekt und ordnet nach Rang |
| [B-367](B-367-doku-nachzug-projekte.md) | INF | Schuld | niedrig | eingeplant | WZ2 | Begriffe aus den alten Bahnen und Spuren sind nach PJ3 ersetzt und neue Projekt-Begriffe im Glossar |
| [B-369](B-369-texte-landingpage.md) | PLAT | Schuld | niedrig | eingeplant | GL1 | Die Landingpage zeigt ihre Texte in der gewählten Sprache |
| [B-372](B-372-check-run-timeout-go.md) | SRV | Problem | mittel | eingeplant | WZ2 | check_run go:test und task:check:go brechen im MCP-Aufruf mit Timeout ab |
| [B-377](B-377-beispiel-rueckmeldung-servertest.md) | SRV | Schuld | niedrig | eingeplant | BD2 | Ein Server-Test prüft das Beispiel der Rückmeldungs-Ereignisse gegen die Simulation |
| [B-378](B-378-castfailed-einmal-je-druck.md) | SIM | Problem | mittel | eingeplant | BD2 | Eine gehaltene Skill-Taste ohne Ziel meldet castFailed einmal je Tastendruck |
| [B-381](B-381-protokoll-haendler-raid-hp.md) | SRV | Problem | niedrig | eingeplant | KB2 | Das Protokoll beschreibt Überfall und HP des Händlers |
| [B-382](B-382-lobby-neuversuch-nur-spielen.md) | CLI | Problem | niedrig | eingeplant | BD2 | Ein gewählter Spielstand startet nie leer neu |
| [B-385](B-385-inselwechsel-ereignis-geht-verloren.md) | SRV | Problem | mittel | eingeplant | KB2 | Das Ereignis `islandSwitch` erreicht beim Inselwechsel keinen Client |
| [B-387](B-387-ticketnummern-parallele-worktrees.md) | DEV | Problem | mittel | eingeplant | WZ2 | Ticket-Nummern bleiben zwischen parallelen Worktrees eindeutig |
| [B-394](B-394-xbox-overlay-gesten-abnahme.md) | PLAT | Frage | niedrig | eingeplant | HW1 | Overlay-Gesten (RB, LB + RB) sind an der Xbox abgenommen (B-195/AC-02) |
| [B-400](B-400-planung-splitter-merken.md) | DEV | Idee | niedrig | eingeplant | WZ2 | Die Planungsseite trennt Backlog und Details mit einem Splitter, dessen Stellung gemerkt wird |
| [B-401](B-401-planung-filter-getrennt.md) | DEV | Idee | niedrig | eingeplant | WZ2 | Der Backlog-Filter wirkt nur auf den Backlog, der Projekt-Filter nur auf die Projekte |
| [B-402](B-402-planung-session-titel.md) | DEV | Idee | mittel | eingeplant | WZ2 | Die Planungsseite zeigt in den Session-Zeilen den Titel der Session |
| [B-408](B-408-bauplaetze-aller-stufen-sichtbar.md) | CLI | Problem | mittel | eingeplant | W11 | Bauplätze aller Stufen werden schon im Hub einer niedrigen Stufe angezeigt |
| [B-409](B-409-bauplaetze-abstand-zu-eng.md) | REG | Problem | mittel | eingeplant | W11 | Bauplätze im Hub stehen zu eng, besonders die Treppen |
| [B-410](B-410-aktion-ausbilden-tauschen-protokoll.md) | SRV | Problem | mittel | eingeplant | W11 | Aktionen-Overlay zeigt Ausbilden und Tauschen nicht, der Server liefert sie nicht in actions |
| [B-411](B-411-spieler-abmelden.md) | CLI | Problem | hoch | eingeplant | BD4 | Lokale Spieler können beitreten, aber nicht einzeln austreten |
| [B-412](B-412-figuren-adern-plantage-lager.md) | CLI | Idee | niedrig | eingeplant | W11 | Adern, Plantage und Lager haben keine Anzeige als Figuren im Client |
| [B-413](B-413-hud-leiste-oben-fluss.md) | CLI | Idee | mittel | eingeplant | BD4 | Gemeinsame HUD-Leiste oben für alle Spieler, Kompass oben, Fluss mit Spiegelung unten |
| [B-422](B-422-w11-b207-nachtragen.md) | CLI | Schuld | mittel | eingeplant | W11 | Die Spec von W11 nennt B-207 (gesperrte Plätze mit Grund) noch nicht |
| [B-425](B-425-fullsave-ausbauregeln.md) | SRV | Problem | mittel | eingeplant | SV2 | Der Voll-Ausbau-Stand hält die Ausbauregeln des Spiels ein |
| [B-426](B-426-fullsave-review-kleinigkeiten.md) | SRV | Schuld | niedrig | eingeplant | SV2 | Kleine Befunde aus dem Review von SV1 sind erledigt |
| [B-435](B-435-gebaeude-grafik-per-prompt.md) | DEV | Idee | mittel | eingeplant | GEB1 | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
| [B-438](B-438-werkstatt-baeume-ins-spiel.md) | CLI | Idee | mittel | eingeplant | GEB6 | Werkstatt und Bäume aus tools/buildings erscheinen im Spiel |
| [B-453](B-453-alte-biom-hintergruende-aufbewahren.md) | CLI | Idee | mittel | eingeplant | GRJ1 | Alte Biom-Hintergründe (Packs) als Grafik aufbewahren |
| [B-454](B-454-baeume-hoeher-dichter-hoehle-wurzeln.md) | DEV | Problem | hoch | eingeplant | GEB10 | Bäume höher und dichter, Höhle mit eigener Unterwelt-Flora (Wurzeln) |
| [B-455](B-455-truhe-geschlossen.md) | DEV | Idee | mittel | eingeplant | GEB10 | Truhe gibt es auch geschlossen |
| [B-456](B-456-fahne-spielerfarbe-wimpel.md) | CLI | Idee | mittel | eingeplant | GEB10 | Markierungsfahne als Wimpel in Spielerfarbe |
| [B-457](B-457-geb6-abnahme-splitscreen-ausgang-geschmack.md) | CLI | Frage | mittel | eingeplant | GEB10 | GEB6: Split-Screen, Ausgang-Beschriftung und Geschmacksfragen später prüfen |
| [B-458](B-458-hub-ausbau-wachstum-feinstufen.md) | DEV | Idee | hoch | eingeplant | GEB11 | Hub wächst in Ausbaustufen sichtbar und kleinteilig (Lagerfeuer bis Burg, K2C-Stil) |
| [B-462](B-462-gebaeude-planer-hub-stufen.md) | DEV | Idee | mittel | eingeplant | WZ9 | Ein Gebäude-Planer bearbeitet, welches Gebäude ab welcher Hub-Stufe verfügbar ist |
| [B-472](B-472-nacht-dunkler-fackeln-lagerfeuer-licht.md) | CLI | Idee | niedrig | eingeplant | W13 | Nacht und Morgengrauen sind dunkler, Fackeln und Lagerfeuer machen Licht |
| [B-500](B-500-fullsave-tiefen-aus-welt.md) | SRV | Schuld | niedrig | eingeplant | SV2 | Voll ausgebauter Spielstand nimmt Tiefen aus den Biomen statt aus der Standard-Welt |
| [B-506](B-506-spielmenue-welten-und-spielstaende.md) | CLI | Problem | hoch | eingeplant | BD5 | Spielmenü zeigt keine Welt und erlaubt kein Anlegen oder Löschen von Spielständen |
| [B-568](B-568-plantage-pappel-niedrig.md) | DEV | Idee | mittel | eingeplant | GEB13 | Plantage bekommt eine eigene niedrige Pappel vor der Farm |
| [B-570](B-570-camp-stufen-look.md) | DEV | Idee | mittel | eingeplant | GEB13 | Das Camp bekommt einen Stufen-Look statt der Grundform |
| [B-571](B-571-heilplatz-stufen-deutlicher.md) | DEV | Idee | mittel | eingeplant | GEB13 | Heilplatz-Stufen sind deutlicher sichtbar (Zelt, Brunnen) |
| [B-576](B-576-sound-skill-je-klasse.md) | CLI | Idee | niedrig | eingeplant | SO8 | Jede Klasse hat einen eigenen Skill-Sound |
| [B-579](B-579-wellenabstand-adern-korridor.md) | REG | Problem | mittel | offen | – | Wellenabstand unter Tage mit besetzten Adern liegt über dem Korridor 4–12 Minuten |
| [B-580](B-580-workbench-git-seite.md) | DEV | Idee | mittel | offen | – | Die Workbench hat eine Git-Seite für dieses Repo |
| [B-581](B-581-offenen-raum-schliessen.md) | CLI | Idee | mittel | eingeplant | BD9 | Ein offener Raum lässt sich in der Lobby über Entfernen schließen |
| [B-582](B-582-hoehlen-flora-variation.md) | DEV | Idee | mittel | eingeplant | GEB15 | Höhlen-Wurzeln sind zahlreicher und unterscheiden sich in Form und Größe |
| [B-583](B-583-minen-felsen-groesser-varianten.md) | DEV | Idee | mittel | eingeplant | GEB15 | Minen und Felsen sind größer und haben zwei bis drei Varianten mehr |
| [B-584](B-584-bauschilder-statt-rahmen.md) | CLI | Idee | mittel | eingeplant | GEB16 | Bauplätze zeigen kleine Bauschilder statt grauer Rahmen |
| [B-585](B-585-wald-chunks-voller-baeume.md) | REG | Problem | mittel | eingeplant | GEB16 | Wald-Chunks stehen voller Bäume |
| [B-586](B-586-baugeruest-in-gebaeudeform.md) | CLI | Idee | mittel | eingeplant | GEB16 | Im Bau zeigt der Platz ein Gerüst in der Form des neuen Gebäudes |
| [B-587](B-587-snapshot-site-payable.md) | SRV | Idee | mittel | eingeplant | GEB16 | Der Server meldet je Bauplatz, ob er jetzt Münzen annimmt |
| [B-588](B-588-abbau-symbole-axt-spitzhacke.md) | CLI | Idee | niedrig | eingeplant | GEB16 | Abbau-Schilder zeigen eine Axt für Bäume und eine Spitzhacke für Felsen |
| [B-589](B-589-holz-balancing-dichter-wald.md) | REG | Idee | mittel | eingeplant | BAL7 | Holz-Balancing nach dem dichteren Wald messen und nachziehen |
| [B-590](B-590-wachstumsreihe-unterscheidbar.md) | DEV | Problem | hoch | eingeplant | GEB17 | Die Wachstumsreihe der Burg zeigt echte Unterschiede statt derselben Burg in Farben |
| [B-591](B-591-burg-ausbau-angebot-fehlt.md) | CLI | Problem | hoch | eingeplant | GEB17 | Vor der Burg erscheint kein Ausbau-Angebot, wenn der Spieler davor steht |
| [B-592](B-592-hub-ausbaustufen-je-inselstufe.md) | REG | Idee | hoch | eingeplant | GEB17 | Der Hub hat zwei bis drei Ausbaustufen je Inselstufe, weiter mit dem Material der nächsten Inselstufe |
| [B-594](B-594-grafik-seite-slot-details.md) | DEV | Idee | hoch | eingeplant | WZ28 | Die Grafik-Seite zeigt je Slot Name, Atlas-Definition und alles für die Zuordnung |
| [B-595](B-595-ticket-button-ohne-ki.md) | DEV | Idee | hoch | eingeplant | WZ28 | Jede Seite hat einen Button, der ohne KI ein Ticket zur Seite anlegt |
| [B-596](B-596-balancing-seite-offline.md) | DEV | Idee | hoch | eingeplant | WZ28 | Eine Balancing-Seite fährt Simulationen und Berichte ohne Internet und ohne Claude |
| [B-597](B-597-sound-hoerproben-differenziert.md) | DEV | Idee | hoch | eingeplant | WZ29 | Die Sound-Seite hat differenzierte Hörproben, auch für Skills |
| [B-598](B-598-grafik-vorschauen-differenziert.md) | DEV | Idee | hoch | eingeplant | WZ29 | Die Grafik-Seite hat differenzierte Vorschauen, auch für Skill-Effekte |
| [B-599](B-599-szenen-sortieren-filtern.md) | DEV | Idee | hoch | eingeplant | WZ29 | Szenen auf Sound- und Grafik-Seite sind sortier- und filterbar |
| [B-600](B-600-chat-unterstuetzung-sound-grafik.md) | DEV | Idee | hoch | eingeplant | WZ29 | Der AI-Chat unterstützt die Zuordnung auf Sound- und Grafik-Seite |
| [B-601](B-601-download-vorschlagsliste-offline-vorrat.md) | DEV | Idee | hoch | eingeplant | WZ29 | Eine Download-Vorschlagsliste für empfohlene Assets ermöglicht Offline-Tests |
| [B-602](B-602-studio-szene-kampf-anzeigen.md) | DEV | Idee | hoch | eingeplant | WZ29 | Das Studio zeigt Boss-Leiste, Phasen, Warnkreis und Event-Banner mit Mock-Zuständen |
| [B-603](B-603-dev-aktionen-boss-event.md) | SRV | Idee | hoch | eingeplant | KB4 | Dev-Aktionen lösen Boss, Phasenwechsel und Events (Vollmond, Blutmond, Händler) sofort aus |
| [B-604](B-604-welt-anzeige-pausenmenue-lobby.md) | CLI | Problem | mittel | eingeplant | BD9 | Die Welt steht groß im Pausenmenü und bei jedem Spielstand in der Lobby |
| [B-605](B-605-schutzplatz-stabil-bei-truppenwechsel.md) | SIM | Schuld | niedrig | offen | – | Schutzplatz bleibt stabil, wenn Truppen sterben oder hinzukommen |
| [B-613](B-613-http-origin-pruefen.md) | SRV | Problem | niedrig | offen | – | Schreibende HTTP-Endpunkte des Spielservers lehnen fremde Webseiten ab |
| [B-614](B-614-workbench-studio-web-client.md) | DEV | Idee | hoch | offen | WZ31 | Workbench und Studio starten als Server, gearbeitet wird im Browser |
| [B-617](B-617-studio-levelview-test-golden.md) | DEV | Problem | hoch | offen | – | Der Studio-Test levelView passt wieder zur Golden-Datei level-forest.json |
| [B-608](B-608-daten-beschreibungen.md) | DEV | Idee | hoch | offen | WZ28 | Jede balancing-relevante Datei in data/ hat eine Beschreibung mit Aufbau und Werten |
| [B-609](B-609-balancing-hilfe-eingaben.md) | DEV | Idee | hoch | offen | WZ28 | Die Balancing-Konfiguration erklärt jedes Eingabefeld |
| [B-610](B-610-balancing-konfigurationsdateien.md) | DEV | Idee | hoch | offen | WZ28 | Die Balancing-Seite listet die Konfigurationsdateien, die das Balancing beeinflussen |
| [B-611](B-611-balancing-bot-je-spieler.md) | DEV | Idee | hoch | offen | WZ28 | Ein Balancing-Lauf kann für jeden Spieler einen eigenen Bot wählen |
| [B-612](B-612-balancing-berichte-verwalten.md) | DEV | Idee | hoch | offen | WZ28 | Balancing-Berichte lassen sich einzeln löschen und bewerten, der Vergleich erklärt sich selbst |
| [B-615](B-615-ticket-nummer-gleichzeitig.md) | DEV | Problem | mittel | offen | – | Gleichzeitiges Anlegen von Tickets vergibt nie dieselbe Nummer |
| [B-616](B-616-lauf-id-nach-neustart.md) | DEV | Problem | niedrig | offen | – | Lauf-IDs der Balancing-Seite bleiben über einen Neustart der Workbench eindeutig |
| [B-619](B-619-check-diff-cache-und-short.md) | INF | Problem | hoch | offen | – | check:diff nutzt den Go-Testcache und lässt lange Tests aus |
| [B-621](B-621-workbench-console-tail-flake.md) | DEV | Problem | niedrig | offen | – | TestSourcesUndConsoleTail flackert, wenn die Ressourcen-Zeile früh kommt |
| [B-623](B-623-hub-stufenfolge-bestaetigen.md) | REG | Frage | mittel | offen | – | Stufenfolge des Hubs (3 Schritte je Hub-Stufe, Material-Tor) bestätigen |
| [B-624](B-624-golden-gzip-bestaetigen.md) | INF | Frage | mittel | offen | – | Golden-Daten als gzip mit lesbaren Diffs bestätigen |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/) (nur auf Nachfrage lesen); die Datei bleibt beim Verschieben
unverändert. Ändert ein Ticket seinen Status auf `erledigt` oder `verworfen`, wandert es per `git mv` dorthin und seine
Zeile in diesen Abschnitt.

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-384](archiv/B-384-inselwechsel-bestaetigung-sim.md) | SIM | Frage | mittel | erledigt | – | Inselwechsel: Wechsel-Bestätigung (confirmIsland) passt nicht zum Presence-Wechsel der Sim |
| [B-153](archiv/B-153-protokoll-berufe-haendler-lager-hub.md) | SRV | Idee | hoch | erledigt | W5 | Das Protokoll kennt Berufe, Händler, Lagerstand, Hub-Stufe und Wartegrund |
| [B-283](archiv/B-283-protokoll-berufe-tausch-grabstein.md) | SRV | Idee | mittel | erledigt | W5 | Das Protokoll kennt Beruf ausbilden, Tauschen, Berufe der Bürger und Grabstein/Wiederbeleben |
| [B-332](archiv/B-332-truppen-limit-im-zustand.md) | SRV | Frage | hoch | erledigt | W10 | Kämpfer-Zahl und Truppen-Limit stehen im Zustand |
| [B-330](archiv/B-330-wirtschaft-eingaben-ohne-sim-funktion.md) | SIM | Frage | hoch | erledigt | W5 | Für Hub-Ausbau, Tausch und Berufswahl ist entschieden, ob es eigene Eingaben gibt |
| [B-014](archiv/B-014-krieger-elite.md) | SIM | Idee | mittel | erledigt | W4 | Krieger und Elite-Truppen sind umgesetzt |
| [B-120](archiv/B-120-wiederbeleben.md) | SIM | Idee | mittel | erledigt | W4 | Gefallene Monarchen bleiben liegen, Mitspieler beleben sie wieder, sonst Respawn nach 15 s |
| [B-121](archiv/B-121-berufe-haendler.md) | SIM | Idee | mittel | erledigt | W4 | Bauern haben Berufe (Bergmann, Baumeister, Handwerker), und ein Händler tauscht Material gegen Gold |
| [B-122](archiv/B-122-elite-ruestung-limit-heilung.md) | SIM | Idee | mittel | erledigt | W4 | Elite-Upgrades, Rüstung, Truppen-Limit je Hub und Heilung der Truppen sind umgesetzt |
| [B-010](archiv/B-010-grafik-gebaeude.md) | CLI | Idee | mittel | erledigt | GR3 | Gebäude, Ressourcen und Hintergrund haben Grafiken |
| [B-001](archiv/B-001-server-framework.md) | SRV | Idee | niedrig | erledigt | SP00 | Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird |
| [B-002](archiv/B-002-diagnose-tui.md) | SRV | Idee | mittel | erledigt | SP10 | Diagnose-TUI zeigt den laufenden Server |
| [B-003](archiv/B-003-server-sprache.md) | SRV | Frage | mittel | erledigt | SP00 | Server-Sprache ist entschieden |
| [B-009](archiv/B-009-komplexitaet-pruefen.md) | INF | Idee | hoch | erledigt | SP01 | Komplexitäts-Budget wird automatisch geprüft |
| [B-012](archiv/B-012-mine.md) | SIM | Idee | mittel | erledigt | W2 | Mine (Tiefe 2) ist vollständig |
| [B-016](archiv/B-016-mehr-lokale-spieler.md) | CLI | Frage | mittel | erledigt | SP08 | Layout für mehr als zwei lokale Spieler ist entschieden |
| [B-018](archiv/B-018-renderer-aufteilen.md) | CLI | Schuld | mittel | verworfen | – | worldRenderer und GameScene liegen unter 300 Zeilen |
| [B-020](archiv/B-020-smoke-test.md) | INF | Schuld | niedrig | erledigt | SP03 | Server-Tests laufen lokal wie in der CI |
| [B-027](archiv/B-027-diagnose-absichern.md) | SRV | Problem | hoch | erledigt | SP03 | Diagnose-Schnittstelle ist abgesichert |
| [B-028](archiv/B-028-spielstand-sicherung.md) | SRV | Idee | mittel | erledigt | SP03 | Spielstände werden rotierend gesichert |
| [B-030](archiv/B-030-wiederverbinden.md) | SRV | Idee | hoch | erledigt | SP07 | Geräte verbinden sich nach Abbruch wieder |
| [B-031](archiv/B-031-online-test.md) | INF | Idee | niedrig | erledigt | SP07 | Online-Verbindung ist automatisch getestet |
| [B-032](archiv/B-032-github-pages.md) | PLAT | Problem | mittel | erledigt | SP09 | GitHub Pages zeigt nur, was ohne Server geht |
| [B-033](archiv/B-033-tests-typecheck.md) | INF | Schuld | mittel | erledigt | SP01 | Tests werden typgeprüft |
| [B-034](archiv/B-034-plat-dateien-aufteilen.md) | PLAT | Schuld | niedrig | verworfen | – | Große PLAT-Dateien liegen unter 300 Zeilen |
| [B-036](archiv/B-036-mehrere-raeume.md) | SRV | Idee | hoch | erledigt | SP07 | Mehrere Spiele laufen gleichzeitig |
| [B-038](archiv/B-038-lokale-und-online-spieler.md) | SRV | Idee | hoch | erledigt | SP07 | Lokale und Online-Spieler teilen sich einen Raum |
| [B-039](archiv/B-039-interpolation.md) | CLI | Idee | mittel | erledigt | SP08 | Bewegungen laufen trotz Snapshots flüssig |
| [B-043](archiv/B-043-portierungs-fallen.md) | SIM | Problem | hoch | erledigt | SP06 | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
| [B-044](archiv/B-044-planer-struktur.md) | INF | Idee | mittel | erledigt | SP00 | Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen |
| [B-045](archiv/B-045-sdd.md) | INF | Idee | hoch | erledigt | SP00 | Tickets und Sprints sind Specs nach Spec-Driven Development |
| [B-046](archiv/B-046-dev-mcp.md) | SRV | Idee | mittel | erledigt | M1 | Entwickler-Werkzeug k3c-workbench gibt Agenten über MCP verdichteten Zugriff auf Prüfungen und Logs |
| [B-047](archiv/B-047-mcp-raeume-simulation.md) | SRV | Idee | mittel | erledigt | M6 | MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen |
| [B-049](archiv/B-049-sp09-domaene.md) | INF | Frage | niedrig | erledigt | SP09 | SP09 bleibt in einer Domäne oder hat einen erlaubten Grenzfall |
| [B-050](archiv/B-050-go-dateilaenge.md) | INF | Schuld | mittel | erledigt | SP01 | Die Dateilänge von Go-Code wird wie bei TypeScript geprüft |
| [B-051](archiv/B-051-oxlint-warnungen.md) | INF | Schuld | niedrig | erledigt | I1 | Oxlint meldet im Bestand keine Warnungen mehr |
| [B-052](archiv/B-052-requirements.md) | INF | Idee | mittel | erledigt | SP01 | Alle vorausgesetzten Installationen stehen in requirements.md |
| [B-054](archiv/B-054-go-verschachtelung.md) | INF | Problem | mittel | erledigt | L1 | Die Verschachtelung von Go-Code wird als Tiefe geprüft |
| [B-055](archiv/B-055-server-lint.md) | INF | Problem | niedrig | verworfen | – | Das Komplexitäts-Budget gilt auch für server/*.mjs |
| [B-056](archiv/B-056-ratsche-nachziehen.md) | INF | Schuld | niedrig | verworfen | – | Die Ratsche zieht gesunkene Werte automatisch nach |
| [B-057](archiv/B-057-go-tiefe-range.md) | INF | Problem | mittel | erledigt | L2 | Die Go-Verschachtelung zählt `for range` und `else if` wie TypeScript |
| [B-059](archiv/B-059-freie-monarchen-reisen-mit.md) | SIM | Idee | hoch | erledigt | SP06 | Nur gesteuerte Monarchen entscheiden über den Stufenwechsel |
| [B-060](archiv/B-060-sp07-protokoll-regeln.md) | SRV | Problem | hoch | erledigt | SP07 | Die Spec von SP07 deckt alle Server-Regeln aus Protokoll v2 ab |
| [B-061](archiv/B-061-sp08-protokoll-regeln.md) | CLI | Problem | hoch | erledigt | SP08 | Die Spec von SP08 deckt alle Client-Regeln aus Protokoll v2 ab |
| [B-062](archiv/B-062-dev-nutzungsstatistik.md) | SRV | Idee | mittel | erledigt | M2 | k3c-workbench wertet MCP-Aufrufe über Sitzungen aus: Perzentile, Ausreißer und Zeitreihe |
| [B-063](archiv/B-063-dev-berichte-spielstaende.md) | SRV | Idee | mittel | erledigt | M2 | k3c-workbench macht Xbox-Berichte und Spielstände für Agenten lesbar |
| [B-064](archiv/B-064-dev-oberflaeche-logs.md) | SRV | Idee | mittel | erledigt | M4 | k3c-workbench hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs |
| [B-065](archiv/B-065-dev-mcp-seite.md) | SRV | Idee | mittel | erledigt | M5 | k3c-workbench zeigt auf der MCP-Seite Server, Tools, Live-Monitore, Aufruf-Log und Statistik |
| [B-066](archiv/B-066-server-json-log.md) | SRV | Idee | mittel | erledigt | D1 | Der Go-Server schreibt sein Log als JSON nach logs/ |
| [B-067](archiv/B-067-dev-dienste.md) | SRV | Idee | mittel | erledigt | M3 | k3c-workbench startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten |
| [B-068](archiv/B-068-dev-dienste-seite.md) | SRV | Idee | mittel | erledigt | M4 | k3c-workbench zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level |
| [B-069](archiv/B-069-ci-k3c-dev.md) | INF | Problem | mittel | erledigt | – | Der CI-Job k3c-workbench ist einmal grün gelaufen |
| [B-070](archiv/B-070-gitignore-verankern.md) | INF | Schuld | niedrig | erledigt | I1 | Die .gitignore ignoriert reports/, saves/ und certs/ nur an der Repo-Wurzel |
| [B-072](archiv/B-072-depguard-rng.md) | INF | Schuld | niedrig | erledigt | I1 | depguard prüft die Schichtgrenze auch für engine/rng |
| [B-073](archiv/B-073-go-task-umstellen.md) | INF | Schuld | hoch | erledigt | I1 | Alle Aufrufer nutzen Go Task statt npm-Skripte |
| [B-074](archiv/B-074-golden-wirtschaft-luecken.md) | SIM | Problem | mittel | erledigt | SP06 | Golden-Läufe decken Tragen, Bauen, Bögen, Truhen und Münz-Rückgabe ab |
| [B-076](archiv/B-076-websocket-bibliothek.md) | INF | Frage | hoch | erledigt | SP07 | Der Go-Server spricht WebSocket über github.com/coder/websocket |
| [B-077](archiv/B-077-race-detector.md) | INF | Schuld | hoch | erledigt | L3 | Die nebenläufigen Go-Pakete werden mit dem Race-Detector geprüft |
| [B-078](archiv/B-078-dev-proxy-go-server.md) | INF | Schuld | hoch | erledigt | SP09 | `task dev` leitet `/ws` an den Go-Server weiter |
| [B-081](archiv/B-081-testseite-szenarien.md) | PLAT | Idee | mittel | erledigt | T1 | Eine Testseite startet Test-Szenarien, zuerst 1–4 Spieler mit Mock-Spielern |
| [B-082](archiv/B-082-start-parameter-mock.md) | CLI | Idee | mittel | erledigt | SP08 | `game.html` startet per Parameter ohne Auswahl und mit Mock-Slots |
| [B-083](archiv/B-083-lobby-nach-ende.md) | CLI | Problem | niedrig | erledigt | – | Die Lobby zeigt nach `replaced` oder `version` keinen bedienbaren Eintrag mehr |
| [B-084](archiv/B-084-hud-ueberlappung.md) | CLI | Problem | niedrig | erledigt | – | HUD-Texte überlappen im 2×2-Raster |
| [B-085](archiv/B-085-serve-go-feste-exe.md) | INF | Problem | mittel | erledigt | – | `task serve:go` startet eine EXE mit festem Pfad |
| [B-086](archiv/B-086-testspielstaende-aufraeumen.md) | SRV | Problem | niedrig | erledigt | – | Test-Spielstände der Testseite bleiben nicht liegen |
| [B-087](archiv/B-087-grafik-referenzseite.md) | PLAT | Idee | mittel | erledigt | G1 | Eine Referenzseite zeigt die gewählten CC0-Grafik-Packs für Gebäude, Ressourcen und Hintergründe |
| [B-088](archiv/B-088-diagnose-endpunkte.md) | SRV | Idee | mittel | erledigt | D1 | Der Server zeigt Speicher, Geräte und Log und führt Diagnose-Aktionen aus |
| [B-089](archiv/B-089-ts-rng-reste.md) | CLI | Schuld | niedrig | erledigt | – | Der Client enthält keinen RNG-Rest der alten TS-Simulation mehr |
| [B-091](archiv/B-091-level-abfrage.md) | SRV | Idee | mittel | erledigt | U2 | Der Server liefert ein generiertes Level per HTTP, ohne einen Raum anzulegen |
| [B-096](archiv/B-096-start-ersetzt-spielstand.md) | SRV | Problem | mittel | erledigt | – | „Im Spiel starten“ ersetzt einen gleichnamigen Spielstand, statt abgewiesen zu werden |
| [B-097](archiv/B-097-dienste-watch-modus.md) | SRV | Idee | niedrig | erledigt | – | k3c-workbench startet den Go-Server bei Code-Änderungen von selbst neu |
| [B-093](archiv/B-093-debug-overlay.md) | CLI | Idee | mittel | erledigt | U4 | Ein Debug-Overlay zeigt Verbindung, Snapshot-Takt und Entitäten im Spiel |
| [B-005](archiv/B-005-online-koop-ziel.md) | REG | Problem | hoch | erledigt | R1 | Game-Design nennt gemischten Koop als Kern |
| [B-021](archiv/B-021-taste-x.md) | REG | Frage | mittel | erledigt | R1 | Belegung der Taste X ist entschieden |
| [B-025](archiv/B-025-kampagnen-ziel.md) | REG | Frage | mittel | erledigt | R1 | Ziel einer Kampagne ist festgelegt |
| [B-108](archiv/B-108-material-und-inseln-offen.md) | REG | Frage | mittel | erledigt | – | Material je Hub oder je Insel, Anzahl und Reihenfolge der Inseln sind entschieden |
| [B-111](archiv/B-111-materialmengen-je-stufe.md) | REG | Frage | hoch | erledigt | R2 | Die Materialmengen je Stufe passen zu den Kosten von Hub-Ausbau, Mauern und Gebäuden |
| [B-109](archiv/B-109-materialien-gebaeude-regelwerk.md) | REG | Idee | hoch | erledigt | R2 | Materialien und Gebäude sind im Regelwerk beschlossen |
| [B-110](archiv/B-110-skillung-klassen-level.md) | REG | Idee | hoch | erledigt | R3 | Skillung, Klassen und Level von Monarchen und Bürgern sind im Regelwerk beschlossen |
| [B-017](archiv/B-017-klassen-preset.md) | REG | Frage | mittel | erledigt | R3 | Klassen-Presets pro Spieler sind entschieden |
| [B-114](archiv/B-114-plantage-adern.md) | SIM | Idee | hoch | erledigt | W2 | Farm-Plantage lässt Holz nachwachsen, Adern liefern Stein bis Kristall unendlich mit Abbaurate |
| [B-115](archiv/B-115-stufen-breite-eisen-kristall.md) | SIM | Idee | mittel | erledigt | W2 | Die Stufen sind nach unten schmaler und dichter, Eisenstollen und Kristallhöhle sind als Stufen angelegt |
| [B-127](archiv/B-127-regelwerk-gegner-bosse.md) | REG | Idee | hoch | erledigt | R4 | Gegner, Wellen, Bosse und Events sind im Regelwerk beschlossen |
| [B-004](archiv/B-004-regelwerk.md) | REG | Idee | hoch | erledigt | – | Regelwerk ist ausführlich diskutiert und ausgearbeitet |
| [B-100](archiv/B-100-mehrstufen-insel.md) | SIM | Idee | hoch | erledigt | SP12 | Eine Insel hat n Stufen, die alle laufen und pro Spieler begehbar sind |
| [B-101](archiv/B-101-raum-optionen-schwierigkeit.md) | SIM | Idee | hoch | erledigt | SP13 | Raum-Optionen und fünf Schwierigkeitsgrade wirken in der Simulation |
| [B-113](archiv/B-113-material-lager.md) | SIM | Idee | hoch | erledigt | SP13 | Fünf Materialien, Lager-Maximum und Tragen zum Lager sind umgesetzt |
| [B-133](archiv/B-133-raum-auf-insel.md) | SRV | Idee | hoch | erledigt | SP14 | Der Raum rechnet mit einer Insel statt mit einer Kampagne |
| [B-104](archiv/B-104-protokoll-stufe-und-optionen.md) | SRV | Idee | hoch | erledigt | SP14 | Das Protokoll kennt die Stufe je Spieler und die Raum-Optionen |
| [B-171](archiv/B-171-dev-seiten-tasks-planung-git.md) | SRV | Idee | mittel | erledigt | M7 | k3c-workbench zeigt Tasks, Planung und Git wie die Workbench der ErpApi |
| [B-174](archiv/B-174-sprints-je-domaene-parallel.md) | INF | Idee | hoch | erledigt | F0 | Je Domäne darf ein Sprint aktiv sein, Sessions werden per Branch beansprucht |
| [B-035](archiv/B-035-raspberry-pi.md) | SRV | Idee | hoch | erledigt | SP11 | Server läuft auf dem Raspberry Pi im Docker |
| [B-175](archiv/B-175-lasttest-werkzeug.md) | SRV | Idee | mittel | erledigt | LT1 | Ein Lasttest-Werkzeug misst Tick-Dauer und CPU gegen das Pi-Ziel |
| [B-180](archiv/B-180-version-nach-sprint.md) | INF | Idee | mittel | erledigt | – | Nach jedem fertigen Sprint wird eine neue Version vorgeschlagen und bei Bestätigung gesetzt |
| [B-177](archiv/B-177-holz-startvorrat.md) | SIM | Idee | hoch | erledigt | H1 | Die Insel startet mit einem Holz-Startvorrat |
| [B-183](archiv/B-183-pages-praesentation.md) | PLAT | Idee | mittel | erledigt | – | GitHub Pages zeigt eine Präsentationsseite des Spiels |
| [B-134](archiv/B-134-zielkorridore.md) | REG | Idee | hoch | erledigt | F1 | Jede Kennzahl des Spiels hat einen Zielkorridor als Zahl |
| [B-135](archiv/B-135-pause-im-raum.md) | REG | Frage | hoch | erledigt | F1 | Pause im gemeinsamen Raum ist als Regel festgelegt |
| [B-136](archiv/B-136-mindest-schriftgroesse.md) | REG | Frage | mittel | erledigt | F1 | Die Mindest-Schriftgröße je Split-Viertel ist festgelegt |
| [B-144](archiv/B-144-verbindungsverlust-latenz.md) | REG | Frage | mittel | erledigt | F1 | Verbindungsverlust und Eingabe-Latenz haben eine Regel mit Zahlen |
| [B-145](archiv/B-145-sprache-nur-deutsch.md) | REG | Frage | niedrig | erledigt | F1 | Das Spiel bleibt dauerhaft deutschsprachig, oder die Lokalisierung ist geplant |
| [B-071](archiv/B-071-golden-arm64.md) | INF | Problem | mittel | erledigt | F2 | Die Golden-Tests laufen auch auf arm64 grün |
| [B-137](archiv/B-137-golden-ablauf-migration.md) | INF | Idee | hoch | erledigt | F2 | Golden-Daten und Spielstand-Formate haben einen festen Änderungsablauf |
| [B-138](archiv/B-138-determinismus-pruefung.md) | INF | Schuld | mittel | erledigt | F2 | Determinismus der Simulation wird gegen Map-Reihenfolge und langsame Ticks geprüft |
| [B-139](archiv/B-139-feedback-events-sim.md) | SIM | Idee | hoch | erledigt | F3 | Die Simulation meldet Feedback-Ereignisse für Treffer, Münzen, Schläge und Tod |
| [B-140](archiv/B-140-feedback-events-protokoll.md) | SRV | Idee | hoch | erledigt | F4 | Feedback-Ereignisse laufen im Protokoll mit gemessener Bandbreite zum Client |
| [B-142](archiv/B-142-pi-betrieb-backup-rotation.md) | SRV | Idee | hoch | erledigt | F4 | Spielstände werden außerhalb des Pi gesichert, Berichte und Logs rotieren |
| [B-143](archiv/B-143-endpunkte-heimnetz-absichern.md) | SRV | Problem | mittel | erledigt | F4 | Restore-, Save- und Report-Endpunkte sind im Heimnetz abgesichert |
| [B-178](archiv/B-178-dev-aktionen-gold-material-zeitraffer.md) | SRV | Idee | hoch | erledigt | DBG1 | Im Dev-Mode lassen sich Gold und Material droppen und die Zeit beschleunigen |
| [B-179](archiv/B-179-debug-overlay-aktionen.md) | CLI | Idee | hoch | erledigt | DBG2 | Das Debug-Overlay bedient Gold, Material und Zeitraffer |
| [B-006](archiv/B-006-xbox-gamepad-test.md) | PLAT | Frage | hoch | erledigt | X1 | Gamepad-Test auf der Xbox ist ausgewertet |
| [B-026](archiv/B-026-skill-tasten.md) | PLAT | Frage | hoch | erledigt | X1 | Skill-Tasten am Controller sind festgelegt |
| [B-166](archiv/B-166-audio-autoplay-formate-xbox.md) | PLAT | Frage | hoch | erledigt | X1 | Audio-Autoplay und Formate auf Edge der Xbox sind geprüft |
| [B-079](archiv/B-079-landing-kacheln-lobby.md) | PLAT | Schuld | mittel | erledigt | F5 | Die Kacheln der Landingpage passen zum Start über die Lobby |
| [B-141](archiv/B-141-doku-drift-version.md) | INF | Schuld | mittel | erledigt | F5 | Doku und CLAUDE.md stimmen mit dem Code überein, die Version ist sichtbar |
| [B-165](archiv/B-165-credits-seite.md) | PLAT | Idee | mittel | erledigt | GR6 | Eine Credits-Seite entsteht aus den CREDITS-Dateien, ein Test prüft die Vollständigkeit |
| [B-029](archiv/B-029-lade-szene.md) | CLI | Idee | mittel | erledigt | GR4 | Lade-Szene zeigt Fortschritt |
| [B-163](archiv/B-163-atlas-build-ladezeit.md) | INF | Idee | mittel | erledigt | GR4 | Die Spiel-Grafiken kommen aus einem Atlas, der Kaltstart hat ein Zeitbudget |
| [B-196](archiv/B-196-pages-workflows-go-fuer-atlas.md) | INF | Problem | hoch | erledigt | GR4 | Die Pages-Workflows bauen mit Go, weil `task build` den Atlas packt |
| [B-106](archiv/B-106-kamera-je-stufe.md) | CLI | Idee | hoch | erledigt | S4 | Jeder Spieler sieht seine Stufe, auch wenn die Spieler in verschiedenen Stufen sind |
| [B-199](archiv/B-199-review-diff-befehl.md) | INF | Schuld | mittel | erledigt | – | Review-Sessions lesen den Sprint-Diff mit dem Befehl aus der Arbeitsweise |
| [B-210](archiv/B-210-planung-ueber-mcp.md) | SRV | Idee | hoch | erledigt | M8 | Agenten pflegen Tickets, Sprints und Sessions über MCP-Tools von k3c-workbench |
| [B-211](archiv/B-211-planungsseite-react.md) | SRV | Schuld | mittel | erledigt | M8 | Die Planungsseite von k3c-workbench ist eine React-Ansicht aus denselben Daten wie die MCP-Tools |
| [B-212](archiv/B-212-github-status-planung.md) | SRV | Idee | mittel | erledigt | M8 | k3c-workbench zeigt PR, CI und Merge-Konflikte je Sprint aus GitHub |
| [B-146](archiv/B-146-optionen-pause-szene.md) | CLI | Idee | hoch | erledigt | S5 | Der Client hat eine Optionen- und Pause-Szene mit getrennter Lautstärke und Barrierefreiheit |
| [B-172](archiv/B-172-sprachauswahl-de-en.md) | CLI | Idee | mittel | erledigt | S5 | Der Client hat Deutsch und Englisch mit Sprachauswahl in den Optionen |
| [B-173](archiv/B-173-monarch-auf-reittier-zeichnen.md) | CLI | Idee | hoch | erledigt | S7 | Der Client zeichnet den Monarchen auf dem Standard-Reittier |
| [B-164](archiv/B-164-juice-treffer-screenshake.md) | CLI | Idee | mittel | erledigt | GR5 | Treffer, Münzen und Bauen haben sichtbare Rückmeldung, Screenshake und Blitze sind abschaltbar |
| [B-216](archiv/B-216-tier-gating-unerreichbar.md) | REG | Frage | hoch | erledigt | S1 | Tier-Gating 5/10/15 je Linie ist mit einem Punkt je Skill unerreichbar |
| [B-221](archiv/B-221-s12c-erlaubte-dateien-passive.md) | SIM | Frage | hoch | erledigt | S1 | S1.2c braucht für die Passive weitere erlaubte Dateien |
| [B-220](archiv/B-220-schild-ersetzt-nur-groesseren.md) | SIM | Problem | niedrig | erledigt | S1 | Ein neuer Schild ersetzt den laufenden nur, wenn er größer ist |
| [B-169](archiv/B-169-hoerprobenseite-soundtest.md) | PLAT | Idee | mittel | erledigt | SO3 | Eine Hörprobenseite spielt Kandidaten für Musik und Effekte ab |
| [B-022](archiv/B-022-monarch-spielstand.md) | SIM | Idee | hoch | erledigt | S1 | Monarch-Level und Skills stehen im Spielstand |
| [B-118](archiv/B-118-monarch-schlag-pool.md) | SIM | Idee | hoch | erledigt | S1 | Der Monarch schlägt zu, Skill-Punkte kommen aus einem Fund-Pool und jeder Spieler verteilt sie für sich |
| [B-119](archiv/B-119-skills-tank-zauberer-heiler.md) | SIM | Idee | hoch | erledigt | S1 | Die Skills von Tank, Zauberer und Heiler wirken in der Simulation |
| [B-152](archiv/B-152-reittiere-mechanik.md) | SIM | Idee | hoch | erledigt | S1 | Jeder Monarch reitet von Anfang an auf einem Standard-Reittier |
| [B-222](archiv/B-222-glossar-spielstand-version-3.md) | REG | Schuld | niedrig | erledigt | S1 | Das Glossar nennt Spielstand-Version 3 |
| [B-231](archiv/B-231-cheat-dialog.md) | CLI | Idee | hoch | erledigt | – | Der Cheat-Dialog ist modal, hält den Raum an und öffnet per Geste auf jedem Gerät |
| [B-182](archiv/B-182-tod-mit-ursache.md) | SIM | Problem | hoch | erledigt | W0 | Das Ereignis playerDown nennt, was den Monarchen getötet hat |
| [B-261](archiv/B-261-camp-neben-linien-platz.md) | SIM | Frage | mittel | erledigt | W0 | Ein Camp liegt nie so nah an einem Linien-Platz, dass Zahlziele sich überlagern |
| [B-206](archiv/B-206-bauplaetze-aus-dem-seed.md) | SIM | Idee | hoch | erledigt | W0 | Alle Bauplätze sind feste Punkte aus Daten und Level-Seed, Mauerlinien schalten je Seite nacheinander frei |
| [B-161](archiv/B-161-grafik-zuordnungstabelle.md) | CLI | Idee | hoch | erledigt | GR1 | Jedes Spielobjekt hat eine Zuordnung zu Asset und Lizenz oder eine dokumentierte Lücke |
| [B-170](archiv/B-170-release-checkliste.md) | INF | Idee | hoch | erledigt | RL1 | Eine Release-Checkliste macht jeden Release prüfbar |
| [B-162](archiv/B-162-grafik-suche-luecken.md) | CLI | Frage | mittel | erledigt | GR2 | Für die Grafik-Lücken liegen Kandidaten mit Vorschau, Lizenz und Stilbewertung vor |
| [B-271](archiv/B-271-s2-3-ohne-todesursache.md) | SRV | Frage | hoch | erledigt | S2 | Der Spielmetrik-Report wartet auf B-182 oder startet ohne Todesursache |
| [B-181](archiv/B-181-latenz-im-debug-overlay.md) | CLI | Idee | mittel | erledigt | N2 | Das Debug-Overlay zeigt die Latenz von Eingabe bis Bild |
| [B-277](archiv/B-277-zeitleiste-vorhersage.md) | CLI | Problem | hoch | erledigt | N2 | Der Client zeichnet trotz schwankender Zustände flüssig und wartet bei der eigenen Laufbewegung nicht auf den Server |
| [B-276](archiv/B-276-tick-budget-async.md) | SRV | Problem | hoch | erledigt | N1 | Der Raum-Tick bleibt im Budget, Kodierung und Speichern laufen außerhalb der Raum-Sperre |
| [B-278](archiv/B-278-protokoll-langsame-geraete.md) | SRV | Schuld | mittel | erledigt | S2 | docs/protocol.md beschreibt, wie der Server langsame Geräte behandelt |
| [B-279](archiv/B-279-protokoll-eingabe-takt.md) | SRV | Schuld | niedrig | erledigt | S2 | docs/protocol.md beschreibt den Eingabe-Takt so, wie der Client ihn seit N2 sendet |
| [B-123](archiv/B-123-protokoll-skills-aktionen.md) | SRV | Idee | hoch | erledigt | S2 | Das Protokoll kennt Schlag, Skills, Pool, Berufe und die gültigen Aktionen je Spieler |
| [B-147](archiv/B-147-speichern-verlassen.md) | SRV | Idee | mittel | erledigt | S2 | Der Server speichert beim Verlassen und wenn das letzte Gerät getrennt ist, der Spielstand zeigt seinen Speicherstand |
| [B-150](archiv/B-150-spielmetrik-report.md) | SRV | Idee | mittel | erledigt | S2 | Der Server schreibt je Sitzung einen Spielmetrik-Report nach reports/ |
| [B-176](archiv/B-176-protokoll-mehrere-stufen.md) | SRV | Idee | hoch | erledigt | S2 | Das Protokoll liefert Level und Zustand jeder Stufe, in der ein lokaler Spieler steht |
| [B-112](archiv/B-112-hub-ausbau-mauerstufen.md) | SIM | Idee | hoch | erledigt | W1 | Der Hub wird in fünf Stufen ausgebaut, Mauern und Türme haben fünf Materialstufen |
| [B-116](archiv/B-116-gebaeude-wirkungen.md) | SIM | Idee | mittel | erledigt | W3 | Tor, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum wirken im Spiel |
| [B-232](archiv/B-232-dungeon-master-seite.md) | PLAT | Idee | hoch | erledigt | DBG3 | Eine Dungeon-Master-Seite unter /dm steuert Räume live vom Handy oder Tablet |
| [B-281](archiv/B-281-monitoring-dashboard.md) | SRV | Idee | hoch | erledigt | MON1 | Der Server sammelt Latenzen, Tick-Dauer und Fehler als Verlauf und liefert sie über /api/metrics |
| [B-282](archiv/B-282-monitoring-seite.md) | PLAT | Idee | hoch | erledigt | MON2 | Die Monitoring-Seite zeichnet Verläufe, Perzentile und die Fehler-Zeitleiste aus /api/metrics |
| [B-158](archiv/B-158-bot-profile-sensitivitaet.md) | SIM | Idee | mittel | erledigt | BAL3 | Der Tester kennt weitere Bot-Profile, Sensitivitäts-Läufe und Kurven je Schwierigkeitsgrad |
| [B-296](archiv/B-296-verlust-kaskade-bricht-delta-test.md) | SRV | Frage | hoch | erledigt | – | Die Verlust-Kaskade (W4.3a) lässt sich ohne Änderung an `engine/net` nicht grün umsetzen |
| [B-297](archiv/B-297-delta-entfernt-felder.md) | SRV | Problem | hoch | erledigt | DL1 | Das Delta überträgt, dass ein Feld aus dem Zustand verschwindet |
| [B-252](archiv/B-252-grafikmanager-seite.md) | PLAT | Idee | mittel | verworfen | – | Eine GrafikManager-Seite zeigt Bestand, Kandidaten und Zuordnung für die feine Auswahl |
| [B-310](archiv/B-310-elite-verhalten-ausserhalb-erlaubter-dateien.md) | SIM | Frage | hoch | erledigt | – | Elite-Werte wirken nur mit Änderungen außerhalb der Erlaubten Dateien von W4.3b |
| [B-311](archiv/B-311-passive-burg-faellt-nicht.md) | REG | Frage | hoch | erledigt | – | Mit der Verlust-Kaskade fällt die Burg bei passivem Spiel nie |
| [B-325](archiv/B-325-gebaeude-ziel-upgrade-test.md) | SIM | Frage | mittel | erledigt | – | K1.2 darf den Testaufbau des Elite-Bogenschützen anpassen |
| [B-326](archiv/B-326-k1-3-grafik-luecken-neue-gegner.md) | SIM | Frage | hoch | erledigt | – | K1.3 darf den sechs neuen Gegnern Platzhalter-Sprites und Zuordnungs-Zeilen geben |
| [B-323](archiv/B-323-welt-spiegelt-lager-hub-ausbau.md) | SIM | Frage | hoch | erledigt | W9 | Die Welt spiegelt Lager-Maximum, Hub-Ausbau mit Kosten und Wartegrund „Gefahr“ für das Protokoll |
| [B-013](archiv/B-013-gegner-elite.md) | SIM | Idee | mittel | erledigt | K1 | Restliche Gegner und Elite-KI sind umgesetzt |
| [B-128](archiv/B-128-traits-kiting-angriffsrate.md) | SIM | Idee | mittel | erledigt | K1 | Die Gegner-Traits aoe, swarm, phases und Kiting wirken, die Angriffsrate steht je Gegner in den Daten |
| [B-129](archiv/B-129-neue-gegner-pools.md) | SIM | Idee | mittel | erledigt | K1 | Eisenstollen und Kristallhöhle haben ihre Gegner und Pools |
| [B-124](archiv/B-124-skill-menue-tasten.md) | CLI | Idee | hoch | erledigt | S3 | Der Client hat Schlag, Skill-Slots, Skill-Menü und die Tasten für Controller, Tastatur und Touch |
| [B-125](archiv/B-125-aktionen-overlay.md) | CLI | Idee | hoch | erledigt | S3 | Gültige Aktionen erscheinen überall in der Welt als Overlay am Ort |
| [B-148](archiv/B-148-onboarding-erste-nacht.md) | CLI | Idee | hoch | erledigt | S6 | Die erste Nacht wird mit kontextuellen Hinweisen geführt, der Grad „Leicht“ kostet keinen Fortschritt |
| [B-149](archiv/B-149-controller-glyphen.md) | CLI | Idee | mittel | erledigt | S6 | Hinweise zeigen Controller-Glyphen statt Tasten-Text |
| [B-292](archiv/B-292-neues-spiel-eindeutiger-name.md) | PLAT | Problem | hoch | erledigt | LP1 | Die Kachel „Neues Spiel“ startet auch bei vorhandenem Spielstand familie |
| [B-340](archiv/B-340-mine-test-miniboss.md) | SIM | Frage | hoch | erledigt | K2 | K2.1a darf den Mine-Test an den Miniboss anpassen |
| [B-275](archiv/B-275-worktree-unter-claude.md) | SRV | Problem | hoch | erledigt | M9 | k3c-workbench und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig |
| [B-213](archiv/B-213-markdown-listen-haekchen.md) | SRV | Problem | niedrig | erledigt | M9 | MarkdownView in k3c-workbench zeigt nummerierte Listen und Häkchen wie die alte Planungsseite |
| [B-348](archiv/B-348-sim-test-workbench.md) | SRV | Idee | hoch | erledigt | TR1 | Jeder Testlauf startet und läuft über das MCP-Tool `sim_test` |
| [B-349](archiv/B-349-bot-eingabe-client.md) | PLAT | Idee | hoch | erledigt | TR2 | Bots steuern im Client die Monarchen über die Bot-Eingabe |
| [B-355](archiv/B-355-projekte-mit-rang.md) | INF | Idee | hoch | erledigt | PJ1 | Projekte bündeln Sprints zu Themen und werden nach Rang abgearbeitet |
| [B-356](archiv/B-356-domaene-je-session.md) | INF | Idee | hoch | erledigt | PJ1 | Eine Session gehört zu genau einer Domäne, ein Sprint darf mehrere Domänen nacheinander enthalten |
| [B-357](archiv/B-357-k3c-dev-projekte.md) | SRV | Idee | hoch | erledigt | PJ2 | Die plan-Tools von k3c-workbench legen Projekte an, ordnen Sprints und Tickets zu und setzen den Rang |
| [B-358](archiv/B-358-workbench-projekte.md) | SRV | Idee | hoch | erledigt | PJ2 | Die Planungsseite der Workbench zeigt Projekte nach Rang mit ihren Sprints |
| [B-338](archiv/B-338-session-status-verworfen.md) | INF | Schuld | hoch | erledigt | PJ1 | Sessions können den Status verworfen tragen |
| [B-353](archiv/B-353-bot-eingabe-einbinden.md) | CLI | Idee | hoch | erledigt | TR3 | Das Spiel fragt die Bot-Eingabe ab und setzt ihre Spieler ohne Tastendruck in den Raum |
| [B-350](archiv/B-350-mcp-seite-tools-zeitfilter.md) | SRV | Idee | hoch | erledigt | M11 | Die MCP-Seite zeigt alle Tools mit Aufruf-Statistik und filtert die Statistik nach Zeit |
| [B-364](archiv/B-364-planung-hierarchie-optik.md) | SRV | Idee | mittel | erledigt | – | Die Planungsseite zeigt Projekt, Sprint und Session als klar unterscheidbare Ebenen |
| [B-359](archiv/B-359-planung-in-projekte.md) | INF | Schuld | hoch | erledigt | PJ3 | Die offene Planung ist in Projekte umgezogen, erledigte und zusammengelegte Sprints sind abgeschlossen |
| [B-322](archiv/B-322-texte-werkzeug-seiten.md) | PLAT | Schuld | niedrig | erledigt | PL2 | Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien |
| [B-365](archiv/B-365-domaene-dev-werkzeug.md) | INF | Idee | hoch | erledigt | DV1 | Das Entwickler-Werkzeug k3c-workbench hat eine eigene Domäne statt SRV |
| [B-361](archiv/B-361-sprint-prio-felder-entfallen.md) | INF | Schuld | mittel | erledigt | DV1 | Die Sprint-Felder Prio und Einschiebbar entfallen in Vorlage, Planungstest und plan-Tools |
| [B-362](archiv/B-362-k3c-dev-token-dienst.md) | DEV | Problem | hoch | erledigt | DV1 | k3c-workbench fragt den selbst gestarteten Spielserver mit dessen Token an |
| [B-192](archiv/B-192-aktionsliste-schliesst-mit-oe.md) | CLI | Problem | hoch | erledigt | U5 | Die Dev-Aktionsliste schließt sich mit Ö |
| [B-205](archiv/B-205-y-belegung-s3.md) | CLI | Problem | mittel | erledigt | S8 | Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ |
| [B-130](archiv/B-130-bosse.md) | SIM | Idee | hoch | erledigt | K2 | Minibosse und Endboss sind spielbar |
| [B-102](archiv/B-102-siegvarianten-niederlage.md) | SIM | Idee | mittel | erledigt | K2 | Siegvarianten und Niederlage-Modi der Raum-Optionen sind umgesetzt |
| [B-103](archiv/B-103-inseln-bosse.md) | SIM | Idee | mittel | erledigt | K2 | Inseln mit Endboss und gemeinsamem Inselwechsel sind spielbar |
| [B-321](archiv/B-321-schlag-skill-ohne-ziel-ereignis.md) | SIM | Problem | mittel | erledigt | S9 | Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis |
| [B-131](archiv/B-131-events.md) | SIM | Idee | niedrig | erledigt | K3 | Vollmond, Blutmond und Händler-Überfall sind als Events umgesetzt |
| [B-373](archiv/B-373-haendler-angreifbar-besuchszaehler.md) | SIM | Idee | mittel | erledigt | K3 | Der Händler ist eine angreifbare Figur mit Besuchszähler im Spielstand |
| [B-345](archiv/B-345-raum-tauscht-insel.md) | SRV | Idee | hoch | erledigt | K4 | Der Raum tauscht die Insel bei `SwitchReady` |
| [B-383](archiv/B-383-sim-zustand-boss-event-wechsel.md) | SIM | Problem | mittel | erledigt | K4 | Die Welt stellt Boss-Phase, Warnkreis, Event-Restzeit und Inselwechsel für das Protokoll bereit |
| [B-037](archiv/B-037-lobby.md) | CLI | Idee | mittel | erledigt | LB1 | Lobby zeigt Räume und startet Spiele |
| [B-388](archiv/B-388-checkout-argument-schreibende-tools.md) | DEV | Problem | hoch | erledigt | WT1 | Schreibende k3c-workbench-Tools nehmen den Ziel-Checkout als Argument, weil der Client aus Desktop-App-Worktrees die Wurzel meldet |
| [B-339](archiv/B-339-glyphen-je-plattform.md) | CLI | Idee | mittel | verworfen | – | Spiel und Seiten zeigen Tastensymbole passend zum gerade benutzten Controller (Xbox, PlayStation, weitere) |
| [B-316](archiv/B-316-tastatur-zwei-spieler.md) | PLAT | Idee | hoch | erledigt | PL1 | Zwei Spieler spielen an einer Tastatur im Split-Screen |
| [B-215](archiv/B-215-texte-eingabe-shell-tools.md) | PLAT | Schuld | niedrig | erledigt | PL1 | Die Texte von Touch-Overlay, Shell und Werkzeug-Seiten kommen aus den zentralen Textdateien |
| [B-395](archiv/B-395-preisschilder-ueberlappen.md) | CLI | Problem | hoch | erledigt | S9 | Nahe Bauplätze zeigen ihre Preisschilder nie übereinander |
| [B-319](archiv/B-319-ein-hinweis-je-weltposition.md) | CLI | Problem | hoch | erledigt | S9 | Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild |
| [B-318](archiv/B-318-schlag-skill-feedback.md) | CLI | Problem | mittel | erledigt | S9 | Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam |
| [B-396](archiv/B-396-schlag-ausholen-animation.md) | CLI | Problem | mittel | erledigt | S9 | Der Monarch holt beim Schlag ohne Ziel sichtbar aus |
| [B-397](archiv/B-397-auto-merge-sprint-pr.md) | INF | Idee | mittel | erledigt | – | Der PR eines Sprints wird mit Auto-Merge geöffnet |
| [B-371](archiv/B-371-glyphen-tastatur-2.md) | CLI | Problem | mittel | erledigt | AZ1 | Hinweise und Glyphen zeigen für Spieler 2 an der Tastatur dessen Tasten |
| [B-333](archiv/B-333-pause-anzeige.md) | CLI | Idee | mittel | erledigt | AZ1 | Der Client zeigt einen angehaltenen Raum deutlich an und hält die Figuren-Animationen an |
| [B-368](archiv/B-368-planung-erpapi-ablage.md) | SRV | Schuld | mittel | verworfen | – | Die Planung liegt in der ErpApi-Ablage und k3c-workbench bedient sie mit den Planungs-Tools der Workbench-Spec |
| [B-336](archiv/B-336-touch-optionen-schliessen.md) | CLI | Problem | hoch | verworfen | U6 | Die Optionen-Szene lässt sich per Touch vollständig bedienen und schließen, ohne vom Touch-Overlay verdeckt zu werden |
| [B-390](archiv/B-390-touch-hud-verdeckt-bei-aufloesung.md) | CLI | Problem | mittel | verworfen | – | Touch-Tasten und Start-Button verdecken HUD-Infozeilen bei bestimmten Auflösungen |
| [B-040](archiv/B-040-server-finden.md) | SRV | Idee | niedrig | verworfen | BT1 | Geräte finden den Server im Heimnetz |
| [B-337](archiv/B-337-hud-elemente-ohne-ueberlagerung.md) | CLI | Idee | hoch | erledigt | U6 | Jede HUD-Anzeige ist ein eigenes Element mit optionalem Hintergrund und Rahmen, und HUD-Elemente überlagern sich nicht |
| [B-405](archiv/B-405-k3c-dev-gotest-haengt.md) | DEV | Problem | mittel | verworfen | – | Der vollständige Go-Test von tools/k3c-workbench läuft ohne Hänger durch |
| [B-403](archiv/B-403-wz2-review-nachtraege.md) | DEV | Schuld | mittel | erledigt | WZ3 | Review-Nachträge zu Sprint WZ2: Filter-Zählung, Schnellfilter, Splitter-Test und Kleinigkeiten |
| [B-404](archiv/B-404-fahrplan-projekt-aktiv.md) | DEV | Problem | niedrig | verworfen | – | Die Fahrplan-Zeile eines aktivierten Sprints trägt sein Projekt statt „–“ |
| [B-342](archiv/B-342-w6-2-siteview-erlaubt.md) | CLI | Frage | mittel | erledigt | – | W6.2 darf den Bauplatz-Wartegrund in `siteView.ts` anbinden |
| [B-407](archiv/B-407-wz3-verweis-b403-ac08.md) | INF | Problem | hoch | erledigt | – | WZ3-README verweist auf B-403/AC-08, das es nicht gibt (planning.test.ts rot) |
| [B-416](archiv/B-416-checkout-git-ohne-fenster.md) | DEV | Problem | hoch | erledigt | – | Der Checkout-Aufruf von k3c-workbench öffnet bei jedem Tool ein Konsolenfenster |
| [B-197](archiv/B-197-partner-zelle-schriftgroesse.md) | CLI | Frage | hoch | erledigt | GR7 | Die Schriftregel nennt eine Mindestgröße für die Mitspieler-Zelle |
| [B-198](archiv/B-198-platzhaltertext-schrift-katalog.md) | CLI | Schuld | hoch | erledigt | GR7 | Der Platzhaltertext einer ungeladenen Stufe liest seine Schrift aus dem Katalog |
| [B-415](archiv/B-415-cockpit-lint-rot.md) | INF | Problem | hoch | erledigt | – | Lint ist rot: tools/cockpit überschreitet die Dateigrenze |
| [B-312](archiv/B-312-wiederaufheben-begrenzen.md) | SIM | Problem | hoch | erledigt | W7 | Sofortiges Wiederaufheben fallengelassener Ausrüstung macht die Burg bei passivem Spiel unverwundbar |
| [B-313](archiv/B-313-vermerk-wirkung-offen-test.md) | SIM | Frage | hoch | erledigt | W7 | W4.3b kann den Vermerk „Wirkung offen“ nur mit einer Änderung an sites_test.go ersetzen |
| [B-202](archiv/B-202-spielstand-versionsfolge.md) | SIM | Problem | hoch | erledigt | W7 | S1 und W1 teilen sich die Spielstand-Version 3 eindeutig |
| [B-201](archiv/B-201-spielstand-wirtschaft.md) | SIM | Schuld | mittel | erledigt | W7 | Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nach dem Laden wieder her |
| [B-075](archiv/B-075-golden-spielstand-hub.md) | SIM | Schuld | mittel | erledigt | W7 | Der Golden-Spielstand enthält einen gebauten und veränderten Hub |
| [B-417](archiv/B-417-golden-groesse-ueber-limit.md) | INF | Problem | niedrig | erledigt | – | Golden-Daten liegen mit 12,7 MB über der Grenze von 8 MB aus SP06 |
| [B-406](archiv/B-406-splitter-dom-test.md) | DEV | Schuld | niedrig | erledigt | WZ4 | Doppelklick und Ziehen des Splitters sind in einer DOM-Umgebung getestet |
| [B-199](archiv/B-199-raum-fuenf-stufen.md) | SRV | Problem | mittel | erledigt | SV1 | Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt |
| [B-290](archiv/B-290-raum-fuenf-stufen.md) | SRV | Idee | mittel | erledigt | SV1 | Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle |
| [B-204](archiv/B-204-test-raeume-sofort-schliessen.md) | SRV | Idee | niedrig | erledigt | SV1 | Leere Test-Räume schließen sofort statt nach der Leer-Frist |
| [B-209](archiv/B-209-client-platz-arten.md) | CLI | Schuld | mittel | erledigt | W8 | `src/model/data.ts` kennt alle Platz-Arten aus `hub.json` |
| [B-188](archiv/B-188-client-rohstoffe-eisen-kristall.md) | CLI | Schuld | niedrig | erledigt | W8 | Der Client kennt alle fünf Rohstoffe des Servers |
| [B-424](archiv/B-424-fullsave-offener-raum.md) | SRV | Problem | mittel | erledigt | – | Ein neuer Voll-Ausbau-Stand wird nicht vom noch offenen alten Raum überschrieben |
| [B-370](archiv/B-370-mond-bild-nacht-hinweis.md) | CLI | Frage | niedrig | erledigt | BD1 | Der Hinweis „Die Nacht naht“ zeigt ein Mond-Bild vor dem Text |
| [B-427](archiv/B-427-mond-bild-quelle.md) | CLI | Frage | mittel | erledigt | – | Aus welcher Quelle kommt das Mond-Bild für den Nacht-Hinweis? |
| [B-392](archiv/B-392-splitscreen-kamera-spieler-2.md) | CLI | Problem | hoch | erledigt | BD1 | Im Split-Screen scrollt nur die Kamera von Spieler 1 mit; Spieler 2 ist nicht an seine Ansicht gebunden |
| [B-391](archiv/B-391-beitritts-hinweis-spieler-2-fehlt.md) | CLI | Problem | mittel | erledigt | BD1 | Der Beitritts-Hinweis für Spieler 2 wird nicht angezeigt |
| [B-389](archiv/B-389-oe-schliesst-dialog-zuerst.md) | CLI | Problem | mittel | erledigt | BD1 | Ö schließt bei offenem Cheat-Dialog zuerst den Dialog, das Overlay bleibt offen |
| [B-399](archiv/B-399-steuerhinweis-verschwindet.md) | CLI | Problem | mittel | erledigt | BD1 | Der Steuerhinweis im HUD verschwindet nach einiger Zeit |
| [B-398](archiv/B-398-gold-im-hud-nicht-ueber-reiter.md) | CLI | Idee | mittel | erledigt | BD1 | Das Gold eines Spielers steht im HUD, nicht als Beutel über dem Reiter |
| [B-414](archiv/B-414-welt-meldungen-im-band.md) | CLI | Idee | mittel | erledigt | BD1 | Welt-Meldungen erscheinen für alle im globalen Band oben |
| [B-393](archiv/B-393-muenzbild-hinweis-fehlt.md) | CLI | Problem | mittel | erledigt | BD1 | Das Münzbild vor „Hinlaufen: Münze aufheben“ erscheint im Browser nicht |
| [B-374](archiv/B-374-preisschild-taste.md) | CLI | Frage | mittel | erledigt | BD1 | Das Preisschild nennt die Taste zum Bezahlen |
| [B-441](archiv/B-441-gebaeude-json-beschreibung.md) | DEV | Idee | hoch | erledigt | GEB8 | Gebäude-JSONs tragen ihre Gebäude-Vorlage (`vorlage`) |
| [B-437](archiv/B-437-katalog-eigene-gebaeude.md) | DEV | Problem | mittel | erledigt | GEB3 | Asset-Katalog von k3c-workbench kennt eigene Gebäude-Grafik nicht |
| [B-442](archiv/B-442-hintergrundmauer-typ-b.md) | CLI | Idee | hoch | erledigt | GEB3 | Hintergrundmauern verbinden die Mauern einer Seite wie in Kingdom Two Crowns |
| [B-449](archiv/B-449-geb5-abnahme-sichtpruefung.md) | CLI | Frage | hoch | erledigt | – | GEB5: Abnahme von Vorschau und Spiel durch Ralf |
| [B-448](archiv/B-448-planning-testrepo-entwurf-sprint.md) | DEV | Problem | mittel | erledigt | – | dev:test TestRepo scheitert an Entwurfs-Sprints ohne Sessions (GEB7) |
| [B-451](archiv/B-451-geb6-abnahme-sichtpruefung.md) | CLI | Frage | hoch | erledigt | – | GEB6: Abnahme von Vorschau und Spiel durch Ralf |
| [B-452](archiv/B-452-geb7-abnahme-sichtpruefung.md) | CLI | Frage | hoch | erledigt | GEB7 | GEB7: Abnahme von Vorschau und Spiel durch Ralf |
| [B-439](archiv/B-439-planung-erpqs-look.md) | DEV | Idee | mittel | erledigt | WZ5 | Die Planungsseite von k3c-workbench zeigt und bedient die Planung wie die erp-qs-Workbench |
| [B-421](archiv/B-421-abnahme-m10-ressourcen-manager.md) | DEV | Idee | mittel | erledigt | – | Abnahme M10: Ressourcen-Manager in k3c-workbench und Mini-Szenen durch 🧑 beobachten |
| [B-298](archiv/B-298-ressourcen-manager.md) | DEV | Idee | mittel | erledigt | M10 | Ein ResourcenManager in k3c-workbench ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu |
| [B-299](archiv/B-299-asset-vorschau-szenen.md) | DEV | Idee | mittel | erledigt | M10 | Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab |
| [B-459](archiv/B-459-asset-werkzeug-eigene-exe.md) | DEV | Idee | hoch | erledigt | WZ6 | Die Asset-Seiten für Grafik und Sound laufen in einer eigenen EXE getrennt von der Workbench |
| [B-443](archiv/B-443-dev-logs-scrollen-liste.md) | DEV | Problem | mittel | erledigt | – | Auf der Log-Seite von k3c-workbench scrollt nur die Log-Liste, nicht die ganze Seite |
| [B-444](archiv/B-444-vite-ports-plus-zehn.md) | DEV | Problem | mittel | erledigt | – | Vite-Ports von Spiel und k3c-workbench liegen 10 höher und kollidieren nicht mit dem Vite-Standardport |
| [B-445](archiv/B-445-dev-splitter-assets-mcp.md) | DEV | Idee | mittel | erledigt | – | Assets- und MCP-Seite von k3c-workbench haben ziehbare Splitter zwischen den Panelen |
| [B-092](archiv/B-092-level-betrachter.md) | PLAT | Idee | mittel | verworfen | HW1 | Eine Testseite zeigt ein generiertes Level (Seed und Biom) ohne zu spielen |
| [B-460](archiv/B-460-welt-betrachter-live-vorschau.md) | DEV | Idee | hoch | erledigt | WZ7 | Der Welt-Betrachter im Asset-Studio zeigt Chunks maximiert und eine Live-Vorschau mit Hub-Stufen und schaltbaren Gebäuden |
| [B-477](archiv/B-477-welt-betrachter-nacharbeit.md) | DEV | Problem | hoch | erledigt | WZ16 | Der Welt-Betrachter zeigt Chunk- und Live-Vorschau in einer gemeinsamen, synchron scrollenden Ansicht |
| [B-482](archiv/B-482-planung-kurzpfad-vergleich.md) | DEV | Problem | hoch | erledigt | – | Planungs-Tools erkennen die Repo-Wurzel auch bei Kurzpfaden (8.3) und die CI-Tests des k3c-workbench-Jobs sind grün |
| [B-480](archiv/B-480-assetkit-downloadhit-tls.md) | DEV | Problem | mittel | verworfen | – | TestDownloadHit in tools/assetkit scheitert lokal am TLS-Zertifikat und blockiert task check:all |
| [B-466](archiv/B-466-glossar-morgengrauen-verweis.md) | INF | Problem | niedrig | erledigt | WZ13 | Der Glossar-Eintrag Tageszyklus verweist für das Morgengrauen auf das richtige Ticket |
| [B-053](archiv/B-053-ci-lauf-sp01.md) | INF | Problem | hoch | erledigt | CI1 | Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen |
| [B-273](archiv/B-273-release-image-dev-mode-aus.md) | INF | Schuld | hoch | erledigt | CI1 | Das Release-Image startet den Server ohne Dev-Mode |
| [B-019](archiv/B-019-test-abdeckung.md) | INF | Idee | niedrig | erledigt | CI1 | Test-Abdeckung der Engine ist sichtbar |
| [B-486](archiv/B-486-chunk-editor-speichern-nach-externer-aenderung.md) | DEV | Problem | hoch | erledigt | – | Chunk-Editor: Speichern nach externer Änderung von islands.json schweigt |
| [B-461](archiv/B-461-chunks-je-biom-planbar.md) | SIM | Idee | mittel | erledigt | WZ8 | Chunks einer Insel-Stufe lassen sich planen, per Seed oder mit festen Objekten |
| [B-302](archiv/B-302-aufraeumen-branches-worktrees.md) | INF | Schuld | mittel | erledigt | RP1 | Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt |
| [B-288](archiv/B-288-alte-spielstaende-ohne-ruecksicht.md) | INF | Schuld | mittel | erledigt | RP1 | Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände |
| [B-094](archiv/B-094-npm-reste.md) | INF | Schuld | niedrig | erledigt | RP1 | Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr |
| [B-058](archiv/B-058-execution-policy.md) | INF | Frage | niedrig | erledigt | RP1 | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 |
| [B-095](archiv/B-095-start-mit-seed-und-tiefe.md) | SRV | Idee | niedrig | erledigt | BT1 | Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe |
| [B-048](archiv/B-048-standardbibliothek-in-001.md) | SRV | Frage | niedrig | erledigt | BT1 | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist |
| [B-167](archiv/B-167-sfx-katalog.md) | CLI | Idee | hoch | erledigt | SO2 | Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz |
| [B-270](archiv/B-270-respec-pruefung-ohne-seiteneffekt.md) | SIM | Schuld | mittel | erledigt | SK1 | Die Sim prüft Respec und Lernen ohne Seiteneffekt |
| [B-485](archiv/B-485-chunk-editor-bedienung.md) | DEV | Idee | mittel | erledigt | WZ17 | Chunk-Editor im Welt-Betrachter komfortabler bedienen |
| [B-498](archiv/B-498-studio-als-seite-im-go-server.md) | DEV | Idee | mittel | erledigt | – | Das Asset-Studio ist eine Seite des Go-Servers statt einer eigenen EXE |
| [B-464](archiv/B-464-gegner-bosse-vorschau.md) | DEV | Idee | mittel | erledigt | WZ11 | Eine Gegner-Seite im Asset-Studio zeigt alle Gegnertypen und Bosse mit ihren Vorschauen |
| [B-463](archiv/B-463-monarch-vorschau-animationen.md) | DEV | Idee | mittel | erledigt | WZ10 | Eine Monarch-Seite im Asset-Studio zeigt Reiten, Schlagen, Skills und weitere Vorschauen |
| [B-436](archiv/B-436-plan-create-entwurf-zeilen.md) | DEV | Problem | niedrig | erledigt | WZ12 | plan_create scheitert an Platzhalter-Zeilen „– (Entwurf)“ in der Session-Tabelle |
| [B-467](archiv/B-467-worktree-pill-am-eintrag.md) | DEV | Idee | mittel | erledigt | WZ12 | Worktree-Änderungen stehen als Pill am Projekt, Sprint oder an der Session statt als Leiste oben |
| [B-513](archiv/B-513-welten-editor-spiel-adresse-waehlbar.md) | DEV | Idee | mittel | erledigt | – | Im Welten-Editor lässt sich die Spiel-Adresse wählen: interne Engine oder Server |
| [B-488](archiv/B-488-welten-inseln-editor-prompt.md) | DEV | Idee | mittel | erledigt | WZ18 | Welten und Inseln im Asset-Studio anlegen und bearbeiten, Welt per Prompt füllen |
| [B-507](archiv/B-507-server-und-studio-trennen-dateiwatch.md) | DEV | Schuld | hoch | erledigt | TS1 | Spielserver und Asset-Studio sind getrennt, der Server beobachtet die Dateien des Studios |
| [B-515](archiv/B-515-studio-chat-anmeldung-umgebung.md) | DEV | Problem | mittel | erledigt | – | Studio-Chat übernimmt die Umgebung einer Claude-Sitzung und zeigt Anmeldefehler doppelt |
| [B-518](archiv/B-518-agentic-tempo.md) | INF | Problem | hoch | erledigt | – | Agentische Sessions laufen schneller: weniger Reviews und Prüfläufe, mehr parallel |
| [B-468](archiv/B-468-geplante-chunks-ruinen-gegnerlager.md) | REG | Idee | mittel | erledigt | WZ14 | Geplante Chunks tragen Ruinen und Gegnerlager |
| [B-423](archiv/B-423-sim-test-im-worktree.md) | DEV | Problem | mittel | erledigt | WZ20 | sim_test rechnet immer mit dem Code der Repo-Wurzel, nicht mit dem Worktree der Session |
| [B-440](archiv/B-440-dev-test-entwurfs-sprints-ohne-sessions.md) | DEV | Problem | mittel | erledigt | WZ20 | k3c-workbench-Planungstest lehnt Entwurfs-Sprints ohne Sessions ab |
| [B-483](archiv/B-483-worktree-eintraege-nur-lesend.md) | DEV | Problem | niedrig | erledigt | WZ20 | Planungsseite: Einträge nur im Worktree sind nur lesbar (Detail, Menüs, Rang) |
| [B-484](archiv/B-484-worktree-neuer-sprint-leer.md) | DEV | Problem | niedrig | erledigt | WZ20 | Worktree-Vergleich: Sessions und Projekt eines neuen Sprints fehlen in den Pills |
| [B-514](archiv/B-514-chat-entwurf-planung-nachbesserung.md) | DEV | Problem | niedrig | erledigt | WZ21 | Chat-Entwurf: Planung bei gleicher Insel-id schützen, CLI-Prozessbaum, Test-Riegel |
| [B-487](archiv/B-487-plan-editor-load-check-ablehnung.md) | DEV | Problem | niedrig | erledigt | WZ22 | Chunk-Editor: abgelehnte Aufrufe von planLoad und planCheck bleiben stumm |
| [B-491](archiv/B-491-welten-editor-kleinigkeiten.md) | DEV | Schuld | niedrig | erledigt | WZ22 | Welten Editor: kleine Nachträge zu Schieber, Klick-Sprung und Planungsliste |
| [B-499](archiv/B-499-studio-spiel-oeffnen-welt-zweimal.md) | DEV | Problem | niedrig | erledigt | WZ22 | Studio „Im Spiel öffnen“ in eigener Welt scheitert beim zweiten Mal mit demselben Seed |
| [B-476](archiv/B-476-preload-sprites-beliebige-reittiere.md) | CLI | Schuld | niedrig | erledigt | WZ23 | preloadSprites und createSpriteAnims laden beliebige Reittiere statt nur das Standard-Reittier |
| [B-471](archiv/B-471-renderreference-abmelden.md) | DEV | Schuld | niedrig | erledigt | WZ23 | renderReference beendet seine Animationsschleife, wenn die Karten verschwinden |
| [B-502](archiv/B-502-monarch-seite-ansicht-wuensche.md) | DEV | Idee | mittel | erledigt | WZ23 | Monarch-Seite: einheitliches Sheet, Zoom, zweispaltige Ansicht |
| [B-475](archiv/B-475-frage-geplante-ereignisse-zaehlung.md) | REG | Frage | niedrig | erledigt | WZ24 | Frage: Wie zählen feste Ereignis-Objekte in geplanten Chunks, und gilt für „kein Platz mehr“ das Minimum oder das Maximum? |
| [B-489](archiv/B-489-spielstand-chunk-anzahl.md) | SIM | Problem | mittel | erledigt | WZ24 | Spielstände laden Chunk-Anzahl und Planung aus den aktuellen Daten |
| [B-490](archiv/B-490-sim-chunk-anzahl-nachtraege.md) | SIM | Schuld | niedrig | erledigt | WZ24 | Sim: Chunk-Anzahl mit Kandidaten-hub.json und gemeinsame Prüfung der Inselgröße |
| [B-479](archiv/B-479-hub-pruefung-geplante-chunks.md) | SIM | Schuld | niedrig | erledigt | WZ24 | Prüfung der Kandidaten-hub.json: geplante Chunks nach WZ8 mitprüfen |
| [B-497](archiv/B-497-grosse-inseln-last-und-umwandlung.md) | SIM | Frage | niedrig | erledigt | WZ24 | Große Inseln: Last im Spiel messen, Umwandlung in allen Biomen testen, PlanSave gegen Parallelität |
| [B-508](archiv/B-508-raeume-nach-neustart-wieder-offen.md) | SRV | Idee | mittel | erledigt | WZ25 | Offene Räume sind nach einem Neustart des Servers wieder offen, Geräte kehren ohne Umweg über die Raumliste zurück |
| [B-509](archiv/B-509-grafik-sound-aus-public-live.md) | SRV | Idee | mittel | erledigt | WZ25 | Im Studio geänderte Grafiken und Sounds erreichen die Clients ohne neuen Build |
| [B-510](archiv/B-510-k3c-dev-kein-doppelter-neustart.md) | DEV | Schuld | niedrig | erledigt | WZ25 | k3c-workbench startet den Spielserver bei Änderungen in data/ nicht mehr selbst neu, der Server macht das allein |
| [B-512](archiv/B-512-k3c-dev-uebernommener-server-neue-pid.md) | DEV | Problem | niedrig | erledigt | WZ25 | k3c-workbench beendet einen übernommenen Spielserver auch nach dessen Selbst-Neustart (neue PID am Port, Supervisor-Prozess) |
| [B-511](archiv/B-511-studio-schreibt-atomar.md) | DEV | Schuld | niedrig | erledigt | WZ25 | Das Studio schreibt jede Datei in data/ und public/ atomar, der Server sieht nie eine halbe Datei |
| [B-516](archiv/B-516-studio-chat-promptgeruest-als-instruktion.md) | DEV | Idee | mittel | erledigt | WZ21 | Prompt-Grundgerüst des Studio-Chats ist eine feste Instruktion im Code statt Text im Eingabefeld |
| [B-501](archiv/B-501-planer-vorschau-fixiert-markiert.md) | DEV | Idee | mittel | erledigt | WZ23 | Gebäude-Planer: Live-Vorschau unten fest, gewähltes Gebäude markiert |
| [B-469](archiv/B-469-gegner-seite-boss-effekte-nach-k5.md) | DEV | Idee | niedrig | erledigt | WZ23 | Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner aus K5.1 |
| [B-214](archiv/B-214-server-pause.md) | SRV | Idee | mittel | erledigt | BD3 | Der Server pausiert den Raum im Couch-Raum und schützt den stehenden Monarchen online |
| [B-420](archiv/B-420-importe-grafikpacks.md) | DEV | Schuld | mittel | erledigt | GR9 | Mit k3c-workbench importierte Kandidaten unter public/grafik/kandidaten/ brechen grafikPacks.test nicht |
| [B-524](archiv/B-524-skillheld-tod-zuruecksetzen.md) | SIM | Problem | niedrig | erledigt | BD7 | skillHeld beim Tod zurücksetzen, damit der erste Druck nach dem Respawn castFailed meldet |
| [B-530](archiv/B-530-wartender-monarch-verwundbar.md) | SIM | Problem | mittel | erledigt | BD7 | Wartende Monarchen nach Verbindungsverlust sind in der Simulation nicht unverwundbar |
| [B-285](archiv/B-285-lernbare-skills-im-protokoll.md) | SRV | Problem | mittel | erledigt | RM1 | Der Server nennt je Spieler die lernbaren Skills |
| [B-495](archiv/B-495-respec-in-aktionsliste.md) | SRV | Schuld | niedrig | erledigt | RM1 | Die Aktionsliste nennt respec, wenn CanRespec erlaubt |
| [B-080](archiv/B-080-dev-tasten-server.md) | SRV | Idee | niedrig | erledigt | KB2 | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
| [B-262](archiv/B-262-camps-nahe-portalen.md) | SIM | Frage | mittel | erledigt | LV1 | Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen |
| [B-291](archiv/B-291-lava-nicht-auf-mauerlinien.md) | SIM | Problem | mittel | erledigt | LV1 | Lava liegt nicht auf den Mauerlinien |
| [B-189](archiv/B-189-toter-gegner-flieht-ins-portal.md) | SIM | Problem | mittel | erledigt | LV1 | Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold fallen |
| [B-217](archiv/B-217-ereignisse-built-playerdown-ort.md) | SIM | Schuld | niedrig | erledigt | LV1 | Die Ereignisse `built` und `playerDown` tragen ihren Ort |
| [B-260](archiv/B-260-schutzplatz-ohne-id.md) | SIM | Schuld | niedrig | erledigt | LV1 | Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID |
| [B-200](archiv/B-200-aggressionspool-adern.md) | SIM | Problem | mittel | erledigt | LV1 | Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor |
| [B-525](archiv/B-525-entscheidung-goldtopf.md) | CLI | Frage | niedrig | erledigt | KR1 | Entscheidung: Wortlaut und Symbol des Goldtopfs in der Spielerkarte |
| [B-526](archiv/B-526-entscheidung-hintergrund-hoehe.md) | CLI | Frage | niedrig | erledigt | KR1 | Entscheidung: Hintergrund-Höhe je Biom gegen WORLD_TOP |
| [B-527](archiv/B-527-entscheidung-abmelden-tastatur.md) | CLI | Frage | niedrig | erledigt | KR1 | Entscheidung: Abmelden mit zwei Tastatur-Spielern und Ziel per Auswahl |
| [B-528](archiv/B-528-entscheidung-pause-regeldetails.md) | SRV | Frage | mittel | erledigt | KR1 | Entscheidung: Regeldetails der Raum-Pause (Couch-Raum, Schutz, Frist) |
| [B-529](archiv/B-529-entscheidung-pause-protokoll.md) | SRV | Frage | mittel | erledigt | KR1 | Entscheidung: Protokoll v8 für pause/resume ohne Rückmeldung im Zustand |
| [B-533](archiv/B-533-entscheidung-spielstand-loeschen.md) | SRV | Frage | mittel | erledigt | KR1 | Entscheidung: Spielstand löschen per HTTP ohne Token, Stand bleibt als Sicherung |
| [B-534](archiv/B-534-entscheidung-spielmenue-lobby.md) | CLI | Frage | niedrig | erledigt | KR1 | Entscheidung: Spielmenü ist die Lobby, neuer Spielstand ohne Welt-Auswahl |
| [B-535](archiv/B-535-entscheidung-pause-nach-wiederverbinden.md) | CLI | Frage | niedrig | erledigt | KR1 | Entscheidung: Pause nach Wiederverbinden erneut senden, Schutz beginnt neu |
| [B-536](archiv/B-536-entscheidung-versionen-bd3-bd7.md) | REG | Frage | hoch | erledigt | KR1 | Entscheidung: Versionen v0.21.0 (BD3) und v0.21.1 (BD7) bestätigen |
| [B-537](archiv/B-537-entscheidung-bd5-vierter-punkt.md) | CLI | Frage | hoch | erledigt | KR1 | Entscheidung: Vierter Punkt der BD5-Meldung zum Spielmenü ist unbekannt |
| [B-567](archiv/B-567-version-bd3-bd7-ein-release.md) | REG | Idee | mittel | erledigt | – | BD3 und BD7 erscheinen zusammen als ein Release v0.21.0 |
| [B-538](archiv/B-538-entscheidung-hub-wachstum-nur-grafik.md) | REG | Frage | hoch | erledigt | KR2 | Entscheidung D1: Hub-Wachstum bleibt reine Grafik, keine feinere Stufenfolge im Server |
| [B-540](archiv/B-540-entscheidung-wald-dichte.md) | REG | Frage | hoch | erledigt | KR2 | Entscheidung D3: Wald-Dichte und Höhlenbüsche einplanen, Werte setzt Ralf |
| [B-541](archiv/B-541-entscheidung-baumgroesse.md) | DEV | Frage | hoch | erledigt | KR2 | Entscheidung D4: Bäume bleiben 11,25 Kacheln hoch |
| [B-542](archiv/B-542-entscheidung-plantage-pappel.md) | DEV | Frage | hoch | erledigt | KR2 | Entscheidung D5: Plantage-Pappel steht hinter der Farm |
| [B-543](archiv/B-543-entscheidung-truhe-ohne-serverzustand.md) | CLI | Frage | hoch | erledigt | KR2 | Entscheidung D6: Geschlossene Truhe ohne Serverzustand |
| [B-544](archiv/B-544-entscheidung-geb9-defaults.md) | DEV | Frage | hoch | erledigt | KR2 | Entscheidung D7: Hub-Gebäude folgen der Hub-Stufe (Defaults aus GEB9) |
| [B-545](archiv/B-545-entscheidung-geruest-bei-gefahr.md) | CLI | Frage | hoch | erledigt | KR2 | Entscheidung D8: Baugerüst nur ohne Gefahr |
| [B-546](archiv/B-546-entscheidung-gegner-figuren.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G1: Die sechs neuen Gegner behalten die GR8-Figuren, die GR7.3-Kandidaten werden nicht importiert |
| [B-547](archiv/B-547-entscheidung-skalierung-krumm.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G2: Krumme Skalierungsfaktoren bleiben bis zur TV-Beobachtung GR7.4 |
| [B-548](archiv/B-548-entscheidung-stil-bibel.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G3: Der Entwurf der Stil-Bibel (DB32, 16 px, Outline, Licht oben links) gilt |
| [B-549](archiv/B-549-entscheidung-umschalten-db32.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G4: Das Spiel lädt weiter die Quellgrafik, nicht public/grafik-db32/ |
| [B-550](archiv/B-550-entscheidung-db32-doppelung.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G5: Die DB32-Palette bleibt doppelt in pipeline und buildings |
| [B-551](archiv/B-551-entscheidung-kandidaten-kein-pack.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G6: public/grafik/kandidaten/ ist kein Grafik-Pack |
| [B-553](archiv/B-553-entscheidung-specs-gr8-gr9-gr10.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G8: Die autonom freigegebenen Specs GR8, GR9 und GR10 gelten |
| [B-555](archiv/B-555-entscheidung-musik-ende.md) | REG | Frage | hoch | erledigt | KR4 | Entscheidung S2: Nach dem Fall der Burg bleibt die Musik still, das Siegstück wartet auf einen Sieg-Auslöser |
| [B-556](archiv/B-556-entscheidung-sound-credit-datei.md) | REG | Frage | hoch | erledigt | KR4 | Entscheidung S3: Die Credits der Sounds bleiben in public/audio/README.md |
| [B-557](archiv/B-557-entscheidung-sound-atlas.md) | REG | Frage | hoch | erledigt | KR4 | Entscheidung S4: Die Effekte laden aus einem Sound-Atlas, Einzeldateien sind der Rückfall |
| [B-558](archiv/B-558-entscheidung-sound-luecken.md) | REG | Frage | hoch | erledigt | KR4 | Entscheidung S5: Die Sound-Lücken bekommen vorhandene Kenney-Töne, UI-Klicks und Reiten bleiben offen |
| [B-559](archiv/B-559-frage-form-lernbare-skills.md) | SRV | Frage | hoch | erledigt | KR5 | Ralf bestätigt die Form der lernbaren Skills im Protokoll |
| [B-560](archiv/B-560-frage-dieb-preset-oder-bauen.md) | REG | Frage | hoch | erledigt | KR5 | Ralf entscheidet, ob die Dieb-Skills gebaut werden oder das Preset leer bleibt |
| [B-561](archiv/B-561-frage-skill-darstellung-je-klasse.md) | CLI | Frage | hoch | erledigt | KR5 | Ralf bestätigt die Darstellung der Skills als Ring je Klasse ohne neue Ereignisse |
| [B-154](archiv/B-154-protokoll-bosse-events-inselwechsel.md) | SRV | Idee | hoch | erledigt | K4 | Das Protokoll kennt Bosse, Phasen, Events und den Inselwechsel |
| [B-343](archiv/B-343-bau-vor-dem-ausgang.md) | SIM | Frage | mittel | erledigt | KR6 | Der Bau des Endbosses liegt an der inneren Kante des Ausgangs-Chunks |
| [B-386](archiv/B-386-netz-budget-kampf-q08.md) | SRV | Frage | mittel | erledigt | KR6 | Netz-Budget des Kampf-Zustands: Q08 gilt für alles oder nur für Ereignisse? |
| [B-328](archiv/B-328-feuergeist-flammen-flaeche.md) | SIM | Frage | niedrig | erledigt | KR6 | Der Feuergeist hinterlässt eine Flammen-Fläche |
| [B-465](archiv/B-465-lint-sprintsbacklog-zeilen.md) | DEV | Problem | hoch | erledigt | WZ26 | task lint ist auf develop rot, weil SprintsBacklog 61 Zeilen hat |
| [B-473](archiv/B-473-ci-rot-k3c-dev-planning-worktree-tests.md) | DEV | Problem | hoch | erledigt | WZ26 | CI auf develop rot: k3c-workbench internal/planning Worktree-Tests scheitern auf dem Windows-Runner |
| [B-474](archiv/B-474-testdownloadhit-norton-tls.md) | DEV | Problem | mittel | erledigt | WZ26 | TestDownloadHit scheitert lokal am TLS-Zertifikat (Norton), check:all rot |
| [B-519](archiv/B-519-plan-next-origin.md) | DEV | Problem | mittel | erledigt | WZ26 | plan_next berücksichtigt den Stand der Sprint-Branches auf origin |
| [B-520](archiv/B-520-check-run-blockiert.md) | DEV | Problem | hoch | erledigt | WZ26 | check_run wartet bis zum Ende des Laufs und meldet Fortschritt statt Lauf-ID zum Nachfragen |
| [B-496](archiv/B-496-studio-fensterposition-merken.md) | DEV | Idee | mittel | erledigt | WZ26 | Das Asset-Studio merkt sich Fensterposition und Größe und stellt sie sicher wieder her |
| [B-418](archiv/B-418-spielstand-kommentare-veraltet.md) | SIM | Problem | niedrig | erledigt | W14 | Kommentare in island_save_v3.go und island_save_v5.go nennen nach W7.2 veraltete Versionen |
| [B-566](archiv/B-566-lobby-welt-auswahl.md) | CLI | Idee | mittel | erledigt | BD8 | Lobby bietet beim Anlegen eines Spielstands eine Welt-Auswahl |
| [B-573](archiv/B-573-musik-niederlage-stueck.md) | CLI | Idee | mittel | erledigt | SO7 | Nach dem Fall der Burg spielt ein eigenes Niederlage-Stück |
| [B-574](archiv/B-574-sound-reiten-hufschlag.md) | CLI | Idee | niedrig | erledigt | SO7 | Reiten hat eine Hufschlag-Schleife je Spieler |
| [B-575](archiv/B-575-sound-ui-klicks.md) | CLI | Idee | niedrig | erledigt | SO7 | Die Menü-Szenen haben UI-Klick-Töne |
| [B-517](archiv/B-517-geplante-chunks-gegnerlager.md) | REG | Idee | mittel | erledigt | WZ27 | Geplante Chunks tragen Gegnerlager mit stationärer Wache |
| [B-522](archiv/B-522-ressourcen-auf-ruine.md) | SIM | Problem | mittel | erledigt | WZ27 | Zufalls-Ressourcen eines geplanten Chunks können auf seiner Ruine liegen |
| [B-523](archiv/B-523-ruine-kollision-hub-platz.md) | SIM | Schuld | niedrig | erledigt | WZ27 | Eine Ruine auf derselben Lage wie ein Hub-Platz gleicher Art wird nicht abgelehnt |
| [B-521](archiv/B-521-types-ts-aufteilen.md) | CLI | Schuld | niedrig | erledigt | WZ27 | src/model/types.ts liegt über der Dateigrenze von 400 Zeilen |
| [B-563](archiv/B-563-endboss-bau-in-aktionsliste.md) | SRV | Idee | niedrig | erledigt | KB3 | Die Aktionsliste nennt den Bau-Auslöser des Endbosses, das Overlay zeigt ihn |
| [B-564](archiv/B-564-dev-seite-spiel-mit-dev-parameter.md) | PLAT | Idee | niedrig | erledigt | KB3 | Die Entwicklerseite öffnet das Spiel mit Debug-Overlay (?dev=1) |
| [B-565](archiv/B-565-messung-wellenabstand-mit-adern.md) | REG | Schuld | niedrig | erledigt | KB3 | Der Wellenabstand unter Tage mit besetzten Adern ist über 100 Seeds gemessen |
| [B-569](archiv/B-569-gebaeude-eigene-stufen.md) | REG | Idee | mittel | erledigt | GEB13 | Gebäude bekommen eigene Stufen (levels je Gebäude) |
| [B-531](archiv/B-531-wald-dichter-hoehle-bueesche.md) | REG | Idee | mittel | erledigt | GEB14 | Wald hat mehr Bäume je Abschnitt, Höhle bekommt Deko-Büsche |
| [B-532](archiv/B-532-snapshot-markiert-von.md) | SIM | Idee | niedrig | erledigt | GEB14 | Snapshot nennt, wer einen Knoten markiert hat |
| [B-572](archiv/B-572-geruest-immer-bei-waitingworker.md) | CLI | Idee | mittel | erledigt | GEB14 | Baugerüst steht immer bei waitingWorker, auch bei Gefahr |
| [B-577](archiv/B-577-dieb-skills-bauen.md) | SIM | Idee | mittel | erledigt | SK3 | Der Dieb bekommt Backstab und Smoke Bomb |
| [B-578](archiv/B-578-skill-effekt-je-skill.md) | CLI | Idee | mittel | erledigt | SK3 | Jeder Skill hat einen eigenen Effekt statt des Rings je Klasse |
| [B-562](archiv/B-562-studio-monarch-skill-ring.md) | DEV | Idee | niedrig | erledigt | SK3 | Die Monarch-Seite des Studios zeigt den Skill-Ring des Spiels statt der Lücke |
| [B-539](archiv/B-539-entscheidung-wimpel-markierer.md) | SIM | Frage | hoch | erledigt | KR2 | Entscheidung D2: Wimpel-Farbe bleibt Client-Annäherung, markedBy erst nach GEB10.5 |
| [B-552](archiv/B-552-entscheidung-monarch-animationen.md) | REG | Frage | hoch | erledigt | KR3 | Entscheidung G7: Monarch-Animationen bleiben aus vorhandenen Frames abgeleitet |
| [B-505](archiv/B-505-monarch-animationen-wurf-treffer-tod-respawn.md) | CLI | Idee | niedrig | erledigt | GR9 | Monarch hat Animationen für Wurf, Treffer, Tod und Respawn |
| [B-251](archiv/B-251-figuren-ganzzahlig-skalieren.md) | CLI | Schuld | hoch | verworfen | GR7 | Figuren werden ganzzahlig skaliert und flimmern nicht |
| [B-593](archiv/B-593-figuren-kaefer-splitterwicht-spinne.md) | CLI | Problem | mittel | verworfen | – | Eisenkäfer, Splitterwicht und Kristallspinne brauchen passende Figuren |
| [B-193](archiv/B-193-figuren-luecken-suche.md) | CLI | Idee | hoch | erledigt | GR7 | Figuren-Lücken unter public/sprites/ haben Kandidaten und eine Auswahl |
| [B-329](archiv/B-329-figuren-neue-gegner.md) | CLI | Schuld | niedrig | erledigt | GR8 | Die sechs neuen Gegner zeigen eigene Figuren statt Platzhalter |
| [B-331](archiv/B-331-einheitlicher-grafikstil-pipeline.md) | CLI | Idee | mittel | erledigt | GR8 | Alle Grafiken laufen durch eine Pipeline mit Ziel-Palette und gleicher Pixeldichte |
| [B-320](archiv/B-320-reiter-sattel-beim-laufen.md) | CLI | Problem | hoch | erledigt | GR7 | Der Reiter sitzt beim Laufen und Sprinten auf dem Sattel, nicht auf der Kruppe |
| [B-554](archiv/B-554-entscheidung-musik-stuecke.md) | REG | Frage | hoch | erledigt | KR4 | Entscheidung S1: Je Musikzustand spielt die erste Präferenz des Slots, die übrigen Kandidaten bleiben bis zur Bestätigung liegen |
| [B-168](archiv/B-168-musik-je-zustand.md) | CLI | Idee | hoch | erledigt | SO4 | Die Musik wechselt je Spielzustand mit Crossfade |
| [B-492](archiv/B-492-sfx-luecken-weitere-ereignisse.md) | CLI | Idee | mittel | erledigt | SO6 | Weitere Ereignisse bekommen eigene Sound-Slots und Sounds |
| [B-493](archiv/B-493-credits-seite-audio.md) | PLAT | Idee | hoch | erledigt | SO6 | Die Credits-Seite zeigt auch die Sounds unter public/audio |
| [B-494](archiv/B-494-sound-atlas-werkzeug.md) | INF | Idee | niedrig | erledigt | SO6 | Ein Werkzeug packt die gewählten Sounds in einen Sound-Atlas |
| [B-011](archiv/B-011-sound.md) | CLI | Idee | mittel | erledigt | SO2 | Spiel hat Sound und Musik |
| [B-218](archiv/B-218-optionen-ambient-lautstaerke.md) | CLI | Idee | hoch | erledigt | SO4 | Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses |
| [B-250](archiv/B-250-audiokern-datei-wiedergabe.md) | CLI | Schuld | hoch | erledigt | SO4 | Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn |
| [B-503](archiv/B-503-monarch-skill-darstellung.md) | CLI | Idee | niedrig | erledigt | SK2 | Die 24 Monarch-Skills haben eine Darstellung im Spiel |
| [B-504](archiv/B-504-preset-skills-backstab-smokebomb.md) | REG | Problem | niedrig | erledigt | SK2 | Die Skills backstab und smokeBomb des Presets Dieb stehen in data/monarch.json |
| [B-007](archiv/B-007-skill-baum.md) | SIM | Idee | hoch | erledigt | – | Skill-Baum mit Tank und Zauberer ist spielbar |
| [B-105](archiv/B-105-anlegen-dialog-optionen.md) | CLI | Idee | mittel | erledigt | K5 | Der Anlegen-Dialog der Lobby wählt Grad, Ziel und Niederlage-Modus |
| [B-107](archiv/B-107-debug-panel-gradwechsel.md) | CLI | Idee | mittel | erledigt | K5 | Ein Debug-Panel im Dev-Mode wechselt den Schwierigkeitsgrad und weitere Optionen |
| [B-098](archiv/B-098-debug-overlay-standard-zurueck.md) | CLI | Schuld | niedrig | erledigt | K5 | Das Debug-Overlay ist vor dem Release wieder nur mit ?dev=1 verfügbar |
| [B-607](archiv/B-607-studio-levelview-test-golden-dichter-wald.md) | DEV | Problem | hoch | verworfen | – | Der Studio-Test levelView erwartet die Baum-Zahl vor dem dichten Wald |
| [B-606](archiv/B-606-studio-tests-rot-nach-dichtem-wald.md) | DEV | Problem | hoch | erledigt | WZ28 | Die Studio-Tests sind nach dem dichten Wald wieder grün und check_run hängt nicht bei k3c-studio:test |
| [B-470](archiv/B-470-morgengrauen-als-phase.md) | SIM | Idee | niedrig | erledigt | W12 | Der Tageszyklus hat eine eigene Phase Morgengrauen (Q65, 3:1:2:1) |
| [B-379](archiv/B-379-vollmond-belohnung-nur-stufe-0.md) | SIM | Problem | mittel | erledigt | KB1 | Die Vollmond-Belohnung gilt für die Stufen, in denen gespielt wird |
| [B-380](archiv/B-380-blutmond-boss-faehigkeiten.md) | SIM | Problem | niedrig | erledigt | KB1 | Der Blutmond verstärkt auch die Flächenangriffe der Bosse |
| [B-327](archiv/B-327-golden-tiefe-stufen.md) | SIM | Frage | niedrig | erledigt | KB1 | Golden-Läufe decken Eisenstollen und Kristallhöhle ab |
| [B-419](archiv/B-419-golden-daten-verkleinern.md) | INF | Idee | niedrig | erledigt | GD1 | Golden-Daten werden gezielt verkleinert, ohne die Prüfschärfe zu verlieren |
| [B-620](archiv/B-620-golden-grenze-20-mb.md) | INF | Frage | mittel | erledigt | – | Golden-Grenze 20 MB nach dem dichteren Wald bestätigen |
