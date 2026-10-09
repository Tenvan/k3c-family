# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Die Reihenfolge der Arbeit ist allein der Projekt-Rang: [`../projekte/README.md`](../projekte/README.md). Die Tabellen stehen nach Rang und in der Sprint-Reihenfolge des Projekts, ABN und ruhende Projekte zuletzt; je Projekt höchstens ein aktiver Sprint.

## Aktiv

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|---|
| W6 | CLI | WRT | Anzeigen für Bau, Lager, Hub und Bürger (B-117, B-126) | HUD und Bauplätze am TV, von 🧑 abgenommen | `aktiv/W6-anzeige-wirtschaft/` |
| K2 | SIM | KMP | Bosse, Siegvarianten und Inselwechsel | Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel | `aktiv/K2-bosse-siege-inseln/` |
| U5 | CLI | BED | Debug-Overlay und Cheat-Dialog bedienbar | Ö schließt Overlay und Liste, HUD bleibt lesbar, Cheat-Dialog mit Fokus und Controller | `aktiv/U5-debug-overlay-bedienbar/` |
| S8 | CLI | BED | Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung | Spielmenü verlässt ins Lobby, Y ohne Bau-Menü, Glyph-Entscheidung umgesetzt | `aktiv/S8-spielmenue-bedienung/` |
| PL1 | PLAT | BED | Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte | „Neues Spiel“ startet immer neu, zwei Tastatur-Spieler, Overlay per Controller | `aktiv/PL1-start-tastatur-koop-texte/` |

## Offen am Gerät

