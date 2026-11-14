# MG4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Migration: Gegner
- **Projekt:** PAP

## Ziel

Als Spec über MCP angelegt und gerendert: alle Gegner (Rig-Typ je Gegner `figur`, `vierbeiner` oder `requisite`). Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Ergebnis

2026-11-07 (autonom, Claude): 17 Gegner unter `public/grafik/gegner/`, alle gültig, gerendert, in `golden.json` (MG4.1–MG4.4). Rigs: 7 `figur`, 2 `vierbeiner`, 8 `requisite`. Review MG4.5 (Opus): 6 wichtige Befunde behoben (Größe der Elite-Gegner, Wächter-Schlag, Fledermaus-Angriff, Lavaschleim-Form, Rahmen); Rest D-MG4-07. `task check` grün; `tools/paper`-Tests 1110/1110. Entscheidungen D-MG4-01 bis -07 in B-701. Sichtprüfung durch Ralf in PS1 offen. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Jedes Asset der Gruppe hat eine gültige Spec (`assets_list {gruppe: gegner}` ohne `ungülti … |
| AC-02 | geprüft | `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen. |
| AC-03 | geprüft | Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
