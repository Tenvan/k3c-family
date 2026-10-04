# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Je Domäne ein aktiver Sprint (B-174); einschiebbare zählen nicht mit.
Reihenfolge in Wellen, Bahnen je Domäne und Zuordnung Mensch/autonom (zwei Accounts): [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.

## Aktiv

| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|
| S2 | SRV | Protokoll für Skills, Speichern beim Verlassen, Spielmetrik | neue Felder in `docs/protocol.md` mit Beispielen unter `testdata/protocol/`, Spielstand nach Trennung mitten in der Nacht, ein Report in `reports/` | `aktiv/S2-protokoll-skills-speichern-metrik/` |

## Offen am Gerät

Hardware-Sessions (`Agent: Mensch`) aus Sprints, deren Review schon abgeschlossen ist; erledigt, sobald das Gerät da ist
(`../arbeitsweise.md` › Hardware entkoppelt). Bis dahin gelten die angenommenen Werte aus
[`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6, dort steht auch die Liste aller Hardware-Sessions.

| Session | Gerät | Kriterium (angenommen) | Ordner |
|---|---|---|---|
| GR4.3 | Xbox (Kaltstart, `MAX_TEXTURE_SIZE`) | AC-04: Budget und Texturgröße gemessen (angenommen: 4096) | `erledigt/GR4-atlas-ladeszene/` |
| DBG2.3 | Controller (PC und Handy geprüft 2026-10-03) | AC-05: Aktionen mit allen drei Eingaben ausprobiert | `erledigt/DBG2-debug-overlay-aktionen/` |
| S4.3 | Xbox am TV (Viertel-Layout lesbar) oder zwei Eingabegeräte; „zwei Stufen“ erst nach B-176 | AC-04: zwei Spieler in verschiedenen Stufen, Layouts 3 und 4 lesbar | `erledigt/S4-kamera-layouts/` |
| LT1.3 | Raspberry Pi (Messlauf 2 Räume × 3 Spieler über eine Nacht, `task load`) | AC-06: Messlauf bewertet, B-042 archiviert (angenommen laut Messung 2026-10-03: Nacht 10,2 bis 10,3 ms, Ziel < 10 ms) | `erledigt/LT1-lasttest-werkzeug/` |
| S5.4 | Xbox am TV und Handy (Optionen/Pause bedienen, englische Texte lesen) | AC-05, AC-07: Szene am TV und Handy abgenommen, englische Texte gelesen | `erledigt/S5-optionen-pause/` |
| SO1.5 | Xbox am TV (Entsperren nach erster Taste, ogg mit mp3-Fallback, Split-Screen-Dämpfung hören; Browser-Pane-Schritte aus SO1.2/SO1.3 offen) | AC-04: Format mit Fallback am TV beobachtet (angenommen: ogg, mp3-Fallback laut X1, B-166) | `erledigt/SO1-audio-kern/` |
| GR5 (TV) | Xbox am TV (Effekte sehen, Blitz-Eindruck, Vibration) | AC-01, AC-02, AC-04: Effekte sichtbar, Schalter „aus“ ruhig (angenommen laut Tests) | `erledigt/GR5-juice/` |
| SO3.3 | Xbox am TV (Hörprobe: Controller bedienen, B frei, View + Menu zurück, Crossfade ohne Knacken, Lautstärke; Browser-Pane-Schritte aus SO3.1/SO3.2 offen) | AC-03, AC-04: Controller-Bedienung und Crossfade am TV bestätigt (angenommen laut Tests) | `erledigt/SO3-hoerprobenseite/` |

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|
| S3 | CLI | Skill-Menü, Tasten und Aktionen-Overlay | 🧑 spielt am Gerät Schlag, Skill, Punkte verteilen und liest die Aktionen im Overlay | bereit | `geplant/S3-skill-menue-overlay/` |
| S6 | CLI | Onboarding „Erste Nacht geführt“ und Controller-Glyphen | Ein Kind spielt die erste Nacht ohne Erklärung | bereit | `geplant/S6-onboarding-glyphen/` |
| S7 | CLI | Monarch auf dem Standard-Reittier | Zwei Spieler im Split-Screen reiten über die Stufe | bereit | `geplant/S7-monarch-reittier/` |
| P1 | REG 🧑 | Spieleabend 1 | Protokoll und Folge-Tickets | Entwurf | `geplant/P1-spieleabend-1/` |
| W0 | SIM | Bauplätze aus dem Seed: feste Hub-Plätze, Mauerlinien, Tor und Farm | `task check:go` grün mit Tests für Linien, Freischaltung und Platz-Abstände, alte Spielstände laden, Level-Golden unverändert | bereit | `geplant/W0-bauplaetze-seed/` |
| W1 | SIM | Hub-Ausbau und Mauerstufen | `task check:go` grün, Tests für Ausbau, Zerstörung und Reparatur, aktualisierte Golden-Daten | bereit | `geplant/W1-hub-ausbau/` |
| W2 | SIM | Plantage, Adern, Stufenbreite und Mine | Tests für Generator, Adern, Plantage und Biome grün, aktualisierte Golden-Level | bereit | `geplant/W2-plantage-adern-stufen/` |
| W3 | SIM | Gebäude-Wirkungen | Tests je Gebäude grün, Werte aus den Daten, aktualisierte Golden-Daten | bereit | `geplant/W3-gebaeude-wirkungen/` |
| W4 | SIM | Wiederbeleben, Berufe, Händler, Elite und Limit | Tests je Regel grün, aktualisierte Golden-Daten | bereit | `geplant/W4-buerger-wiederbeleben/` |
| W5 | SRV | Protokoll für Berufe, Händler, Lager und Hub-Stufe | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | Entwurf | `geplant/W5-protokoll-wirtschaft/` |
| W6 | CLI | Anzeigen für Bau, Lager, Hub und Bürger | HUD und Bauplätze am TV, von 🧑 abgenommen | Entwurf | `geplant/W6-anzeige-wirtschaft/` |
| BR1 | REG 🧑 | Balancing-Runde Wirtschaft und Spieleabend 2 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR1-balancing-wirtschaft/` |
| K1 | SIM | Gegner-Traits, neue Gegner und Elite-KI | Tests je Trait und Gegner grün, aktualisierte Golden-Daten | bereit | `geplant/K1-gegner-traits/` |
| K2 | SIM | Bosse, Siegvarianten und Inselwechsel | Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel | Entwurf | `geplant/K2-bosse-siege-inseln/` |
| K3 | SIM | Events Vollmond, Blutmond und Händler-Überfall | Tests je Event grün, aktualisierte Golden-Daten | bereit | `geplant/K3-events/` |
| K4 | SRV | Protokoll für Bosse, Events und Inselwechsel | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | bereit | `geplant/K4-protokoll-kampf/` |
| K5 | CLI | Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel | Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen | bereit | `geplant/K5-anzeige-kampf/` |
| BR2 | REG 🧑 | Balancing-Runde Kampf und Bosse und Spieleabend 3 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR2-balancing-kampf/` |

**Einschiebbar** (Schienen Balancing, Grafik, Sound, Betrieb; unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Thema | Reife | Ordner |
|---|---|---|---|---|
| BAL3 | SIM | Bot-Profile, Sensitivität und Grad-Kurven | Entwurf | `geplant/BAL3-bot-profile-sensitivitaet/` |
| BAL4 | REG | Abgleich Spielmetrik und Simulator | Entwurf | `geplant/BAL4-metrik-abgleich/` |
| GR1 | CLI | Grafik-Zuordnungstabelle | bereit | `geplant/GR1-grafik-zuordnung/` |
| GR2 | CLI | Grafik-Suche für Lücken | bereit | `geplant/GR2-grafik-suche/` |
| GR3 | CLI | Grafik im Renderer | Entwurf | `geplant/GR3-grafik-renderer/` |
| RL1 | INF | Release-Checkliste | bereit | `geplant/RL1-release-checkliste/` |
| SO2 | CLI | SFX-Katalog und Einbau | Entwurf | `geplant/SO2-sfx-katalog/` |
| SO4 | CLI | Musik je Zustand | Entwurf | `geplant/SO4-musik/` |
| DBG3 | PLAT | Dungeon-Master-Seite /dm | Entwurf | `geplant/DBG3-dungeon-master-seite/` |

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
| DBG2 | Debug-Overlay bedient Gold, Material und Zeitraffer (B-179; Abnahme am Gerät offen) | `erledigt/DBG2-debug-overlay-aktionen/` |
| F3 | Feedback-Ereignisse in der Simulation: Treffer, Kill, Münzen, Bau, Obergrenze je Tick (B-139) | `erledigt/F3-feedback-events-sim/` |
| X1 | Xbox-Machbarkeit: Steuerung, Sprite-Budget, HTTPS und Audio gemessen (B-006, B-026, B-166) | `erledigt/X1-xbox/` |
| LT1 | Lasttest-Werkzeug `task load`: Bots, Tick-Dauer und CPU im Bericht, Bewertung gegen < 10 ms (B-175; Messlauf am Pi offen) | `erledigt/LT1-lasttest-werkzeug/` |
| F4 | Feedback-Ereignisse im Protokoll, Rotation und Backup, Restore mit Token (B-140, B-142, B-143; Restore-Probe am Pi offen) | `erledigt/F4-feedback-events-protokoll-pi/` |
| F5 | Doku-Drift, Version in Landing-Fußzeile und Landing-Kacheln zur Lobby (B-141, B-079) | `erledigt/F5-doku-version/` |
| GR6 | Credits-Seite aus den CREDITS-Dateien mit Vollständigkeits-Test (B-165) | `erledigt/GR6-credits-seite/` |
| GR4 | Atlas und Lade-Szene (B-163, B-029; Messung an der Xbox offen, GR4.3) | `erledigt/GR4-atlas-ladeszene/` |
| S4 | Kamera je Stufe und Layouts 1–4: Zelle zeigt Stufe, Radar und HUD je Zelle, Mindest-Schrift (B-106; Abnahme am Gerät offen) | `erledigt/S4-kamera-layouts/` |
| GR5 | Juice: Treffer, Screenshake, Münzen (B-164; Abnahme am TV offen) | `erledigt/GR5-juice/` |
| S5 | Optionen- und Pause-Szene mit getrennter Lautstärke, Screenshake/Flash, Farbschwäche-Symbolen und Sprache de/en (B-146, B-172; Abnahme am Gerät offen) | `erledigt/S5-optionen-pause/` |
| M8 | k3c-dev VIII: Planung über MCP, React-Planungsseite, GitHub-Status (einschiebbar) | `erledigt/M8-dev-planung-mcp/` |
| BAL1 | Balancing-Tester: Kern und Replay: Bots, Kennzahlen-Report und Replay-Datei in k3c-dev (B-099 teils, B-159; einschiebbar) | `erledigt/BAL1-balancing-tester-kern/` |
| SO1 | Audio-Kern: Mixer mit Bus-Lautstärke je Gerät, Entsperren per Eingabe, Sound-Atlas, Positions-Dämpfung, Demo-Ton (B-011 teils; einschiebbar; Hörprobe am TV offen) | `erledigt/SO1-audio-kern/` |
| BAL2 | Zielkorridor-Prüfung und `task balance`: Pass/Fail je Kennzahl für 100 feste Seeds, Vergleich mit Baseline, CI-Bericht (B-157; einschiebbar) | `erledigt/BAL2-zielkorridor-pruefung/` |
| S1 | Monarch: Schlag, Fund-Pool, Skills von Tank, Zauberer und Heiler, Standard-Reittier, Spielstand v3 (B-118, B-119, B-022, B-152) | `erledigt/S1-monarch-schlag-skills/` |