Abnahmen am Gerät (`Agent: Mensch`) sammelt das Projekt ABN im Sprint HW1: [`../projekte/ABN-abnahmen-geraet.md`](../projekte/ABN-abnahmen-geraet.md). Bis zur Abnahme gelten die angenommenen Werte aus [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6 (`../arbeitsweise.md` › Hardware entkoppelt).

## Geplant (nach Projekt-Rang)

| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|---|
| PM1 | CLI | LST | Leistung messen: Diagnose-Zeile und Performance-Modus | – | Entwurf | `geplant/PM1-leistung-messen/` |
| PF1 | CLI | LST | Split-Screen flüssig auf der Xbox | Zwei Spieler im Split-Screen ohne sichtbares Ruckeln auf der Xbox | bereit | `geplant/PF1-splitscreen-leistung/` |
| NT1 | SRV | LST | Stabile Tests, Warteschlange und Snapshot-Budget | – | Entwurf | `geplant/NT1-netz-tests-stabil/` |
| ST1 | SRV | LST | Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik | HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt | Entwurf | `geplant/ST1-speichern-robust/` |
| GR7 | CLI | GRA | Figuren-Lücken, ganzzahlige Skalierung und Schrift | – | Entwurf | `geplant/GR7-figuren-schrift/` |
| M10 | SRV | GRA | Ressourcen-Manager für Grafik- und Sound-Slots in k3c-dev | – | Entwurf | `geplant/M10-ressourcen-manager/` |
| SO2 | CLI | SND | SFX-Katalog und Einbau | – | bereit | `geplant/SO2-sfx-katalog/` |
| SO4 | CLI | SND | Musik je Zustand | – | bereit | `geplant/SO4-musik/` |
| S9 | CLI | BED | Rückmeldung für Schlag und Skills, ein Hinweis je Spieler | Jeder Tastendruck auf Schlag oder Skill ist sichtbar, das Aktionen-Overlay zeigt je Spieler einen Hinweis | bereit | `geplant/S9-rueckmeldung-overlay/` |
| U6 | CLI | BED | HUD ohne Überlagerung, Optionen per Touch | Optionen per Touch bedienbar, HUD bei 1–4 Spielern ohne Überlagerung, Gesamtabnahme Anzeige | Entwurf | `geplant/U6-hud-ohne-ueberlagerung/` |
| LB1 | CLI | BED | Lobby zeigt Räume und startet Spiele | Lobby listet offene Räume, Beitritt ohne Raumcode | Entwurf | `geplant/LB1-lobby/` |
| W7 | SIM | WRT | Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig | Passive Burg kann fallen, Spielstand stellt W2–W4 wieder her, Golden-Hub geprüft | bereit | `geplant/W7-ausruestung-spielstand/` |
| SV1 | SRV | WRT | Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume | Neuer Raum mit allen fünf Stufen, Level-Betrachter startet einen voll ausgebauten Spielstand | bereit | `geplant/SV1-raeume-stufen-testspielstand/` |
| W8 | CLI | WRT | Bauplätze mit Grund und alle Rohstoffe im Client | Gesperrte Plätze zeigen den Grund, Client-Typen passen zu `hub.json` und den fünf Rohstoffen | Entwurf | `geplant/W8-bauplaetze-rohstoffe-client/` |
| SK1 | SIM | SKL | Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar | Skill-Baum spielbar, Respec und Lernen ohne Seiteneffekt prüfbar | bereit | `geplant/SK1-skill-baum/` |
| RM1 | SRV | SKL | Raum-Pause im Couch-Raum und lernbare Skills vom Server | Pause hält den Couch-Raum an; Skill-Menü zeigt nur, was der Server annimmt | Entwurf | `geplant/RM1-pause-lernbare-skills/` |
| K4 | SRV | KMP | Protokoll für Bosse, Events und Inselwechsel | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | bereit | `geplant/K4-protokoll-kampf/` |
| K5 | CLI | KMP | Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel | Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen | bereit | `geplant/K5-anzeige-kampf/` |
| K3 | SIM | KMP | Events Vollmond, Blutmond und Händler-Überfall | Tests je Event grün, aktualisierte Golden-Daten | bereit | `geplant/K3-events/` |
| LV1 | SIM | KMP | Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs | Keine Lava auf Linien, Camps mit Abstand, besiegte Gegner lassen Gold fallen, Ereignisse mit Ort | Entwurf | `geplant/LV1-level-gegner-korrektur/` |
| CI1 | INF | REL | CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung | CI grün mit SP01-Prüfungen, Release-Image lehnt Dev-Aktionen ab, Abdeckung im CI-Bericht | Entwurf | `geplant/CI1-ci-release-image/` |
| RP1 | INF | REL | Repo-Hygiene: Branches aufräumen, Altlasten, Regeln | – | Entwurf | `geplant/RP1-repo-hygiene/` |
| BT1 | SRV | REL | Server im Heimnetz finden, Windows-Starter, Start mit Seed | – | Entwurf | `geplant/BT1-heimnetz-start/` |
| PG1 | PLAT | REL | Präsentationsseite mit echten Spielbildern | – | Entwurf | `geplant/PG1-praesentation-bilder/` |
| PB1 | INF | REL | Veröffentlichung auf itch.io | – | Entwurf | `geplant/PB1-itch-io/` |
| HW1 | PLAT, SRV, INF, CLI | ABN | Zurückgestellte Controller-Prüfungen nachholen | – | Entwurf | `geplant/HW1-controller-pruefungen/` |
| RG1 | REG | BAL | Werte-Runde: Burg hält Nacht, Hub-Stufe 4–5, Adern-Takt | Beschlüsse in `docs/rules/`, geänderte Werte in `data/`, `task balance` grün | Entwurf | `geplant/RG1-werte-burg-hub-adern/` |
| BAL6 | SIM | BAL | Balancing-Tester misst die Wirtschaft | – | Entwurf | `geplant/BAL6-tester-misst-wirtschaft/` |
| RG2 | REG | BAL | Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4 | – | Entwurf | `geplant/RG2-regelwerk-klaerungen/` |
| P1 | REG | BAL | Spieleabend 1 | Protokoll und Folge-Tickets | bereit | `geplant/P1-spieleabend-1/` |
| BAL4 | REG | BAL | Abgleich Spielmetrik und Simulator | – | bereit | `geplant/BAL4-metrik-abgleich/` |
| BR1 | REG | BAL | Balancing-Runde Wirtschaft und Spieleabend 2 (BR1.1 erledigt; zurückgestellt am 2026-10-07, Vorrang Performance, Grafik und Sound) | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | bereit | `geplant/BR1-balancing-wirtschaft/` |
| BR2 | REG | BAL | Balancing-Runde Kampf und Bosse und Spieleabend 3 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | bereit | `geplant/BR2-balancing-kampf/` |

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
| M1 | k3c-dev I: MCP-Kern über HTTP | `erledigt/M1-dev-mcp/` |
| M2 | k3c-dev II: Nutzungsstatistik, Berichte, Spielstände | `erledigt/M2-dev-statistik/` |
| M3 | k3c-dev III: Dienste führen, `svc_*`-Tools | `erledigt/M3-dev-dienste/` |
| M4 | k3c-dev IV: Oberfläche (Wails) mit Dienste- und Logs-Seite | `erledigt/M4-dev-oberflaeche/` |
| SP03 | Go-Server Basis: Auslieferung, Spielstände mit Sicherungen, Status, Docker | `erledigt/SP03-go-server/` |
| M5 | k3c-dev V: MCP-Seite mit Monitoren und Statistik | `erledigt/M5-dev-mcp-seite/` |
| SP04 | Golden-Tests, RNG und Level-Generator in Go | `erledigt/SP04-golden-level/` |
| SP05 | Port I: Welt, Zyklus, Truppen, Wirtschaft in Go | `erledigt/SP05-port-welt/` |
| SP06 | Port II: Gegner, Wellen, Reisen, Kampagne, Spielstand in Go | `erledigt/SP06-port-einheiten/` |
| SP07 | Räume & WebSocket (Protokoll v2) in Go | `erledigt/SP07-raeume/` |
| SP08 | Browser als reiner Client (Protokoll v2, Lobby, lokale Spieler, Layout 1–4) | `erledigt/SP08-client/` |
| SP10 | Diagnose-TUI (Bubble Tea): `k3c-tui` zeigt Räume live, Aktionen, Log (B-002) | `erledigt/SP10-tui/` |
| D1 | Diagnose-Schnittstelle des Servers: JSON-Log, Speicher, Geräte, Log, Aktionen (B-066, B-088) | `erledigt/D1-diagnose-server/` |
| M6 | k3c-dev VI: MCP-Tools für Räume und Simulation (B-047) | `erledigt/M6-dev-raeume/` |
| SP09 | Aufräumen: TS-Sim und Node-Server gelöscht, Release `v0.2.0` | `erledigt/SP09-aufraeumen/` |
| G1 | Referenzseite für die gewählten Grafik-Packs, Auswahl auf den Referenzseiten (B-087) | `erledigt/G1-grafiken/` |
| I1 | Ein Weg für alle Befehle: `task`, `package.json` ohne Skripte, CI und k3c-dev rufen `task` (B-073, B-070, B-072, B-051) | `erledigt/I1-ein-weg/` |
| U1 | Radar-Leiste im HUD (B-090) | `erledigt/U1-radar/` |
| U2 | Level-Abfrage per HTTP: GET /api/level (B-091) | `erledigt/U2-level-abfrage/` |
| U3 | Level-Betrachter: leveltest.html, Einstieg von der Testseite (B-092) | `erledigt/U3-level-betrachter/` |
| U4 | Debug-Overlay: Verbindung, Takt und Entitäten per Ö oder Stick-Klick (B-093) | `erledigt/U4-debug-overlay/` |
| R1 | Regelwerk I: Wirtschaft, Stufen, Schwierigkeitsgrade, Spielstruktur Inseln/Stufen (B-004, B-005, B-021, B-025) | `erledigt/R1-regelwerk-1/` |
| R2 | Regelwerk I b: Materialien, Hub-Ausbau, Gebäude, Stufenbreite, Plantage und Adern (B-109, B-111) | `erledigt/R2-materialien-gebaeude/` |
| R3 | Regelwerk II: Monarch und Bürger, freie Skillung, Fund-Pool, Schlag, Berufe, Händler (B-110, B-017) | `erledigt/R3-monarch-buerger/` |
| R4 | Regelwerk III: Gegner, Wellen, Bosse, Events (B-127, B-004) | `erledigt/R4-gegner-bosse/` |
| SP12 | Insel-Kern: mehrere Stufen ticken, Einzelwechsel, Vorrat je Insel, Spielstand Version 2 (B-100) | `erledigt/SP12-insel-kern/` |
| SP13 | Raum-Optionen, Grade, fünf Materialien, Lager-Maximum und Tragen in der Insel (B-101, B-113) | `erledigt/SP13-optionen-material/` |
| SP14 | Raum rechnet die Insel, Protokoll Version 3, Raum-Optionen mit Dev-Modus (B-133, B-104) | `erledigt/SP14-raum-auf-insel/` |
| M7 | k3c-dev VII: Seiten Tasks, Planung und Git (B-171) | `erledigt/M7-dev-seiten/` |
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
| M8 | k3c-dev VIII: Planung über MCP, React-Planungsseite, GitHub-Status (einschiebbar) | `erledigt/M8-dev-planung-mcp/` |
| BAL1 | Balancing-Tester: Kern und Replay: Bots, Kennzahlen-Report und Replay-Datei in k3c-dev (B-099 teils, B-159; einschiebbar) | `erledigt/BAL1-balancing-tester-kern/` |
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
| M9 | k3c-dev in Worktrees und Markdown-Ansicht: Checkout in Antworten, Vite-Watcher, nummerierte Listen (B-275, B-213; Ursache des fehlenden Headers: B-341) | `erledigt/M9-dev-worktrees/` |
| TR1 | Testläufe über `sim_test` in der Workbench: offline/online, headless/1–4 Clients (B-348; Browser-Nachweis nach TR2.1) | `erledigt/TR1-testlaeufe-workbench/` |
| TR2 | Bot-Eingabe im Client: `BotInput` und `?botfeed` in `src/input/` (B-349; Einbindung ins Spiel: B-353, TR3) | `erledigt/TR2-bot-eingabe-client/` |
| PJ1 | Projekte, Rang und Domäne je Session in Regeln, Vorlagen und Planungstest | `erledigt/PJ1-projekte-regeln/` |
| PJ2 | k3c-dev plant mit Projekten: plan-Tools und Planungsseite | `erledigt/PJ2-projekte-k3c-dev/` |
| TR3 | Bot-Eingabe im Spiel einbinden (B-353; einschiebbar) | `erledigt/TR3-bot-eingabe-spiel/` |
| M11 | MCP-Seite: alle Tools mit Aufruf-Statistik, Zeitfilter der Statistik (B-350; einschiebbar) | `erledigt/M11-mcp-seite-tools/` |
| DBG3 | Dungeon-Master-Seite `/dm` mit Dev-API, Welle und Tageszeit (B-232; Abnahme am Handy offen) | `erledigt/DBG3-dungeon-master-seite/` |
| LP1 | Landingpage für Spieler, Entwicklerseite für Werkzeuge | `erledigt/LP1-landingpage-aufraeumen/` |
| LT1 | Lasttest-Werkzeug `task load`: Bots, Tick-Dauer und CPU im Bericht, Bewertung gegen < 10 ms (B-175; Messlauf am Pi offen) | `erledigt/LT1-lasttest-werkzeug/` |
| MON2 | Monitoring-Seite `monitor.html`: Ampel je Raum, Verläufe mit Perzentilen, Fehler-Zeitleiste (B-282; Abnahme am Handy offen) | `erledigt/MON2-monitoring-seite/` |
| RL1 | Release-Checkliste: Abschnitt „Release“ in `docs/arbeitsweise.md`, Probelauf ohne Tag (B-170; einschiebbar; Pi und Xbox offen) | `erledigt/RL1-release-checkliste/` |
| SO1 | Audio-Kern: Mixer mit Bus-Lautstärke je Gerät, Entsperren per Eingabe, Sound-Atlas, Positions-Dämpfung, Demo-Ton (B-011 teils; einschiebbar; Hörprobe am TV offen) | `erledigt/SO1-audio-kern/` |
| SO3 | Hörprobenseite `soundtest.html` | `erledigt/SO3-hoerprobenseite/` |
| PJ3 | Planung in Projekte umziehen und aufräumen | `erledigt/PJ3-planung-umziehen/` |
| PL2 | Werkzeug-Seiten in der gewählten Sprache | `erledigt/PL2-texte-werkzeug-seiten/` |
| DV1 | Domäne DEV für k3c-dev, Sprint ohne Prio und Einschiebbar | `erledigt/DV1-domaene-dev-werkzeug/` |
