# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Die Reihenfolge der Arbeit ist allein der Projekt-Rang: [`../projekte/README.md`](../projekte/README.md). Die Tabellen stehen nach Rang und in der Sprint-Reihenfolge des Projekts, ABN und ruhende Projekte zuletzt; je Projekt können mehrere Sprints aktiv sein.

## Aktiv

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|---|
| BS1 | DEV | BAS | Schneller Balancing-Lauf mit Seed k3c-dev | – | `aktiv/BS1-balancing-seed-k3c-dev/` |
| WZ29 | DEV | WZG | Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat | – | `aktiv/WZ29-sound-grafik-seiten-ausbau/` |
| BD9 | SRV, CLI | BED | Welt-Anzeige größer und je Spielstand, offene Räume schließen | – | `aktiv/BD9-welt-anzeige-raum-schliessen/` |
| WZ31 | DEV | WZG | Workbench und Studio als Web-Client: Server starten, im Browser arbeiten | – | `aktiv/WZ31-web-client-first/` |
| WZ28 | DEV | WZG | Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite | – | `aktiv/WZ28-offline-werkzeuge/` |
| WZ30 | DEV | WZG | Oberflächen zur Abnahme: Szenen, Filter, Quellen, Chat-Karten mit Mock-Daten | – | `aktiv/WZ30-oberflaechen-abnahme/` |
| GEB18 | DEV, SIM, CLI | GEB | Hintergründe der fünf Stufen: vierfache Auflösung, fünf Varianten je Stufe nach Seed | – | `aktiv/GEB18-hintergruende-detail-varianten/` |
| GEB15 | DEV | GEB | Höhlen-Wurzeln, Felsen und Minen: mehr Varianten, größer | – | `aktiv/GEB15-hoehle-fels-varianten/` |
| GEB16 | SRV, CLI, DEV, REG | GEB | Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik | – | `aktiv/GEB16-bauschilder-wald-geruest/` |
| W11 | REG, SRV, CLI | WRT | Bauplätze im Protokoll, Abstand der Treppen, Ausbilden und Tauschen, Figuren | – | `aktiv/W11-bauplaetze-protokoll-abstand/` |
| W13 | CLI, DEV | WRT | Morgengrauen im Client und Welt-Betrachter, dunklere Nacht mit Fackeln und Lagerfeuer als Licht | – | `aktiv/W13-morgengrauen-licht-client/` |
| GEB17 | CLI, REG, SIM, DEV | GEB | Hub-Ausbau: Angebot vor der Burg, Ausbaustufen je Inselstufe, unterscheidbare Wachstumsreihe | – | `aktiv/GEB17-hub-ausbau-je-inselstufe/` |
| SO8 | DEV, CLI | SND | Skill-Sounds je Klasse | – | `aktiv/SO8-skill-sounds-je-klasse/` |
| PM1 | CLI | LST | Leistung messen: Diagnose-Zeile und Performance-Modus | – | `aktiv/PM1-leistung-messen/` |
| ST1 | SRV | LST | Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik | HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt | `aktiv/ST1-speichern-robust/` |
| KB4 | SRV, CLI | KMP | Dev-Aktionen für Boss, Phasenwechsel, Events und Nacht | – | `aktiv/KB4-dev-aktionen-boss-event/` |
| BT1 | SRV, DEV | REL | Server im Heimnetz finden, Windows-Starter, Start mit Seed | – | `aktiv/BT1-heimnetz-start/` |
| PB1 | INF | REL | Veröffentlichung auf itch.io | – | `aktiv/PB1-itch-io/` |
| KR1 | REG | KRT | Kritische Fragen zu BED | – | `aktiv/KR1-fragen-bed/` |
| KR5 | REG | KRT | Kritische Fragen zu SKL | – | `aktiv/KR5-fragen-skl/` |
| KR6 | REG | KRT | Kritische Fragen zu KMP | – | `aktiv/KR6-fragen-kmp/` |
| RG1 | REG | BAL | Werte-Runde: Burg hält Nacht, Hub-Stufe 4–5, Adern-Takt | Beschlüsse in `docs/rules/`, geänderte Werte in `data/`, `task balance` grün | `aktiv/RG1-werte-burg-hub-adern/` |
| BR2 | REG | BAL | Balancing-Runde Kampf und Bosse und Spieleabend 3 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | `aktiv/BR2-balancing-kampf/` |
| HW1 | PLAT, SRV, INF, CLI | ABN | Zurückgestellte Controller-Prüfungen nachholen | – | `aktiv/HW1-controller-pruefungen/` |
| PF1 | CLI | LST | Split-Screen flüssig auf der Xbox | Zwei Spieler im Split-Screen ohne sichtbares Ruckeln auf der Xbox | `aktiv/PF1-splitscreen-leistung/` |

