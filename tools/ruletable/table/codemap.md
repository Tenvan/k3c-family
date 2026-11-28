# tools/ruletable/table/

## Responsibility
Paket `table`: erzeugt aus `data/hub.json` den Block „Hub-Stufe → Gebäude“ der Regeln (`docs/rules/materialien-gebaeude.md` § 3) und tauscht ihn zwischen den Marken im Regeltext aus. Reine Funktionen ohne Dateizugriff; die Zahl der Hub-Stufen kommt aus `levels`.

## Design
- Marken `Start`/`End` (`<!-- rules:hub-levels:start|end -->`), je genau einmal im Dokument.
- Typen `hub`, `level` und `site` spiegeln die benötigten Felder der `hub.json`: Plätze in `sites` und `islandSites`, Stufen mit Kosten direkt oder verteilt auf Ausbauschritte (`steps`, `level.cost` summiert beides), `sitePlacement` (Zonen, Baugrund Zone 1) und `merchant`.
- `Block(hubJSON)`: eine Zeile je Stufe mit Material (`material`: das eine Material der Kosten, Gold zählt nicht, ohne Kosten Holz; mehrere oder unbekannte = Fehler), Plätzen (`places`, nach Art gruppiert in Reihenfolge des ersten Auftretens, `siteText` mit Tiefe, „nur mit tieferer Stufe“, „je Insel“), Mauer- und Turm-Zone (`line`: Zone n ab Hub-Stufe n, Zone 1 mit festem Baugrund) und Händler (`merchant`, nur Stufe 1). `checkLevels` meldet Plätze außerhalb von `levels`; `hubLevelOf` macht 0 zu Stufe 1; `num`/`signed` formatieren Zahlen. LF-Zeilenenden, Marken eingeschlossen.
- `Ersetzen(doc, block)`: tauscht Start- bis End-Marke (je genau einmal, Ende nach Start; sonst Fehler und Dokument unverändert), Rest byte-gleich, Zeilenenden folgen dem Dokument (`span`).
- `Gegenprobe(doc, hubJSON)`: vergleicht den Block im Dokument mit dem erzeugten und liefert beide Texte für die Abweichungsanzeige.

## Flow
`Block(hub.json)` → `Ersetzen(doc, block)` → neuer Dokumenttext; `Gegenprobe` erzeugt den Block und vergleicht ohne zu schreiben.

## Integration
- Aufrufer: `tools/k3c-studio/internal/hubplan` (Go-Studio). Der Task `gen:rules` läuft seit RU71 über die TypeScript-Fassung `../table.ts`; `rust/tools/crates/k3c-studio/src/hubplan/table.rs` ist der Rust-Port für das Rust-Studio. Wer die Regeln ändert, ändert alle Fassungen.
- Test: `table_test.go`.
