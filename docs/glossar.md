# Glossar

Verbindliche Begriffe für Regeln, Tickets, Sprints, Sessions und Code-Kommentare (Stand 2026-10-04, feste Bauplätze nach Q43–Q59, Begriffe und Tageszyklus nach Q60–Q65, Verlust-Kaskade nach Q66–Q69).
Ein neuer Begriff wird **hier zuerst eingetragen**, bevor ihn ein Ticket, eine Session oder eine Regel benutzt. Bei Widerspruch gilt das
Regelwerk in [`rules/`](rules/), und das Glossar wird angepasst. Zahlen sind, wo nicht anders vermerkt, **Startwerte** (gelten bis zur Prüfung mit B-099).

| Begriff | Bedeutung | Quelle |
|---|---|---|
| Abbaurate | Zielrate einer Ader bei 2 Bauern: Stein 60, Kupfer 45, Eisen 35, Kristall 25 je Minute (Startwerte). Sie steuert den Materialfluss, nicht die Fundmenge. | `rules/materialien-gebaeude.md` § 1 |
| Abend | Kein eigener Begriff: Der Musik-Zustand „Abend“ (Q16) ist die Dämmerung. | Q16, Q65 |
| AC (Akzeptanzkriterium) | Prüfbares Kriterium einer Spec mit stabiler ID `AC-01`, `AC-02` … (lückenlos, nie umnummeriert). Sprints verweisen auf Ticket-Kriterien als `B-009/AC-01`, Sessions nennen sie im Feld `Kriterien`. | `arbeitsweise.md` › SDD |
| Ader | Unendliche Quelle für Stein, Kupfer, Eisen oder Kristall, 2 je Stufe. Wird einmal markiert (`markCost`), höchstens 2 Bauern gleichzeitig; zählt nicht für „Alles abbauen“. | `rules/materialien-gebaeude.md` § 1, Q25; `data/economy.json` › `veins` |
| Aggressionspool | Wellen-Auslöser unter Tage: +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource; bei 100 % kommt eine Welle, danach zurück auf 0. | `rules/stufen.md` § 2, `rules/gegner.md` § 3 |
| Aktionen-Overlay | Anzeige der gerade gültigen Aktion am Ort in der Welt mit passender Taste (z. B. „A halten: Bauen“), passend zum zuletzt benutzten Gerät. | `rules/monarch.md` § 4 |
| Ampel | Zustand eines Raums auf der Monitoring-Seite aus den letzten 5 min: rot bei Absturz, Tick p99 > 33,3 ms, RTT > 250 ms oder kein Pong; gelb bei anderem Diagnose-Ereignis, Tick p99 > 16,7 ms oder RTT > 100 ms; grau ohne Daten; sonst grün (Schwellen angenommen). | B-282, `src/tools/monitorData.ts` |
| Angebot | Etwas, das ein Gebäude verkauft oder ausbildet (Bogen, Schwert, Beruf, Elite, Rüstung). Jedes Angebot hat ein eigenes Zahlziel, keine Auswahl per Taste. | Q34, `rules/materialien-gebaeude.md` § 3 |
| Angebots-Anhang (`dx`) | Zahlziel eines Angebots mit festem Abstand `dx` zum Gebäude (`data/buildings.json`); entsteht mit dem Bau, alle `dx` liegen frei. Beispiel: Schwert an der Werkstatt `dx +4`. | Q52, Q53 |
| Ausrüstung | Was einen Bauern zum Kämpfer oder Beruf macht: Bogen, Schwert (auch Elite, mit ihrer Stufe) und die Ausrüstung von Bergmann, Baumeister, Handwerker. Fällt bei 0 HP zu Boden; Bürger heben sie wieder auf (Abholauftrag wie am Waffenregal), ein Gegner trägt sie zum Portal (dann verloren) oder lässt sie bei seinem Tod fallen. | `rules/buerger.md` § 3, Q67, Q68, Q69 |
| Äußerste Sperre (äußerste Linie) | Die äußerste gebaute Mauerlinie einer Seite: Mauer, mit Tor bis zum Tor. Dort stehen Krieger, und nur dort ist ein neues Tor bezahlbar; innere Türme und Tore wirken weiter. | Q46, Q47, Q54 |
| Bau-Menü | Gibt es nicht: Gebaut wird nur an Bauplätzen durch Bezahlen, die Taste Y bleibt frei. | Q06, Q34 |
| Bauer | Rekrutierter Landstreicher (1 Gold): sammelt markierte Ressourcen, baut, repariert, holt Waffen, flieht bei Gefahr in die Burg. Zählt nicht zum Truppen-Limit. Bei 0 HP lässt er seine Münze fallen und wird Landstreicher; ein Kämpfer oder Beruf, der seine Ausrüstung verliert, wird wieder Bauer (volle HP). | `rules/buerger.md` §§ 1, 3, Q67 |
| Baumeister | Beruf (20 Gold): Bauer, der +50 % schneller baut und repariert. | `rules/buerger.md` §§ 1–2 |
| Bauplatz | Fester Ort, an dem genau ein Gebäude gebaut und am selben Ort ausgebaut wird; kein Platz bewegt sich. Klassen: Hub-Platz, Mauerlinie (Mauer, Turm), Tor-Platz, Farm-Weltplatz, Angebots-Anhang. | Q43, `rules/materialien-gebaeude.md` § 3; `data/hub.json` › `sites` |
| Bauzeit | Zeit, die ein Bauer nach dem Bezahlen baut (z. B. Hub-Ausbau 20/30/40/50 s); Baumeister sind schneller. | `rules/materialien-gebaeude.md` §§ 2–3, Q44, Q45 |
| Bergmann | Beruf (20 Gold): +50 % Abbaurate an Adern, Fels und Kupfererz; zählt als einer der höchstens 2 Bauern an einer Ader. | `rules/buerger.md` § 1, Q35 |
| Beruf | Spezialisierung eines Bauern gegen Gold: Bergmann, Baumeister, Handwerker. Umschulung kostet erneut. | `rules/buerger.md` § 1 |
| Biom | Thema und Eckdaten einer Stufe (Länge, Chunks, Ressourcen, Gegner, Zyklus) in `data/biomes/<biom>.json`: heute `forest`, `cave`, `mine`; geplant `ironhold`, `crystal`. | `game-design.md` › Welt & Stufen, `rules/stufen.md` § 1 |
| Bogenschütze | Kämpfer: Bauer plus Bogen aus der Werkstatt. Posten auf einem Turm oder hinter der äußersten Mauer (`outerWall`), Fernkampf. | `rules/buerger.md` § 1 |
| Börse | Das Gold, das ein Spieler trägt (Beutel, höchstens 100); beim Verkaufen an den Händler fließt Gold hinein. | `rules/wirtschaft.md` § 1 (`purse`), W4.2 |
| Bot-Eingabe (`BotInput`) | Eingabe-Quelle im Client für Testläufe: Die Aktionen der lokalen Spieler kommen aus dem Bot-Feed der Workbench (`?botfeed=…`); der Client entscheidet nichts. | B-349 |
| Bot-Feed | WebSocket der Workbench je Testlauf, über den Bots die Kommandos je Slot an einen Client schicken; nur Loopback. | B-348, B-349 |
| Burg | Hub-Kern in der Hub-Mitte und Basislager; Zahlziel des Hub-Ausbaus, Ort für Respawn und Respec. Fällt sie, wirkt der Niederlage-Modus. | `rules/materialien-gebaeude.md` §§ 2–3, `rules/stufen.md` § 4 |
| Bürger | Alle Figuren des Hubs, die kein Spieler steuert: Landstreicher, Bauer, Berufe, Kämpfer (Truppen), Händler. Kein Level, keine Skills. Regeltexte sagen „Bürger“, wo alle Figuren gemeint sind. Bürger sterben nicht, siehe Verlust-Kaskade. | `rules/buerger.md`, Q63, Q67 |
| Camp | Rekrutierungs-Camp in der Welt mit höchstens 2 Landstreichern (Nachwuchs 25 s); darf innerhalb der Mauerlinien liegen. | `rules/wirtschaft.md` § 1, Q57 |
| Cheat-Dialog | Modaler Dialog im Spiel mit den wichtigsten Dev-Aktionen für Tester; hält den Raum an, solange er offen ist. Aufruf: Ä, LB + RB 3 s, Doppeltap mit zwei Fingern. | B-231 |
| Chunk | Abschnitt eines Levels, 50 Units breit; der Generator reiht Chunks links und rechts vom Hub nach `chunkWeights`. | `game-design.md` › Prozedurale Generierung |
| Couch-Koop | Mehrere Spieler an einem Gerät mit eigener Eingabe und Split-Screen; mit Online-Spielern im selben Raum mischbar. | `game-design.md` › Koop |
| Couch-Raum | Raum, in dem alle Spieler an einem Gerät sitzen; dort hält die Pause den ganzen Raum an. | `rules/bedienung.md` § 1, Q01 |
| Dämmerung (`dusk`) | Phase des Tageszyklus zwischen Tag und Nacht, Startwert 2 min (heute 1 min, bis B-213); Ereignis `dusk` = „Nacht naht“. Der Musik-Zustand „Abend“ (Q16) ist die Dämmerung. | `rules/wirtschaft.md` § 3, `engine/sim/cycle.go`, Q65 |
| `dawn` | Ereignis bei Beginn des Morgengrauens (heute bis B-213: Beginn des Tages); siehe Tagesanbruch. | `engine/sim/cycle.go`, Q65 |
| Delta | Nachricht `delta`: nur die Änderungen zum vorigen Tick; Gegenstück zum vollen `snap`. | `protocol.md` › Nachrichten |
| Determinismus | Gleicher Seed und gleiche Eingaben ergeben dasselbe Ergebnis. Zufall nur über `engine/rng` (`rng.New(seed)`), nie `math/rand` oder `Math.random()`. | `CLAUDE.md` › Regeln |
| Dev-Mode | Entwicklungsmodus des Servers (`K3C_DEV`): erlaubt den Grad Dev, das Debug-Panel und Dev-Aktionen. | `rules/wirtschaft.md` § 4, `protocol.md` |
| Diagnose | Debug-Anzeige oben links (Raum, Takt, Snapshot, Puffer, Latenz, FPS, Version), nur lesend. Aufruf: Ö, RB 3 s, Doppeltap mit einem Finger. | B-093, B-231 |
| Diagnose-Ereignis | Eintrag der Fehler-Zeitleiste des Servers (`/api/metrics` › `events`): Absturz, 🐢-Tick, Trennung oder Client-Fehler mit Zeit, Raum und Art; die letzten 200. Nicht das Ereignis der Simulation. | B-281, `engine/room/monitor.go` |
| `disarmed` | Ereignis (geplant): Ein Bürger verliert seine Ausrüstung (Felder `kind`, `x`, `cause`); ersetzt `troopLost`. | `rules/buerger.md` § 3, Q69 |
| Domäne | Fachbereich, dem eine Session genau zugeordnet ist und dessen Dateien sie ändert: REG (Regelwerk), SIM (Spiel-Logik Go), SRV (Server), CLI (Client), PLAT (Plattform), INF (Tooling, Arbeitsweise), DEV (Entwickler-Werkzeug: `tools/k3c-dev/`, `cmd/k3c-load/`, `cmd/k3c-tui/`, `src/tools/`). Ein Sprint nennt die Domänen seiner Sessions; eine Session `in Arbeit` sperrt ihre Domäne für parallele Läufe. | `arbeitsweise.md` › Domänen, B-356 |
| Dungeon-Master-Seite | Responsive Seite unter `/dm` für Handy und Tablet mit Live-Anpassungen und Diagnose laufender Räume (geplant). | B-232 |
| Ebene | Spätere Variante: mehrere Inseln je Schwierigkeits-Ebene in freier Reihenfolge, die nächste Ebene öffnet nach k besiegten Inseln. Nicht der Schwierigkeitsgrad. | `rules/stufen.md` § 1 |
| Einschiebbar | Entfallenes Sprint-Feld (B-174, entfallen mit B-361, wie die Sprint-`Prio`). Die Reihenfolge kommt aus dem Rang der Projekte. | `arbeitsweise.md` › Sprint-Lebenslauf |
| Einzelwechsel | Ein Spieler wechselt allein die Stufe (2 s am Tiefen-Eingang oder an einer Treppe); es gibt keine gemeinsame Reise zwischen Stufen. | `rules/stufen.md` § 1 |
| Eisen | Material der Hub-Stufe 4 aus Adern im Eisenstollen; für Eisenmauer, Eisenturm, Rüstkammer, Rüstung. | `rules/materialien-gebaeude.md` § 1 |
| Eisenstollen | Stufe der Tiefe 3 (Biom `ironhold`, geplant) mit Eisen-Adern, Lava und 3 Portalen. | `rules/stufen.md` § 1, Q28 |
| Elite | (1) Elite-Gegner: stärkerer Gegner einer Stufe mit genau einer Fähigkeit. (2) Elite-Bogenschütze/-Krieger: Kämpfer nach Upgrade in der Schmiede. | `rules/gegner.md` § 1, `rules/buerger.md` § 1 |
| Endboss | Boss einer Insel in seinem Bau, sitzt immer in der tiefsten Stufe (bis W2 die Mine, danach die Kristallhöhle); ausgelöst bei Ankunft eines Spielers, sein Sieg öffnet den Inselwechsel. | `rules/bosse.md` § 1, `rules/stufen.md` § 3, Q60 |
| `equipmentTaken` | Ereignis (geplant): Ein Gegner hat Ausrüstung zum Portal getragen, sie ist verloren. | `rules/buerger.md` § 3, `rules/gegner.md` § 4, Q68, Q69 |
| Ereignis (Event) | Meldung der Simulation im letzten Tick einer Stufe (`events` in `snap`/`delta`), z. B. `hit`, `kill`, `built`, `playerDown`, `revive` (Respawn), `revived` (Wiederbeleben, kommt mit W4.1/W5), `disarmed` und `equipmentTaken` (geplant, Q69); höchstens 32 je Tick und Stufe. | `protocol.md` › Ereignisse, `engine/sim/events.go`, Q62 |
| Extrapolation | Läuft die Zeitleiste leer, laufen Figuren mit ihrer letzten Bewegung höchstens 100 ms weiter (angenommen) und bleiben dann stehen. | B-277 |
| Farm | Gebäude der Hub-Stufe 1 auf einem festen Farm-Weltplatz je Seite zwischen Linie 1 und 2 (±52); wirkt als Plantage. | `rules/materialien-gebaeude.md` § 3, Q51 |
| Fixture | Kleiner, aus dem Code erzeugter Spielstand je Version unter `testdata/saves/v<n>/`; alte bleiben unverändert, `TestJedeVersionHatFixture` verlangt eines je Version. | `arbeitsweise.md` › Spielstand-Format |
| Freifläche | Bildschirmbereich, den kein HUD-Element belegen darf: Home-Button, „☰ Optionen“, Touch-Knöpfe und der Bereich der Diagnose. Die Layout-Funktion spart ihn aus. | B-337, `src/scenes/hudLayout.ts` |
| Freigabe | Ausdrückliche Zustimmung von 🧑 zu genau einer Revision einer Spec (Feld `Freigabe`: Datum und Quelle); erst dann `Spec: freigegeben`. | `arbeitsweise.md` › SDD |
| Getrennter Monarch (`Player.Free`) | Monarch, dessen Gerät die Verbindung verloren hat: 60 s reserviert (`waiting`), danach frei (`free`); unverwundbar, ausgeblendet, nicht wiederbelebbar. | `rules/bedienung.md` § 3, Q04, Q33 |
| Gold | Währung je Spieler (Start 100, höchstens 100, morgens +5); bezahlt Bauten, Rekrutierung, Markierung. Gehört dem Spieler, ist kein Material. | `rules/wirtschaft.md` § 1 |
| Golden (Golden-Daten) | Referenzergebnisse von Simulation und Level-Generator in `testdata/golden/`; Update nur bei gewollter Regeländerung per `task golden:update` mit Begründung. | `arbeitsweise.md` › Golden aktualisieren |
| Grabstein | Gefallener Monarch, der am Ort liegen bleibt, bis er wiederbelebt wird oder respawnt. | `rules/monarch.md` § 5 |
| Händler | Besucher je Insel im Hub der Tiefe 0 (Hub-Platz +8/+12): kommt bei `dawn` alle 3 Tage (mit Taverne alle 2), bleibt einen Tag, tauscht 10 Material = 5 Gold an „Kaufen“ und „Verkaufen“. | `rules/buerger.md` § 1, Q36, Q55 |
| Handwerker | Beruf (30 Gold) in Werkstatt, Schmiede oder Rüstkammer (1–2 je Gebäude): +50 % Herstellungstempo je Handwerker. | `rules/buerger.md` § 1, Q35 |
| Heilplatz | Gebäude der Hub-Stufe 3: heilt Truppen (Kämpfer) und Spieler im Radius 6 mit 5 HP/s, immer, auch im Kampf. Sonst gibt es keine Regeneration. | `rules/materialien-gebaeude.md` § 3.2, Q32 |
| Heimbereich | Bereich um die Hub-Mitte mit Radius `homeRadiusUnits` (12 Units), in dem Figuren ohne Auftrag wandern. | `data/hub.json`, `engine/sim/units.go`, Fragenkatalog Q38 |
| Höhle | Stufe der Tiefe 1 (Biom `cave`) mit Stein-Adern; Wellen über den Aggressionspool. | `game-design.md` › Welt & Stufen |
| Holz | Material der Hub-Stufe 1 aus Bäumen im Wald und aus Farm-Plantagen; Grundgebäude, Holzmauer und -turm, Bogen und Schwert. | `rules/materialien-gebaeude.md` § 1 |
| Home-Kombi | View + Menu gemeinsam halten (Tastatur Pos1) = zurück zur Landingpage; auf keiner Seite anders belegt. | `CLAUDE.md` › Seiten & Navigation |
| Hub | Basis einer Stufe mit Burg, eigenen Bauplätzen, Bürgern und Hub-Stufe; jede Stufe hat einen eigenen Hub, der von Grund auf gebaut wird. | `rules/stufen.md` § 1 |
| Hub-Ausbau | Erhöhen der Hub-Stufe auf 2–5, bezahlt an der Burg mit Gold plus Material der neuen Stufe (z. B. 100 Stein + 50 Gold); ein Bauer baut. | `rules/materialien-gebaeude.md` § 2, Q44 |
| Hub-Mitte | Seed-abhängige Mitte des Hubs mit der Burg; Hub-Plätze und Mauerlinien sind Offsets von ihr. | `rules/materialien-gebaeude.md` § 3 |
| Hub-Platz | Bauplatz mit festem Offset zur Hub-Mitte (`data/hub.json`) für Werkstatt, Lager, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer, Treppen (+16/+24) und Händler (+8/+12); streut nicht. | Q43, Q55 |
| Hub-Stufe | Ausbaustufe 1–5 eines Hubs (Holz bis Kristall); Stufe n schaltet Linie n, Mauer- und Turm-Stufe n und die Gebäude der Stufe frei. Code: `World.HubLevel` (ab W0). | `rules/materialien-gebaeude.md` § 2, Q59 |
| HUD-Element | Bildschirmfeste Anzeige mit Anker (z. B. oben links in der Zelle), Rang und schaltbarem Hintergrund und Rahmen. Elemente überlagern sich nie; passt eines nicht, fällt es nach Rang weg, Rang 0 (Pflicht) nie. Weltgebundene Anzeigen sind keine HUD-Elemente. | B-337, `src/scenes/hudLayout.ts` |
| Insel | Teil eines Spielstands, Sammlung ihrer n Stufen (je Stufe ein Level) mit einem gemeinsamen Material-Vorrat und einem Endboss; heute Insel 1 mit 5 Stufen, davon 3 gebaut. | `rules/stufen.md` § 1, `decisions/003-spielstruktur-inseln-stufen.md`, Q61 |
| Insel-Vorrat | Gemeinsames Baumaterial aller Stufen und Hubs einer Insel (`World.stock`); Kapazität je Rohstoff 300 je Hub plus 300 je Lager. | `rules/materialien-gebaeude.md` § 1 |
| Inselwechsel | Gemeinsamer Wechsel aller lebenden Spieler zur nächsten Insel (Boot oder Portal) nach dem Sieg über den Endboss; noch nicht gebaut. | `rules/stufen.md` § 1, B-103 |
| Kampagne | Das gesamte Spiel eines Spielstands bis zum Ziel des Raums (Standard: Endboss der letzten Insel). | `rules/stufen.md` § 3 |
| Kämpfer | Bürger, die kämpfen und zum Truppen-Limit zählen: Bogenschützen und Krieger, auch Elite. Gleichbedeutend mit Truppe. | `rules/buerger.md` § 3, Q63 |
| Kaserne | Gebäude der Hub-Stufe 2: Truppen-Limit +10. | `rules/materialien-gebaeude.md` § 3.2 |
| `kind@x` | Schlüssel aus Art und Position, mit dem der Spielstand Plätze und Level-Objekte zuordnet; bleibt stabil, weil kein Platz wandert. | Q42, B-201, B-202 |
| Krieger | Kämpfer: Bauer plus Schwert aus der Werkstatt, Nahkampf; Posten hinter der äußersten gebauten Sperre. | `rules/buerger.md` § 1, Q46 |
| Kristall | Material der Hub-Stufe 5 aus Adern in der Kristallhöhle; für Kristallmauer und Zaubertum. | `rules/materialien-gebaeude.md` § 1 |
| Kristallhöhle | Stufe der Tiefe 4 (Biom `crystal`, geplant) mit Kristall-Adern, Lava und 3 Portalen. | `rules/stufen.md` § 1, Q28 |
| Kupfer | Material der Hub-Stufe 3 aus Adern in der Mine und endlichem Kupfererz; für Kupfermauer und -turm, Schmiede, Heilplatz. | `rules/materialien-gebaeude.md` § 1 |
| Lager | Gebäude der Hub-Stufe 2: +300 Kapazität je Rohstoff für die Insel; Arbeiter bringen Material hierher oder zur Burg. | `rules/materialien-gebaeude.md` § 3.2 |
| Landingpage | `index.html`: bleibt dauerhaft offen und zeigt alle anderen Seiten im Vollflächen-iframe, damit Vollbild auf der Xbox erhalten bleibt. | `CLAUDE.md` › Seiten & Navigation |
| Landstreicher | Nicht rekrutierte Figur aus Camp oder Taverne; eine Münze (1 Gold) macht ihn zum Bauern. Wird nicht angegriffen; ein Bauer, der seine Münze verliert, wird Landstreicher und läuft zum Camp. | `rules/buerger.md` §§ 1, 3, Q67 |
| Latenz | Zeit von einer gesendeten Eingabe (`seq`) bis zum ersten Zustand mit `ack` ≥ `seq`, Mittel und p95 über 60 s. Diagnose-Zeile „Latenz 62 ms (p95 110 ms)“, ohne Messung „Latenz –“. | B-181, `src/online/clientLatency.ts` |
| Lauf-Modus | Kombination eines Testlaufs aus `mode` (`offline` mit Mocks im Prozess, `online` gegen den Spielserver) und `clients` (0 = headless, 1–4 = laufende Clients mit Bot-Eingabe). | B-348 |
| Lava | Boden der tiefen Stufen: Figuren darauf erleiden 5 Schaden/s; ob Gegner betroffen sind, ist offen. | `rules/stufen.md` § 1, Q28 |
| Level | Eine Stufe: aus Biom-Daten und Seed prozedural erzeugte Welt (der Generator erzeugt je Stufe ein Level); gespeichert wird nur der Seed. Die Insel ist die Sammlung ihrer Stufen. | `game-design.md` › Prozedurale Generierung, `rules/stufen.md` § 1, Q61 |
| Linie | Kurz für Mauerlinie; „Linie k“ ist die k-te Linie einer Seite von innen. | Q48, Q49 |
| Markieren (`markCost`) | Auftrag an Bauern, eine Ressource abzubauen, gegen Gold (Baum, Fels, Erz 1–2 Gold); Adern einmalig mit `markCost` (Stein 1, Kupfer 2, Eisen 2, Kristall 2). Plantage-Bäume ohne Markierung. | `rules/materialien-gebaeude.md` § 1, Q25 |
| Material | Baumaterial der Insel: Holz, Stein, Kupfer, Eisen, Kristall, eines je Hub-Stufe. Gold ist kein Material. | `rules/materialien-gebaeude.md` § 1 |
| Material-Stufe | Stufe 1–5 einer Mauer oder eines Turms nach Material (Holz bis Kristall); Ausbau am selben Platz. Bei Zerstörung geht sie verloren (neu ab Holz). | `rules/materialien-gebaeude.md` §§ 3.1, 4 |
| Mauer | Bau auf dem Mauer-Platz einer Linie; blockiert Gegner außer `ignoresWalls`; Material-Stufen 1–5. | `rules/materialien-gebaeude.md` §§ 3.1–3.2 |
| Mauerlinie | Je Seite 5 feste Linien bei ±44/64/84/104/124 Units mit Mauer-Platz, Turm-Platz (8 innen) und Tor-Platz (4 außen). Linie k ist ab Hub-Stufe k bezahlbar, wenn die Mauer der Linie k−1 derselben Seite steht. | `rules/materialien-gebaeude.md` § 3, Q48, Q49, Q58 |
| Messreihe | Verlauf einer Kennzahl im Server (Tick-Dauer je Raum, RTT und Warteschlange je Gerät, Heap, CPU …): ein Punkt je Sekunde in einem Ring-Puffer fester Größe (3600 = 1 h), älteste Punkte fallen raus. Ausgabe über `/api/metrics`. | B-281, `engine/room/monitor.go` |
| Mine | Stufe der Tiefe 2 (Biom `mine`) mit Kupfer-Adern und Kupfererz; Wellen über den Aggressionspool. | `game-design.md` › Welt & Stufen |
| Miniboss | Boss je Stufe; kommt mit Welle 5 (Wald) bzw. Welle 3 (unten) und kehrt nach dem Sieg nie zurück. | `rules/bosse.md` § 1 |
| Monarch | Figur eines Spielers (Index 0–3), immer beritten; kämpft mit Schlag und Skills, seine Hauptrolle ist das Management der Bürger. | `protocol.md` › Begriffe, `rules/monarch.md` |
| Monitoring-Seite | Seite `monitor.html` (Kachel „Monitor“) mit Ampel je Raum, Verläufen der Messreihen mit Perzentilen und der Fehler-Zeitleiste aus `/api/metrics`; rechnet nur Statistik, keine Spiel-Logik. | B-282 |
| Morgengrauen | Phase des Tageszyklus zwischen Nacht und Tag, Startwert 2 min; beginnt mit `dawn` (Nacht endet, Gegner ziehen ab). Noch nicht gebaut, Code-Name legt B-213 fest. | `rules/wirtschaft.md` § 3, Q65, B-213 |
| Münze | Ein Gold als Gegenstand: per A-Halten bezahlt (alle 0,25 s, Reichweite 2 Units); fallen gelassen nach 1,5 s für andere aufhebbar. | `rules/wirtschaft.md` § 1 |
| Nacht | Phase des Tageszyklus zwischen Dämmerung und Morgengrauen, Startwert 4 min (heute 5 min, bis B-213); in der Oberwelt kommt eine Welle je Nacht. | `rules/wirtschaft.md` § 3, Q65 |
| Niederlage-Modus | Raum-Option für den Fall einer Burg: Gold/Material-Verlust, Stufenverlust oder Komplett verloren (`defeat`: `resources`, `stage`, `lost`). | `rules/stufen.md` § 4 |
| Oberwelt | Die Stufe der Tiefe 0, der Wald. | `rules/stufen.md` § 1 |
| Pixeldichte | Größe eines Quell-Pixels im Spiel, gemessen am 16-px-Raster (Q13). Assets mit gröberen oder feineren Pixeln wirken fremd, auch bei gleicher Palette. | B-331 |
| Plantage | Wirkung der Farm: 6 Plätze, je Platz alle 30 s ein Baum mit 10 Holz, ohne Markierung; zählt nicht für „Alles abbauen“. | `rules/materialien-gebaeude.md` § 1, Q25 |
| Platz-Stufe | Gespeicherte Stufe eines Bauplatzes im Spielstand v3 (`SiteSave`). | B-202, Q42 |
| Portal | Ausgangspunkt der Gegnerwellen: 2 je Stufe, ab Tiefe 3 drei; mindestens 150 Units vom Hub und außerhalb der Linie 5 inklusive Streuung. | `rules/gegner.md` § 3, Q49, Q57 |
| Posten | Standplatz eines Kämpfers: Bogenschützen auf dem Turm oder hinter der äußersten Mauer, Krieger hinter der äußersten gebauten Sperre. Posten sind keine Bauplätze. | `rules/buerger.md` § 1, Q46 |
| Preset | Startverteilung der Skill-Punkte beim Beitritt (Tank, Zauberer, Heiler, Dieb); keine feste Klasse. | `rules/monarch.md` § 2 |
| Prio | Dringlichkeit eines Tickets (`hoch`, `mittel`, `niedrig`, `?`); ordnet Tickets innerhalb eines Projekts beim Einplanen. Die Reihenfolge der Sprints kommt aus dem Rang. | `arbeitsweise.md` › Projekte und Rang |
| Projekt | Thema über mehrere Sprints (z. B. Grafik, Sound, Leistung & Stabilität) mit Kürzel aus drei Großbuchstaben, Status `aktiv`, `ruht` oder `erledigt` und Rang; hält seine Sprints in fester Reihenfolge. Ebenen: Projekt → Sprint → Session. | `arbeitsweise.md` › Projekte und Rang, B-355 |
| Protokoll | Nachrichten zwischen Gerät und Server über WebSocket `/ws`, heute Version 3 (`hello.v` = 3). Eine Änderung bekommt eine eigene Session für beide Enden. | `protocol.md`, `arbeitsweise.md` › Grenzfälle |
| Puffer (Verzögerung) | Zeit, um die die Zeitleiste hinter der geschätzten Server-Zeit zeichnet: 1 Tick plus die doppelte Ankunfts-Schwankung, höchstens 150 ms (angenommen). Diagnose-Zeile „Puffer 33 ms“. | B-277 |
| Rang | Reihenfolge der aktiven Projekte (1 = zuerst, lückenlos), von 🧑 gesetzt; bestimmt, welcher Sprint und welche Session als Nächstes drankommt. | `arbeitsweise.md` › Projekte und Rang, B-355 |
| Raum | Ein laufendes Spiel auf dem Server mit genau einem Spielstand und einem Code aus 4 Buchstaben; tickt unabhängig von anderen Räumen. | `protocol.md` › Begriffe |
| Raum-Option | Beim Anlegen gewählte Einstellung des Raums: Schwierigkeitsgrad, Ziel (Siegvariante), Niederlage-Modus; steht im Spielstand. | `rules/stufen.md` § 5 |
| Reife | Sprint-Feld: `Entwurf` (Sessions als Stichpunkte) oder `bereit` (jede Session als Datei, jedes Kriterium hat eine Session). | `arbeitsweise.md` › Sprint-Lebenslauf |
| Reittier | Standard-Pferd (`horse`), das jeder Monarch immer reitet; Tempo- und Sprintfaktor 1,0, kein Auf- und Absteigen. | `rules/monarch.md` § 7 |
| Respawn | Rückkehr eines gefallenen Monarchen an der Burg seiner Stufe mit voller HP nach der Wartezeit, wenn niemand ihn wiederbelebt: heute 5 s, Ziel 15 s mit B-120/W4.1. Ereignis `revive`. | `rules/monarch.md` §§ 1, 5, Q62 |
| Respec | Kostenloses Umverteilen der Skill-Punkte an der Burg jedes Hubs, nur am Tag. | `rules/monarch.md` § 3 |
| Ressource (Gatherable) | Endliches, abbaubares Level-Objekt: Baum, Fels, Kupfererz (Code `ResourceNode`, Arten in `economy.Gatherables`). Adern und Plantagen sind keine Gatherables. | `rules/wirtschaft.md` § 1, W2.1; `data/economy.json` › `gatherables` |
| Resurrection | Ultimate-Skill des Heilers (50 % HP, CD 180 s): belebt nur gefallene Monarchen, keine Bürger. | `game-design.md` › Skill-Tabelle, `rules/monarch.md` § 6, Q66 |
| Review-Session | Letzte Session eines Code-Sprints: `task check` und `task check:go`, nur den Diff lesen, nur schwere Befunde, Abnahme, PR des Sprints. | `arbeitsweise.md` › Review-Session |
| Revision | Zähler der Spec-Fassung; jede Änderung erhöht ihn und setzt die Spec auf `Entwurf` zurück. | `arbeitsweise.md` › SDD |
| `revive` | Ereignis: Monarch steht nach der Wartezeit an der Burg wieder (Respawn). | `protocol.md` › Ereignisse, Q62 |
| `revived` | Ereignis (geplant, B-120/W4.1, Protokoll mit W5): Monarch wurde von einem Mitspieler wiederbelebt. | Q62, B-120 |
| RTT (Ping) | Umlaufzeit eines WebSocket-Pings vom Server zum Gerät und zurück, vom Server einmal je Sekunde gemessen (Messreihe je Gerät). Nicht die Latenz (die misst der Client über `ack`). | B-281, `engine/net/ping.go` |
| Rüstkammer | Gebäude der Hub-Stufe 4: Rüstungsstufen für alle Kämpfer. | `rules/materialien-gebaeude.md` § 3.2 |
| Rüstungsstufe | Upgrade in der Rüstkammer, 2 Stufen (ab Hub-Stufe 4 und 5): +20 % bzw. +40 % Basis-HP für alle Kämpfer, sofort, ohne Vollheilung. | `rules/buerger.md` § 2, Q37 |
| Schlag | Einfacher Nahkampfangriff des Monarchen (X bzw. E): 10 Schaden, Reichweite 1,5 Units, Abklingzeit 0,7 s. | `rules/monarch.md` § 1 |
| Schmiede | Gebäude der Hub-Stufe 3: Elite-Upgrades für Bogenschützen und Krieger. | `rules/materialien-gebaeude.md` § 3.2 |
| Schwierigkeitsgrad | Raum-Option Dev, Leicht, Normal, Hart, Ultra (`grade`); ändert nur Wellen und Gegner, nicht die Wirtschaft. Dev nur im Dev-Mode. | `rules/wirtschaft.md` § 4 |
| SDD | Spec-Driven Development: Ticket und Sprint-README sind die Spec mit Kriterien, Sessions erfüllen genannte Kriterien. | `arbeitsweise.md` › SDD |
| Seed | Zahl, aus der der Generator ein Level deterministisch erzeugt; gespeichert wird nur der Seed. | `game-design.md` › Prozedurale Generierung |
| Session | Arbeitsschritt eines Sprints als Datei nach Vorlage mit genau einem Commit in genau einer Domäne; Typ Umsetzung, Review oder Workshop, Agent autonom oder Mensch; Status `offen`, `in Arbeit`, `fertig`, `blockiert` oder `verworfen`. | `arbeitsweise.md`, `vorlagen/session.md` |
| Shell | Seiten-Rahmen aus `src/core/shell.ts`: Landingpage mit iframe, auf jeder anderen Seite `installPageChrome()` mit Home-Button, Home-Kombi und Zurück-Falle für B. | `CLAUDE.md` › Seiten & Navigation |
| Siegvariante | Ziel des Raums (`goal`): Endboss (Standard), Gold sammeln, Tage überleben, Alles abbauen, Alles ausbauen. | `rules/stufen.md` § 3 |
| Sim (Engine) | Die Spiel-Logik in Go (`engine/sim/`, `engine/level/`), rechnet deterministisch auf dem Server; der Browser zeichnet nur. | `decisions/001-server-engine-go.md`, `CLAUDE.md` |
| Skill | Aktive oder passive Fähigkeit des Monarchen aus vier Linien (Tank, Zauberer, Heiler, Dieb) mit Tier 1–4; vier Skill-Slots. | `rules/monarch.md` §§ 3–4, `game-design.md` |
| Skill-Pool | Fund-Pool der Skill-Punkte je Insel: Ein gefundener Punkt zählt für jeden Spieler, jeder verteilt für sich (`World.SkillPoints`). | `rules/monarch.md` § 3 |
| Slot | Lokaler Spieler eines Geräts (Index 0–3). Daneben: Skill-Slot, eine der vier Skill-Tasten. | `protocol.md` › Begriffe, `rules/monarch.md` § 4 |
| Snapshot | Zustand der Welt, den der Server an ein Gerät schickt: voll als `snap`, danach als `delta`; der Client zeichnet nur ihn. | `protocol.md` › Nachrichten |
| Spec | Anforderung eines Tickets oder Sprints (README) mit Akzeptanzkriterien; Status `Entwurf`, `freigegeben` oder `rückwirkend`. | `arbeitsweise.md` › SDD |
| Spielstand | Gespeicherter Zustand eines Raums (`saves/<name>.json`) mit Seeds, Hubs, Vorrat und Raum-Optionen; Gegner und Level-Layout werden nicht gespeichert. | `protocol.md` › Begriffe, `game-design.md` › Speichern |
| Spielstand-Version | `IslandSaveVersion` (`engine/sim/island_save.go`), heute 3 (Fund-Pool `skillPool`, je Spieler `skills` und `slots`, S1). Jede Formatänderung erhöht sie und bringt eine Fixture; Hub- und Platz-Stufe kommen mit W1.3 optional in v3 dazu (Q42). | `arbeitsweise.md` › Spielstand-Format, Q42 |
| Split-Screen | Geteilter Bildschirm für 1–4 lokale Spieler, jeder mit eigener Kamera (`src/scenes/layout.ts`). | `game-design.md` › Koop, `rules/bedienung.md` § 2 |
| Sprint | 3–6 Sessions eines Projekts, auch aus mehreren Domänen nacheinander, auf einem Branch `sprint/<präfix>` mit einem PR; Ordner unter `docs/sprints/` (`geplant/`, `aktiv/`, `erledigt/`). | `arbeitsweise.md` |
| Standardszenario | Messrahmen der Zielkorridore: Insel 1, Wald-Start, Normal, 2 Spieler, Bot „sparsam“, 100 Seeds. | `rules/zielkorridore.md` |
| Startvorrat | 100 Holz im Insel-Vorrat jeder neuen Insel (`islandStartStock`), auch nach dem Inselwechsel. | `rules/materialien-gebaeude.md` § 1, `rules/stufen.md` § 1, B-177 |
| Startwert | Vorläufiger Zahlenwert aus Beschluss oder Vorschlag; gilt, bis der Balancing-Tester (B-099) ihn bestätigt oder ändert. | Kopf jeder Datei in `rules/` |
| Stein | Material der Hub-Stufe 2 aus Adern in der Höhle und endlichen Felsen; für Steinmauer und -turm, Tor, Kaserne, Lager, Taverne, Treppen. | `rules/materialien-gebaeude.md` § 1 |
| Streuung | Seed-abhängige Verschiebung der Linien 2–5 nur nach außen um 0 bis +4 ganze Units, je Seite und Linie, über einen eigenen RNG-Strom (`…:sites`). Linie 1 und Hub-Plätze streuen nicht. | Q56 (präzisiert Q50) |
| Stufe | Ort einer Insel in einer Tiefe (0 Wald bis 4 Kristallhöhle) mit eigenem Hub, eigenen Wellen und Biom; pro Spieler frei begehbar, alle laufen weiter. Nicht verwechseln mit Hub-Stufe oder Material-Stufe. | `rules/stufen.md` § 1 |
| Tag | Erste Phase des Tageszyklus, Startwert 6 min (heute Wald 10 min, bis B-213); Respec nur am Tag. | `rules/wirtschaft.md` § 3, Q65 |
| Tagesanbruch (`dawn`) | Ende der Nacht bei Beginn des Morgengrauens (heute bis B-213: Beginn des Tages): Ereignis `dawn` mit +5 Gold je lebendem Spieler, Taverne und Händler, Autosave; Gegner ziehen ab (fliehen nur in der Oberwelt). | `rules/wirtschaft.md` §§ 1, 3, `rules/gegner.md` § 3, Q10, Q65 |
| Tageszyklus | Globaler Zyklus aus Tag, Dämmerung, Nacht, Morgengrauen im Verhältnis 3:1:2:1, Startwert 6/2/4/2 min (14 min); läuft in allen Stufen. Heute 10/1/5 min ohne Morgengrauen, Umbau mit B-213. | `rules/wirtschaft.md` § 3, `data/biomes/forest.json` › `cycle`, `engine/sim/cycle.go`, Q65 |
| Taverne | Gebäude der Hub-Stufe 2: bei jedem `dawn` ein Landstreicher, solange dort weniger als 2 stehen. | `rules/materialien-gebaeude.md` § 3.2, Q30 |
| Testlauf (`sim_test`) | Lauf über das MCP-Tool `sim_test` der Workbench für Balancing, Performance oder Stabilität, mit Lauf-ID, Status und Bericht unter `reports/`; jeder autonome Testlauf geht darüber. | B-348 |
| Tick (Takt) | Ein Rechenschritt eines Raums; ein Raum tickt mit 30 Hz und schickt je Tick einen Zustand an jedes Gerät. | `protocol.md` › Nachrichten |
| Ticket | Idee, Problem, Schuld oder Frage als Datei `docs/backlog/B-NNN-name.md` nach Vorlage, mit eigener Spec. | `arbeitsweise.md` › Ablage |
| Tiefe | Index einer Stufe auf der Insel (0 = Wald); Gegner skalieren je Tiefe (`depthScaling`), im Protokoll `depth`. | `rules/stufen.md` § 1, `protocol.md` |
| Tiefen-Eingang | Eingang am Ende einer Stufe zur nächsttieferen (2 s stehen); nach oben geht es nur über Treppen. | `rules/stufen.md` § 1, `game-design.md` |
| Tor | Bau auf dem Tor-Platz einer Linie (Mauer +4 außen), ab Hub-Stufe 2: eigene Bürger und Spieler passieren, Gegner nicht. Bezahlbar nur an der äußersten gebauten Linie („wandert“); ein inneres Tor bleibt stehen. | Q27, Q47, Q54 |
| Treppe | Hub-Platz (+16/+24) ab Hub-Stufe 2, verbindet mit der Stufe darüber bzw. darunter. | `rules/materialien-gebaeude.md` § 3.2, Q55 |
| `troopLost` | Entfällt (Q69): Bürger sterben nicht; ersetzt durch `disarmed` und `equipmentTaken`. Früher geplant für fallende Kämpfer (Q39, Q64). | `rules/buerger.md` § 3, Q69 |
| Truhe | Level-Objekt mit 10–25 Gold; jede 3. gefundene Truhe der Insel gibt einen Skill-Punkt. | `rules/wirtschaft.md` § 1, `rules/monarch.md` § 3 |
| Truppe | Kämpfer: Bogenschütze, Krieger, Elite. Ausnahme Code: `World.Troops` und `data/troops.json` umfassen alle Bürger; die Namen bleiben. | `rules/buerger.md`, Q63 |
| Truppen-Limit | Höchstzahl Kämpfer je Hub: 10, mit Kaserne 20; geprüft beim Waffe-Holen, bei vollem Limit bleibt die Waffe im Regal. | `rules/buerger.md` § 3, Q29 |
| Turm | Bau auf dem Turm-Platz einer Linie (8 Units innen): Posten für 2 Bogenschützen, Material-Stufen 1–5, Stufe 5 ist der Zaubertum. | `rules/materialien-gebaeude.md` § 3.1, Q45 |
| Umgebung | Feld an Ticket und Session: `offline` = ohne laufende Dienste prüfbar (Code, Unit-/Mock-Tests, Werkzeuge ohne Serverzugriff), `live` = braucht laufenden Server, Browser oder Gerät, `?` = noch nicht eingeordnet. Nur `autonom` · `offline` läuft in einem Worktree. | `arbeitsweise.md` › Ablage, `vorlagen/` |
| Unit | Längeneinheit der Welt: 1 Unit = `UNIT_PX` = 32 px (`src/core/constants.ts`). | `CLAUDE.md` › Regeln |
| Verlust-Kaskade (Rückstufung) | Statt Tod: Ein Treffer auf 0 HP kostet einen Bürger die nächste Schicht. Mit Ausrüstung → fällt zu Boden, er wird Bauer (volle HP, nicht mehr im Truppen-Limit); Bauer → Münze fällt, er wird Landstreicher; Landstreicher werden nicht angegriffen. Wie im Vorbild Kingdom Two Crowns. | `rules/buerger.md` § 3, Q67 |
| Vorhersage | Anzeige-Vorhersage des eigenen Monarchen: Nur seine x-Position läuft im Client sofort mit der Eingabe und wird weich zum Server-Zustand (plus Vorlauf um die Latenz) zurückgeführt. Keine Spiel-Logik; fremde Figuren werden nicht vorhergesagt. | B-039, B-277, `src/online/clientPredict.ts` |
| Vorlage | Pflicht-Kopiervorlage für Ticket, Sprint und Session in `docs/vorlagen/`; `tests/planning.test.ts` prüft sie. | `arbeitsweise.md` › Ablage |
| Wald | Stufe der Tiefe 0 (Oberwelt, Biom `forest`) mit Bäumen und Tag-Nacht-Zyklus; eine Welle je Nacht. | `game-design.md` › Welt & Stufen |
| Welle | Gruppe Gegner aus den Portalen: in der Oberwelt eine je Nacht, unten bei 100 % Aggressionspool. Größe nach Tabelle, Spieleranzahl der Insel und Grad; Zähler je Stufe. | `rules/gegner.md` § 3 |
| Weltposition | Eine x-Stelle der Welt in Units. Je Weltposition zeigt die Welt höchstens **ein** Anzeige-Element (Hinweis des Aktionen-Overlays, Preisschild, Meldung); treffen mehrere zusammen, gilt nur das wichtigste bzw. nächste. | `rules/monarch.md` § 4, Beschluss 🧑 2026-10-06 (S3.4) |
| Werkstatt | Gebäude der Hub-Stufe 1: Bogen und Schwert (je bis 3 im Waffenregal), Ausbildung von Bergmann und Baumeister. | `rules/materialien-gebaeude.md` § 3.2, `rules/buerger.md` § 2 |
| Wiederbeleben | Ein Mitspieler hält 3 s A neben dem Grabstein (Reichweite 2 Units): Der Monarch steht am Ort mit 50 % HP auf. Ereignis `revived` (nicht `revive`). | `rules/monarch.md` § 5, Q33, Q62 |
| Workshop | Session-Typ (meist `Agent: Mensch`), in dem 🧑 Regeln oder Werte beschließt (z. B. R2.2, F1.4). | `vorlagen/session.md`, `rules/` (Köpfe) |
| Zahlziel | Ort, an dem Münzen per A-Halten etwas bezahlen: Bauplatz, Burg (Hub-Ausbau), Angebots-Anhang, Händler „Kaufen“/„Verkaufen“. Zahlziele halten ≥ 4 Units Abstand zueinander. | Q34, Q44, Q57, W0 › AC-04 |
| Zaubertum | Turm der Material-Stufe 5: Flächenschaden (40 Schaden, Radius 3, Reichweite 13, alle 1,5 s) statt Bogen; die Schützen steigen ab. | `rules/materialien-gebaeude.md` § 3.1, Q31 |
| Zeitleiste | Puffer der empfangenen Zustände im Client: gezeichnet wird zur geschätzten Server-Zeit minus der Verzögerung, zwischen zwei Zuständen interpoliert; Stufenwechsel leert sie. | B-277, `src/online/clientTimeline.ts` |
| Ziel-Palette | Feste Farbliste, auf die jede Grafik umgerechnet wird; festgelegt in der Stil-Bibel. | B-331 |
| Zielkorridor | Kennzahl mit Unter- und Obergrenze im Standardszenario; Pass/Fail für das Balancing. | `rules/zielkorridore.md` |

## Unklar und Widersprüche

Keine (geklärt 2026-10-04, Q60–Q69 im [Fragenkatalog](fragenkatalog.md)). Offen ohne Widerspruch: Ereignis für „Bauer verliert Münze“ (Q69).
