# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Je Domäne ein aktiver Sprint (B-174); einschiebbare zählen nicht mit.
Reihenfolge in Wellen, Bahnen je Domäne und Zuordnung Mensch/autonom (zwei Accounts): [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.

## Aktiv

| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|
| F4 | SRV | Feedback-Ereignisse im Protokoll und Pi-Betrieb | Benchmark mit Bytes je Tick, README-Abschnitt zum Backup, Test „Restore ohne Berechtigung abgelehnt“ | `aktiv/F4-feedback-events-protokoll-pi/` |
| GR4 | INF | Atlas und Lade-Szene | `task atlas`, ein Balken beim Start, ein Messwert vom TV | `aktiv/GR4-atlas-ladeszene/` |

## Offen am Gerät

Hardware-Sessions (`Agent: Mensch`) aus Sprints, deren Review schon abgeschlossen ist; erledigt, sobald das Gerät da ist
(`../arbeitsweise.md` › Hardware entkoppelt). Bis dahin gelten die angenommenen Werte aus
[`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6, dort steht auch die Liste aller Hardware-Sessions.

| Session | Gerät | Kriterium (angenommen) | Ordner |
|---|---|---|---|
| DBG2.3 | Controller (PC und Handy geprüft 2026-10-03) | AC-05: Aktionen mit allen drei Eingaben ausprobiert | `erledigt/DBG2-debug-overlay-aktionen/` |

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|
| F5 | INF | Doku-Drift, Version und Landing-Kacheln | Fußzeile „Server <Version> · Client <Version>“ auf der Landingpage am TV, keine toten Kacheln | bereit | `geplant/F5-doku-version/` |
| S1 | SIM | Monarch: Schlag, Fund-Pool und Skills | Go-Tests und Golden-Daten grün (Minimum der Phase 1: Schlag plus ein Skill je Klasse) | Entwurf | `geplant/S1-monarch-schlag-skills/` |
| S2 | SRV | Protokoll für Skills, Speichern beim Verlassen, Spielmetrik | neue Felder in `docs/protocol.md` mit Beispielen unter `testdata/protocol/`, Spielstand nach Trennung mitten in der Nacht, ein Report in `reports/` | bereit | `geplant/S2-protokoll-skills-speichern-metrik/` |
| LT1 | SRV | Lasttest-Werkzeug | Messlauf am Pi mit Tabelle und Bewertung gegen das Ziel (< 10 ms) | bereit | `geplant/LT1-lasttest-werkzeug/` |
| S3 | CLI | Skill-Menü, Tasten und Aktionen-Overlay | 🧑 spielt am Gerät Schlag, Skill, Punkte verteilen und liest die Aktionen im Overlay | bereit | `geplant/S3-skill-menue-overlay/` |
| S4 | CLI | Kamera je Stufe und Layouts 1–4 | 2 Spieler am selben Gerät in verschiedenen Stufen | Entwurf | `geplant/S4-kamera-layouts/` |
| S5 | CLI | Optionen- und Pause-Szene | Einstellungen bleiben nach dem Neuladen erhalten | Entwurf | `geplant/S5-optionen-pause/` |
| S6 | CLI | Onboarding „Erste Nacht geführt“ und Controller-Glyphen | Ein Kind spielt die erste Nacht ohne Erklärung | bereit | `geplant/S6-onboarding-glyphen/` |
| S7 | CLI | Monarch auf dem Standard-Reittier | Zwei Spieler im Split-Screen reiten über die Stufe | bereit | `geplant/S7-monarch-reittier/` |
| P1 | REG 🧑 | Spieleabend 1 | Protokoll und Folge-Tickets | Entwurf | `geplant/P1-spieleabend-1/` |
| W1 | SIM | Hub-Ausbau und Mauerstufen | `task check:go` grün, Tests für Ausbau, Zerstörung und Reparatur, aktualisierte Golden-Daten | bereit | `geplant/W1-hub-ausbau/` |
| W2 | SIM | Plantage, Adern, Stufenbreite und Mine | Tests für Generator, Adern, Plantage und Biome grün, aktualisierte Golden-Level | Entwurf | `geplant/W2-plantage-adern-stufen/` |
| W3 | SIM | Gebäude-Wirkungen | Tests je Gebäude grün, Werte aus den Daten, aktualisierte Golden-Daten | Entwurf | `geplant/W3-gebaeude-wirkungen/` |
| W4 | SIM | Wiederbeleben, Berufe, Händler, Elite und Limit | Tests je Regel grün, aktualisierte Golden-Daten | Entwurf | `geplant/W4-buerger-wiederbeleben/` |
| W5 | SRV | Protokoll für Berufe, Händler, Lager und Hub-Stufe | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | Entwurf | `geplant/W5-protokoll-wirtschaft/` |
| W6 | CLI | Anzeigen für Bau, Lager, Hub und Bürger | HUD und Bauplätze am TV, von 🧑 abgenommen | Entwurf | `geplant/W6-anzeige-wirtschaft/` |
| BR1 | REG 🧑 | Balancing-Runde Wirtschaft und Spieleabend 2 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR1-balancing-wirtschaft/` |
| K1 | SIM | Gegner-Traits, neue Gegner und Elite-KI | Tests je Trait und Gegner grün, aktualisierte Golden-Daten | Entwurf | `geplant/K1-gegner-traits/` |
| K2 | SIM | Bosse, Siegvarianten und Inselwechsel | Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel | Entwurf | `geplant/K2-bosse-siege-inseln/` |
| K3 | SIM | Events Vollmond, Blutmond und Händler-Überfall | Tests je Event grün, aktualisierte Golden-Daten | Entwurf | `geplant/K3-events/` |
| K4 | SRV | Protokoll für Bosse, Events und Inselwechsel | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | Entwurf | `geplant/K4-protokoll-kampf/` |
| K5 | CLI | Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel | Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen | Entwurf | `geplant/K5-anzeige-kampf/` |
| BR2 | REG 🧑 | Balancing-Runde Kampf und Bosse und Spieleabend 3 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR2-balancing-kampf/` |

**Einschiebbar** (Schienen Balancing, Grafik, Sound, Betrieb; unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Thema | Reife | Ordner |
|---|---|---|---|---|
| BAL1 | SIM | Balancing-Tester: Kern und Replay | bereit | `geplant/BAL1-balancing-tester-kern/` |
| BAL2 | SIM | Zielkorridor-Prüfung und `task balance` | bereit | `geplant/BAL2-zielkorridor-pruefung/` |
| BAL3 | SIM | Bot-Profile, Sensitivität und Grad-Kurven | Entwurf | `geplant/BAL3-bot-profile-sensitivitaet/` |
| BAL4 | REG | Abgleich Spielmetrik und Simulator | Entwurf | `geplant/BAL4-metrik-abgleich/` |
| GR1 | CLI | Grafik-Zuordnungstabelle | bereit | `geplant/GR1-grafik-zuordnung/` |
| GR2 | CLI | Grafik-Suche für Lücken | bereit | `geplant/GR2-grafik-suche/` |
| GR3 | CLI | Grafik im Renderer | Entwurf | `geplant/GR3-grafik-renderer/` |
| GR5 | CLI | Juice: Treffer, Screenshake, Münzen | Entwurf | `geplant/GR5-juice/` |
| GR6 | PLAT | Credits-Seite | bereit | `geplant/GR6-credits-seite/` |
| RL1 | INF | Release-Checkliste | bereit | `geplant/RL1-release-checkliste/` |
| SO1 | CLI | Audio-Kern | bereit | `geplant/SO1-audio-kern/` |
| SO2 | CLI | SFX-Katalog und Einbau | Entwurf | `geplant/SO2-sfx-katalog/` |
| SO3 | PLAT | Hörprobenseite `soundtest.html` | bereit | `geplant/SO3-hoerprobenseite/` |
| SO4 | CLI | Musik je Zustand | Entwurf | `geplant/SO4-musik/` |

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
