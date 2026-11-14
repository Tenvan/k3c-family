# MG6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Migration: Gebäude mit Ausbaustufen
- **Projekt:** PAP

## Ziel

Als Spec über MCP angelegt und gerendert: alle Gebäude mit Ausbaustufen als Rig `gebaeude`, Stufen als Delta (Materialkette Holz → Stein → Kupfer → Eisen → Kristall), dazu die statischen Gebäude der Wege und der Küste. Umfang (20 Assets): Mauer, Tor, Turm, Hintergrundmauer, Werkstatt, Taverne, Heilplatz, Schmiede, Rüstkammer, Lager, Kaserne, Lagerfeuer, Burg, Burg-Ausbau, Rekrutierungslager, Farm, …

## Ergebnis

2026-11-07: AC-01–AC-03 erfüllt (MG6.1–MG6.5, Review MG6.6): 20 Gebäude als Papierschnitt in `public/grafik/gebaeude-paper/`, Golden eingetragen, `assets_review` ohne roten Eintrag. Behoben im Review: Eisenbänder in Stufe 5, Runenband Tor, Lagerfeuer mittig. Kleinbefunde zurückgestellt: D-MG6-08. Defaults D-MG6-01–D-MG6-08 in B-701; keine neuen Tickets. Sichtprüfung durch 🧑 in PS1.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Jedes Gebäude hat eine gültige Spec mit allen Stufen (`assets_list` ohne `ungültig`). |
| AC-02 | erfüllt | `task gen:paper` erzeugt alle Stufen; Golden-Hashes eingetragen. |
| AC-03 | erfüllt | Jedes Asset in jeder Stufe selbst geprüft (Stufe 1 `assets_preview`, ab Stufe 2 Ausgabe), … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
