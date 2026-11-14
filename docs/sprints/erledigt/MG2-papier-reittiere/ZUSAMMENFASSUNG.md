# MG2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Migration: Reittiere
- **Projekt:** PAP

## Ziel

Als Spec über MCP angelegt und gerendert: alle Reittiere als `vierbeiner` mit Schritt, Trab, Galopp. Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Ergebnis

2026-11-06 · AC-01 bis AC-03 laut Ergebnissen MG2.1–MG2.4 erfüllt: 11 Reittiere als `vierbeiner` nur über MCP angelegt, je Gangart geprüft, Golden über `gen:paper` (B-707). Behoben im Review: Schweif im Galopp abgeschnitten (Rahmen links erweitert), D-Zeilen an den Stand angepasst. Keine neuen Tickets; Befunde als D-MG2-01 bis D-MG2-08 in B-701 (u. a. Rig-Grenzen an RD1, `pferd`-Rahmen an MG1, wackelnde Workbench-Frontend-Tests). Offen bei 🧑: Sichtprüfung in PS1.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Jedes Asset der Gruppe hat eine gültige Spec. |
| AC-02 | geprüft | `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen. |
| AC-03 | geprüft | Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
