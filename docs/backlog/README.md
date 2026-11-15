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
| [B-907](B-907-abnahme-wr1-gen-race.md) | SRV | Schuld | niedrig | offen | WR2 | Abnahme WR1: gen:paper samt Grafik-Golden und check:race laufen nach dem Rückbau grün |
| [B-889](B-889-nativ-debug-werte-und-logs.md) | CLI | Idee | hoch | offen | CH1 | Das native Spiel zeigt Debug-Werte der Sim (F10) und loggt Zustandswechsel und Zusammenfassungen |
| [B-890](B-890-nativ-hud-icons-statt-namen.md) | CLI | Idee | hoch | offen | CH1 | Das HUD des nativen Spiels zeigt Ressourcen als Icon mit Zahl statt als Namen |
| [B-899](B-899-abnahme-ch1.md) | CLI | Schuld | mittel | offen | CH1 | Abnahme CH1: Sichtprüfung der Entwickler-Tasten, Cheats, Debug-Werte und HUD-Icons und Gen-Läufe sind erledigt |
| [B-891](B-891-biom-eigenschaften-je-welt.md) | SIM | Idee | mittel | offen | BI1 | Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt |
| [B-895](B-895-workbench-webview-speicherleck.md) | DEV | Problem | hoch | offen | WZ60 | Die Oberfläche der Workbench wächst nicht unbegrenzt im Speicher und bleibt nach Stunden mit parallelen Prüfläufen bedienbar |
| [B-909](B-909-planungsdaten-schlank.md) | DEV | Schuld | mittel | offen | WZ60 | Die Planungsseite lädt bei einer Änderung nur das Geänderte statt 5,5 MiB PlanningData |
| [B-908](B-908-pages-bausatz.md) | PLAT | Idee | mittel | offen | GP1 | Die Präsentationsseite zeigt das Spiel als Bausatz: Einzelspieler und Couch-Koop, nativ, frei für nicht-kommerzielle Nutzung, eigene Welten mit dem MCP-Server |
| [B-910](B-910-abnahme-bi2.md) | CLI | Idee | mittel | eingeplant | BI2 | Abnahme BI2: Biom-Darstellung ist gesichtet, Grafik gerendert und Golden geprüft |
| [B-911](B-911-papierschnitt-tasten-icons.md) | PAP | Idee | mittel | offen | EP1 | Tastenkappen und Xbox-Tasten gibt es als Papierschnitt-Icons der Gruppe icons |
| [B-912](B-912-e2e-flow-skillpunkt-rot.md) | CLI | Problem | hoch | offen | EP1 | Der Ende-zu-Ende-Lauf TestFlow scheitert, weil Spieler 2 bis Takt 600 keinen Skill-Punkt findet |
| [B-913](B-913-abnahme-ep1.md) | CLI | Idee | mittel | offen | EP1 | Ralf nimmt EP1 ab: Eingabe-Profile und Tasten-Icons an TV und PC geprüft |
| [B-914](B-914-abnahme-st2.md) | CLI | Idee | mittel | offen | ST2 | Ralf nimmt ST2 ab: Testing-Menü und Stresstest-Bericht am PC geprüft |
| [B-915](B-915-stresstest-overlay-radar.md) | CLI | Problem | mittel | offen | FB1 | Stresstest-Overlay überdeckt das Radar; Radar wandert an die Unterkante im Fluss |
| [B-916](B-916-stresstest-ueberschreiben.md) | CLI | Problem | hoch | offen | FB1 | Ein bestehendes Stresstest-Spiel lässt sich beim Start eines neuen einfach überschreiben |
| [B-917](B-917-ausloeser-leertaste.md) | CLI | Problem | mittel | offen | FB1 | Auslösetaste ist immer die Leertaste, unabhängig von den Richtungstasten |
| [B-918](B-918-tasten-nach-menues-tot.md) | CLI | Problem | hoch | offen | FB1 | Tastensteuerung fällt nach Debug-, Cheat- und Menü-Nutzung aus |
| [B-919](B-919-hud-geldsack.md) | CLI | Idee | mittel | offen | FB1 | HUD zeigt einen Geldsack statt Healthbar und P1/P2 |
| [B-922](B-922-geldsack-skill.md) | SIM | Idee | mittel | offen | FB2 | Geldsack in drei Größen, per Skillpunkte setzbar |

## Archiv

Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in
[`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder
`verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.
