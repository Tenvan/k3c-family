# WZ34 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Prüf- und Build-Werkzeuge: Studio-Test, check:diff, buildall
- **Projekt:** WZG

## Ziel

Die Prüf- und Build-Wege der Werkzeuge sind verlässlich: Studio-Test grün, kein flackernder Workbench-Test, `task buildall` liefert alle EXEs auf dem Stand des Checkouts. Am Ende sichtbar: `task check:studio` grün, zwei `task buildall`-Läufe mit identischem, aktuellem `bin/`.

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `task check:studio` ist grün, der Test `levelView` passt zu `level-forest.json` (`B-617/AC … |
| AC-02 | geprüft | `TestSourcesUndConsoleTail` ist unabhängig vom Zeitpunkt der Ressourcen-Zeile, 20 Läufe gr … |
| AC-03 | geprüft | `task buildall` baut die Web-EXEs mit, alle EXEs in `bin/` stehen auf dem Stand des Checko … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-617 | verworfen | AC-01 geprüft | Der Studio-Test levelView passt wieder zur Golden-Datei level-forest.json |
| B-621 | erledigt | AC-02 geprüft | TestSourcesUndConsoleTail flackert, wenn die Ressourcen-Zeile früh kommt |
| B-629 | verworfen | AC-03 geprüft | task buildall baut die Web-EXEs von Studio und Workbench nicht mit |
