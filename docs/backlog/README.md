# Backlog

Alle Tickets, eine Zeile pro Ticket. Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md).
Jedes Ticket ist eine Spec (SDD, siehe [`../arbeitsweise.md`](../arbeitsweise.md)). Neues Ticket: nächste freie Nummer,
Datei `B-NNN-kurzname.md` aus der Vorlage, `Spec: Entwurf`, Zeile hier ergänzen. `task test` prüft beides.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-004](B-004-regelwerk.md) | REG | Idee | hoch | offen | – | Regelwerk ist ausführlich diskutiert und ausgearbeitet |
| [B-006](B-006-xbox-gamepad-test.md) | PLAT | Frage | hoch | eingeplant | X1 | Gamepad-Test auf der Xbox ist ausgewertet |
| [B-007](B-007-skill-baum.md) | SIM | Idee | hoch | offen | – | Skill-Baum mit Tank und Zauberer ist spielbar |
| [B-008](B-008-spieleabend.md) | REG | Frage | hoch | offen | – | Familie hat einen Spieleabend gespielt und Feedback gegeben |
| [B-010](B-010-grafik-gebaeude.md) | CLI | Idee | mittel | offen | – | Gebäude, Ressourcen und Hintergrund haben Grafiken |
| [B-011](B-011-sound.md) | CLI | Idee | mittel | offen | – | Spiel hat Sound und Musik |
| [B-012](B-012-mine.md) | SIM | Idee | mittel | offen | – | Mine (Tiefe 2) ist vollständig |
| [B-013](B-013-gegner-elite.md) | SIM | Idee | mittel | offen | – | Restliche Gegner und Elite-KI sind umgesetzt |
| [B-014](B-014-krieger-elite.md) | SIM | Idee | mittel | offen | – | Krieger und Elite-Truppen sind umgesetzt |
| [B-015](B-015-gebaeude-werte.md) | REG | Problem | mittel | offen | – | Gebäude-HP und -Kosten sind gebalanced |
| [B-017](B-017-klassen-preset.md) | REG | Frage | mittel | offen | – | Klassen-Presets pro Spieler sind entschieden |
| [B-019](B-019-test-abdeckung.md) | INF | Idee | niedrig | offen | – | Test-Abdeckung der Engine ist sichtbar |
| [B-022](B-022-monarch-spielstand.md) | SIM | Idee | hoch | offen | – | Monarch-Level und Skills stehen im Spielstand |
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | offen | – | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | offen | – | Tiefe 3 und 4 sind beschrieben |
| [B-026](B-026-skill-tasten.md) | PLAT | Frage | hoch | eingeplant | X1 | Skill-Tasten am Controller sind festgelegt |
| [B-029](B-029-lade-szene.md) | CLI | Idee | mittel | offen | – | Lade-Szene zeigt Fortschritt |
| [B-035](B-035-raspberry-pi.md) | SRV | Idee | hoch | eingeplant | SP11 | Server läuft auf dem Raspberry Pi im Docker |
| [B-037](B-037-lobby.md) | CLI | Idee | mittel | offen | – | Lobby zeigt Räume und startet Spiele |
| [B-040](B-040-server-finden.md) | SRV | Idee | niedrig | offen | – | Geräte finden den Server im Heimnetz |
| [B-041](B-041-wails-starter.md) | SRV | Idee | niedrig | offen | – | Wails-Starter für Windows existiert |
| [B-042](B-042-pi-leistungsziel.md) | SRV | Frage | hoch | eingeplant | SP11 | Pi-Modell und Leistungsziel sind festgelegt |
| [B-048](B-048-standardbibliothek-in-001.md) | SRV | Frage | niedrig | offen | – | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist |
| [B-053](B-053-ci-lauf-sp01.md) | INF | Problem | hoch | offen | – | Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen |
| [B-058](B-058-execution-policy.md) | INF | Frage | niedrig | offen | – | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 |
| [B-071](B-071-golden-arm64.md) | INF | Problem | mittel | offen | – | Die Golden-Tests laufen auch auf arm64 grün |
| [B-075](B-075-golden-spielstand-hub.md) | SIM | Schuld | mittel | offen | – | Der Golden-Spielstand enthält einen gebauten und veränderten Hub |
| [B-079](B-079-landing-kacheln-lobby.md) | PLAT | Schuld | mittel | offen | – | Die Kacheln der Landingpage passen zum Start über die Lobby |
| [B-080](B-080-dev-tasten-server.md) | SRV | Idee | niedrig | offen | – | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
| [B-090](B-090-radar.md) | CLI | Idee | mittel | eingeplant | U1 | Ein Radar im HUD zeigt Burg, Portale, Ausgang, Mitspieler und Gegner |
| [B-092](B-092-level-betrachter.md) | PLAT | Idee | mittel | eingeplant | U3 | Eine Testseite zeigt ein generiertes Level (Seed und Biom) ohne zu spielen |
| [B-094](B-094-npm-reste.md) | INF | Schuld | niedrig | offen | – | Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr |
| [B-095](B-095-start-mit-seed-und-tiefe.md) | SRV | Idee | niedrig | offen | – | Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe |
| [B-098](B-098-debug-overlay-standard-zurueck.md) | CLI | Schuld | niedrig | offen | – | Das Debug-Overlay ist vor dem Release wieder nur mit ?dev=1 verfügbar |
| [B-099](B-099-balancing-tester.md) | SIM | Idee | mittel | offen | – | Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele |
| [B-100](B-100-mehrstufen-insel.md) | SIM | Idee | hoch | offen | – | Eine Insel hat n Stufen, die alle laufen und pro Spieler begehbar sind |
| [B-101](B-101-raum-optionen-schwierigkeit.md) | SIM | Idee | hoch | offen | – | Raum-Optionen und fünf Schwierigkeitsgrade wirken in der Simulation |
| [B-102](B-102-siegvarianten-niederlage.md) | SIM | Idee | mittel | offen | – | Siegvarianten und Niederlage-Modi der Raum-Optionen sind umgesetzt |
| [B-103](B-103-inseln-bosse.md) | SIM | Idee | mittel | offen | – | Inseln mit Endboss und gemeinsamem Inselwechsel sind spielbar |
| [B-104](B-104-protokoll-stufe-und-optionen.md) | SRV | Idee | hoch | offen | – | Das Protokoll kennt die Stufe je Spieler und die Raum-Optionen |
| [B-105](B-105-anlegen-dialog-optionen.md) | CLI | Idee | mittel | offen | – | Der Anlegen-Dialog der Lobby wählt Grad, Ziel und Niederlage-Modus |
| [B-106](B-106-kamera-je-stufe.md) | CLI | Idee | hoch | offen | – | Jeder Spieler sieht seine Stufe, auch wenn die Spieler in verschiedenen Stufen sind |
| [B-107](B-107-debug-panel-gradwechsel.md) | CLI | Idee | mittel | offen | – | Ein Debug-Panel im Dev-Mode wechselt den Schwierigkeitsgrad und weitere Optionen |
| [B-109](B-109-materialien-gebaeude-regelwerk.md) | REG | Idee | hoch | eingeplant | R2 | Materialien und Gebäude sind im Regelwerk beschlossen |
| [B-110](B-110-skillung-klassen-level.md) | REG | Idee | hoch | offen | – | Skillung, Klassen und Level von Monarchen und Bürgern sind im Regelwerk beschlossen |
| [B-111](B-111-materialmengen-je-stufe.md) | REG | Frage | hoch | offen | – | Die Materialmengen je Stufe passen zu den Kosten von Hub-Ausbau, Mauern und Gebäuden |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/) (nur auf Nachfrage lesen); die Datei bleibt beim Verschieben
unverändert. Ändert ein Ticket seinen Status auf `erledigt` oder `verworfen`, wandert es per `git mv` dorthin und seine
Zeile in diesen Abschnitt.

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-001](archiv/B-001-server-framework.md) | SRV | Idee | niedrig | erledigt | SP00 | Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird |
| [B-002](archiv/B-002-diagnose-tui.md) | SRV | Idee | mittel | erledigt | SP10 | Diagnose-TUI zeigt den laufenden Server |
| [B-003](archiv/B-003-server-sprache.md) | SRV | Frage | mittel | erledigt | SP00 | Server-Sprache ist entschieden |
| [B-009](archiv/B-009-komplexitaet-pruefen.md) | INF | Idee | hoch | erledigt | SP01 | Komplexitäts-Budget wird automatisch geprüft |
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
