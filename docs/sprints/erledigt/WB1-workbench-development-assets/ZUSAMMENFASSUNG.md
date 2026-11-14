# WB1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Workbench mit Bereichen Development und Assets, Studio-Programm abgebaut
- **Projekt:** PAP

## Ziel

Die Workbench hat die Bereiche **Development** (alle heutigen Seiten) und **Assets** (Review, Katalog, Styleguide, Sound, Quellen) ohne schreibende Controls, mit Prompt-Buttons; das Studio-Programm samt Chat ist abgebaut. Am Ende sichtbar: `task k3c-workbench:web` zeigt beide Bereiche; `bin/k3c-studio*.exe` entstehen nicht mehr.

## Ergebnis

2026-11-06, Claude (Review WB1.6, autonom). AC-01–AC-04 erfüllt laut WB1.1–WB1.5; `task check`, `task buildall` grün, `check:race` mangels gcc übersprungen. Behoben im Review: `Game.Close`/`Open` (Raum-Takt, Fristen, nach Close abgelehnt). Rot ohne WB1-Bezug: lastabhängige Go-Tests und `simrun`-TempDir → B-704. Neue Defaults D-18–D-22 in B-701.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Kopfleiste mit Development und Assets; alle bisherigen Seiten unter Development erreichbar … |
| AC-02 | erfüllt | Assets: Katalog (Animations-Vorschau, Abzeichen „im Diff“, **roter Eintrag** für ungültige … |
| AC-03 | erfüllt | Prompt-Buttons am Asset und am angeklickten Teil kopieren den gefüllten Prompt aus `Assets … |
| AC-04 | erfüllt | `tools/k3c-studio/` samt Chat, EXEs, Ports 8091/8182 und Studio-Tasks entfernt; `task buil … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
