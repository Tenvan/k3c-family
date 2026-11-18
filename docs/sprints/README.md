# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Die Reihenfolge der Arbeit ist allein der Projekt-Rang: [`../projekte/README.md`](../projekte/README.md). Die Tabellen stehen nach Rang und in der Sprint-Reihenfolge des Projekts, ABN und ruhende Projekte zuletzt; je Projekt können mehrere Sprints aktiv sein.

## Aktiv

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|---|
| KS1 | REG | KSY | Kampfregeln K3C festschreiben (Portale, Angriff, Ausrüstung, Krone) | – | `aktiv/KS1-kampfregeln/` |
| KS2 | SIM | KSY | Portale zerstörbar, Stufe sicher, Kämpfer werden Bürger | – | `aktiv/KS2-portale-ausruestung/` |
| KS3 | SIM | KSY | Monarch: Gold statt HP, Krone fällt, Raub statt Burgfall | – | `aktiv/KS3-krone-gold/` |
| KS4 | SIM | KSY | Aktiver Angriff: Gruppen mit Schild und Auslöser, General mit Banner, Variante je Welt | – | `aktiv/KS4-angriff-gruppen-general/` |
| KS5 | CLI | KSY | Anzeige im Spiel: Portal-HP, Krone, Münzverlust, Schild, Banner, Auslöser | – | `aktiv/KS5-kampf-anzeige/` |
| KS6 | SIM, PAP | KSY | Kampf-Nachträge: Engine-Export für Zahlstellen und Kronen, Papierschnitt-Assets für Kampf | – | `aktiv/KS6-kampf-nachtraege/` |
| KS7 | SIM | KSY | Kampf-Feedback aus der Sichtprüfung: Auslöser am Pfahl vor dem Tor, Schild-Gestell an der Burg | – | `aktiv/KS7-kampf-abnahme-feedback/` |
| WZ60 | DEV | WZG | Workbench-Oberfläche bleibt bei langen Läufen stabil | – | `aktiv/WZ60-workbench-oberflaeche-speicher/` |
| FB3 | DEV | WZG | Live-Test-Feedback: Gebäudeplaner in der Workbench | – | `aktiv/FB3-feedback-workbench/` |
| WE1 | DEV | WZG | Workbench ruft die Engine wieder direkt auf, k3c-sim wird dünn | – | `aktiv/WE1-workbench-engine-inprocess/` |
| WE2 | DEV | WZG | Welten-Editor per MCP: Welt anlegen, Inseln und Tiefen setzen, Overrides und geplante Chunks | – | `aktiv/WE2-welten-mcp/` |
| WE3 | DEV | WZG | Neue Welten anlegen: Sengoku, Nil, Britannien, Polar (nach Welten-MCP B-954) | – | `aktiv/WE3-welten-entwuerfe/` |
| WE4 | SIM, CLI, DEV | WZG | Etappen und Seekarte: Inseln je Etappe frei wählbar, Querschnitt-Editor | – | `aktiv/WE4-etappen-seekarte/` |
| GS1 | REG, SIM, CLI, DEV | WZG | Geschichten je Welt und Insel in Kunstsprache | – | `aktiv/GS1-geschichten-je-welt/` |
| GS2 | PAP, CLI | WZG | Seekarte im Spiel als Papierschnitt-Diorama mit Schiffsfahrt | – | `aktiv/GS2-seekarte-papierschnitt/` |
| WE7 | PAP | WZG | Papierschnitt-Assets für Sengoku, Nil und Britannien | – | `aktiv/WE7-welten-assets/` |
| WE5 | SIM, DEV | WZG | Welt-Format-Nachträge: Endboss je Insel, Overrides für Events und Plätze, Skins, Welten-MCP | – | `aktiv/WE5-welt-format-nachtraege/` |
| WZ61 | DEV, INF | WZG | Workbench-Nachträge: schlanke Planungsdaten, alte EXE in bin/ aufräumen | – | `aktiv/WZ61-planungsdaten-schlank-bin-aufraeumen/` |
| WG1 | PAP | PAP | Weltbilder: großes Titelbild für die Weltauswahl, kleines für Spielstand und Pause | – | `aktiv/WG1-weltbilder/` |
| W15 | SIM | WRT | Fackeln als Spielobjekt mit Licht in der Nacht | – | `aktiv/W15-fackeln-spielobjekt/` |
| SR1 | CLI | SND | Ruhigerer Klang für Couch-Abende: weniger Piepen, langsamer, warme Stücke | – | `aktiv/SR1-sound-ruhig/` |
| RG2 | REG | BAL | Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4 | – | `aktiv/RG2-regelwerk-klaerungen/` |
| WI1 | SIM | WIR | Handwerker mit Hammer bauen, Bogner-Shop, Gold beim Holzfällen, Truhen ×10 mit Fontäne | – | `aktiv/WI1-handwerker-gold/` |
| RU47 | SIM | RSU | Abnahme-Stand Rust-Sim: ignorierte Tests und Go-Seite (Second Worker) | – | `aktiv/RU47-abnahme-stand-rust-sim/` |

## Offen am Gerät

Offene Mensch-Sessions aktiver Sprints nach Gerät (Zeile `Gerät:` im Kontext der Session). Die Workbench schreibt den Abschnitt bei jedem schreibenden `plan_*`-Aufruf neu; mit Freigaben und Entscheidungen: `plan_queue`.

### PC

- W15.3 · Abnahme am Gerät: Fackeln an Mauer und Turm nachts (Sprint W15, offen, Agent Mensch)
- WZ61.5 · Ralf räumt bin/ am PC auf (Sprint WZ61, offen, Agent Mensch)

## Geplant (nach Projekt-Rang)

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|---|
| GS3 | SIM, PAP, CLI, SRV | WZG | Andenken und Chronik abgeschlossener Inseln auf der Seekarte | – | bereit | `geplant/GS3-andenken-chronik/` |
| GS4 | REG, SIM, PAP, CLI | WZG | Flaschenpost am Strand mit Geschichtszeile und Markierung | – | bereit | `geplant/GS4-flaschenpost/` |
| WE6 | CLI, DEV | WZG | Welt-Format im Spiel und im Welten-Editor: Endboss-Bau je Biom, Skins, neue Felder | – | bereit | `geplant/WE6-welt-format-spiel-editor/` |
| WE8 | REG, SIM, CLI | WZG | Welten-Inhalte: Events Ninja-Überfall, Sandsturm, Nebel und Zuordnung in den Welten | – | bereit | `geplant/WE8-welten-events-zuordnung/` |
| BB1 | PAP | PAP | Assets nach Bildaufbau: Maßstab in Monarchenhöhen, Waldwand, Leitmotiv je Welt | – | Entwurf | `geplant/BB1-bildaufbau-assets/` |
| BB2 | CLI | PAP | Bildaufbau im Spiel: Gehlinie, Spiegel-Fluss, Weite, Licht und Tönung | – | Entwurf | `geplant/BB2-bildaufbau-spiel/` |
| SI7 | DEV | KMP | Welten-Editor wählt Wetter, Reittier und Figur je Welt | – | bereit | `geplant/SI7-welten-editor-kataloge/` |
| RU18 | SIM | RSU | Sim: Wirtschaft, Bau, Hub, Monarch, Skills | – | bereit | `geplant/RU18-sim-wirtschaft-bau-monarch/` |
| RU19 | SIM | RSU | Sim: Events, Geschichte, Dev-Eingriffe | – | bereit | `geplant/RU19-sim-events-geschichte-dev/` |
| RU22 | SIM | RSU | Sim-Abgleich: alle Sim- und Kampagnen-Golden bitgleich (Gate G3) | – | bereit | `geplant/RU22-sim-golden-bitgleich/` |
| RU24 | SRV | RSU | Raum Teil 2 und Spielstand-Format | – | bereit | `geplant/RU24-raum-teil-2-spielstand/` |
| RU25 | SIM | RSU | Balancing Teil 1: Bots | – | bereit | `geplant/RU25-balancing-teil-1-bots/` |
| RU27 | CLI | RSU | Lokales Spiel: Sitzung, Kampagne, Spielstände, Einstellungen | – | bereit | `geplant/RU27-lokales-spiel/` |
| RU28 | SIM | RSU | Balancing Teil 2: Messungen und Auswertung | – | bereit | `geplant/RU28-balancing-teil-2-messungen/` |
| RU29 | CLI | RSU | Testing-, Debug- und Cheat-Menüs im Spiel | – | bereit | `geplant/RU29-dev-werkzeuge-im-spiel/` |
| RU30 | CLI | RSU | Spielschleife Teil 2: echtes Spiel, Aufnahme-Modus, Start | – | bereit | `geplant/RU30-spielschleife-teil-2/` |
| RU32 | CLI | RSU | Ende-zu-Ende-Test und Bildvergleich gegen das Go-Spiel | – | bereit | `geplant/RU32-ende-zu-ende-test/` |
| RU35 | INF | RSU | Build, Paket und CI auf das Rust-Spiel umstellen | – | bereit | `geplant/RU35-build-paket-ci-umstellen/` |
| RU36 | CLI | RSU | Abnahme am PC: Rust-Spiel gegen Go-Spiel (Gate G4) | – | bereit | `geplant/RU36-abnahme-am-pc/` |
| RU31 | SIM | RSU | Balancing Teil 3: Einsichten, Berichte, Tuning | – | bereit | `geplant/RU31-balancing-teil-3-einsichten/` |
| RU33 | CLI | RSU | Läufe im Game-Client: Sim, Level, Welten und Hub, headless oder mit Fenster per Parameter | – | bereit | `geplant/RU33-laeufe-im-client/` |
| RU34 | DEV | RSU | Workbench: MCP-Tools für Sim, Level, Welten und Hub rufen die Spiel-EXE | – | bereit | `geplant/RU34-workbench-ruft-spiel-exe/` |
| RU37 | INF | RSU | Go-Spiel entfernen, Doku und Regeln nachziehen | – | bereit | `geplant/RU37-go-spiel-entfernen/` |
| RU40 | CLI, DEV | RSU | Balancing-Läufe im Game-Client, sim_test und Balancing der Workbench über die Spiel-EXE | – | bereit | `geplant/RU40-balancing-laeufe-im-client/` |
| RU41 | INF | RSU | Go-Engine entfernen, Entscheidung 011 abschließen | – | bereit | `geplant/RU41-go-engine-entfernen/` |
| RU43 | CLI | RSU | Optimierung nach Messung | – | Entwurf | `geplant/RU43-optimierung-nach-messung/` |
| RU49 | SRV | RSU | Smoke-Raum: Welt anlegen, Inselwechsel und Pause im Raum | – | bereit | `geplant/RU49-smoke-raum-welt-und-pause/` |
| RU48 | CLI | RSU | Smoke-Client: erstes lauffähiges Spiel vom Hauptmenü bis zur Niederlage | – | bereit | `geplant/RU48-smoke-client-erstes-lauffaehiges-spiel/` |
| RU50 | CLI | RSU | Lokales Spiel Teil 2: Spielstände, Aktionen, Entwickler-Rolle | – | bereit | `geplant/RU50-lokales-spiel-teil-2-spielstaende-aktionen/` |

