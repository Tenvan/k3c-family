# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-023](B-023-itch-io.md) | INF | Idee | niedrig | eingeplant | PB1 | Spiel ist auf itch.io veröffentlicht |
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | eingeplant | RG2 | Tiefe 3 und 4 sind beschrieben |
| [B-041](B-041-wails-starter.md) | DEV | Idee | niedrig | eingeplant | BT1 | Wails-Starter für Windows existiert |
| [B-185](B-185-verluste-je-welle-angleichen.md) | REG | Schuld | niedrig | eingeplant | RG2 | Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger |
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-219](B-219-doku-gating-und-schlag.md) | REG | Schuld | niedrig | eingeplant | RG2 | Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen |
| [B-295](B-295-handwerker-schmiede-ruestkammer.md) | REG | Frage | mittel | eingeplant | RG2 | Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden |
| [B-315](B-315-spielstand-voll-ausgebaut.md) | SRV | Idee | hoch | eingeplant | SV1 | Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut |
| [B-363](B-363-planung-flags-bearbeiten.md) | DEV | Idee | mittel | eingeplant | WZ2 | Die Planungsseite ändert Prio, Umgebung, Agent und Projekt direkt im Detail |
| [B-400](B-400-planung-splitter-merken.md) | DEV | Idee | niedrig | eingeplant | WZ2 | Die Planungsseite trennt Backlog und Details mit einem Splitter, dessen Stellung gemerkt wird |
| [B-401](B-401-planung-filter-getrennt.md) | DEV | Idee | niedrig | eingeplant | WZ2 | Der Backlog-Filter wirkt nur auf den Backlog, der Projekt-Filter nur auf die Projekte |
| [B-402](B-402-planung-session-titel.md) | DEV | Idee | mittel | eingeplant | WZ2 | Die Planungsseite zeigt in den Session-Zeilen den Titel der Session |
| [B-409](B-409-bauplaetze-abstand-zu-eng.md) | REG | Problem | mittel | eingeplant | W11 | Bauplätze im Hub stehen zu eng, besonders die Treppen |
| [B-410](B-410-aktion-ausbilden-tauschen-protokoll.md) | SRV | Problem | mittel | eingeplant | W11 | Aktionen-Overlay zeigt Ausbilden und Tauschen nicht, der Server liefert sie nicht in actions |
| [B-454](B-454-baeume-hoeher-dichter-hoehle-wurzeln.md) | DEV | Problem | hoch | eingeplant | GEB10 | Bäume höher und dichter, Höhle mit eigener Unterwelt-Flora (Wurzeln) |
| [B-458](B-458-hub-ausbau-wachstum-feinstufen.md) | DEV | Idee | hoch | eingeplant | GEB11 | Hub wächst in Ausbaustufen sichtbar und kleinteilig (Lagerfeuer bis Burg, K2C-Stil) |
| [B-622](B-622-fackeln-als-spielobjekt.md) | SIM | Idee | niedrig | eingeplant | W15 | Fackeln gibt es als Spielobjekt mit Licht in der Nacht |
| [B-635](B-635-muenzskala-2-bis-30.md) | REG | Idee | hoch | eingeplant | BS2 | Münzpreise liegen auf einer kleinen Skala von 2 bis 30 Gold |
| [B-636](B-636-farm-heisst-baumschule.md) | SIM | Idee | hoch | eingeplant | BS4 | Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen |
| [B-637](B-637-hafen-chunk-boot.md) | SIM | Idee | hoch | eingeplant | BS5 | Ein Hafen-Chunk auf Inselstufe 1 baut das Boot zum Verlassen der Insel |
| [B-650](B-650-boss-hp-je-spieler-eigenes-feld.md) | SIM | Idee | hoch | eingeplant | BS3 | Boss-HP je Zusatzspieler bekommt ein eigenes Feld, getrennt von der Wellengröße |
| [B-652](B-652-mauern-tuerme-verteilt-tor-ist-aeusserste-mauer.md) | SIM | Idee | hoch | eingeplant | BS12 | Mauern und Türme stehen verteilt in jedem Chunk, das Tor ist die äußerste gebaute Mauer |
| [B-680](B-680-welt-werte-ueberschreiben.md) | REG | Idee | hoch | eingeplant | BAL8 | Eine Welt-Datei erbt die Basis-Werte und übersteuert nur, was sie ändern will |
| [B-681](B-681-sim-werte-je-raum.md) | SIM | Idee | hoch | eingeplant | BAL8 | Die Simulation rechnet mit den Werten ihrer Welt statt mit globalen Datei-Variablen |
| [B-699](B-699-papierschnitt-grafik-mcp-review.md) | DEV | Idee | hoch | eingeplant | MG11 | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench |
| [B-711](B-711-reiter-variante-reittier.md) | PAP | Frage | mittel | eingeplant | MG9 | Variante des Reiters tauscht die Materialien des Reittiers nicht |
| [B-720](B-720-bosse-neu.md) | PAP | Problem | hoch | eingeplant | MG10 | Bosse sind neu gestaltet, jeder mit eigener Silhouette |
| [B-721](B-721-gebaeude-stufen-form.md) | PAP | Problem | mittel | eingeplant | MG11 | Gebäude steigern sich in Form und Material, Kristall leuchtet, nichts wird abgeschnitten |
| [B-722](B-722-hintergruende-neu-ohne-baeume.md) | PAP | Problem | hoch | eingeplant | MG11 | Hintergründe sind neu gestaltet, mit Tiefe und ohne Bäume |
| [B-723](B-723-natur-icons-muenze.md) | PAP | Problem | mittel | eingeplant | MG11 | Natur hat Spielhöhe und mehrere Formen, Icons sind bei 32 px lesbar, die Münze dreht sich |
| [B-730](B-730-check-diff-schneller.md) | INF | Problem | hoch | offen | WZ40 | check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel |
| [B-727](B-727-styleguide-prompt-buttons.md) | DEV | Idee | mittel | eingeplant | WZ45 | Der Styleguide hat Prompt-Buttons für Materialien und Töne |
| [B-728](B-728-sound-seite-filter-prompts.md) | DEV | Idee | mittel | eingeplant | WZ45 | Die Sound-Seite hat Filter und Prompt-Buttons |
| [B-729](B-729-quellen-seite-filter-prompts.md) | DEV | Idee | mittel | eingeplant | WZ45 | Die Quellen-Seite hat Filter und Prompt-Buttons |
| [B-746](B-746-workbench-tests-ohne-engine-server.md) | DEV | Schuld | niedrig | offen | – | Workbench-Tests laufen gegen einen Fake-Spielserver statt engine/net und engine/room |
| [B-775](B-775-native-umstellung-rust-bevy.md) | CLI | Idee | hoch | eingeplant | NG9 | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server |
| [B-777](B-777-ebitengine-prototyp-vergleich.md) | CLI | Frage | hoch | eingeplant | NV1 | Ein Ebitengine-Prototyp zeigt dasselbe Level wie der Bevy-Prototyp und liefert vergleichbare Messwerte |
| [B-778](B-778-rig-prototyp-monarch-aus-teilen.md) | CLI | Frage | hoch | eingeplant | NV2 | Ein dritter Prototyp setzt den Monarchen in Ebitengine zur Laufzeit aus Rig-Teilen zusammen |
| [B-780](B-780-rig-fuer-alle-figuren-gegnerzahl.md) | CLI | Frage | mittel | eingeplant | NG3 | Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen |
| [B-833](B-833-wz41-abnahme-ci-develop-check-all.md) | INF | Schuld | hoch | offen | – | Abnahme WZ41: CI auf develop mit Tests grün, check:all nachgeholt |
| [B-836](B-836-abnahme-laeufe-cu1.md) | INF | Schuld | mittel | offen | CU1 | Abnahme-Läufe CU1 |
| [B-838](B-838-wellen-sessions-eines-sprints.md) | DEV | Frage | mittel | offen | – | Regel für Sessions desselben Sprints in einer Welle ist entschieden und in Code und Doku gleich |
| [B-827](B-827-native-build-nach-dist.md) | CLI | Problem | mittel | offen | – | task native:game:build legt das komplette Spiel nach dist/k3c-game und kopiert geänderte Grafik neu |
| [B-846](B-846-server-ohne-client-auslieferung.md) | SRV | Schuld | niedrig | offen | – | Der Spielserver startet und läuft ohne dist/ und liefert keinen Browser-Client mehr aus |
| [B-848](B-848-browser-client-code-entfernen.md) | INF | Schuld | niedrig | offen | – | src/, Phaser, die Spielseiten und die Web-Tasks des Browser-Clients sind entfernt, CI, Docker und Release bauen ohne dist |
| [B-849](B-849-site-papier-bilder.md) | PLAT | Schuld | niedrig | offen | – | Die Präsentationsseite site/ zeigt Papierschnitt-Bilder statt Pixel-Grafik |
| [B-850](B-850-server-net-load-tui-zukunft.md) | SRV | Frage | niedrig | offen | – | Es ist entschieden, ob k3c-server, engine/net, k3c-load und k3c-tui ohne Online-Spiel bleiben oder entfallen |
| [B-852](B-852-abnahme-laeufe-wz43.md) | DEV | Schuld | mittel | offen | WZ43 | Abnahme-Läufe WZ43 |
| [B-853](B-853-review-groessenvergleich-vorher-bild.md) | DEV | Idee | niedrig | offen | – | Das Review zeigt den Größenvergleich auch für das Vorher-Bild |
| [B-854](B-854-abnahme-laeufe-wz48.md) | INF | Schuld | mittel | offen | WZ48 | Abnahme-Läufe WZ48 |
| [B-861](B-861-doppelte-nummern-b199-b696.md) | DEV | Schuld | niedrig | offen | – | Die doppelt vergebenen Ticket-Nummern B-199 und B-696 sind aufgelöst |
| [B-864](B-864-mcp-plan-index.md) | DEV | Idee | niedrig | offen | – | Ein MCP-Tool plan_index baut Haupt- und Archiv-Index neu auf |
| [B-872](B-872-workbench-web-client-entfernen.md) | DEV | Schuld | niedrig | offen | – | Der Web-Client der Workbench ist entfernt, die Workbench läuft nur als EXE |
| [B-873](B-873-workbench-git-ohne-develop.md) | DEV | Schuld | mittel | offen | – | Git-Seite und GitHub-Leiste der Workbench arbeiten gegen main statt develop |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
