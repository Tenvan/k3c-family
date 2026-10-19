# tools/ruletable/table/

## Responsibility
Paket `table`: erzeugt aus `data/hub.json` den Block „Hub-Stufe → Gebäude“ der Regeln (`docs/rules/materialien-gebaeude.md` § 3) und tauscht ihn zwischen den Marken im Regeltext aus. Reine Funktionen ohne Dateizugriff; die Zahl der Hub-Stufen kommt aus `levels`.

## Design
- Typen `hub` und `site` spiegeln die benötigten Felder der `hub.json` (Plätze in `sites` und `islandSites`, Stufen, Ausbaukosten, Tor-Stufe).
- `Block(hubJSON)`: eine Zeile je Stufe mit Material (`material`: das eine Material der Ausbaukosten, Gold zählt nicht, ohne Kosten Holz; mehrere oder unbekannte = Fehler), Plätzen (`places`, nach Art gruppiert in Reihenfolge des ersten Auftretens, `siteText`), Mauerlinie (`line`: Mauer, Turm, Tor ab `gateHubLevel`) und Händler (`merchant`, nur Stufe 1). `checkLevels` meldet Plätze außerhalb von `levels`; `hubLevelOf` macht 0 zu Stufe 1; `num`/`signed` formatieren Zahlen. LF-Zeilenenden, Marken Start und Ende eingeschlossen.
- `Ersetzen(doc, block)`: tauscht Start- bis End-Marke (je genau einmal, Ende nach Start; sonst Fehler und Dokument unverändert), Rest des Dokuments byte-gleich, Zeilenenden folgen dem Dokument (`span`).
- `Gegenprobe(doc, hubJSON)`: vergleicht den Block im Dokument mit dem erzeugten und liefert beide Texte für die Abweichungsanzeige.

## Flow
`Block(hub.json)` → `Ersetzen(doc, block)` → neuer Dokumenttext; `Gegenprobe` erzeugt den Block und vergleicht ohne zu schreiben.

## Integration
- Aufrufer: `tools/ruletable/main.go` (Task `rules:update`) und das Studio (`tools/k3c-studio/app_planer_rules.go`, `app_planer_save.go`).
- Test: `table_test.go`.
