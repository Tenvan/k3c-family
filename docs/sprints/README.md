# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.
Je Domäne ein aktiver Sprint (B-174); einschiebbare zählen nicht mit.
Prio eines Sprints = höchste Prio seiner Tickets; abgearbeitet wird nach Prio, bei Gleichstand in Tabellen-Reihenfolge.
Reihenfolge in Wellen, Bahnen je Domäne und Zuordnung Mensch/autonom (zwei Accounts): [`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.

## Aktiv

| Sprint | Domäne | Prio | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|---|
| GR3 | CLI | mittel | Grafik im Renderer: Gebäude, Ressourcen, Portale, Truhen, Münzen und Parallax je Biom als Sprites mit Platzhalter-Rückfall (B-010; Sicht am TV offen) | – | `aktiv/GR3-grafik-renderer/` |
| GR5 | CLI | mittel | Juice: Treffer, Screenshake, Münzen (B-164; Abnahme am TV offen) | – | `aktiv/GR5-juice/` |
| GR4 | INF | mittel | Atlas und Lade-Szene (B-163, B-029; Messung an der Xbox offen, GR4.3) | – | `aktiv/GR4-atlas-ladeszene/` |
| DBG3 | PLAT | hoch | Dungeon-Master-Seite `/dm` mit Dev-API, Welle und Tageszeit (B-232; Abnahme am Handy offen) | – | `aktiv/DBG3-dungeon-master-seite/` |
| LT1 | SRV | hoch | Lasttest-Werkzeug `task load`: Bots, Tick-Dauer und CPU im Bericht, Bewertung gegen < 10 ms (B-175; Messlauf am Pi offen) | – | `aktiv/LT1-lasttest-werkzeug/` |
| N2 | CLI | hoch | Flüssige Darstellung: Zeitleiste mit Puffer und Extrapolation, Vorhersage des eigenen Monarchen, Latenz im Debug-Overlay (B-277, B-181; Abnahme am Gerät offen) | – | `aktiv/N2-zeitleiste-vorhersage/` |
| S3 | CLI | hoch | Skill-Menü, Tasten und Aktionen-Overlay | – | `aktiv/S3-skill-menue-overlay/` |
| S4 | CLI | hoch | Kamera je Stufe und Layouts 1–4: Zelle zeigt Stufe, Radar und HUD je Zelle, Mindest-Schrift (B-106; Abnahme am Gerät offen) | – | `aktiv/S4-kamera-layouts/` |
| S5 | CLI | hoch | Optionen- und Pause-Szene mit getrennter Lautstärke, Screenshake/Flash, Farbschwäche-Symbolen und Sprache de/en (B-146, B-172; Abnahme am Gerät offen) | – | `aktiv/S5-optionen-pause/` |
| S7 | CLI | hoch | Monarch beritten auf dem Standard-Reittier: `mountPose`, Reittier-Sheet als einzelne Spritesheets (B-173; Abnahme am Gerät offen) | – | `aktiv/S7-monarch-reittier/` |
| SO1 | CLI | mittel | Audio-Kern: Mixer mit Bus-Lautstärke je Gerät, Entsperren per Eingabe, Sound-Atlas, Positions-Dämpfung, Demo-Ton (B-011 teils; einschiebbar; Hörprobe am TV offen) | – | `aktiv/SO1-audio-kern/` |
| SO3 | PLAT | mittel | Hörprobenseite `soundtest.html` | – | `aktiv/SO3-hoerprobenseite/` |
| RL1 | INF | hoch | Release-Checkliste: Abschnitt „Release“ in `docs/arbeitsweise.md`, Probelauf ohne Tag (B-170; einschiebbar; Pi und Xbox offen) | – | `aktiv/RL1-release-checkliste/` |
| MON2 | PLAT | hoch | Monitoring-Seite `monitor.html`: Ampel je Raum, Verläufe mit Perzentilen, Fehler-Zeitleiste (B-282; Abnahme am Handy offen) | – | `aktiv/MON2-monitoring-seite/` |
| S6 | CLI | hoch | Onboarding „Erste Nacht geführt“ und Controller-Glyphen (B-148, B-149; Abnahme am TV offen) | – | `aktiv/S6-onboarding-glyphen/` |
| W6 | CLI | mittel | Anzeigen für Bau, Lager, Hub und Bürger (B-117, B-126) | HUD und Bauplätze am TV, von 🧑 abgenommen | `aktiv/W6-anzeige-wirtschaft/` |
| W10 | SRV | hoch | Kämpfer-Zahl und Truppen-Limit im Zustand: `sim.EconomyOf` und Protokoll v5 nennen `fighters`, `troopLimit` (B-331; einschiebbar, Domänen-Ausnahme SIM `economy_view`) | `fighters` und `troopLimit` in `docs/protocol.md` und `testdata/protocol/` | `aktiv/W10-truppen-limit-zustand/` |

## Offen am Gerät

Hardware-Sessions (`Agent: Mensch`) aus Sprints, deren Review schon abgeschlossen ist; erledigt, sobald das Gerät da ist
(`../arbeitsweise.md` › Hardware entkoppelt). Bis dahin gelten die angenommenen Werte aus
[`../plan-weiterentwicklung.md`](../plan-weiterentwicklung.md) § 11.6, dort steht auch die Liste aller Hardware-Sessions.

| Session | Gerät | Kriterium (angenommen) | Ordner |
|---|---|---|---|
| GR4.3 | Xbox (Kaltstart, `MAX_TEXTURE_SIZE`) | AC-04: Budget und Texturgröße gemessen (angenommen: 4096) | `aktiv/GR4-atlas-ladeszene/` |
| S6.4 | TV mit Kind (erste Nacht im Grad „leicht“) | AC-06: Führung und Glyphen am TV abgenommen (angenommen: Hinweise verständlich, Glyphen im Split-Viertel lesbar) | `aktiv/S6-onboarding-glyphen/` |
| S4.3 | Xbox am TV (Viertel-Layout lesbar) oder zwei Eingabegeräte; „zwei Stufen“ erst nach B-176 | AC-04: zwei Spieler in verschiedenen Stufen, Layouts 3 und 4 lesbar | `aktiv/S4-kamera-layouts/` |
| LT1.3 | Raspberry Pi (Messlauf 2 Räume × 3 Spieler über eine Nacht, `task load`) | AC-06: Messlauf bewertet, B-042 archiviert (angenommen laut Messung 2026-10-03: Nacht 10,2 bis 10,3 ms, Ziel < 10 ms) | `aktiv/LT1-lasttest-werkzeug/` |
| S5.4 | Xbox am TV und Handy (Optionen/Pause bedienen, englische Texte lesen) | AC-05, AC-07: Szene am TV und Handy abgenommen, englische Texte gelesen | `aktiv/S5-optionen-pause/` |
| SO1.5 | Xbox am TV (Entsperren nach erster Taste, ogg mit mp3-Fallback, Split-Screen-Dämpfung hören; Browser-Pane-Schritte aus SO1.2/SO1.3 offen) | AC-04: Format mit Fallback am TV beobachtet (angenommen: ogg, mp3-Fallback laut X1, B-166) | `aktiv/SO1-audio-kern/` |
| GR3.4 | Xbox am TV (Hub-Stufen und Materialstufen unterscheidbar, 2 Spieler im Split-Screen, Parallax je Biom ansehen) | AC-04, AC-06: Sicht am TV abgenommen (angenommen laut Tests und Browser-Pane) | `aktiv/GR3-grafik-renderer/` |
| GR5.4 | Xbox am TV (Effekte sehen, Blitz-Eindruck, Vibration) | AC-01, AC-02, AC-04: Effekte sichtbar, Schalter „aus“ ruhig (angenommen laut Tests) | `aktiv/GR5-juice/` |
| SO3.3 | Xbox am TV (Hörprobe: Controller bedienen, B frei, View + Menu zurück, Crossfade ohne Knacken, Lautstärke; Browser-Pane-Schritte aus SO3.1/SO3.2 offen) | AC-03, AC-04: Controller-Bedienung und Crossfade am TV bestätigt (angenommen laut Tests) | `aktiv/SO3-hoerprobenseite/` |
| S7.3 | Xbox am TV und Handy (zwei Spieler im Split-Screen reiten: Reittier animiert, Stehen/Laufen/Sprint verschieden, Sattelsitz) | AC-02, AC-04: Darstellung am TV und Handy abgenommen (angenommen laut Tests; Sichtnachweis fehlt auch aus S7.2) | `aktiv/S7-monarch-reittier/` |
| RL1.2 | Raspberry Pi und Xbox (Pi-Image ziehen, Versionszeile der Landingpage gegen den Tag) | AC-03: Punkte „Pi-Image“ und „Version stimmt“ am Gerät (angenommen laut CI und Tests) | `aktiv/RL1-release-checkliste/` |
| N2.4 | Xbox am TV (2 Controller, Split-Screen, Server auf dem Pi; FPS, Latenz, Puffer aus dem Debug-Overlay) | AC-06: kein sichtbares Ruckeln (angenommen laut Tests) | `aktiv/N2-zeitleiste-vorhersage/` |
| DBG3.4 | Handy (neben laufendem Spiel am TV, Server mit `K3C_DEV=1`) | AC-04: `/dm` am Handy bedient, 375 px ohne waagerechtes Scrollen (angenommen laut Tests) | `aktiv/DBG3-dungeon-master-seite/` |
| MON2.4 | Handy (neben `task load` oder am Spieleabend, Server mit `K3C_STATUS_TOKEN`) | AC-05: Monitor am Handy bedient, 375 px ohne waagerechtes Scrollen (angenommen laut Browser-Pane-Nachweis) | `aktiv/MON2-monitoring-seite/` |
| S3.4 | Xbox am TV, Handy und Tastatur (Schlag, Skill-Slots, Skill-Menü, Aktionen-Overlay bedienen und lesen; Overlay und Preisschild können sich überlappen) | AC-05: Slots, Menü, Tasten und Overlay am Gerät abgenommen (angenommen laut Tests und Browser-Pane) | `aktiv/S3-skill-menue-overlay/` |

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Prio | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|---|
| P1 | REG 🧑 | hoch | Spieleabend 1 | Protokoll und Folge-Tickets | Entwurf | `geplant/P1-spieleabend-1/` |
| BR1 | REG 🧑 | hoch | Balancing-Runde Wirtschaft und Spieleabend 2 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR1-balancing-wirtschaft/` |
| K2 | SIM | hoch | Bosse, Siegvarianten und Inselwechsel | Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel | bereit | `geplant/K2-bosse-siege-inseln/` |
| K3 | SIM | niedrig | Events Vollmond, Blutmond und Händler-Überfall | Tests je Event grün, aktualisierte Golden-Daten | bereit | `geplant/K3-events/` |
| K4 | SRV | hoch | Protokoll für Bosse, Events und Inselwechsel | `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün | bereit | `geplant/K4-protokoll-kampf/` |
| K5 | CLI | mittel | Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel | Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen | bereit | `geplant/K5-anzeige-kampf/` |
| BR2 | REG 🧑 | hoch | Balancing-Runde Kampf und Bosse und Spieleabend 3 | Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/` | Entwurf | `geplant/BR2-balancing-kampf/` |
| M9 | SRV | hoch | k3c-dev in Worktrees und Markdown-Ansicht | Planung, Prüfläufe und Dienste treffen den Worktree der Session; Session-Dateien lesbar im Detail-Panel | bereit | `geplant/M9-dev-worktrees/` |
| SV1 | SRV | hoch | Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume | Neuer Raum mit allen fünf Stufen, Level-Betrachter startet einen voll ausgebauten Spielstand | bereit | `geplant/SV1-raeume-stufen-testspielstand/` |
| ST1 | SRV | mittel | Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik | HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt | Entwurf | `geplant/ST1-speichern-robust/` |
| RM1 | SRV | mittel | Raum-Pause im Couch-Raum und lernbare Skills vom Server | Pause hält den Couch-Raum an; Skill-Menü zeigt nur, was der Server annimmt | Entwurf | `geplant/RM1-pause-lernbare-skills/` |
| W7 | SIM | hoch | Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig | Passive Burg kann fallen, Spielstand stellt W2–W4 wieder her, Golden-Hub geprüft | bereit | `geplant/W7-ausruestung-spielstand/` |
| LV1 | SIM | mittel | Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs | Keine Lava auf Linien, Camps mit Abstand, besiegte Gegner lassen Gold fallen, Ereignisse mit Ort | Entwurf | `geplant/LV1-level-gegner-korrektur/` |
| SK1 | SIM | hoch | Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar | Skill-Baum spielbar, Respec und Lernen ohne Seiteneffekt prüfbar | bereit | `geplant/SK1-skill-baum/` |
| RG1 | REG | mittel | Werte-Runde: Burg hält Nacht, Hub-Stufe 4–5, Adern-Takt | Beschlüsse in `docs/rules/`, geänderte Werte in `data/`, `task balance` grün | Entwurf | `geplant/RG1-werte-burg-hub-adern/` |
| U5 | CLI | hoch | Debug-Overlay und Cheat-Dialog bedienbar | Ö schließt Overlay und Liste, HUD bleibt lesbar, Cheat-Dialog mit Fokus und Controller | bereit | `geplant/U5-debug-overlay-bedienbar/` |
| S8 | CLI | hoch | Spielmenü „Spiel verlassen“, Y-Belegung und Glyphen-Entscheidung | Spielmenü verlässt ins Lobby, Y ohne Bau-Menü, Glyph-Entscheidung umgesetzt | bereit | `geplant/S8-spielmenue-bedienung/` |
| W8 | CLI | mittel | Bauplätze mit Grund und alle Rohstoffe im Client | Gesperrte Plätze zeigen den Grund, Client-Typen passen zu `hub.json` und den fünf Rohstoffen | Entwurf | `geplant/W8-bauplaetze-rohstoffe-client/` |
| PF1 | CLI | hoch | Split-Screen flüssig auf der Xbox | Zwei Spieler im Split-Screen ohne sichtbares Ruckeln auf der Xbox | bereit | `geplant/PF1-splitscreen-leistung/` |
| LB1 | CLI | mittel | Lobby zeigt Räume und startet Spiele | Lobby listet offene Räume, Beitritt ohne Raumcode | Entwurf | `geplant/LB1-lobby/` |
| PL1 | PLAT | hoch | Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte | „Neues Spiel“ startet immer neu, zwei Tastatur-Spieler, Overlay per Controller | bereit | `geplant/PL1-start-tastatur-koop-texte/` |
| CI1 | INF | hoch | CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung | CI grün mit SP01-Prüfungen, Release-Image lehnt Dev-Aktionen ab, Abdeckung im CI-Bericht | Entwurf | `geplant/CI1-ci-release-image/` |
| S9 | CLI | hoch | Rückmeldung für Schlag und Skills, ein Hinweis je Spieler | Jeder Tastendruck auf Schlag oder Skill ist sichtbar, das Aktionen-Overlay zeigt je Spieler einen Hinweis | bereit | `geplant/S9-rueckmeldung-overlay/` |

**Einschiebbar** (Schienen Balancing, Grafik, Sound, Betrieb; unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Prio | Thema | Reife | Ordner |
|---|---|---|---|---|---|
| BAL4 | REG | mittel | Abgleich Spielmetrik und Simulator | Entwurf | `geplant/BAL4-metrik-abgleich/` |
| SO2 | CLI | mittel | SFX-Katalog und Einbau | Entwurf | `geplant/SO2-sfx-katalog/` |
| SO4 | CLI | mittel | Musik je Zustand | Entwurf | `geplant/SO4-musik/` |
| M10 | SRV | mittel | Ressourcen-Manager für Grafik- und Sound-Slots in k3c-dev | Entwurf | `geplant/M10-ressourcen-manager/` |
| NT1 | SRV | mittel | Stabile Tests, Warteschlange und Snapshot-Budget | Entwurf | `geplant/NT1-netz-tests-stabil/` |
| BT1 | SRV | niedrig | Server im Heimnetz finden, Windows-Starter, Start mit Seed | Entwurf | `geplant/BT1-heimnetz-start/` |
| BAL5 | SIM | mittel | Balancing-Tester: Profil „Mauern zuerst“ und Sensitivität ohne Wirkung | Entwurf | `geplant/BAL5-balancing-tester-nachschaerfen/` |
| RG2 | REG | mittel | Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4 | Entwurf | `geplant/RG2-regelwerk-klaerungen/` |
| GR7 | CLI | mittel | Figuren-Lücken, ganzzahlige Skalierung und Schrift | Entwurf | `geplant/GR7-figuren-schrift/` |
| SO5 | CLI | niedrig | Audio-Kern: ganze Dateien mit Crossfade, Ambient-Lautstärke | Entwurf | `geplant/SO5-audio-kern-dateien/` |
| HW1 | PLAT | niedrig | Zurückgestellte Controller-Prüfungen nachholen | Entwurf | `geplant/HW1-controller-pruefungen/` |
| PG1 | PLAT | niedrig | Präsentationsseite mit echten Spielbildern | Entwurf | `geplant/PG1-praesentation-bilder/` |
| DBG4 | PLAT | mittel | Dev-Seite Asset-Vorschau im Spielmaßstab | Entwurf | `geplant/DBG4-asset-vorschau/` |
| RP1 | INF | mittel | Repo-Hygiene: Branches aufräumen, Altlasten, Regeln | Entwurf | `geplant/RP1-repo-hygiene/` |
| PB1 | INF | niedrig | Veröffentlichung auf itch.io | Entwurf | `geplant/PB1-itch-io/` |
| PL2 | PLAT | niedrig | Werkzeug-Seiten in der gewählten Sprache | Entwurf | `geplant/PL2-texte-werkzeug-seiten/` |

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
