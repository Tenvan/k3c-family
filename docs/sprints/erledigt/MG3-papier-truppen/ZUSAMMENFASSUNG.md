# MG3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Migration: Truppen
- **Projekt:** PAP

## Ziel

Als Spec über MCP angelegt und gerendert: alle Truppen als `figur` mit ihren Animationen. Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Ergebnis

- 2026-11-07, autonom (Claude): AC-01 bis AC-03 belegt (Ergebnisse MG3.1–MG3.3); sechs Truppen unter `public/grafik/truppen/`, nur über die Asset-MCP-Tools geschrieben. - Behoben aus dem Review (Opus): Bogen-Angriff mit Auszug und Pfeil, Elite-Bogenschütze mit Feder und goldenem Bogen, Heugabel aus dem Gesicht. - Defaults und Kleinbefunde: D-MG3-01 bis D-MG3-08 in B-701; keine neuen Tickets. - Offen: Sichtprüfung durch 🧑 Ralf in PS1.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Jedes Asset der Gruppe hat eine gültige Spec (`assets_list {gruppe: truppen}` ohne `ungült … |
| AC-02 | geprüft | `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen. |
| AC-03 | geprüft | Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