## Offen am Gerät

Abnahmen am Gerät (`Agent: Mensch`) sammelt das Projekt ABN im Sprint HW1: [`../projekte/ABN-abnahmen-geraet.md`](../projekte/ABN-abnahmen-geraet.md). Bis zur Abnahme gelten die angenommenen Werte aus [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6 (`../arbeitsweise.md` › Hardware entkoppelt).

## Geplant (nach Projekt-Rang)

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|---|
| BS2 | REG | BAS | Regelwerk der neuen Basis: Münzskala, Baumschule, Hafen und Schiff | – | bereit | `geplant/BS2-regelwerk-neue-basis/` |
| BS3 | REG, DEV | BAS | Werte auf der Münzskala: Meilenstein-Lauf coop2 standard läuft durch | – | bereit | `geplant/BS3-werte-muenzskala-meilenstein/` |
| BS4 | SIM | BAS | Baumschule auf gerodeten Waldbauplätzen | – | Entwurf | `geplant/BS4-baumschule-waldbauplaetze/` |
| BS5 | SIM | BAS | Hafen-Chunk, Schiff bauen und Inselwechsel am Hafen | – | Entwurf | `geplant/BS5-hafen-schiff-inselwechsel/` |
| BS6 | CLI | BAS | Client: Baumschule, Hafen und Schiff, neue Münzskala | – | Entwurf | `geplant/BS6-client-baumschule-hafen-skala/` |
| BS7 | DEV | BAS | Delta-Balancing: Werte-Gruppen, Baseline je Meilenstein, Basis-Läufe im Editor | – | bereit | `geplant/BS7-delta-balancing-basis-laeufe/` |
| BS8 | DEV | BAS | Meilenstein 2: Bots und Delta-Runde Baumschule gegen Baseline M1 | – | Entwurf | `geplant/BS8-meilenstein-2-delta-baumschule/` |
| BS9 | DEV | BAS | Meilenstein 3: Bots bauen das Schiff, Delta-Runde Schiff gegen Baseline M2 | – | Entwurf | `geplant/BS9-meilenstein-3-delta-schiff/` |
| BS10 | DEV | BAS | Grafiken für Hafen und Schiff | – | Entwurf | `geplant/BS10-grafik-hafen-schiff/` |
| W6 | CLI | WRT | Anzeigen für Bau, Lager, Hub und Bürger (B-117, B-126) | HUD und Bauplätze am TV, von 🧑 abgenommen | bereit | `geplant/W6-anzeige-wirtschaft/` |
| RG2 | REG | BAL | Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4 | – | Entwurf | `geplant/RG2-regelwerk-klaerungen/` |
| P1 | REG | BAL | Spieleabend 1 | Protokoll und Folge-Tickets | bereit | `geplant/P1-spieleabend-1/` |
| BR1 | REG | BAL | Balancing-Runde Wirtschaft und Spieleabend 2 (BR1.1 erledigt; zurückgestellt am 2026-10-07, Vorrang Performance, Grafik und Sound) | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | bereit | `geplant/BR1-balancing-wirtschaft/` |

Gesamtplan und Begründung: [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md). Offene Entscheidungen: [`../fragenkatalog.md`](../fragenkatalog.md).

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
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
