# MG7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Migration: Hintergründe je Biom
- **Projekt:** PAP

## Ziel

Als Spec über MCP angelegt und gerendert: Hintergründe je Biom als Rig `hintergrund` mit den Ebenen fern, mitte und nah, je zwei Varianten (D-MG7-01), in der neuen Gruppe `public/grafik/hintergruende/`. Am Ende sichtbar: 30 Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Ergebnis

- 2026-11-07, Review MG7.3 (Opus-Reviewer, Claude): AC-01 bis AC-03 erfüllt (Nachweise MG7.1–MG7.3); Sichtprüfung durch 🧑 in PS1 offen. - Behoben: Kerben an Streben (Mine, Eisenstollen), Schatten der Höhlen-Blöcke, Löcher am Säulenfuß (Höhle mitte). - Offen als D-MG7-01 bis D-MG7-06 in B-701. Keine neuen Tickets. - `task check:diff` lief in die Zeitgrenze (Workbench-Go-Tests des feature/pap-Stands); `task test` 3173/3173 und `task test -- paper` 187/187 grün.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Jede Ebene und Variante hat eine gültige Spec. |
| AC-02 | erfüllt | `task gen:paper` erzeugt alle Ebenen; Golden-Hashes eingetragen. |
| AC-03 | erfüllt | Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