Gesamtplan und Begründung: [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md). Offene Entscheidungen: [`../fragenkatalog.md`](../fragenkatalog.md).

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
| KB6 | Kampf-Nachträge: Wellenabstand unter Tage, stabiler Schutzplatz | `erledigt/KB6-kampf-nachtraege-wellen-schutz/` |
| ALT | Vorgeschichte vor der Sprint-Einteilung | `erledigt/ALT-vorgeschichte/` |
| SP00 | Arbeitsweise einführen | `erledigt/SP00-arbeitsweise/` |
| SP01 | Leitplanken + Go-Gerüst | `erledigt/SP01-leitplanken/` |
| L1 | Go-Verschachtelung als Tiefe prüfen (B-054) | `erledigt/L1-go-verschachtelung/` |
| L2 | Go-Tiefe wie TypeScript zählen (B-057) | `erledigt/L2-go-tiefe/` |
| L3 | Race-Detector für die nebenläufigen Go-Pakete (B-077) | `erledigt/L3-race/` |
| T1 | Testseite mit Szenarien und Mock-Spielern (B-081) | `erledigt/T1-testseite/` |
| SP02 | Protokoll v2 & Raummodell, Entscheidung 002 | `erledigt/SP02-protokoll/` |
| M1 | k3c-workbench I: MCP-Kern über HTTP | `erledigt/M1-dev-mcp/` |
| M2 | k3c-workbench II: Nutzungsstatistik, Berichte, Spielstände | `erledigt/M2-dev-statistik/` |
| M3 | k3c-workbench III: Dienste führen, `svc_*`-Tools | `erledigt/M3-dev-dienste/` |
| M4 | k3c-workbench IV: Oberfläche (Wails) mit Dienste- und Logs-Seite | `erledigt/M4-dev-oberflaeche/` |
| SP03 | Go-Server Basis: Auslieferung, Spielstände mit Sicherungen, Status, Docker | `erledigt/SP03-go-server/` |
| M5 | k3c-workbench V: MCP-Seite mit Monitoren und Statistik | `erledigt/M5-dev-mcp-seite/` |
| SP04 | Golden-Tests, RNG und Level-Generator in Go | `erledigt/SP04-golden-level/` |
| SP05 | Port I: Welt, Zyklus, Truppen, Wirtschaft in Go | `erledigt/SP05-port-welt/` |
| SP06 | Port II: Gegner, Wellen, Reisen, Kampagne, Spielstand in Go | `erledigt/SP06-port-einheiten/` |
| SP07 | Räume & WebSocket (Protokoll v2) in Go | `erledigt/SP07-raeume/` |
| SP08 | Browser als reiner Client (Protokoll v2, Lobby, lokale Spieler, Layout 1–4) | `erledigt/SP08-client/` |
| SP10 | Diagnose-TUI (Bubble Tea): `k3c-tui` zeigt Räume live, Aktionen, Log (B-002) | `erledigt/SP10-tui/` |
| D1 | Diagnose-Schnittstelle des Servers: JSON-Log, Speicher, Geräte, Log, Aktionen (B-066, B-088) | `erledigt/D1-diagnose-server/` |
| M6 | k3c-workbench VI: MCP-Tools für Räume und Simulation (B-047) | `erledigt/M6-dev-raeume/` |
| SP09 | Aufräumen: TS-Sim und Node-Server gelöscht, Release `v0.2.0` | `erledigt/SP09-aufraeumen/` |
| G1 | Referenzseite für die gewählten Grafik-Packs, Auswahl auf den Referenzseiten (B-087) | `erledigt/G1-grafiken/` |
| I1 | Ein Weg für alle Befehle: `task`, `package.json` ohne Skripte, CI und k3c-workbench rufen `task` (B-073, B-070, B-072, B-051) | `erledigt/I1-ein-weg/` |
| U1 | Radar-Leiste im HUD (B-090) | `erledigt/U1-radar/` |
| U2 | Level-Abfrage per HTTP: GET /api/level (B-091) | `erledigt/U2-level-abfrage/` |
| U3 | Level-Betrachter: Seite mit Einstieg von der Testseite (B-092; ersetzt durch den Welt-Betrachter im Studio, B-460) | `erledigt/U3-level-betrachter/` |
| U4 | Debug-Overlay: Verbindung, Takt und Entitäten per Ö oder Stick-Klick (B-093) | `erledigt/U4-debug-overlay/` |
| R1 | Regelwerk I: Wirtschaft, Stufen, Schwierigkeitsgrade, Spielstruktur Inseln/Stufen (B-004, B-005, B-021, B-025) | `erledigt/R1-regelwerk-1/` |
| R2 | Regelwerk I b: Materialien, Hub-Ausbau, Gebäude, Stufenbreite, Plantage und Adern (B-109, B-111) | `erledigt/R2-materialien-gebaeude/` |
| R3 | Regelwerk II: Monarch und Bürger, freie Skillung, Fund-Pool, Schlag, Berufe, Händler (B-110, B-017) | `erledigt/R3-monarch-buerger/` |
| R4 | Regelwerk III: Gegner, Wellen, Bosse, Events (B-127, B-004) | `erledigt/R4-gegner-bosse/` |
| SP12 | Insel-Kern: mehrere Stufen ticken, Einzelwechsel, Vorrat je Insel, Spielstand Version 2 (B-100) | `erledigt/SP12-insel-kern/` |
| SP13 | Raum-Optionen, Grade, fünf Materialien, Lager-Maximum und Tragen in der Insel (B-101, B-113) | `erledigt/SP13-optionen-material/` |
| SP14 | Raum rechnet die Insel, Protokoll Version 3, Raum-Optionen mit Dev-Modus (B-133, B-104) | `erledigt/SP14-raum-auf-insel/` |
| M7 | k3c-workbench VII: Seiten Tasks, Planung und Git (B-171) | `erledigt/M7-dev-seiten/` |
| F0 | Parallele Sprints je Domäne, Sessions per Branch beanspruchen (B-174) | `erledigt/F0-sprint-regel/` |
| SP11 | Raspberry Pi: Image in ghcr, `docker compose pull`, Betrieb am Pi (B-035; Lastmessung nach LT1) | `erledigt/SP11-raspberry-pi/` |
| H1 | Holz-Startvorrat: neue Insel startet mit 100 Holz (B-177) | `erledigt/H1-holz-startvorrat/` |
| F1 | Zielkorridore und Bedienungsregeln, von 🧑 bestätigt (B-134, B-135, B-136, B-144, B-145) | `erledigt/F1-zielkorridore-regeln/` |
| DBG1 | Dev-Aktionen im Raum: Gold, Material, Zeitraffer (B-178) | `erledigt/DBG1-dev-aktionen-server/` |
| F2 | Golden-Ablauf, Spielstand-Migration und Determinismus (B-137, B-138, B-071) | `erledigt/F2-golden-migration-determinismus/` |
| F3 | Feedback-Ereignisse in der Simulation: Treffer, Kill, Münzen, Bau, Obergrenze je Tick (B-139) | `erledigt/F3-feedback-events-sim/` |
| X1 | Xbox-Machbarkeit: Steuerung, Sprite-Budget, HTTPS und Audio gemessen (B-006, B-026, B-166) | `erledigt/X1-xbox/` |
| F4 | Feedback-Ereignisse im Protokoll, Rotation und Backup, Restore mit Token (B-140, B-142, B-143; Restore-Probe am Pi offen) | `erledigt/F4-feedback-events-protokoll-pi/` |
| F5 | Doku-Drift, Version in Landing-Fußzeile und Landing-Kacheln zur Lobby (B-141, B-079) | `erledigt/F5-doku-version/` |
| GR6 | Credits-Seite aus den CREDITS-Dateien mit Vollständigkeits-Test (B-165) | `erledigt/GR6-credits-seite/` |
| M8 | k3c-workbench VIII: Planung über MCP, React-Planungsseite, GitHub-Status (einschiebbar) | `erledigt/M8-dev-planung-mcp/` |
| BAL1 | Balancing-Tester: Kern und Replay: Bots, Kennzahlen-Report und Replay-Datei in k3c-workbench (B-099 teils, B-159; einschiebbar) | `erledigt/BAL1-balancing-tester-kern/` |
| BAL2 | Zielkorridor-Prüfung und `task balance`: Pass/Fail je Kennzahl für 100 feste Seeds, Vergleich mit Baseline, CI-Bericht (B-157; einschiebbar) | `erledigt/BAL2-zielkorridor-pruefung/` |
| BAL3 | Bot-Profile, Sensitivität und Grad-Kurven: vier neue Profile, Sensitivitäts-Lauf (±10/±25 %) und Kurven je Grad im Balancing-Tester (B-158; einschiebbar) | `erledigt/BAL3-bot-profile-sensitivitaet/` |
| S1 | Monarch: Schlag, Fund-Pool, Skills von Tank, Zauberer und Heiler, Standard-Reittier, Spielstand v3 (B-118, B-119, B-022, B-152) | `erledigt/S1-monarch-schlag-skills/` |
| GR1 | Grafik-Zuordnungstabelle: Pack-Stil bestätigt (Q13), jedes Spielobjekt zugeordnet oder Lücke mit Ticket, Vollständigkeits-Test (B-161; einschiebbar) | `erledigt/GR1-grafik-zuordnung/` |
| W0 | Bauplätze aus dem Seed: feste Hub-Plätze, fünf Mauerlinien je Seite mit Tor, Farm, `cause` in `playerDown`, Camps auf Abstand (B-206, B-182, B-261) | `erledigt/W0-bauplaetze-seed/` |
| GR2 | Grafik-Suche für Lücken: Kandidatenseite, 9 gewählte Packs, 20 nicht gewählte Kandidaten in der Gruppe `kandidaten` (B-162; einschiebbar; Ansicht `grafiken.html` durch 🧑 offen) | `erledigt/GR2-grafik-suche/` |
| N1 | Raum-Tick im Budget: ein Zustandsaufbau je Stufe, Delta/JSON in der Schreib-Goroutine, Spielstand asynchron, Benchmark Faktor ~10 (B-276; einschiebbar; Messung am Pi offen) | `erledigt/N1-tick-asynchron/` |
| S2 | Protokoll v4 für Skills und mehrere Stufen je Gerät, Speichern bei jedem Verlassen, Spielmetrik-Report (B-123, B-147, B-150, B-176; Rest B-123 → B-283) | `erledigt/S2-protokoll-skills-speichern-metrik/` |
| W1 | Hub-Ausbau und Mauerstufen: Hub-Stufe 1–5 an der Burg, Mauer/Turm 1–5 am Platz, Reparatur, Stufen im Spielstand (B-112) | `erledigt/W1-hub-ausbau/` |
| W2 | Plantage, Adern, Eisenstollen und Kristallhöhle, Lava, fünf Stufen, Mine-Dichte (B-114, B-115, B-012) | `erledigt/W2-plantage-adern-stufen/` |
| W3 | Gebäude-Wirkungen: Tor wie Mauer, Kämpfer-Limit mit Kaserne, Taverne, Heilplatz, Zaubertum; Schmiede und Rüstkammer baubar (B-116) | `erledigt/W3-gebaeude-wirkungen/` |
| MON1 | Metrik-Sammler: Messreihen je Sekunde, RTT je Gerät, Ereignis-Ring und `GET /api/metrics` mit Token (B-281) | `erledigt/MON1-metrik-sammler/` |
| DBG2 | Debug-Overlay bedient Gold, Material und Zeitraffer (B-179) | `erledigt/DBG2-debug-overlay-aktionen/` |
| DL1 | Delta überträgt verschwundene Felder in `unset`, Delta-Test deckt `castle` mit W4.3a ab (B-297) | `erledigt/DL1-delta-felder/` |
| W4 | Wiederbeleben, Berufe, Händler, Krieger, Elite, Rüstung und Limit je Hub in der Simulation (B-120, B-121, B-122, B-014) | `erledigt/W4-buerger-wiederbeleben/` |
| K1 | Gegner-Traits aoe, swarm, phases, Kiting, Angriffsrate je Gegner, Tor-Blockade, Pools und sechs neue Gegner (B-128, B-129, B-013) | `erledigt/K1-gegner-traits/` |
| W9 | Welt spiegelt Lager-Maximum, Hub-Ausbau und Gefahr über `sim.EconomyOf` (B-323; einschiebbar) | `erledigt/W9-welt-spiegel-wirtschaft/` |
| W5 | Protokoll v5: Hub-Stufe, Lager, Wartegrund, Händler, Berufe und Ereignisse im Zustand; Eingaben bleiben `input.pay` (B-153, B-283, B-330) | `erledigt/W5-protokoll-wirtschaft/` |
| W10 | Kämpfer-Zahl und Truppen-Limit im Zustand: `fighters`, `troopLimit` in `sim.EconomyOf` und Protokoll v5 (B-332; einschiebbar) | `erledigt/W10-truppen-limit-zustand/` |
| N2 | Flüssige Darstellung: Zeitleiste mit Puffer und Extrapolation, Vorhersage des eigenen Monarchen, Latenz im Debug-Overlay (B-277, B-181; Abnahme am Gerät offen) | `erledigt/N2-zeitleiste-vorhersage/` |
| S3 | Skill-Menü, Tasten und Aktionen-Overlay | `erledigt/S3-skill-menue-overlay/` |
| S4 | Kamera je Stufe und Layouts 1–4: Zelle zeigt Stufe, Radar und HUD je Zelle, Mindest-Schrift (B-106; Abnahme am Gerät offen) | `erledigt/S4-kamera-layouts/` |
| S5 | Optionen- und Pause-Szene mit getrennter Lautstärke, Screenshake/Flash, Farbschwäche-Symbolen und Sprache de/en (B-146, B-172; Abnahme am Gerät offen) | `erledigt/S5-optionen-pause/` |
| S6 | Onboarding „Erste Nacht geführt“ und Controller-Glyphen (B-148, B-149; Abnahme am TV offen) | `erledigt/S6-onboarding-glyphen/` |
| S7 | Monarch beritten auf dem Standard-Reittier: `mountPose`, Reittier-Sheet als einzelne Spritesheets (B-173; Abnahme am Gerät offen) | `erledigt/S7-monarch-reittier/` |
| GR3 | Grafik im Renderer: Gebäude, Ressourcen, Portale, Truhen, Münzen und Parallax je Biom als Sprites mit Platzhalter-Rückfall (B-010; Sicht am TV offen) | `erledigt/GR3-grafik-renderer/` |
| GR4 | Atlas und Lade-Szene (B-163, B-029; Messung an der Xbox offen, GR4.3) | `erledigt/GR4-atlas-ladeszene/` |
| GR5 | Juice: Treffer, Screenshake, Münzen (B-164; Abnahme am TV offen) | `erledigt/GR5-juice/` |
| M9 | k3c-workbench in Worktrees und Markdown-Ansicht: Checkout in Antworten, Vite-Watcher, nummerierte Listen (B-275, B-213; Ursache des fehlenden Headers: B-341) | `erledigt/M9-dev-worktrees/` |
| TR1 | Testläufe über `sim_test` in der Workbench: offline/online, headless/1–4 Clients (B-348; Browser-Nachweis nach TR2.1) | `erledigt/TR1-testlaeufe-workbench/` |
| TR2 | Bot-Eingabe im Client: `BotInput` und `?botfeed` in `src/input/` (B-349; Einbindung ins Spiel: B-353, TR3) | `erledigt/TR2-bot-eingabe-client/` |
| PJ1 | Projekte, Rang und Domäne je Session in Regeln, Vorlagen und Planungstest | `erledigt/PJ1-projekte-regeln/` |
| PJ2 | k3c-workbench plant mit Projekten: plan-Tools und Planungsseite | `erledigt/PJ2-projekte-k3c-dev/` |
| TR3 | Bot-Eingabe im Spiel einbinden (B-353; einschiebbar) | `erledigt/TR3-bot-eingabe-spiel/` |
| M11 | MCP-Seite: alle Tools mit Aufruf-Statistik, Zeitfilter der Statistik (B-350; einschiebbar) | `erledigt/M11-mcp-seite-tools/` |
| DBG3 | Dungeon-Master-Seite `/dm` mit Dev-API, Welle und Tageszeit (B-232; Abnahme am Handy offen) | `erledigt/DBG3-dungeon-master-seite/` |
| LP1 | Landingpage für Spieler, Entwicklerseite für Werkzeuge | `erledigt/LP1-landingpage-aufraeumen/` |
| LT1 | Lasttest-Werkzeug `task k3c-load`: Bots, Tick-Dauer und CPU im Bericht, Bewertung gegen < 10 ms (B-175; Messlauf am Pi offen) | `erledigt/LT1-lasttest-werkzeug/` |
| MON2 | Monitoring-Seite `monitor.html`: Ampel je Raum, Verläufe mit Perzentilen, Fehler-Zeitleiste (B-282; Abnahme am Handy offen) | `erledigt/MON2-monitoring-seite/` |
| RL1 | Release-Checkliste: Abschnitt „Release“ in `docs/arbeitsweise.md`, Probelauf ohne Tag (B-170; einschiebbar; Pi und Xbox offen) | `erledigt/RL1-release-checkliste/` |
| SO1 | Audio-Kern: Mixer mit Bus-Lautstärke je Gerät, Entsperren per Eingabe, Sound-Atlas, Positions-Dämpfung, Demo-Ton (B-011 teils; einschiebbar; Hörprobe am TV offen) | `erledigt/SO1-audio-kern/` |
| SO3 | Hörprobenseite `soundtest.html` | `erledigt/SO3-hoerprobenseite/` |
| PJ3 | Planung in Projekte umziehen und aufräumen | `erledigt/PJ3-planung-umziehen/` |
| PL2 | Werkzeug-Seiten in der gewählten Sprache | `erledigt/PL2-texte-werkzeug-seiten/` |
| DV1 | Domäne DEV für k3c-workbench, Sprint ohne Prio und Einschiebbar | `erledigt/DV1-domaene-dev-werkzeug/` |
| K2 | Bosse, Siegvarianten und Inselwechsel | `erledigt/K2-bosse-siege-inseln/` |
| K3 | Events Vollmond, Blutmond und Händler-Überfall | `erledigt/K3-events/` |
| WT1 | Worktree-Sessions richten k3c-workbench-Tools per Argument auf ihren Checkout | `erledigt/WT1-checkout-argument/` |
| K4 | Protokoll für Bosse, Events und Inselwechsel (Protokoll v6; AC-05 offen → B-386) | `erledigt/K4-protokoll-kampf/` |
| LB1 | Lobby zeigt Räume und startet Spiele | `erledigt/LB1-lobby/` |
| U5 | Debug-Overlay und Cheat-Dialog bedienbar | `erledigt/U5-debug-overlay-bedienbar/` |
| S8 | Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung | `erledigt/S8-spielmenue-bedienung/` |
| PL1 | Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte | `erledigt/PL1-start-tastatur-koop-texte/` |
| S9 | Rückmeldung für Schlag und Skills, ein Hinweis je Spieler | `erledigt/S9-rueckmeldung-overlay/` |
| AZ1 | Tasten von Spieler 2 an der Tastatur, Pause-Anzeige | `erledigt/AZ1-tasten-spieler2-pause/` |
| U6 | HUD ohne Überlagerung, Optionen per Touch | `erledigt/U6-hud-ohne-ueberlagerung/` |
| GL1 | Landingpage in der gewählten Sprache | `erledigt/GL1-glyphen-controller-landing-sprache/` |
| WZ2 | Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe | `erledigt/WZ2-planungs-werkzeuge/` |
| WZ3 | Review-Nachträge zu WZ2: Filter-Zählung, Splitter-Test, Kleinigkeiten | `erledigt/WZ3-review-nachtraege/` |
| W7 | Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig | `erledigt/W7-ausruestung-spielstand/` |
| WZ4 | Splitter-Test in DOM-Umgebung | `erledigt/WZ4-lint-splitter-test/` |
| M10 | Ressourcen-Manager für Grafik- und Sound-Slots in k3c-workbench | `erledigt/M10-ressourcen-manager/` |
| W8 | Bauplätze mit Grund und alle Rohstoffe im Client | `erledigt/W8-bauplaetze-rohstoffe-client/` |
| SV1 | Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume | `erledigt/SV1-raeume-stufen-testspielstand/` |
| BD1 | Kamera, Beitritts-Hinweis, Overlay-Taste und HUD-Nachträge | `erledigt/BD1-hud-overlay-nachtraege/` |
| GEB1 | Grundregeln für Gebäude-Grafik und Werkstatt | `erledigt/GEB1-grundregeln-werkstatt/` |
| GEB2 | Gebäude ohne Grafik: Taverne, Heilplatz, Schmiede, Rüstkammer | `erledigt/GEB2-luecken-gebaeude/` |
| GEB8 | Gebäude-Vorlagen in den Daten-JSONs | `erledigt/GEB8-gebaeude-vorlagen/` |
| GEB3 | Mauer und Turm, Stufen 1 bis 5 | `erledigt/GEB3-mauer-turm-stufen/` |
| GEB4 | Burg und Hub-Stufen 1 bis 5, Tor | `erledigt/GEB4-burg-hub-tor/` |
| GEB5 | Lager, Farm mit Plantage, Kaserne, Treppen | `erledigt/GEB5-hub-gebaeude-rest/` |
| GEB6 | Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager | `erledigt/GEB6-natur-objekte/` |
| GEB7 | Hintergründe je Biom | `erledigt/GEB7-hintergruende/` |
| WZ5 | Planungsseite im Look der erp-qs-Workbench | `erledigt/WZ5-planung-erpqs-look/` |
| WZ6 | Asset-Studio als eigene EXE mit Seiten für Grafik und Sound | `erledigt/WZ6-asset-werkzeug-exe/` |
| WZ7 | Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau | `erledigt/WZ7-welt-betrachter/` |
| WZ13 | Glossar und Regeln: Verweis für das Morgengrauen auf B-470 | `erledigt/WZ13-glossar-morgengrauen-verweis/` |
| WZ15 | Hub-Planung Vorlauf: Regeltabelle erzeugt, Prüfung ohne globalen Zustand | `erledigt/WZ15-hub-planung-vorlauf/` |
| WZ16 | Welt-Betrachter: eine gemeinsame Ansicht mit Sprungleiste, Zoom und Gebäude-Filter | `erledigt/WZ16-welt-betrachter-ansicht/` |
| CI1 | CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung | `erledigt/CI1-ci-release-image/` |
| WZ8 | Chunks je Insel und Stufe planen: Seed oder feste Objekte | `erledigt/WZ8-chunk-planung/` |
| RP1 | Repo-Hygiene: Branches aufräumen, Altlasten, Regeln | `erledigt/RP1-repo-hygiene/` |
| SK1 | Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar | `erledigt/SK1-skill-baum/` |
| WZ17 | Chunk-Editor komfortabel: Inselgrößen, getrennter Zoom, Seed-Chunk umwandeln | `erledigt/WZ17-chunk-editor-komfort/` |
| WZ10 | Abspiel-Bausteine und Monarch-Vorschau im Asset-Studio | `erledigt/WZ10-figuren-vorschau/` |
| WZ11 | Gegner- und Boss-Vorschau im Asset-Studio, Figuren-Seiten entfernen | `erledigt/WZ11-gegner-bosse-vorschau/` |
| WZ9 | Gebäude-Planer für die Freischaltung je Hub-Stufe | `erledigt/WZ9-gebaeude-planer/` |
| WZ12 | Planungs-Werkzeug: Platzhalter-Zeilen und Worktree-Pills | `erledigt/WZ12-planungs-werkzeug-nachtraege/` |
| WZ18 | Welten im Spiel: Welt-Dateien, Welt-Wahl im Raum und Welten-Editor | `erledigt/WZ18-welten-im-spiel/` |
| WZ19 | Chat im Studio mit Claude Code CLI und Welt per Prompt füllen | `erledigt/WZ19-chat-und-prompt/` |
| TS1 | Spielserver und Asset-Studio trennen, Server beobachtet Dateien | `erledigt/TS1-trennung-server-studio/` |
| WZ14 | Geplante Chunks tragen Ruinen und Gegnerlager | `erledigt/WZ14-ruinen-gegnerlager/` |
| WZ20 | k3c-workbench: Planungstest für Entwürfe, Worktree-Einträge vollständig und lesend, sim_test im Worktree | `erledigt/WZ20-planung-worktree-simtest/` |
| WZ22 | Studio-Editoren: Fehler sichtbar, Im Spiel öffnen wiederholbar | `erledigt/WZ22-studio-editoren-nachtraege/` |
| WZ24 | Geplante Chunks in Simulation und Spielstand: Regeln geklärt, Planung im Stand, eine Prüfung der Inselgröße | `erledigt/WZ24-chunk-planung-sim-spielstand/` |
| WZ25 | Nachträge zum Server-Neustart: Räume bleiben offen, Assets ohne Build, Werkzeuge passen dazu | `erledigt/WZ25-server-neustart-nachtraege/` |
| WZ21 | Studio-Chat: eigene Umgebung, Grundgerüst als Instruktion, Entwurf geschützt | `erledigt/WZ21-studio-chat-nachtraege/` |
| WZ23 | Studio-Vorschauen: Reittiere aus dem Spielcode, Monarch-Seite, Boss-Anzeigen, Gebäude-Planer abgenommen | `erledigt/WZ23-studio-vorschauen-nachtraege/` |
| BD3 | Raum-Pause im Couch-Raum, Schutz des pausierten Monarchen online | `erledigt/BD3-raum-pause/` |
| BD2 | Rückmeldungs-Ereignisse gedrosselt und geprüft, Lobby startet nie still leer | `erledigt/BD2-rueckmeldung-lobby-nachtraege/` |
| GR8 | Eigene Figuren für neue Gegner und Grafik-Pipeline mit Ziel-Palette | `erledigt/GR8-figuren-pipeline/` |
| BD7 | Pause-Client, Skill-Reset beim Tod, Schutz wartender Monarchen | `erledigt/BD7-pause-client-skill-schutz/` |
| RM1 | Raum-Pause im Couch-Raum und lernbare Skills vom Server | `erledigt/RM1-pause-lernbare-skills/` |
| WZ26 | CI rot beheben, Prüf- und Planungswerkzeuge härten | `erledigt/WZ26-ci-check-tooling/` |
| W14 | Spielstand-Kommentare nach W7.2 nachziehen | `erledigt/W14-spielstand-kommentare/` |
| SO7 | Niederlage-Stück, Hufschlag, UI-Klicks und Skill-Sounds je Klasse | `erledigt/SO7-sound-ergaenzungen/` |
| BD8 | Lobby bietet beim Anlegen eines Spielstands eine Welt-Auswahl | `erledigt/BD8-lobby-welt-auswahl/` |
| WZ27 | Geplante Chunks: Gegnerlager, Ruinen-Kollision, Dateigrenze | `erledigt/WZ27-chunks-lager-ruine/` |
| KB3 | Endboss-Auslöser anzeigen, Dev-Overlay-Link, Wellenabstand unter Tage messen | `erledigt/KB3-endboss-dev-wellen/` |
| GEB14 | Wald und Höhle dichter, Gerüst bei Wartenden, Knoten-Markierer im Snapshot | `erledigt/GEB14-welt-deko-geruest/` |
| SK3 | Dieb-Skills Backstab und Smoke Bomb, Skill-Effekte, Skill-Ring im Studio | `erledigt/SK3-dieb-skills-effekte/` |
| BD5 | Spielmenü zeigt die Welt, legt Spielstände an und löscht sie | `erledigt/BD5-spielmenue-welt-spielstaende/` |
| BD4 | Gemeinsame HUD-Leiste oben, Fluss unten, lokale Spieler treten einzeln aus | `erledigt/BD4-hud-leiste-austreten/` |
| BD6 | Autonome Entscheidungen zu BED prüfen und festschreiben | `erledigt/BD6-entscheidungen-bed/` |
| GEB12 | Entscheidungen zu GEB9 bis GEB11 (Server-Stufenfolge, Markierer im Snapshot, Wald-Dichte) | `erledigt/GEB12-entscheidungen-geb9-11/` |
| KR2 | Kritische Fragen zu GEB | `erledigt/KR2-fragen-geb/` |
| GEB13 | Gebäude-Stufen in den Daten und Stufen-Look für Plantage, Camp, Heilplatz | `erledigt/GEB13-gebaeude-stufen-look/` |
| GEB10 | Natur-Nachträge: hohe Bäume, Höhlen-Flora, geschlossene Truhe, Wimpel in Spielerfarbe | `erledigt/GEB10-natur-truhe-wimpel/` |
| GEB9 | Ausbaustufen 1 bis 5 für alle Hub-Gebäude | `erledigt/GEB9-hub-ausbaustufen/` |
| GEB11 | Hub wächst kleinteilig vom Lagerfeuer bis zur Burg | `erledigt/GEB11-hub-waechst-kleinteilig/` |
| GR9 | Monarch-Animationen für Wurf, Treffer, Tod und Respawn, Kandidaten-Import ohne roten Test | `erledigt/GR9-monarch-animationen-kandidaten/` |
| GR10 | Autonome Grafik-Entscheidungen prüfen (GR8, GR9) | `erledigt/GR10-entscheidungen-grafik/` |
| KR3 | Kritische Fragen zu GRA | `erledigt/KR3-fragen-gra/` |
| GR7 | Figuren-Lücken, ganzzahlige Skalierung und Schrift | `erledigt/GR7-figuren-schrift/` |
| KR4 | Kritische Fragen zu SND | `erledigt/KR4-fragen-snd/` |
| SO4 | Musik je Zustand | `erledigt/SO4-musik/` |
| SO2 | SFX-Katalog und Einbau | `erledigt/SO2-sfx-katalog/` |
| SO6 | Sound-Credits auf der Credits-Seite, Sound-Atlas und Sounds für die übrigen Ereignisse | `erledigt/SO6-sound-credits-atlas-luecken/` |
| SK2 | Skills im Spiel sichtbar, Dieb-Skills in den Daten | `erledigt/SK2-skills-sichtbar-dieb/` |
| K5 | Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel | `erledigt/K5-anzeige-kampf/` |
| LV1 | Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs | `erledigt/LV1-level-gegner-korrektur/` |
| KB2 | Protokoll-Nachträge Kampf: Inselwechsel, Game Over, Händler, Client-Typen | `erledigt/KB2-protokoll-kampf-nachtraege/` |
| NT1 | Stabile Tests, Warteschlange und Snapshot-Budget | `erledigt/NT1-netz-tests-stabil/` |
| TR4 | Bot-Eingabe ohne verlorene Drücke, Beobachter ohne Platz im Raum | `erledigt/TR4-bot-feed-beobachter/` |
| GD1 | Golden-Daten kleiner bei gleicher Prüfschärfe | `erledigt/GD1-golden-verkleinern/` |
| KB1 | Kampf-Nachträge: Mond-Events, Endboss-Bau, Golden-Läufe, Feuergeist | `erledigt/KB1-events-boss-golden/` |
| PG1 | Präsentationsseite mit echten Spielbildern | `erledigt/PG1-praesentation-bilder/` |
| BAL6 | Balancing-Tester misst die Wirtschaft | `erledigt/BAL6-tester-misst-wirtschaft/` |
| GRJ1 | Alte Biom-Hintergründe mit Lizenz im Archiv | `erledigt/GRJ1-alte-hintergruende-archiv/` |
| BAL4 | Abgleich Spielmetrik und Simulator | `erledigt/BAL4-metrik-abgleich/` |
| BAL7 | Holz-Balancing nach dem dichteren Wald | `erledigt/BAL7-holz-balancing-wald/` |
| SV2 | Voll-Ausbau-Stand regelkonform aus der Welt-Datei, Review-Nachträge SV1 | `erledigt/SV2-voll-ausbau-regelkonform/` |
| W12 | Morgengrauen als eigene Phase in Simulation, Protokoll und Regeln | `erledigt/W12-morgengrauen-phase/` |
| BS2 | Regelwerk der neuen Basis: Münzskala, Baumschule, Hafen und Schiff | `erledigt/BS2-regelwerk-neue-basis/` |
| BS1 | Schneller Balancing-Lauf mit Seed k3c-dev | `erledigt/BS1-balancing-seed-k3c-dev/` |
| BS11 | Balancing-Tester spielt die ganze Insel bis zum Endboss | `erledigt/BS11-tester-ganze-insel/` |
| BS3 | Werte auf der Münzskala: Meilenstein-Lauf coop2 standard läuft durch | `erledigt/BS3-werte-muenzskala-meilenstein/` |
| BS12 | Mauern und Türme verteilt, Tor ist die äußerste Mauer | `erledigt/BS12-mauern-tuerme-verteilt-tor/` |
| BS4 | Baumschule auf gerodeten Waldbauplätzen | `erledigt/BS4-baumschule-waldbauplaetze/` |
| BS5 | Hafen-Chunk, Schiff bauen und Inselwechsel am Hafen | `erledigt/BS5-hafen-schiff-inselwechsel/` |
| BS10 | Grafiken für Hafen und Schiff | `erledigt/BS10-grafik-hafen-schiff/` |
| BS7 | Delta-Balancing: Werte-Gruppen, Baseline je Meilenstein, Basis-Läufe im Editor | `erledigt/BS7-delta-balancing-basis-laeufe/` |
| BS8 | Meilenstein 2: Bots und Delta-Runde Baumschule gegen Baseline M1 | `erledigt/BS8-meilenstein-2-delta-baumschule/` |
| BS9 | Meilenstein 3: Bots bauen das Schiff, Delta-Runde Schiff gegen Baseline M2 | `erledigt/BS9-meilenstein-3-delta-schiff/` |
| BS6 | Client: Baumschule, Hafen und Schiff, neue Münzskala | `erledigt/BS6-client-baumschule-hafen-skala/` |
| BS13 | Baustellen-Schilder, Bauoptionen und Holzzaun | `erledigt/BS13-baustellen-fehler-holzzaun/` |
| W11 | Bauplätze im Protokoll, Abstand der Treppen, Ausbilden und Tauschen, Figuren | `erledigt/W11-bauplaetze-protokoll-abstand/` |
| WZ32 | Balancing-Seite: feste Fläche, eine Liste, Lauf-Editor mit Varianten-Tabelle | `erledigt/WZ32-balancing-seite-bedienung/` |
| WZ33 | Eine Seite Assets statt Grafik, Sound und Quellen (B-677) | `erledigt/WZ33-assets-seite/` |
| ST1 | Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik | `erledigt/ST1-speichern-robust/` |
| BT1 | Server im Heimnetz finden, Windows-Starter, Start mit Seed | `erledigt/BT1-heimnetz-start/` |
| KR1 | Kritische Fragen zu BED | `erledigt/KR1-fragen-bed/` |
| KR5 | Kritische Fragen zu SKL | `erledigt/KR5-fragen-skl/` |
| KR6 | Kritische Fragen zu KMP | `erledigt/KR6-fragen-kmp/` |
| WZ28 | Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite | `erledigt/WZ28-offline-werkzeuge/` |
| WZ30 | Oberflächen zur Abnahme: Szenen, Filter, Quellen, Chat-Karten mit Mock-Daten | `erledigt/WZ30-oberflaechen-abnahme/` |
| WZ29 | Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat | `erledigt/WZ29-sound-grafik-seiten-ausbau/` |
| WZ31 | Workbench und Studio als Web-Client: Server starten, im Browser arbeiten | `erledigt/WZ31-web-client-first/` |
| GEB15 | Höhlen-Wurzeln, Felsen und Minen: mehr Varianten, größer | `erledigt/GEB15-hoehle-fels-varianten/` |
| GEB16 | Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik | `erledigt/GEB16-bauschilder-wald-geruest/` |
| GEB18 | Hintergründe der fünf Stufen: vierfache Auflösung, fünf Varianten je Stufe nach Seed | `erledigt/GEB18-hintergruende-detail-varianten/` |
| W13 | Morgengrauen im Client und Welt-Betrachter, dunklere Nacht mit Fackeln und Lagerfeuer als Licht | `erledigt/W13-morgengrauen-licht-client/` |
| SO8 | Skill-Sounds je Klasse | `erledigt/SO8-skill-sounds-je-klasse/` |
| GEB17 | Hub-Ausbau: Angebot vor der Burg, Ausbaustufen je Inselstufe, unterscheidbare Wachstumsreihe | `erledigt/GEB17-hub-ausbau-je-inselstufe/` |
| WZ34 | Prüf- und Build-Werkzeuge: Studio-Test, check:diff, buildall | `erledigt/WZ34-pruef-build-werkzeuge/` |
| WZ35 | Workbench-Nachträge: Ticket-Nummern, Lauf-IDs, schmale Fenster, Auswertung | `erledigt/WZ35-workbench-nachtraege/` |
| WZ36 | Studio-Chat: Aktionskarten für jede Bearbeitung, Resume, Welt-Prüfung | `erledigt/WZ36-studio-chat-aktionen-resume/` |
| WZ38 | Umgesetzte Werkzeug-Tickets abnehmen: Welten-Editor, Studio-Seiten, Git-Seite, check:diff | `erledigt/WZ38-umgesetzte-werkzeuge-abnehmen/` |
| WZ39 | Studio-Nachträge: Downloads, Overrides-Editor, Inspektor, Testlücken, Quellen | `erledigt/WZ39-studio-nachtraege/` |
| BAL10 | Welt-Wertesatz: Nachträge aus dem Review BAL8.2 | `erledigt/BAL10-wertesatz-nachtraege/` |
| BAL8 | Welt-Werte: Basis erben, Overrides je Welt, Simulation rechnet mit dem Wertesatz der Welt | `erledigt/BAL8-welt-werte-overrides/` |
| WZ37 | Welt-Overrides im Welten-Editor und im Studio-Chat | `erledigt/WZ37-overrides-editor-chat/` |
| BAL9 | Meilenstein 1: Titan über acht Seeds, castleHeld im Fenster, Bot-Performance | `erledigt/BAL9-meilenstein-1-nachtraege/` |
| PV1 | Papierschnitt-Verträge: Domäne, Spec-Schema, Styleguide, Render-, MCP- und Review-Vertrag | `erledigt/PV1-papierschnitt-vertraege/` |
| WB1 | Workbench mit Bereichen Development und Assets, Studio-Programm abgebaut | `erledigt/WB1-workbench-development-assets/` |
| MG6 | Papierschnitt-Migration: Gebäude mit Ausbaustufen | `erledigt/MG6-papier-gebaeude/` |
| US1 | Spiel lädt Papierschnitt-Grafik, alte Pixel-Assets und Gebäude-Renderer entfernt | `erledigt/US1-spiel-auf-papierschnitt/` |
| US2 | Spiel lädt Natur, Objekte, Münzen, Icons und Reittiere als Papierschnitt | `erledigt/US2-spiel-natur-reittiere-papier/` |
| PS1 | Papierschnitt-Sichtprüfungen und Abnahmen gebündelt | `erledigt/PS1-papierschnitt-sichtpruefungen/` |
| MG1 | Papierschnitt-Migration: Monarch und Spielerfarben | `erledigt/MG1-papier-monarch-spielerfarben/` |
| MG2 | Papierschnitt-Migration: Reittiere | `erledigt/MG2-papier-reittiere/` |
| MG3 | Papierschnitt-Migration: Truppen | `erledigt/MG3-papier-truppen/` |
| MG4 | Papierschnitt-Migration: Gegner | `erledigt/MG4-papier-gegner/` |
| MG5 | Papierschnitt-Migration: Bosse | `erledigt/MG5-papier-bosse/` |
| MG7 | Papierschnitt-Migration: Hintergründe je Biom | `erledigt/MG7-papier-hintergruende/` |
| MG8 | Papierschnitt-Migration: Natur, Objekte, Münzen, Icons | `erledigt/MG8-papier-natur-objekte-icons/` |
| WB2 | Review und Katalog: feste Schieber, getrennte Scrollbereiche, Lade-Anzeige | `erledigt/WB2-assets-review-katalog-ruhig/` |
| RD2 | Renderer und Stil nach der Sichtprüfung: Schatten, Materialien, Sichtbarkeit, Rig-Proportionen | `erledigt/RD2-renderer-stil-nachschaerfen/` |
| MG9 | Papierschnitt nachbessern: Monarch, Reittiere, Truppen, Gegner | `erledigt/MG9-papier-figuren-nachbessern/` |
| BD11 | Eine Startseite, Navigation im Pausenmenü, Vollbild halten | `erledigt/BD11-startseite-pausenmenue/` |
| NV1 | Bevy-Prototyp: Papier-Sheets, zwei Gamepads, Messung | `erledigt/NV1-bevy-prototyp/` |
| NV2 | Rig-Prototyp: Monarch aus Teilen in Ebitengine | `erledigt/NV2-rig-prototyp-monarch/` |
| NV3 | Prototyp-Ausbau: Fluss, fester Hintergrund, alle Figuren als Rig | `erledigt/NV3-prototyp-fluss-rigs/` |
| NG1 | Natives Spiel: Fundament mit Modul, Vertrag, Simulation im Prozess und Spielschleife | `erledigt/NG1-native-fundament/` |
| NG2 | Natives Spiel: Welt, Kamera, Schichten, Fluss und Licht | `erledigt/NG2-native-welt-kamera/` |
| NG3 | Natives Spiel: Figuren als Rig, Bein-Bänder und Gegnerzahl | `erledigt/NG3-native-figuren/` |
| NG4 | Natives Spiel: Gebäude, Baustellen, Natur und Objekte | `erledigt/NG4-native-bauten-natur/` |
| NG5 | Natives Spiel: Eingabe mit Gamepads und Tastatur | `erledigt/NG5-native-eingabe/` |
| NG6 | Natives Spiel: HUD, Hinweise und Effekte | `erledigt/NG6-native-hud-effekte/` |
| NG7 | Natives Spiel: Menüs, Lobby, Spielstände, Optionen und Pause | `erledigt/NG7-native-menues-spielstaende/` |
| NG8 | Natives Spiel: Klänge und Musik | `erledigt/NG8-native-ton/` |
| NG10 | Rust und Bevy entfernen | `erledigt/NG10-rust-bevy-entfernen/` |
| RS1 | Messen, Go-Fixes, Rust-Spike für den Simulationskern | `erledigt/RS1-messen-go-fixes-rust-spike/` |
| RS2 | Etappe 2 vorbereiten: Workbench entkoppeln, Zustand ohne JSON-Roundtrip, Inventar | `erledigt/RS2-vorarbeit-inventar/` |
| NG12 | Natives Spiel: Anzeigen an Figuren, Bäume und Geschosse | `erledigt/NG12-native-figuren-darstellung/` |
| NG16 | Raum: Absturzbehandlung nach außen, Position nach dem Laden | `erledigt/NG16-raum-laden-absturz/` |
| NG18 | Natives Spiel: freie Schrift statt Debug-Schrift | `erledigt/NG18-native-schrift/` |
| NG13 | Natives Spiel: Menüs und HUD nachziehen | `erledigt/NG13-native-menues-hud/` |
| NG17 | Papier: Reittier Wolf und Pfoten der Gegner-Wölfe | `erledigt/NG17-papier-wolf/` |
| CU2 | Altlasten: Testseiten des Browser-Clients und Pixel-Reste entfernen | `erledigt/CU2-altlasten-testseiten-pixel/` |
| RX1 | Xbox hält den WebGL-Kontext: Render-Log, Kontextverlust abfangen, Texturen auf 2048 px | `erledigt/RX1-xbox-webgl-stabil/` |
| PF1 | Split-Screen flüssig auf der Xbox | `erledigt/PF1-splitscreen-leistung/` |
| LG1 | Remote-Logs vom Pi und Xbox mit WebGL 1 und 130 MiB | `erledigt/LG1-remote-logs-webgl-budget/` |
| PM1 | Leistung messen: Diagnose-Zeile und Performance-Modus | `erledigt/PM1-leistung-messen/` |
| BD9 | Welt-Anzeige größer und je Spielstand, offene Räume schließen | `erledigt/BD9-welt-anzeige-raum-schliessen/` |
| BD10 | Schreibende HTTP-Endpunkte lehnen fremde Webseiten ab | `erledigt/BD10-schreibende-endpunkte-schutz/` |
| W6 | Anzeigen für Bau, Lager, Hub und Bürger (B-117, B-126) | `erledigt/W6-anzeige-wirtschaft/` |
| HW1 | Zurückgestellte Controller-Prüfungen nachholen | `erledigt/HW1-controller-pruefungen/` |
| RG1 | Werte-Runde: Burg hält Nacht, Hub-Stufe 4–5, Adern-Takt | `erledigt/RG1-werte-burg-hub-adern/` |
| BR1 | Balancing-Runde Wirtschaft und Spieleabend 2 (BR1.1 erledigt; zurückgestellt am 2026-10-07, Vorrang Performance, Grafik und Sound) | `erledigt/BR1-balancing-wirtschaft/` |
| BR2 | Balancing-Runde Kampf und Bosse und Spieleabend 3 | `erledigt/BR2-balancing-kampf/` |
| GRJ2 | Grafik-Zielmaß und Ausschnitt-Rahmen abnehmen | `erledigt/GRJ2-zielmass-ausschnitt-abnehmen/` |
| US3 | Spiel: Bogenschützen auf der Plattform, Reit-Tempo und -Idle, Größen von Bäumen und Tor | `erledigt/US3-spielbefunde-papierschnitt/` |
| WZ40 | check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel | `erledigt/WZ40-check-diff-schneller/` |
| WZ41 | CI und Release grün, check:diff-Protokoll belegt | `erledigt/WZ41-ci-release-gruen/` |
| PT1 | gen:paper schneller: Cache je Asset, paralleles Rastern, Fortschritt belegt | `erledigt/PT1-gen-paper-schneller/` |
| CU1 | Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen | `erledigt/CU1-alte-grafik-aufraeumen/` |
| WZ42 | Workbench-Tests schnell und stabil | `erledigt/WZ42-workbench-tests-schnell-stabil/` |
| RD1 | Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests | `erledigt/RD1-papierschnitt-renderer/` |
| MC1 | Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt | `erledigt/MC1-asset-mcp-tools/` |
| RV1 | Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung | `erledigt/RV1-asset-review/` |
| WZ44 | Planungs-Werkzeuge für Wellen: plan_waves, plan_queue, plan_check, Sim-Sperre im Werkzeug | `erledigt/WZ44-planung-wellen-werkzeuge/` |
| PT2 | Paper-CLI: Fehlerpfade, Reittier-Boxen, Stil-Option und Ersatz-Materialien | `erledigt/PT2-paper-cli-und-stil/` |
| NG9 | Natives Spiel: Zusammenführung, Messung und Abnahme | `erledigt/NG9-native-zusammenfuehrung-abnahme/` |
| NG11 | Natives Spiel: Fluss mit Shader | `erledigt/NG11-native-fluss-shader/` |
| NG14 | Natives Spiel: Fehler- und Ladeanzeige, Shader-Umschalter | `erledigt/NG14-native-fehleranzeige-shader-umschalter/` |
| NG15 | Natives Spiel: Ton-Gerät ohne Absturz | `erledigt/NG15-native-ton-geraet/` |
| NG19 | Natives Spiel: Menü-Kacheln mit Illustrationen, Hintergrund-Batching, Version im Paket | `erledigt/NG19-native-menue-kacheln-batching-version/` |
| PT3 | Paper-CLI: --ohne mit Reittier-Teilen, Bein-Box mit Wulst | `erledigt/PT3-paper-frame-reittier-ohne/` |
| PT4 | Paper-CLI: --ohne blendet Beinpfade von Vierbeinern aus | `erledigt/PT4-paper-frame-vierbeiner-ohne/` |
| WZ48 | Tests und CI: check:go ohne Sim, Workbench-Go-Tests in der CI, stabile und schnelle Tests | `erledigt/WZ48-tests-ci-stabil/` |
| WZ51 | check:dev testet die Sim-Pakete der Workbench nicht, solange die Sim-Sperre gilt | `erledigt/WZ51-check-dev-ohne-sim/` |
| WZ50 | Katalog-Vitests der Workbench bleiben unter Last grün | `erledigt/WZ50-katalog-vitests-stabil/` |
| WZ52 | readonly-Vitest der Workbench bleibt unter Last grün | `erledigt/WZ52-readonly-vitest-stabil/` |
| WZ47 | Ticket-Relationen, schlanker Index und Sprint-Zusammenfassungen | `erledigt/WZ47-ticket-relationen-index/` |
| WZ53 | Vitests unter src/assets bleiben unter Last grün | `erledigt/WZ53-assets-vitests-stabil/` |
| WZ49 | Planungs-Werkzeuge und check_run: Restbefunde, Fahrplan aus Dateien, Im Spiel ansehen nativ | `erledigt/WZ49-planung-checkrun-restbefunde/` |
| WZ54 | Planungs-Index: Statuswechsel ohne Zeilenverlust, Ticket-Nummern ab B-1000 | `erledigt/WZ54-index-atomar-nummern-b1000/` |
| WZ55 | assets_set ruft die Paper-CLI gesammelt auf | `erledigt/WZ55-paper-cli-sammelaufruf/` |
| WZ56 | Sammelaufruf: Fehlerantwort mit Teilergebnis, Task-Version gemerkt | `erledigt/WZ56-sammelaufruf-fehlerpfad-taskversion/` |
| KB4 | Dev-Aktionen für Boss, Phasenwechsel, Events und Nacht | `erledigt/KB4-dev-aktionen-boss-event/` |
| KB5 | Skills am Wirkort, DoTs und Buffs sichtbar, im Spiel und im Studio | `erledigt/KB5-skills-wirkort-status/` |
| WZ43 | Asset-MCP-Tools und Review: Restbefunde, Teil-Patch, Basis-Stil, Katalog belegt | `erledigt/WZ43-asset-tools-review/` |
| WZ57 | Workbench ohne Web-Client, doppelte Ticket-Nummern, Abnahme WZ43 | `erledigt/WZ57-workbench-aufraeumen/` |
| PT5 | Abnahme-Läufe CU1 (B-836) | `erledigt/PT5-abnahme-cu1/` |
| NG20 | Natives Spiel komplett nach dist (B-827) | `erledigt/NG20-native-build-dist/` |
| MG11 | Papierschnitt nachbessern: Gebäude, Hintergründe ohne Bäume, Natur und Icons | `erledigt/MG11-papier-welt-nachbessern/` |
| WZ45 | Asset-Seiten: Filter und Prompt-Buttons für Styleguide, Sound und Quellen | `erledigt/WZ45-asset-seiten-prompts-filter/` |
| PT6 | Entscheidungsvorlage: Asset-Pakete je Welt (B-880) | `erledigt/PT6-asset-pakete-vorlage/` |
| MG10 | Papierschnitt: Bosse neu mit eigener Silhouette | `erledigt/MG10-papier-bosse-neu/` |
| WZ58 | Welten-Editor zurück, Wellen-Regel, Größenvergleich im Review | `erledigt/WZ58-welten-editor-planung-review/` |
| PB1 | Veröffentlichung auf itch.io | `erledigt/PB1-itch-io/` |
| WR1 | Rückbau von Browser-Client, Spielserver und Online-Werkzeugen | `erledigt/WR1-web-und-server-rueckbau/` |
| CH1 | Cheats, Entwickler-Tasten, Debug-Werte und HUD-Icons im nativen Spiel | `erledigt/CH1-cheats-und-entwickler-tasten/` |
| WZ59 | Studio-Seiten neu ordnen, Sound und Quellen zusammenlegen | `erledigt/WZ59-studio-seiten/` |
| WR2 | Reste des Server- und Web-Rückbaus aufräumen | `erledigt/WR2-rueckbau-reste/` |
| BI2 | Biom-Eigenschaften sichtbar: Kristallfarben, Decke, Tropfsteine, Verfall, Glut, Leuchten | `erledigt/BI2-biom-darstellung/` |
| EP1 | Eingabe-Profile je Spieler und Tasten-Icons je Gerät | `erledigt/EP1-eingabe-profile/` |
| ST2 | Stresstests im Spielmenü (Testing) | `erledigt/ST2-stresstests-im-menue/` |
| RU0 | Ruckler im Go-Spiel messen: GC, Bildtakt, Laden (Gate G0) | `erledigt/RU0-ruckler-im-go-spiel-messen/` |
| RU1 | Rust-Workspace, Toolchain, Taskfile und CI | `erledigt/RU1-rust-workspace-toolchain/` |
| RU2 | Spike: schlanker Rust-Client gegen die Ebitengine-Messung (Gate G1) | `erledigt/RU2-spike-schlanker-rust-client/` |
| NA1 | Nachträge natives Spiel: rote Tests, Tasten-Icons, Rig-Export, Steg-Eingabe | `erledigt/NA1-nachtraege-natives-spiel/` |
| BI1 | Biome mit Eigenschaften: Dichte, Chunk-Anteile und Varianten je Welt | `erledigt/BI1-biom-eigenschaften/` |
| GP1 | Präsentationsseite als Bausatz mit Lizenz und eigenen Welten | `erledigt/GP1-pages-bausatz/` |
| FB1 | Live-Test-Feedback: Stresstest-Oberfläche, Tasten, HUD | `erledigt/FB1-feedback-client/` |
| FB2 | Live-Test-Feedback: Gegnerwellen, Stressmodus, Geldsack-Skill | `erledigt/FB2-feedback-sim/` |
| WG2 | Weltbilder im Spiel: Weltauswahl vor den Spieloptionen, Bild in Spielständen und Pausenmenü | `erledigt/WG2-weltbilder-anzeige/` |
| FB4 | Live-Test 2: Ruckeln, Xbox-Tasten, Menü-Tastatur, Ladescreen, Mauern, Bäume | `erledigt/FB4-live-test-2/` |
| RU3 | rng, Go-gleiche Zahlen und Daten-Schemas in Rust | `erledigt/RU3-rng-zahlen-daten/` |
| SI1 | Fracht auf dem Schiff: Regel, Engine, Assets, Anzeige | `erledigt/SI1-fracht-auf-dem-schiff/` |
| SI2 | Koop-Aktion Doppel-Truhe: Regel, Engine, Asset, Anzeige | `erledigt/SI2-doppel-truhe-koop/` |
| SI3 | Wetter je Welt: Regel, Engine, Assets, Anzeige | `erledigt/SI3-wetter-je-welt/` |
| SI4 | Reittiere je Welt: Regel, Assets, Engine, Anzeige | `erledigt/SI4-reittiere-je-welt/` |
| SI5 | Besondere Figur der Welt mit einmaligem Bauwerk: Regel, Engine, Assets, Anzeige | `erledigt/SI5-einsiedler-bauwerk/` |
| SI6 | Fest nach dem Sieg und friedliche Insel: Regel, Assets, Engine, Anzeige, Festmusik | `erledigt/SI6-fest-nach-dem-sieg/` |
| RU8 | Sim-Zustand in Rust: Zustandsmodell, Typen, Welt-Grundgerüst | `erledigt/RU8-sim-zustand-typen/` |
| RU5 | Rig in Rust: Rechnung und Zeichnen der Rig-Figuren | `erledigt/RU5-rig-in-rust/` |
| RU7 | Grafik laden und Ansicht: Papierschnitt-Index, Sheets, Kamera | `erledigt/RU7-grafik-laden-ansicht/` |
| RU4 | Plattform-Schicht: Schleife mit und ohne Fenster, Eingabe, Ton, Text | `erledigt/RU4-plattform-fenster-eingabe-ton/` |
| RU13 | Welt und Akteure zeichnen | `erledigt/RU13-welt-und-akteure-zeichnen/` |
| RU38 | Zustands-Aufzeichnung aus Go und Wiedergabe im Rust-Client | `erledigt/RU38-aufzeichnung-und-wiedergabe/` |
| RU16 | Gebäude und Objekte zeichnen | `erledigt/RU16-gebaeude-und-objekte-zeichnen/` |
| RU17 | Effekte und Ton im Spiel | `erledigt/RU17-effekte-und-ton-im-spiel/` |
| RU20 | HUD Teil 1: Anzeigen je Spielerhälfte | `erledigt/RU20-hud-teil-1/` |
| RU39 | Ruckler-Messung: Rust-Client in Wiedergabe gegen das Go-Spiel (Gate G2) | `erledigt/RU39-ruckler-messung-rust-gegen-go/` |
| RU9 | Menüs Teil 1: Hauptmenü, Lobby, Einstellungen | `erledigt/RU9-menues-teil-1/` |
| RU11 | Sim: Einheiten und Gegner | `erledigt/RU11-sim-einheiten-gegner/` |
| RU42 | Leitplanken für Leistung und Stabilität | `erledigt/RU42-leitplanken-leistung-stabilitaet/` |
| RU6 | Level-Generator in Rust, Level-Golden bitgleich | `erledigt/RU6-level-generator-rust/` |
| RU10 | Sim: Insel mit Stufen, Takt und Material-Vorrat | `erledigt/RU10-sim-insel/` |
| RU15 | Sim: Kampf, Bosse, Krone, Portale | `erledigt/RU15-sim-kampf-bosse/` |
| RU23 | HUD Teil 2: Overlays, Skills, Geschichte, Seekarte im Spiel | `erledigt/RU23-hud-teil-2/` |
| RU14 | Sim: Welt-Schritt, Kampagne, Schiff und Vorschau | `erledigt/RU14-sim-welt-stufen-kampagne/` |
| RU44 | Bild-Pfad ohne Allokationen | `erledigt/RU44-bild-pfad-ohne-allokationen/` |
| RU12 | Menüs Teil 2: Spielstände, Weltwahl, Pause, Test-Menüs | `erledigt/RU12-menues-teil-2/` |
| RU45 | Abnahme- und Messläufe auf dem Second Worker | `erledigt/RU45-abnahme-und-messlaeufe-second-worker/` |
| RU26 | Spielschleife Teil 1: Zustände, Split-Screen, Zeichnen | `erledigt/RU26-spielschleife-teil-1/` |
| RU21 | Spielstände und Raum Teil 1 | `erledigt/RU21-spielstaende-und-raum-teil-1/` |
| RU46 | Rust-Schulden in k3c-level und Insel (Second Worker) | `erledigt/RU46-rust-schulden-level-und-insel/` |
