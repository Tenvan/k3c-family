# Backlog

Alle Tickets, eine Zeile pro Ticket. Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md).
Jedes Ticket ist eine Spec (SDD, siehe [`../arbeitsweise.md`](../arbeitsweise.md)). Neues Ticket: nächste freie Nummer,
Datei `B-NNN-kurzname.md` aus der Vorlage, `Spec: Entwurf`, Zeile hier ergänzen. `task test` prüft beides.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-275](B-275-worktree-unter-claude.md) | SRV | Problem | hoch | eingeplant | M9 | k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig |
| [B-007](B-007-skill-baum.md) | SIM | Idee | hoch | eingeplant | SK1 | Skill-Baum mit Tank und Zauberer ist spielbar |
| [B-008](B-008-spieleabend.md) | REG | Frage | hoch | eingeplant | P1 | Familie hat einen Spieleabend gespielt und Feedback gegeben |
| [B-011](B-011-sound.md) | CLI | Idee | mittel | eingeplant | SO1 | Spiel hat Sound und Musik |
| [B-015](B-015-gebaeude-werte.md) | REG | Problem | mittel | eingeplant | BR1 | Gebäude-HP und -Kosten sind gebalanced |
| [B-019](B-019-test-abdeckung.md) | INF | Idee | niedrig | eingeplant | CI1 | Test-Abdeckung der Engine ist sichtbar |
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | eingeplant | PB1 | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | eingeplant | RG2 | Tiefe 3 und 4 sind beschrieben |
| [B-037](B-037-lobby.md) | CLI | Idee | mittel | eingeplant | LB1 | Lobby zeigt Räume und startet Spiele |
| [B-040](B-040-server-finden.md) | SRV | Idee | niedrig | eingeplant | BT1 | Geräte finden den Server im Heimnetz |
| [B-041](B-041-wails-starter.md) | SRV | Idee | niedrig | eingeplant | BT1 | Wails-Starter für Windows existiert |
| [B-042](B-042-pi-leistungsziel.md) | SRV | Frage | hoch | eingeplant | LT1 | Pi-Modell und Leistungsziel sind festgelegt |
| [B-048](B-048-standardbibliothek-in-001.md) | SRV | Frage | niedrig | eingeplant | BT1 | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist |
| [B-053](B-053-ci-lauf-sp01.md) | INF | Problem | hoch | eingeplant | CI1 | Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen |
| [B-058](B-058-execution-policy.md) | INF | Frage | niedrig | eingeplant | RP1 | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 |
| [B-075](B-075-golden-spielstand-hub.md) | SIM | Schuld | mittel | eingeplant | W7 | Der Golden-Spielstand enthält einen gebauten und veränderten Hub |
| [B-080](B-080-dev-tasten-server.md) | SRV | Idee | niedrig | eingeplant | K4 | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
| [B-090](B-090-radar.md) | CLI | Idee | mittel | eingeplant | U1 | Ein Radar im HUD zeigt Burg, Portale, Ausgang, Mitspieler und Gegner |
| [B-092](B-092-level-betrachter.md) | PLAT | Idee | mittel | eingeplant | U3 | Eine Testseite zeigt ein generiertes Level (Seed und Biom) ohne zu spielen |
| [B-094](B-094-npm-reste.md) | INF | Schuld | niedrig | eingeplant | RP1 | Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr |
| [B-095](B-095-start-mit-seed-und-tiefe.md) | SRV | Idee | niedrig | eingeplant | BT1 | Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe |
| [B-098](B-098-debug-overlay-standard-zurueck.md) | CLI | Schuld | niedrig | eingeplant | K5 | Das Debug-Overlay ist vor dem Release wieder nur mit ?dev=1 verfügbar |
| [B-099](B-099-balancing-tester.md) | SIM | Idee | mittel | eingeplant | BAL1 | Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele |
| [B-102](B-102-siegvarianten-niederlage.md) | SIM | Idee | mittel | eingeplant | K2 | Siegvarianten und Niederlage-Modi der Raum-Optionen sind umgesetzt |
| [B-103](B-103-inseln-bosse.md) | SIM | Idee | mittel | eingeplant | K2 | Inseln mit Endboss und gemeinsamem Inselwechsel sind spielbar |
| [B-105](B-105-anlegen-dialog-optionen.md) | CLI | Idee | mittel | eingeplant | K5 | Der Anlegen-Dialog der Lobby wählt Grad, Ziel und Niederlage-Modus |
| [B-107](B-107-debug-panel-gradwechsel.md) | CLI | Idee | mittel | eingeplant | K5 | Ein Debug-Panel im Dev-Mode wechselt den Schwierigkeitsgrad und weitere Optionen |
| [B-117](B-117-anzeige-bau-lager.md) | CLI | Idee | mittel | eingeplant | W6 | Der Client zeigt Wartezeit, Lagerstand, Hub-Stufe, Adern und Plantage |
| [B-126](B-126-buerger-ui.md) | CLI | Idee | mittel | eingeplant | W6 | Der Client zeigt Berufe, Ausbildung, Händler, Truppen-Limit und Heilung |
| [B-130](B-130-bosse.md) | SIM | Idee | hoch | eingeplant | K2 | Minibosse und Endboss sind spielbar |
| [B-131](B-131-events.md) | SIM | Idee | niedrig | eingeplant | K3 | Vollmond, Blutmond und Händler-Überfall sind als Events umgesetzt |
| [B-132](B-132-anzeige-bosse-events.md) | CLI | Idee | mittel | eingeplant | K5 | Der Client zeigt Gegner-Fähigkeiten, Bosse, Phasen und Events |
| [B-151](B-151-spieleabend-fragebogen.md) | REG | Idee | mittel | eingeplant | P1 | Der Spieleabend hat einen kindgerechten Fragebogen und eine Playtest-Vorlage |
| [B-154](B-154-protokoll-bosse-events-inselwechsel.md) | SRV | Idee | hoch | eingeplant | K4 | Das Protokoll kennt Bosse, Phasen, Events und den Inselwechsel |
| [B-155](B-155-balancing-runde-wirtschaft.md) | REG | Idee | hoch | eingeplant | BR1 | Die Wirtschaft ist in einer Balancing-Runde gegen die Zielkorridore abgestimmt |
| [B-156](B-156-balancing-runde-kampf-bosse.md) | REG | Idee | hoch | eingeplant | BR2 | Kampf, Gegner und Bosse sind in einer Balancing-Runde gegen die Zielkorridore abgestimmt |
| [B-160](B-160-abgleich-spielmetrik-simulator.md) | REG | Idee | mittel | eingeplant | BAL4 | Spielmetrik echter Abende und Simulatorwerte sind abgeglichen |
| [B-167](B-167-sfx-katalog.md) | CLI | Idee | mittel | eingeplant | SO2 | Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz |
| [B-168](B-168-musik-je-zustand.md) | CLI | Idee | mittel | eingeplant | SO4 | Die Musik wechselt je Spielzustand mit Crossfade |
| [B-250](B-250-audiokern-datei-wiedergabe.md) | CLI | Schuld | niedrig | eingeplant | SO5 | Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn |
| [B-270](B-270-respec-pruefung-ohne-seiteneffekt.md) | SIM | Schuld | mittel | eingeplant | SK1 | Die Sim prüft Respec und Lernen ohne Seiteneffekt |
| [B-272](B-272-rotation-session-reports.md) | SRV | Schuld | mittel | eingeplant | ST1 | Die Rotation in reports/ erfasst auch die Spielmetrik-Reports |
| [B-260](B-260-schutzplatz-ohne-id.md) | SIM | Schuld | niedrig | eingeplant | LV1 | Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID |
| [B-262](B-262-camps-nahe-portalen.md) | SIM | Frage | mittel | eingeplant | LV1 | Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen |
| [B-263](B-263-snapshot-groesse-plaetze.md) | SRV | Problem | niedrig | eingeplant | NT1 | Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget |
| [B-251](B-251-figuren-ganzzahlig-skalieren.md) | CLI | Schuld | niedrig | eingeplant | GR7 | Figuren werden ganzzahlig skaliert und flimmern nicht |
| [B-184](B-184-pages-screenshots.md) | PLAT | Idee | niedrig | eingeplant | PG1 | Die Präsentationsseite zeigt echte Bilder aus dem Spiel |
| [B-185](B-185-verluste-je-welle-angleichen.md) | REG | Schuld | niedrig | eingeplant | RG2 | Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger |
| [B-186](B-186-autospeichern-takt.md) | SRV | Idee | mittel | eingeplant | ST1 | Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ |
| [B-187](B-187-speichern-windows-rename.md) | SRV | Problem | mittel | eingeplant | ST1 | Speichern übersteht unter Windows eine kurz gesperrte Zieldatei |
| [B-188](B-188-client-rohstoffe-eisen-kristall.md) | CLI | Schuld | niedrig | eingeplant | W8 | Der Client kennt alle fünf Rohstoffe des Servers |
| [B-189](B-189-toter-gegner-flieht-ins-portal.md) | SIM | Problem | mittel | eingeplant | LV1 | Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold fallen |
| [B-190](B-190-events-dropped-im-protokoll.md) | SRV | Problem | niedrig | eingeplant | NT1 | Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden |
| [B-191](B-191-debug-overlay-links-unten.md) | CLI | Problem | mittel | eingeplant | U5 | Debug-Overlay und Aktionsliste verdecken das HUD nicht |
| [B-192](B-192-aktionsliste-schliesst-mit-oe.md) | CLI | Problem | hoch | eingeplant | U5 | Die Dev-Aktionsliste schließt sich mit Ö |
| [B-193](B-193-figuren-luecken-suche.md) | CLI | Idee | mittel | eingeplant | GR7 | Figuren-Lücken unter public/sprites/ haben Kandidaten und eine Auswahl |
| [B-194](B-194-splitscreen-ruckelt-xbox.md) | CLI | Problem | hoch | eingeplant | PF1 | Der Split-Screen läuft auf der Xbox flüssig |
| [B-195](B-195-debug-overlay-xbox.md) | PLAT | Problem | mittel | eingeplant | PL1 | Das Debug-Overlay lässt sich auf der Xbox öffnen |
| [B-197](B-197-partner-zelle-schriftgroesse.md) | CLI | Frage | niedrig | eingeplant | GR7 | Die Schriftregel nennt eine Mindestgröße für die Mitspieler-Zelle |
| [B-198](B-198-platzhaltertext-schrift-katalog.md) | CLI | Schuld | niedrig | eingeplant | GR7 | Der Platzhaltertext einer ungeladenen Stufe liest seine Schrift aus dem Katalog |
| [B-157](archiv/B-157-zielkorridor-pruefung.md) | SIM | Idee | mittel | erledigt | BAL2 | Der Balancing-Tester prüft Zielkorridore und meldet Pass oder Fail je Kennzahl |
| [B-159](archiv/B-159-replay-repro-format.md) | SIM | Idee | mittel | erledigt | BAL1 | Ein Lauf ist als Datei aus Seed und Eingaben wiederholbar |
| [B-199](B-199-raum-fuenf-stufen.md) | SRV | Problem | mittel | eingeplant | SV1 | Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt |
| [B-200](B-200-aggressionspool-adern.md) | SIM | Problem | mittel | eingeplant | LV1 | Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor |
| [B-201](B-201-spielstand-wirtschaft.md) | SIM | Schuld | mittel | eingeplant | W7 | Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nach dem Laden wieder her |
| [B-202](B-202-spielstand-versionsfolge.md) | SIM | Problem | hoch | eingeplant | W7 | S1 und W1 teilen sich die Spielstand-Version 3 eindeutig |
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-204](B-204-test-raeume-sofort-schliessen.md) | SRV | Idee | niedrig | eingeplant | SV1 | Leere Test-Räume schließen sofort statt nach der Leer-Frist |
| [B-205](B-205-y-belegung-s3.md) | CLI | Problem | mittel | eingeplant | S8 | Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“ |
| [B-207](B-207-bauplaetze-anzeige.md) | CLI | Idee | mittel | eingeplant | W8 | Der Client zeigt freie und gesperrte Bauplätze mit Grund (ab Hub-Stufe n, Linie fehlt) |
| [B-208](B-208-protokoll-bauplaetze.md) | SRV | Idee | mittel | eingeplant | W5 | Das Protokoll trägt die Bauplätze des Layouts sowie Platz- und Hub-Stufe zum Client |
| [B-209](B-209-client-platz-arten.md) | CLI | Schuld | mittel | eingeplant | W8 | `src/model/data.ts` kennt alle Platz-Arten aus `hub.json` |
| [B-214](B-214-server-pause.md) | SRV | Idee | mittel | eingeplant | RM1 | Der Server pausiert den Raum im Couch-Raum und schützt den stehenden Monarchen online |
| [B-215](B-215-texte-eingabe-shell-tools.md) | PLAT | Schuld | niedrig | eingeplant | PL1 | Die Texte von Touch-Overlay, Shell und Werkzeug-Seiten kommen aus den zentralen Textdateien |
| [B-230](B-230-burg-haelt-nur-47-prozent.md) | REG | Problem | mittel | eingeplant | RG1 | Burg hält Nacht 1–5 nur in 47 % der Seeds (Bot saver), Ziel 75–90 %: Ursache klären |
| [B-217](B-217-ereignisse-built-playerdown-ort.md) | SIM | Schuld | niedrig | eingeplant | LV1 | Die Ereignisse `built` und `playerDown` tragen ihren Ort |
| [B-213](B-213-markdown-listen-haekchen.md) | SRV | Problem | niedrig | eingeplant | M9 | MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planungsseite |
| [B-219](B-219-doku-gating-und-schlag.md) | REG | Schuld | niedrig | eingeplant | RG2 | Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen |
| [B-218](B-218-optionen-ambient-lautstaerke.md) | CLI | Idee | niedrig | eingeplant | SO5 | Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses |
| [B-273](B-273-release-image-dev-mode-aus.md) | INF | Schuld | hoch | eingeplant | CI1 | Das Release-Image startet den Server ohne Dev-Mode |
| [B-274](B-274-testrestore-flackert-windows.md) | SRV | Problem | mittel | eingeplant | NT1 | TestRestore läuft unter Windows auch in task check:all stabil grün |
| [B-280](B-280-warteschlange-nicht-zustaende.md) | SRV | Problem | niedrig | eingeplant | NT1 | Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwischen Zuständen stehen |
| [B-284](B-284-lasttest-eingaben-flake.md) | SRV | Problem | niedrig | eingeplant | NT1 | TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läuft |
| [B-285](B-285-lernbare-skills-im-protokoll.md) | SRV | Problem | mittel | eingeplant | RM1 | Der Server nennt je Spieler die lernbaren Skills |
| [B-286](B-286-lasttest-tick-reihe-wackelt.md) | SRV | Problem | niedrig | eingeplant | NT1 | TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl |
| [B-287](B-287-hub-ausbau-beutel-maximum.md) | REG | Problem | mittel | eingeplant | RG1 | Hub-Stufe 4 und 5 sind mit dem Beutel-Maximum bezahlbar |
| [B-288](B-288-alte-spielstaende-ohne-ruecksicht.md) | INF | Schuld | mittel | eingeplant | RP1 | Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände |
| [B-289](B-289-aggressionspool-mit-adern.md) | REG | Problem | mittel | eingeplant | RG1 | Der Aggressionspool steigt mit Adern nicht zu schnell |
| [B-290](B-290-raum-fuenf-stufen.md) | SRV | Idee | mittel | eingeplant | SV1 | Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle |
| [B-291](B-291-lava-nicht-auf-mauerlinien.md) | SIM | Problem | mittel | eingeplant | LV1 | Lava liegt nicht auf den Mauerlinien |
| [B-292](B-292-neues-spiel-eindeutiger-name.md) | PLAT | Problem | hoch | eingeplant | LP1 | Die Kachel „Neues Spiel“ startet auch bei vorhandenem Spielstand familie |
| [B-293](B-293-spiel-im-menue-verlassen.md) | CLI | Idee | hoch | eingeplant | S8 | Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“ |
| [B-294](B-294-hinweis-glyph-muenze-nacht.md) | CLI | Frage | mittel | eingeplant | S8 | Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph |
| [B-295](B-295-handwerker-schmiede-ruestkammer.md) | REG | Frage | mittel | eingeplant | RG2 | Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden |
| [B-298](B-298-ressourcen-manager.md) | SRV | Idee | mittel | eingeplant | M10 | Ein ResourcenManager in k3c-dev ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu |
| [B-299](B-299-asset-vorschau-szenen.md) | PLAT | Idee | mittel | eingeplant | DBG4 | Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab |
| [B-300](B-300-mauern-zuerst-wie-sparsam.md) | SIM | Frage | mittel | eingeplant | BAL5 | Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ |
| [B-301](B-301-vary-ohne-wirkung.md) | SIM | Problem | niedrig | eingeplant | BAL5 | Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler |
| [B-302](B-302-aufraeumen-branches-worktrees.md) | INF | Schuld | mittel | eingeplant | RP1 | Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt |
| [B-312](B-312-wiederaufheben-begrenzen.md) | SIM | Problem | hoch | eingeplant | W7 | Sofortiges Wiederaufheben fallengelassener Ausrüstung macht die Burg bei passivem Spiel unverwundbar |
| [B-313](B-313-vermerk-wirkung-offen-test.md) | SIM | Frage | hoch | eingeplant | W7 | W4.3b kann den Vermerk „Wirkung offen“ nur mit einer Änderung an sites_test.go ersetzen |
| [B-314](B-314-controller-pruefungen-zurueckgestellt.md) | PLAT | Schuld | niedrig | eingeplant | HW1 | Alle Controller-Prüfungen sind gesammelt nachgeholt |
| [B-315](B-315-spielstand-voll-ausgebaut.md) | SRV | Idee | hoch | eingeplant | SV1 | Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut |
| [B-316](B-316-tastatur-zwei-spieler.md) | PLAT | Idee | hoch | eingeplant | PL1 | Zwei Spieler spielen an einer Tastatur im Split-Screen |
| [B-317](B-317-cheat-dialog-fokus-tastatur.md) | CLI | Problem | hoch | eingeplant | U5 | Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller bedienen |
| [B-318](B-318-schlag-skill-feedback.md) | CLI | Problem | mittel | eingeplant | S9 | Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam |
| [B-319](B-319-ein-hinweis-je-weltposition.md) | CLI | Problem | hoch | eingeplant | S9 | Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild |
| [B-320](B-320-reiter-sattel-beim-laufen.md) | CLI | Problem | mittel | eingeplant | GR7 | Der Reiter sitzt beim Laufen und Sprinten auf dem Sattel, nicht auf der Kruppe |
| [B-321](B-321-schlag-skill-ohne-ziel-ereignis.md) | SIM | Problem | mittel | eingeplant | SK1 | Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis |
| [B-322](B-322-texte-werkzeug-seiten.md) | PLAT | Schuld | niedrig | eingeplant | PL2 | Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien |
| [B-324](B-324-client-typen-gegnerdaten.md) | CLI | Schuld | niedrig | offen | – | Die Client-Typen der Gegner- und Wellendaten passen zu den JSON-Dateien |
| [B-327](B-327-golden-tiefe-stufen.md) | SIM | Frage | niedrig | offen | K1 | Golden-Läufe decken Eisenstollen und Kristallhöhle ab |
| [B-328](B-328-feuergeist-flammen-flaeche.md) | SIM | Frage | niedrig | offen | K1 | Der Feuergeist hinterlässt eine Flammen-Fläche |
| [B-329](B-329-figuren-neue-gegner.md) | CLI | Schuld | niedrig | offen | – | Die sechs neuen Gegner zeigen eigene Figuren statt Platzhalter |
| [B-331](B-331-einheitlicher-grafikstil-pipeline.md) | CLI | Idee | mittel | offen | – | Alle Grafiken laufen durch eine Pipeline mit Ziel-Palette und gleicher Pixeldichte |
| [B-333](B-333-pause-anzeige.md) | CLI | Idee | mittel | offen | – | Der Client zeigt einen angehaltenen Raum deutlich an und hält die Figuren-Animationen an |
| [B-334](B-334-performance-modus.md) | CLI | Idee | hoch | offen | – | Der Client misst Leistung in einem Performance-Modus automatisch und überträgt die Werte an den Server |
| [B-335](B-335-landingpage-spieler-entwicklung.md) | PLAT | Idee | hoch | eingeplant | LP1 | Die Landingpage zeigt nur Spieler-Kacheln, Entwicklungs-, Performance- und Balancing-Aufrufe liegen auf einer eigenen Entwicklerseite |
| [B-336](B-336-touch-optionen-schliessen.md) | CLI | Problem | hoch | offen | – | Die Optionen-Szene lässt sich per Touch vollständig bedienen und schließen, ohne vom Touch-Overlay verdeckt zu werden |
| [B-337](B-337-hud-elemente-ohne-ueberlagerung.md) | CLI | Idee | hoch | offen | – | Jede HUD-Anzeige ist ein eigenes Element mit optionalem Hintergrund und Rahmen, und HUD-Elemente überlagern sich nicht |
| [B-338](B-338-session-status-verworfen.md) | INF | Schuld | niedrig | offen | – | Sessions können den Status verworfen tragen |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/) (nur auf Nachfrage lesen); die Datei bleibt beim Verschieben
unverändert. Ändert ein Ticket seinen Status auf `erledigt` oder `verworfen`, wandert es per `git mv` dorthin und seine
Zeile in diesen Abschnitt.

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
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
| [B-046](archiv/B-046-dev-mcp.md) | SRV | Idee | mittel | erledigt | M1 | Entwickler-Werkzeug k3c-dev gibt Agenten über MCP verdichteten Zugriff auf Prüfungen und Logs |
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
| [B-062](archiv/B-062-dev-nutzungsstatistik.md) | SRV | Idee | mittel | erledigt | M2 | k3c-dev wertet MCP-Aufrufe über Sitzungen aus: Perzentile, Ausreißer und Zeitreihe |
| [B-063](archiv/B-063-dev-berichte-spielstaende.md) | SRV | Idee | mittel | erledigt | M2 | k3c-dev macht Xbox-Berichte und Spielstände für Agenten lesbar |
| [B-064](archiv/B-064-dev-oberflaeche-logs.md) | SRV | Idee | mittel | erledigt | M4 | k3c-dev hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs |
| [B-065](archiv/B-065-dev-mcp-seite.md) | SRV | Idee | mittel | erledigt | M5 | k3c-dev zeigt auf der MCP-Seite Server, Tools, Live-Monitore, Aufruf-Log und Statistik |
| [B-066](archiv/B-066-server-json-log.md) | SRV | Idee | mittel | erledigt | D1 | Der Go-Server schreibt sein Log als JSON nach logs/ |
| [B-067](archiv/B-067-dev-dienste.md) | SRV | Idee | mittel | erledigt | M3 | k3c-dev startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten |
| [B-068](archiv/B-068-dev-dienste-seite.md) | SRV | Idee | mittel | erledigt | M4 | k3c-dev zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level |
| [B-069](archiv/B-069-ci-k3c-dev.md) | INF | Problem | mittel | erledigt | – | Der CI-Job k3c-dev ist einmal grün gelaufen |
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
| [B-097](archiv/B-097-dienste-watch-modus.md) | SRV | Idee | niedrig | erledigt | – | k3c-dev startet den Go-Server bei Code-Änderungen von selbst neu |
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
| [B-171](archiv/B-171-dev-seiten-tasks-planung-git.md) | SRV | Idee | mittel | erledigt | M7 | k3c-dev zeigt Tasks, Planung und Git wie die Workbench der ErpApi |
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
| [B-210](archiv/B-210-planung-ueber-mcp.md) | SRV | Idee | hoch | erledigt | M8 | Agenten pflegen Tickets, Sprints und Sessions über MCP-Tools von k3c-dev |
| [B-211](archiv/B-211-planungsseite-react.md) | SRV | Schuld | mittel | erledigt | M8 | Die Planungsseite von k3c-dev ist eine React-Ansicht aus denselben Daten wie die MCP-Tools |
| [B-212](archiv/B-212-github-status-planung.md) | SRV | Idee | mittel | erledigt | M8 | k3c-dev zeigt PR, CI und Merge-Konflikte je Sprint aus GitHub |
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
