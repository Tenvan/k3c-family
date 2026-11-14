# PV1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt-Verträge: Domäne, Spec-Schema, Styleguide, Render-, MCP- und Review-Vertrag
- **Projekt:** PAP

## Ziel

Nach PV1 kann jede Lane ohne Rückfrage gegen feste, getestete Verträge und Fixtures arbeiten, und drei DEV-Lanes dürfen gleichzeitig laufen. Am Ende sichtbar: `docs/standards/paper-spec.md` und `docs/standards/paper-mcp.md`, Schemas und Fixtures unter `tools/paper/`, eine resvg-gerenderte Fixture-PNG im Papierschnitt-Look, `plan_next` meldet parallele Sessions je Lane.

## Ergebnis

2026-11-06, Review PV1.5 (Claude, autonom): AC-01–AC-07 erfüllt, Nachweise in PV1.1, PV1.2, PV1.6, PV1.3, PV1.4, PV1.8. AC-05: resvg-Probe auf Windows und Linux (CI-Run 42259675603) byte-gleich → Golden-Regel exakt; Sichtprüfung gegen den Browser folgt in PS1. Behoben: Slider-Atom brach das Studio (jetzt `ValueSlider`), Timeout des Git-Fixture-Tests, Pfade in 005 und RV1. Vorab-Entscheidungen D-13–D-17 in B-701 (Bestätigung in PS1.1); `check:race` lokal übersprungen (kein C-Compiler). Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `docs/arbeitsweise.md` kennt die Domäne PAP, die Session-Felder `Lane` und `Modell` und di … |
| AC-02 | erfüllt | `plan_next` liefert zwei offene autonome Sessions derselben Domäne mit verschiedenen `Lane … |
| AC-03 | erfüllt | `tools/paper/schema/paper-spec.schema.json`, `stil.schema.json` und `check.ts` validieren … |
| AC-04 | erfüllt | `docs/standards/paper-spec.md` beschreibt Spec, Stil-Datei, Lagen-Arten, Rig-Typen mit Pfl … |
| AC-05 | erfüllt | Die Fixture-SVGs aus dem Prototyp rendern mit `@resvg/resvg-js` zweimal byte-gleich; die G … |
| AC-06 | erfüllt | `docs/standards/paper-mcp.md` legt alle Asset-Tools (alle mit `checkout`), die RPC-Methode … |
| AC-07 | erfüllt | Die RPC-Methoden der Assets, der Platzhalter `ReviewPage` und das Slider-Atom liegen als S … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
