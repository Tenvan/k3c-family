# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Die Reihenfolge der Arbeit ist allein der Projekt-Rang: [`../projekte/README.md`](../projekte/README.md). Die Tabellen stehen nach Rang und in der Sprint-Reihenfolge des Projekts, ABN und ruhende Projekte zuletzt; je Projekt können mehrere Sprints aktiv sein.

## Aktiv

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|---|
| WZ41 | INF | WZG | CI und Release grün, check:diff-Protokoll belegt | – | `aktiv/WZ41-ci-release-gruen/` |
| WZ42 | DEV | WZG | Workbench-Tests schnell und stabil | – | `aktiv/WZ42-workbench-tests-schnell-stabil/` |
| WZ44 | DEV, INF | WZG | Planungs-Werkzeuge für Wellen: plan_waves, plan_queue, plan_check, Sim-Sperre im Werkzeug | – | `aktiv/WZ44-planung-wellen-werkzeuge/` |
| RD1 | PAP | PAP | Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests | – | `aktiv/RD1-papierschnitt-renderer/` |
| MC1 | DEV | PAP | Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt | – | `aktiv/MC1-asset-mcp-tools/` |
| RV1 | DEV | PAP | Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung | – | `aktiv/RV1-asset-review/` |
| CU1 | PAP, CLI, DEV, INF | PAP | Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen | – | `aktiv/CU1-alte-grafik-aufraeumen/` |
| WZ40 | INF | WZG | check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel | – | `aktiv/WZ40-check-diff-schneller/` |
| PT1 | PAP | PAP | gen:paper schneller: Cache je Asset, paralleles Rastern, Fortschritt belegt | – | `aktiv/PT1-gen-paper-schneller/` |
| W15 | SIM | WRT | Fackeln als Spielobjekt mit Licht in der Nacht | – | `aktiv/W15-fackeln-spielobjekt/` |
| KB4 | SRV, CLI | KMP | Dev-Aktionen für Boss, Phasenwechsel, Events und Nacht | – | `aktiv/KB4-dev-aktionen-boss-event/` |
| KB5 | SRV, CLI, DEV | KMP | Skills am Wirkort, DoTs und Buffs sichtbar, im Spiel und im Studio | fireball am Gegner, Betäubung, Schild und Sturm als Zeichnung | `aktiv/KB5-skills-wirkort-status/` |
| PB1 | INF | REL | Veröffentlichung auf itch.io | – | `aktiv/PB1-itch-io/` |

## Offen am Gerät

Abnahmen am Gerät (`Agent: Mensch`) sammelt das Projekt ABN im Sprint HW1: [`../projekte/ABN-abnahmen-geraet.md`](../projekte/ABN-abnahmen-geraet.md). Bis zur Abnahme gelten die angenommenen Werte aus [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6 (`../arbeitsweise.md` › Hardware entkoppelt).

## Geplant (nach Projekt-Rang)

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|---|
| WZ43 | DEV | WZG | Asset-MCP-Tools und Review: Restbefunde, Teil-Patch, Basis-Stil, Katalog belegt | – | bereit | `geplant/WZ43-asset-tools-review/` |
| WZ47 | DEV | WZG | Ticket-Relationen, schlanker Index und Sprint-Zusammenfassungen | – | bereit | `geplant/WZ47-ticket-relationen-index/` |
| WZ45 | DEV | WZG | Asset-Seiten: Filter und Prompt-Buttons für Styleguide, Sound und Quellen | – | Entwurf | `geplant/WZ45-asset-seiten-prompts-filter/` |
| WZ46 | DEV | WZG | Workbench-Tests ohne Engine-Spielserver | – | Entwurf | `geplant/WZ46-workbench-tests-ohne-spielserver/` |
| MG10 | PAP | PAP | Papierschnitt: Bosse neu mit eigener Silhouette | – | Entwurf | `geplant/MG10-papier-bosse-neu/` |
| MG11 | PAP | PAP | Papierschnitt nachbessern: Gebäude, Hintergründe ohne Bäume, Natur und Icons | – | Entwurf | `geplant/MG11-papier-welt-nachbessern/` |
| PT2 | PAP | PAP | Paper-CLI: Fehlerpfade, Reittier-Boxen, Stil-Option und Ersatz-Materialien | – | bereit | `geplant/PT2-paper-cli-und-stil/` |
| LG1 | SRV, DEV, CLI | LST | Remote-Logs vom Pi und Xbox mit WebGL 1 und 130 MiB | – | Entwurf | `geplant/LG1-remote-logs-webgl-budget/` |
| RG2 | REG | BAL | Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4 | – | Entwurf | `geplant/RG2-regelwerk-klaerungen/` |
| P1 | REG | BAL | Spieleabend 1 | Protokoll und Folge-Tickets | bereit | `geplant/P1-spieleabend-1/` |

Gesamtplan und Begründung: [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md). Offene Entscheidungen: [`../fragenkatalog.md`](../fragenkatalog.md).

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
| KB6 | SIM, REG | KMP | Kampf-Nachträge: Wellenabstand unter Tage, stabiler Schutzplatz | – | `erledigt/KB6-kampf-nachtraege-wellen-schutz/` |
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
| PV1 | INF, DEV, PAP | `erledigt/PV1-papierschnitt-vertraege/` |
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
| NG9 | Natives Spiel: Zusammenführung, Messung und Abnahme | `erledigt/NG9-native-zusammenfuehrung-abnahme/` |
| NG19 | Natives Spiel: Menü-Kacheln mit Illustrationen, Hintergrund-Batching, Version im Paket | `erledigt/NG19-native-menue-kacheln-batching-version/` |
| NG15 | Natives Spiel: Ton-Gerät ohne Absturz | `erledigt/NG15-native-ton-geraet/` |
| NG14 | Natives Spiel: Fehler- und Ladeanzeige, Shader-Umschalter | `erledigt/NG14-native-fehleranzeige-shader-umschalter/` |
| NG11 | Natives Spiel: Fluss mit Shader | `erledigt/NG11-native-fluss-shader/` |
