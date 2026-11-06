# PAP · Papierschnitt-Grafik, Assets über MCP, Review

- **Status:** aktiv
- **Rang:** 1
- **Ziel-Tickets:** B-699, B-700, B-701

## Ziel

Alle Grafiken haben die Papierschnitt-Optik und entstehen reproduzierbar aus je einer Spec; Agenten ändern sie nur über MCP, 🧑 reviewt jede Änderung im Bereich Assets der Workbench mit Markierung im Git-Diff. Erledigt, wenn das Spiel nur noch Papierschnitt-PNGs lädt und US1 abgenommen ist. Entscheidung `docs/decisions/005-papierschnitt-assets-mcp.md`, Ticket B-699.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| PV1 | Papierschnitt-Verträge: Domäne, Spec-Schema, Styleguide, Render-, MCP- und Review-Vertrag | erledigt |
| RD1 | Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests | aktiv |
| MC1 | Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt | aktiv |
| WB1 | Workbench mit Bereichen Development und Assets, Studio-Programm abgebaut | erledigt |
| RV1 | Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung | aktiv |
| MG1 | Papierschnitt-Migration: Monarch und Spielerfarben | aktiv |
| MG2 | Papierschnitt-Migration: Reittiere | geplant |
| MG3 | Papierschnitt-Migration: Truppen | geplant |
| MG4 | Papierschnitt-Migration: Gegner | geplant |
| MG5 | Papierschnitt-Migration: Bosse | geplant |
| MG6 | Papierschnitt-Migration: Gebäude mit Ausbaustufen | geplant |
| MG7 | Papierschnitt-Migration: Hintergründe je Biom | geplant |
| MG8 | Papierschnitt-Migration: Natur, Objekte, Münzen, Icons | geplant |
| US1 | Spiel lädt Papierschnitt-Grafik, alte Pixel-Assets und Gebäude-Renderer entfernt | geplant |
| PS1 | Papierschnitt-Sichtprüfungen und Abnahmen gebündelt | geplant |
| CU1 | Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen | geplant |

## Nicht-Ziele

Skalierung für Xbox und Mobile, Engine-Schatten (Folgeticket nach US1); Sound (SND); Gebräuchliche Workbench-Funktionen unter Development bleiben unverändert (WZG).

## Notizen

**Wellen und Agenten (Entscheidung 005 › Umsetzung):**

| Welle | Sprint | Domäne/Lane | Agenten gleichzeitig | Modelle |
|---|---|---|---|---|
| 0 | PV1 Verträge (seriell auf einem Branch) | INF→DEV→PAP→DEV | 1 | Sonnet; Opus für PV1.3, PV1.4, Review |
| 1 | RD1 Renderer | PAP/`renderer` | 4 parallel | Opus (Rig-Mathematik), RD1.4 Sonnet |
| 1 | MC1 MCP | DEV/`mcp` | | Sonnet, Review Opus |
| 1 | WB1 Workbench | DEV/`ui` (+ INF-Session) | | Sonnet, Review Opus |
| 1 | RV1 Review | DEV/`review` | | Opus für Diff/Seite, sonst Sonnet |
| 2 | MG1–MG8 Migration | PAP/je Gruppe | max. 4 parallel, MG1.1 Integration zuerst | Sonnet, Reviews Opus |
| 3 | US1 Spiel-Umstellung | CLI→SRV→INF | 1 | Sonnet, Doku Haiku, Review Opus |
| 4 | PS1 Sichtprüfungen und Fragen B-701 | Mensch | 🧑 | PS1.5 Sonnet |
| 5 | CU1 Cleanup alte Grafik (B-700) | CLI→INF | 1 | Sonnet, Review Opus |

Regeln: max. 4 Agenten gleichzeitig; Sessions eines Sprints nacheinander; parallele Sessions derselben Domäne brauchen verschiedene `Lane` und disjunkte Erlaubte Dateien (PV1.1/PV1.2). Sprints der Wellen 1–5 sind Entwürfe und werden nach PV1 bereit gemacht (Session-Dateien mit `Lane`, `Modell`, Domäne PAP statt Platzhalter).

**Betrieb der Workbench während der Umsetzung (B-701 D-10):** Eine Workbench aus der Repo-Wurzel läuft im Dev-Modus (`task k3c-workbench`, baut bei Go-Änderungen neu); Agenten in Worktrees starten keine eigene Instanz. Neue oder geänderte MCP-Tools testet MC1 in-process; live nutzbar sind sie erst nach Merge nach `develop`, Pull in der Wurzel und Neubau. Deshalb startet Welle 2 erst, wenn RD1 und MC1 gemergt sind.

**Orchestrierung:** Claude startet je Welle die Lanes als eigene Sessions in eigenen Worktrees, wartet auf ihre Berichte, prüft Diff und Kriterien, öffnet den PR je Sprint; 🧑 merged. Sichtprüfungen und Abnahmen sammeln sich in PS1, Entscheidungen mit Default in B-701.

Bestandsgrafik und altes Register `data/assets.json` sind ab MC1 eingefroren und werden in CU1 entfernt.

**Review-Fokus (in den Sprint-Regeln verankert):** CRLF ändert den Spec-Hash nicht (PV1.3, RV1); Stiländerungen ändern `renderKey` und erscheinen im Review (PV1.3, RV1); `origin/develop` ohne Fetch wird als lokaler Stand gemeldet (RV1); `assets_render {alle}` läuft als Job (MC1); gleichzeitige Handänderung hinterlässt keine halbe Spec (MC1); fehlende PNGs aus einem Gruppen-Index lassen den Server beim alten Stand (US1).
