# Backlog

Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).
Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe
[`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);
`task test` prüft Dateien und beide Index-Dateien.

## Offen

| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |
|---|---|---|---|---|---|---|
| [B-024](B-024-tiefe-3-4.md) | REG | Idee | niedrig | eingeplant | RG2 | Tiefe 3 und 4 sind beschrieben |
| [B-185](B-185-verluste-je-welle-angleichen.md) | REG | Schuld | niedrig | eingeplant | RG2 | Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger |
| [B-203](B-203-gold-schwelle-kennzahl.md) | REG | Frage | niedrig | eingeplant | RG2 | Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung |
| [B-219](B-219-doku-gating-und-schlag.md) | REG | Schuld | niedrig | eingeplant | RG2 | Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen |
| [B-295](B-295-handwerker-schmiede-ruestkammer.md) | REG | Frage | mittel | eingeplant | RG2 | Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden |
| [B-622](B-622-fackeln-als-spielobjekt.md) | SIM | Idee | niedrig | eingeplant | W15 | Fackeln gibt es als Spielobjekt mit Licht in der Nacht |
| [B-699](B-699-papierschnitt-grafik-mcp-review.md) | DEV | Idee | hoch | eingeplant | MG11 | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench |
| [B-720](B-720-bosse-neu.md) | PAP | Problem | hoch | eingeplant | MG10 | Bosse sind neu gestaltet, jeder mit eigener Silhouette |
| [B-775](B-775-native-umstellung-rust-bevy.md) | CLI | Idee | hoch | eingeplant | NG9 | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server |
| [B-780](B-780-rig-fuer-alle-figuren-gegnerzahl.md) | CLI | Frage | mittel | eingeplant | NG3 | Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen |
| [B-879](B-879-welten-editor-workbench.md) | DEV | Problem | hoch | eingeplant | WZ58 | Die Workbench hat den Welten-Editor aus dem alten Studio wieder |
| [B-880](B-880-asset-pakete-je-welt.md) | PAP | Idee | mittel | eingeplant | PT6 | Assets lassen sich zusammen mit einer Welt zu Paketen bündeln |
| [B-884](B-884-abnahme-mg11.md) | PAP | Schuld | mittel | offen | MG11 | Abnahme MG11: Sichtprüfung der Welt-Grafik und Gen-Läufe sind erledigt |
| [B-882](B-882-abnahme-laeufe-wz45.md) | DEV | Schuld | mittel | offen | WZ45 | Abnahme-Läufe WZ45 |
| [B-885](B-885-native-rig-export-bosse.md) | PAP | Schuld | mittel | offen | MG10 | Nativer Rig-Export der Bosse ist nach MG10 veraltet |
| [B-886](B-886-abnahme-mg10.md) | PAP | Schuld | mittel | offen | MG10 | Abnahme MG10: Sichtprüfung der Bosse und Gen-Läufe sind erledigt |
| [B-888](B-888-nativ-cheat-dialog-und-tastenhilfe.md) | CLI | Idee | hoch | offen | CH1 | Das native Spiel hat wieder einen Cheat-Dialog (F9) und eine Tastenhilfe der UI-Tasten (F1) |
| [B-897](B-897-server-reste-nach-wr1.md) | SRV | Schuld | niedrig | offen | WR2 | Nach dem Server-Rückbau sind Server-Reste in engine/room, k3c-sim bot, simexe.StartBot und der Dienste-Infrastruktur der Workbench entfernt |
| [B-898](B-898-doku-nach-wr1.md) | INF | Schuld | niedrig | offen | WR2 | Prozess-Doku, Rechte und Mocks nennen keinen Spielserver, keinen Browser-Client und keine Xbox-Seiten mehr |
| [B-907](B-907-abnahme-wr1-gen-race.md) | SRV | Schuld | niedrig | offen | WR2 | Abnahme WR1: gen:paper samt Grafik-Golden und check:race laufen nach dem Rückbau grün |
| [B-889](B-889-nativ-debug-werte-und-logs.md) | CLI | Idee | hoch | offen | CH1 | Das native Spiel zeigt Debug-Werte der Sim (F10) und loggt Zustandswechsel und Zusammenfassungen |
| [B-890](B-890-nativ-hud-icons-statt-namen.md) | CLI | Idee | hoch | offen | CH1 | Das HUD des nativen Spiels zeigt Ressourcen als Icon mit Zahl statt als Namen |
| [B-899](B-899-abnahme-ch1.md) | CLI | Schuld | mittel | offen | CH1 | Abnahme CH1: Sichtprüfung der Entwickler-Tasten, Cheats, Debug-Werte und HUD-Icons und Gen-Läufe sind erledigt |
| [B-891](B-891-biom-eigenschaften-je-welt.md) | SIM | Idee | mittel | offen | BI1 | Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt |
| [B-893](B-893-eingabe-profile-je-spieler.md) | CLI | Idee | mittel | offen | EP1 | Ein Tastenlayout: jeder Spieler wählt WASD, Pfeiltasten oder Gamepad, die Tastatur hat höchstens ein Spieler |
| [B-894](B-894-tasten-icons-je-eingabegeraet.md) | CLI | Idee | mittel | offen | EP1 | Die Oberfläche zeigt Tasten-Hinweise als Icon des Eingabegeräts des jeweiligen Spielers (Tastenkappe oder Xbox-Taste) |
| [B-900](B-900-kristall-adern-farben-papier.md) | PAP | Idee | niedrig | offen | BI2 | Kristall-Adern gibt es als Papierschnitt in drei Farben (blau, violett, grün) |
| [B-901](B-901-kristallfarbe-im-spiel.md) | CLI | Idee | niedrig | offen | BI2 | Das native Spiel zeigt die Kristallfarbe jeder Kristall-Ader |
| [B-902](B-902-hoehle-deckenhoehe.md) | CLI | Idee | niedrig | offen | BI2 | Die Höhle zeigt ihre Deckenhöhe (Natur-Eigenschaft ceilingHeight) |
| [B-903](B-903-hoehle-tropfsteine.md) | CLI | Idee | niedrig | offen | BI2 | Die Höhle zeigt Tropfsteine nach der Natur-Eigenschaft stalactites |
| [B-904](B-904-mine-stuetzbalken-verfall.md) | CLI | Idee | niedrig | offen | BI2 | Die Mine zeigt Stützbalken und Verfall nach der Natur-Eigenschaft decay |
| [B-905](B-905-eisenstollen-glut.md) | CLI | Idee | niedrig | offen | BI2 | Der Eisenstollen zeigt Hitze und Glut nach der Natur-Eigenschaft glow |
| [B-906](B-906-kristallhoehle-leuchtstaerke.md) | CLI | Idee | niedrig | offen | BI2 | Die Kristallhöhle leuchtet nach der Natur-Eigenschaft brightness |
| [B-895](B-895-workbench-webview-speicherleck.md) | DEV | Problem | hoch | offen | WZ60 | Die Oberfläche der Workbench wächst nicht unbegrenzt im Speicher und bleibt nach Stunden mit parallelen Prüfläufen bedienbar |
| [B-896](B-896-stresstests-im-menue.md) | CLI | Idee | hoch | offen | ST2 | Das native Spiel hat im Menü einen Bereich Testing, in dem einstellbare Stresstests starten |
| [B-909](B-909-planungsdaten-schlank.md) | DEV | Schuld | mittel | offen | WZ60 | Die Planungsseite lädt bei einer Änderung nur das Geänderte statt 5,5 MiB PlanningData |
| [B-908](B-908-pages-bausatz.md) | PLAT | Idee | mittel | offen | GP1 | Die Präsentationsseite zeigt das Spiel als Bausatz: Einzelspieler und Couch-Koop, nativ, frei für nicht-kommerzielle Nutzung, eigene Welten mit dem MCP-Server |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
