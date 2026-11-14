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
| [B-185](B-185-verluste-je-welle-angleichen.md) | REG | Schuld | niedrig | eingeplant | RG2 | Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger |
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-219](B-219-doku-gating-und-schlag.md) | REG | Schuld | niedrig | eingeplant | RG2 | Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen |
| [B-295](B-295-handwerker-schmiede-ruestkammer.md) | REG | Frage | mittel | eingeplant | RG2 | Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden |
| [B-622](B-622-fackeln-als-spielobjekt.md) | SIM | Idee | niedrig | eingeplant | W15 | Fackeln gibt es als Spielobjekt mit Licht in der Nacht |
| [B-699](B-699-papierschnitt-grafik-mcp-review.md) | DEV | Idee | hoch | eingeplant | MG11 | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench |
| [B-711](B-711-reiter-variante-reittier.md) | PAP | Frage | mittel | eingeplant | MG10 | Variante des Reiters tauscht die Materialien des Reittiers nicht |
| [B-720](B-720-bosse-neu.md) | PAP | Problem | hoch | eingeplant | MG10 | Bosse sind neu gestaltet, jeder mit eigener Silhouette |
| [B-721](B-721-gebaeude-stufen-form.md) | PAP | Problem | mittel | eingeplant | MG11 | Gebäude steigern sich in Form und Material, Kristall leuchtet, nichts wird abgeschnitten |
| [B-722](B-722-hintergruende-neu-ohne-baeume.md) | PAP | Problem | hoch | eingeplant | MG11 | Hintergründe sind neu gestaltet, mit Tiefe und ohne Bäume |
| [B-723](B-723-natur-icons-muenze.md) | PAP | Problem | mittel | eingeplant | MG11 | Natur hat Spielhöhe und mehrere Formen, Icons sind bei 32 px lesbar, die Münze dreht sich |
| [B-746](B-746-workbench-tests-ohne-engine-server.md) | DEV | Schuld | niedrig | offen | – | Workbench-Tests laufen gegen einen Fake-Spielserver statt engine/net und engine/room |
| [B-775](B-775-native-umstellung-rust-bevy.md) | CLI | Idee | hoch | eingeplant | NG9 | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server |
| [B-780](B-780-rig-fuer-alle-figuren-gegnerzahl.md) | CLI | Frage | mittel | eingeplant | NG3 | Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen |
| [B-846](B-846-server-ohne-client-auslieferung.md) | SRV | Schuld | niedrig | offen | – | Der Spielserver startet und läuft ohne dist/ und liefert keinen Browser-Client mehr aus |
| [B-848](B-848-browser-client-code-entfernen.md) | INF | Schuld | niedrig | offen | – | src/, Phaser, die Spielseiten und die Web-Tasks des Browser-Clients sind entfernt, CI, Docker und Release bauen ohne dist |
| [B-849](B-849-site-papier-bilder.md) | PLAT | Schuld | niedrig | offen | – | Die Präsentationsseite site/ zeigt Papierschnitt-Bilder statt Pixel-Grafik |
| [B-850](B-850-server-net-load-tui-zukunft.md) | SRV | Frage | niedrig | offen | – | Es ist entschieden, ob k3c-server, engine/net, k3c-load und k3c-tui ohne Online-Spiel bleiben oder entfallen |
| [B-853](B-853-review-groessenvergleich-vorher-bild.md) | DEV | Idee | niedrig | eingeplant | WZ58 | Das Review zeigt den Größenvergleich auch für das Vorher-Bild |
| [B-879](B-879-welten-editor-workbench.md) | DEV | Problem | hoch | eingeplant | WZ58 | Die Workbench hat den Welten-Editor aus dem alten Studio wieder |
| [B-880](B-880-asset-pakete-je-welt.md) | PAP | Idee | mittel | eingeplant | PT6 | Assets lassen sich zusammen mit einer Welt zu Paketen bündeln |
| [B-882](B-882-abnahme-laeufe-wz45.md) | DEV | Schuld | mittel | offen | WZ45 | Abnahme-Läufe WZ45 |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
