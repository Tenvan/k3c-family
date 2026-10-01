# Backlog

Alle Tickets, eine Zeile pro Ticket. Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md).
Jedes Ticket ist eine Spec (SDD, siehe [`../arbeitsweise.md`](../arbeitsweise.md)). Neues Ticket: nächste freie Nummer,
Datei `B-NNN-kurzname.md` aus der Vorlage, `Spec: Entwurf`, Zeile hier ergänzen. `npm test` prüft beides.

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-001](B-001-server-framework.md) | SRV | Idee | niedrig | erledigt | SP00 | Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird |
| [B-002](B-002-diagnose-tui.md) | SRV | Idee | mittel | eingeplant | SP10 | Diagnose-TUI zeigt den laufenden Server |
| [B-003](B-003-server-sprache.md) | SRV | Frage | mittel | erledigt | SP00 | Server-Sprache ist entschieden |
| [B-004](B-004-regelwerk.md) | REG | Idee | hoch | eingeplant | R1 | Regelwerk ist ausführlich diskutiert und ausgearbeitet |
| [B-005](B-005-online-koop-ziel.md) | REG | Problem | hoch | eingeplant | R1 | Game-Design nennt gemischten Koop als Kern |
| [B-006](B-006-xbox-gamepad-test.md) | PLAT | Frage | hoch | eingeplant | X1 | Gamepad-Test auf der Xbox ist ausgewertet |
| [B-007](B-007-skill-baum.md) | SIM | Idee | hoch | offen | – | Skill-Baum mit Tank und Zauberer ist spielbar |
| [B-008](B-008-spieleabend.md) | REG | Frage | hoch | offen | – | Familie hat einen Spieleabend gespielt und Feedback gegeben |
| [B-009](B-009-komplexitaet-pruefen.md) | INF | Idee | hoch | erledigt | SP01 | Komplexitäts-Budget wird automatisch geprüft |
| [B-010](B-010-grafik-gebaeude.md) | CLI | Idee | mittel | offen | – | Gebäude, Ressourcen und Hintergrund haben Grafiken |
| [B-011](B-011-sound.md) | CLI | Idee | mittel | offen | – | Spiel hat Sound und Musik |
| [B-012](B-012-mine.md) | SIM | Idee | mittel | offen | – | Mine (Tiefe 2) ist vollständig |
| [B-013](B-013-gegner-elite.md) | SIM | Idee | mittel | offen | – | Restliche Gegner und Elite-KI sind umgesetzt |
| [B-014](B-014-krieger-elite.md) | SIM | Idee | mittel | offen | – | Krieger und Elite-Truppen sind umgesetzt |
| [B-015](B-015-gebaeude-werte.md) | REG | Problem | mittel | offen | – | Gebäude-HP und -Kosten sind gebalanced |
| [B-016](B-016-mehr-lokale-spieler.md) | CLI | Frage | mittel | eingeplant | SP08 | Layout für mehr als zwei lokale Spieler ist entschieden |
| [B-017](B-017-klassen-preset.md) | REG | Frage | mittel | offen | – | Klassen-Presets pro Spieler sind entschieden |
| [B-018](B-018-renderer-aufteilen.md) | CLI | Schuld | mittel | verworfen | – | worldRenderer und GameScene liegen unter 300 Zeilen |
| [B-019](B-019-test-abdeckung.md) | INF | Idee | niedrig | offen | – | Test-Abdeckung der Engine ist sichtbar |
| [B-020](B-020-smoke-test.md) | INF | Schuld | niedrig | erledigt | SP03 | Server-Tests laufen lokal wie in der CI |
| [B-021](B-021-taste-x.md) | REG | Frage | mittel | eingeplant | R1 | Belegung der Taste X ist entschieden |
| [B-022](B-022-monarch-spielstand.md) | SIM | Idee | hoch | offen | – | Monarch-Level und Skills stehen im Spielstand |
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | offen | – | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | offen | – | Tiefe 3 und 4 sind beschrieben |
| [B-025](B-025-kampagnen-ziel.md) | REG | Frage | mittel | eingeplant | R1 | Ziel einer Kampagne ist festgelegt |
| [B-026](B-026-skill-tasten.md) | PLAT | Frage | hoch | eingeplant | X1 | Skill-Tasten am Controller sind festgelegt |
| [B-027](B-027-diagnose-absichern.md) | SRV | Problem | hoch | erledigt | SP03 | Diagnose-Schnittstelle ist abgesichert |
| [B-028](B-028-spielstand-sicherung.md) | SRV | Idee | mittel | erledigt | SP03 | Spielstände werden rotierend gesichert |
| [B-029](B-029-lade-szene.md) | CLI | Idee | mittel | offen | – | Lade-Szene zeigt Fortschritt |
| [B-030](B-030-wiederverbinden.md) | SRV | Idee | hoch | erledigt | SP07 | Geräte verbinden sich nach Abbruch wieder |
| [B-031](B-031-online-test.md) | INF | Idee | niedrig | erledigt | SP07 | Online-Verbindung ist automatisch getestet |
| [B-032](B-032-github-pages.md) | PLAT | Problem | mittel | eingeplant | SP09 | GitHub Pages zeigt nur, was ohne Server geht |
| [B-033](B-033-tests-typecheck.md) | INF | Schuld | mittel | erledigt | SP01 | Tests werden typgeprüft |
| [B-034](B-034-plat-dateien-aufteilen.md) | PLAT | Schuld | niedrig | verworfen | – | Große PLAT-Dateien liegen unter 300 Zeilen |
| [B-035](B-035-raspberry-pi.md) | SRV | Idee | hoch | eingeplant | SP11 | Server läuft auf dem Raspberry Pi im Docker |
| [B-036](B-036-mehrere-raeume.md) | SRV | Idee | hoch | erledigt | SP07 | Mehrere Spiele laufen gleichzeitig |
| [B-037](B-037-lobby.md) | CLI | Idee | mittel | eingeplant | SP08 | Lobby zeigt Räume und startet Spiele |
| [B-038](B-038-lokale-und-online-spieler.md) | SRV | Idee | hoch | erledigt | SP07 | Lokale und Online-Spieler teilen sich einen Raum |
| [B-039](B-039-interpolation.md) | CLI | Idee | mittel | eingeplant | SP08 | Bewegungen laufen trotz Snapshots flüssig |
| [B-040](B-040-server-finden.md) | SRV | Idee | niedrig | offen | – | Geräte finden den Server im Heimnetz |
| [B-041](B-041-wails-starter.md) | SRV | Idee | niedrig | offen | – | Wails-Starter für Windows existiert |
| [B-042](B-042-pi-leistungsziel.md) | SRV | Frage | hoch | eingeplant | SP11 | Pi-Modell und Leistungsziel sind festgelegt |
| [B-043](B-043-portierungs-fallen.md) | SIM | Problem | hoch | erledigt | SP06 | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
| [B-044](B-044-planer-struktur.md) | INF | Idee | mittel | erledigt | SP00 | Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen |
| [B-045](B-045-sdd.md) | INF | Idee | hoch | erledigt | SP00 | Tickets und Sprints sind Specs nach Spec-Driven Development |
| [B-046](B-046-dev-mcp.md) | SRV | Idee | mittel | erledigt | M1 | Entwickler-Werkzeug k3c-dev gibt Agenten über MCP verdichteten Zugriff auf Prüfungen und Logs |
| [B-047](B-047-mcp-raeume-simulation.md) | SRV | Idee | mittel | eingeplant | M6 | MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen |
| [B-048](B-048-standardbibliothek-in-001.md) | SRV | Frage | niedrig | offen | – | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist |
| [B-049](B-049-sp09-domaene.md) | INF | Frage | niedrig | offen | – | SP09 bleibt in einer Domäne oder hat einen erlaubten Grenzfall |
| [B-050](B-050-go-dateilaenge.md) | INF | Schuld | mittel | erledigt | SP01 | Die Dateilänge von Go-Code wird wie bei TypeScript geprüft |
| [B-051](B-051-oxlint-warnungen.md) | INF | Schuld | niedrig | offen | – | Oxlint meldet im Bestand keine Warnungen mehr |
| [B-052](B-052-requirements.md) | INF | Idee | mittel | erledigt | SP01 | Alle vorausgesetzten Installationen stehen in requirements.md |
| [B-053](B-053-ci-lauf-sp01.md) | INF | Problem | hoch | offen | – | Die CI hat die Prüfungen aus SP01 einmal grün durchlaufen |
| [B-054](B-054-go-verschachtelung.md) | INF | Problem | mittel | erledigt | L1 | Die Verschachtelung von Go-Code wird als Tiefe geprüft |
| [B-055](B-055-server-lint.md) | INF | Problem | niedrig | verworfen | – | Das Komplexitäts-Budget gilt auch für server/*.mjs |
| [B-056](B-056-ratsche-nachziehen.md) | INF | Schuld | niedrig | verworfen | – | Die Ratsche zieht gesunkene Werte automatisch nach |
| [B-057](B-057-go-tiefe-range.md) | INF | Problem | mittel | erledigt | L2 | Die Go-Verschachtelung zählt `for range` und `else if` wie TypeScript |
| [B-058](B-058-execution-policy.md) | INF | Frage | niedrig | offen | – | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 |
| [B-059](B-059-freie-monarchen-reisen-mit.md) | SIM | Idee | hoch | erledigt | SP06 | Nur gesteuerte Monarchen entscheiden über den Stufenwechsel |
| [B-060](B-060-sp07-protokoll-regeln.md) | SRV | Problem | hoch | erledigt | SP07 | Die Spec von SP07 deckt alle Server-Regeln aus Protokoll v2 ab |
| [B-061](B-061-sp08-protokoll-regeln.md) | CLI | Problem | hoch | eingeplant | SP08 | Die Spec von SP08 deckt alle Client-Regeln aus Protokoll v2 ab |
| [B-062](B-062-dev-nutzungsstatistik.md) | SRV | Idee | mittel | erledigt | M2 | k3c-dev wertet MCP-Aufrufe über Sitzungen aus: Perzentile, Ausreißer und Zeitreihe |
| [B-063](B-063-dev-berichte-spielstaende.md) | SRV | Idee | mittel | erledigt | M2 | k3c-dev macht Xbox-Berichte und Spielstände für Agenten lesbar |
| [B-064](B-064-dev-oberflaeche-logs.md) | SRV | Idee | mittel | erledigt | M4 | k3c-dev hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs |
| [B-065](B-065-dev-mcp-seite.md) | SRV | Idee | mittel | erledigt | M5 | k3c-dev zeigt auf der MCP-Seite Server, Tools, Live-Monitore, Aufruf-Log und Statistik |
| [B-066](B-066-server-json-log.md) | SRV | Idee | mittel | offen | – | Der Go-Server schreibt sein Log als JSON nach logs/ |
| [B-067](B-067-dev-dienste.md) | SRV | Idee | mittel | erledigt | M3 | k3c-dev startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten |
| [B-068](B-068-dev-dienste-seite.md) | SRV | Idee | mittel | erledigt | M4 | k3c-dev zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level |
| [B-069](B-069-ci-k3c-dev.md) | INF | Problem | mittel | erledigt | – | Der CI-Job k3c-dev ist einmal grün gelaufen |
| [B-070](B-070-gitignore-verankern.md) | INF | Schuld | niedrig | offen | – | Die .gitignore ignoriert reports/, saves/ und certs/ nur an der Repo-Wurzel |
| [B-071](B-071-golden-arm64.md) | INF | Problem | mittel | offen | – | Die Golden-Tests laufen auch auf arm64 grün |
| [B-072](B-072-depguard-rng.md) | INF | Schuld | niedrig | offen | – | depguard prüft die Schichtgrenze auch für engine/rng |
| [B-073](B-073-go-task-umstellen.md) | INF | Schuld | hoch | offen | – | Alle Aufrufer nutzen Go Task statt npm-Skripte |
| [B-074](B-074-golden-wirtschaft-luecken.md) | SIM | Problem | mittel | erledigt | SP06 | Golden-Läufe decken Tragen, Bauen, Bögen, Truhen und Münz-Rückgabe ab |
| [B-075](B-075-golden-spielstand-hub.md) | SIM | Schuld | mittel | offen | – | Der Golden-Spielstand enthält einen gebauten und veränderten Hub |
| [B-076](B-076-websocket-bibliothek.md) | INF | Frage | hoch | erledigt | SP07 | Der Go-Server spricht WebSocket über github.com/coder/websocket |
| [B-077](B-077-race-detector.md) | INF | Schuld | hoch | erledigt | L3 | Die nebenläufigen Go-Pakete werden mit dem Race-Detector geprüft |
| [B-078](B-078-dev-proxy-go-server.md) | INF | Schuld | hoch | offen | – | `task dev` leitet `/ws` an den Go-Server weiter |
| [B-079](B-079-landing-kacheln-lobby.md) | PLAT | Schuld | mittel | offen | – | Die Kacheln der Landingpage passen zum Start über die Lobby |
| [B-080](B-080-dev-tasten-server.md) | SRV | Idee | niedrig | offen | – | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
| [B-081](B-081-testseite-szenarien.md) | PLAT | Idee | mittel | eingeplant | T1 | Eine Testseite startet Test-Szenarien, zuerst 1–4 Spieler mit Mock-Spielern |
| [B-082](B-082-start-parameter-mock.md) | CLI | Idee | mittel | eingeplant | SP08 | `game.html` startet per Parameter ohne Auswahl und mit Mock-Slots |
