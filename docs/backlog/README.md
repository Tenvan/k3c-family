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
| [B-009](B-009-komplexitaet-pruefen.md) | INF | Idee | hoch | eingeplant | SP01 | Komplexitäts-Budget wird automatisch geprüft |
| [B-010](B-010-grafik-gebaeude.md) | CLI | Idee | mittel | offen | – | Gebäude, Ressourcen und Hintergrund haben Grafiken |
| [B-011](B-011-sound.md) | CLI | Idee | mittel | offen | – | Spiel hat Sound und Musik |
| [B-012](B-012-mine.md) | SIM | Idee | mittel | offen | – | Mine (Tiefe 2) ist vollständig |
| [B-013](B-013-gegner-elite.md) | SIM | Idee | mittel | offen | – | Restliche Gegner und Elite-KI sind umgesetzt |
| [B-014](B-014-krieger-elite.md) | SIM | Idee | mittel | offen | – | Krieger und Elite-Truppen sind umgesetzt |
| [B-015](B-015-gebaeude-werte.md) | REG | Problem | mittel | offen | – | Gebäude-HP und -Kosten sind gebalanced |
| [B-016](B-016-mehr-lokale-spieler.md) | CLI | Frage | mittel | eingeplant | SP08 | Layout für mehr als zwei lokale Spieler ist entschieden |
| [B-017](B-017-klassen-preset.md) | REG | Frage | mittel | offen | – | Klassen-Presets pro Spieler sind entschieden |
| [B-018](B-018-renderer-aufteilen.md) | CLI | Schuld | mittel | eingeplant | SP08 | worldRenderer und GameScene liegen unter 300 Zeilen |
| [B-019](B-019-test-abdeckung.md) | INF | Idee | niedrig | offen | – | Test-Abdeckung der Engine ist sichtbar |
| [B-020](B-020-smoke-test.md) | INF | Schuld | niedrig | eingeplant | SP03 | Server-Tests laufen lokal wie in der CI |
| [B-021](B-021-taste-x.md) | REG | Frage | mittel | eingeplant | R1 | Belegung der Taste X ist entschieden |
| [B-022](B-022-monarch-spielstand.md) | SIM | Idee | hoch | offen | – | Monarch-Level und Skills stehen im Spielstand |
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | offen | – | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | offen | – | Tiefe 3 und 4 sind beschrieben |
| [B-025](B-025-kampagnen-ziel.md) | REG | Frage | mittel | eingeplant | R1 | Ziel einer Kampagne ist festgelegt |
| [B-026](B-026-skill-tasten.md) | PLAT | Frage | hoch | eingeplant | X1 | Skill-Tasten am Controller sind festgelegt |
| [B-027](B-027-diagnose-absichern.md) | SRV | Problem | hoch | eingeplant | SP03 | Diagnose-Schnittstelle ist abgesichert |
| [B-028](B-028-spielstand-sicherung.md) | SRV | Idee | mittel | eingeplant | SP03 | Spielstände werden rotierend gesichert |
| [B-029](B-029-lade-szene.md) | CLI | Idee | mittel | offen | – | Lade-Szene zeigt Fortschritt |
| [B-030](B-030-wiederverbinden.md) | SRV | Idee | hoch | eingeplant | SP07 | Geräte verbinden sich nach Abbruch wieder |
| [B-031](B-031-online-test.md) | INF | Idee | niedrig | eingeplant | SP07 | Online-Verbindung ist automatisch getestet |
| [B-032](B-032-github-pages.md) | PLAT | Problem | mittel | eingeplant | SP09 | GitHub Pages zeigt nur, was ohne Server geht |
| [B-033](B-033-tests-typecheck.md) | INF | Schuld | mittel | eingeplant | SP01 | Tests werden typgeprüft |
| [B-034](B-034-plat-dateien-aufteilen.md) | PLAT | Schuld | niedrig | offen | – | Große PLAT-Dateien liegen unter 300 Zeilen |
| [B-035](B-035-raspberry-pi.md) | SRV | Idee | hoch | eingeplant | SP11 | Server läuft auf dem Raspberry Pi im Docker |
| [B-036](B-036-mehrere-raeume.md) | SRV | Idee | hoch | eingeplant | SP07 | Mehrere Spiele laufen gleichzeitig |
| [B-037](B-037-lobby.md) | CLI | Idee | mittel | eingeplant | SP08 | Lobby zeigt Räume und startet Spiele |
| [B-038](B-038-lokale-und-online-spieler.md) | SRV | Idee | hoch | eingeplant | SP07 | Lokale und Online-Spieler teilen sich einen Raum |
| [B-039](B-039-interpolation.md) | CLI | Idee | mittel | eingeplant | SP08 | Bewegungen laufen trotz Snapshots flüssig |
| [B-040](B-040-server-finden.md) | SRV | Idee | niedrig | offen | – | Geräte finden den Server im Heimnetz |
| [B-041](B-041-wails-starter.md) | SRV | Idee | niedrig | offen | – | Wails-Starter für Windows existiert |
| [B-042](B-042-pi-leistungsziel.md) | SRV | Frage | hoch | eingeplant | SP11 | Pi-Modell und Leistungsziel sind festgelegt |
| [B-043](B-043-portierungs-fallen.md) | SIM | Problem | hoch | eingeplant | SP04 | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
| [B-044](B-044-planer-struktur.md) | INF | Idee | mittel | erledigt | SP00 | Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen |
| [B-045](B-045-sdd.md) | INF | Idee | hoch | erledigt | SP00 | Tickets und Sprints sind Specs nach Spec-Driven Development |
| [B-046](B-046-dev-mcp.md) | SRV | Idee | mittel | eingeplant | M1 | Entwickler-MCP-Server gibt Agenten verdichteten Zugriff auf Prüfungen, Berichte und Spielstände |
| [B-047](B-047-mcp-raeume-simulation.md) | SRV | Idee | mittel | eingeplant | SP07 | MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen |
| [B-048](B-048-standardbibliothek-in-001.md) | SRV | Frage | niedrig | offen | – | Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist |
| [B-049](B-049-sp09-domaene.md) | INF | Frage | niedrig | offen | – | SP09 bleibt in einer Domäne oder hat einen erlaubten Grenzfall |
| [B-050](B-050-go-dateilaenge.md) | INF | Schuld | mittel | eingeplant | SP01 | Die Dateilänge von Go-Code wird wie bei TypeScript geprüft |
