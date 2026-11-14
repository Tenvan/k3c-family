# GR3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Grafik im Renderer
- **Projekt:** –

## Ziel

Gebäude, Ressourcen und Hintergründe werden mit den zugeordneten Grafiken gezeichnet. Am Ende sichtbar: Hub mit Sprites statt Formen, Parallax je Biom, Hub- und Materialstufen unterscheidbar am TV.

## Ergebnis

- 2026-10-04, Review GR3.3: AC-01, AC-02, AC-03, AC-05, AC-06 umgesetzt und im Diff geprüft (Nachweise in GR3.1/GR3.2, `task check` und `task check:go` grün, Browser-Pane im Split-Screen). - AC-04: Auswahl Stufe → Sprite getestet; ohne Stufe im Snapshot zeigen Mauer und Turm die Stein-Grafik (Holz ist Lücke), bewusste Abweichung. Sicht am TV (AC-04, AC-06): angenommen, Validierung offen (🧑, TV; im Fahrplan unter „Offen am Gerät“). - Keine schweren Befunde (kein `Math.random()`, Rückfall auf Platzhalter, Credits vollständig, Dateien ≤ 400). B-010 archiviert, keine neuen Tickets. - Version: v0.1 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Alle Gebäude, Ressourcen, Portale, Truhen, Münzen und Parallax-Ebenen, die in der Zuordnun … |
| AC-02 | – | Die Credits aller eingebauten Grafiken stehen in `public/` (B-010/AC-02). |
| AC-03 | – | Fehlt eine Grafik, bleibt die Platzhalter-Form sichtbar (Test oder Beobachtung). |
| AC-04 | – | Hub-Stufen und Mauer-/Turm-Materialstufen sind unterscheidbar, sobald B-112 sie im Snapsho … |
| AC-05 | – | Wald, Höhle und Mine haben je eigene Parallax-Ebenen oder einen dokumentierten Platzhalter … |
| AC-06 | – | `task check` ist grün, mit 2 Spielern im Split-Screen sind keine Darstellungsfehler zu seh … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-010 | erledigt | AC-01 –, AC-02 – | Gebäude, Ressourcen und Hintergrund haben Grafiken |
