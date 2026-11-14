# MC1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt
- **Projekt:** PAP

## Ziel

Alle Asset-Tools und MCP-Prompts aus dem Vertrag laufen in der Workbench; die alten Register-Tools sind entfernt. Am Ende sichtbar: `assets_preview {id: monarch}` liefert ein Bild in Claude Code; `asset-material-aendern` erscheint als Prompt.

## Ergebnis

2026-11-06 · AC-01 bis AC-06 erfüllt laut Ergebnissen MC1.1–MC1.5 (In-Process-Tests, dazu zwei Tests gegen die echte CLI aus RD1; `task check:diff` exit 0 nach Merge von `feature/pap`; `task check:race` übersprungen, kein gcc). Behoben im Review: Gruppen-Index bei create/set/render, Vierbeiner-Vorlage für die echte CLI, CLI-Zeitlimit, Gruppe „alt“, Temp-Ordner mit Nachbar-Specs (Reittier), Review-Quelle auf `assetsui.Dienst` (RV1). Neue Tickets: B-702 (wackelnder mcpsrv-Test), B-706 (Restbefunde). D-Zeilen D-MC1-01 bis D-MC1-15 in B-701. Offen bei 🧑: Live-Prüfung nach Merge und Neustart der Wo …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Jedes Tool aus dem Vertrag hat einen In-Process-Test gegen die Attrappe, Antworten wie `fi … |
| AC-02 | erfüllt | `assets_set` mit ungültigem Patch lässt Spec und Ausgaben unverändert (Hash vorher/nachher … |
| AC-03 | erfüllt | `assets_preview` liefert MCP-ImageContent `image/png`. |
| AC-04 | erfüllt | Jede Vorlage aus `data/asset-prompts.json` ist als Prompt `asset-<id>` mit ihren Platzhalt … |
| AC-05 | erfüllt | `assets_render {alle}` antwortet sofort mit `{job}`, `assets_render {job, wait: true}` lie … |
| AC-06 | erfüllt | Register-Varianten von `assets_list/get/set` und `assets_catalog` entfernt; `task check:de … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
