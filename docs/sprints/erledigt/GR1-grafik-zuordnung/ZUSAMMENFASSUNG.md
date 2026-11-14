# GR1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Grafik-Zuordnungstabelle
- **Projekt:** –

## Ziel

Jedes Spielobjekt hat eine Zuordnung zu Asset, Stil und Lizenz oder eine dokumentierte Lücke. Am Ende sichtbar: `docs/assets/zuordnung.md` und ein grüner Vollständigkeits-Test.

## Ergebnis

2026-10-04, Agent (Claude Sonnet) in GR1.4, leichtes Review. AC-01, AC-03 bis AC-05: Ergebnis GR1.2 (Tabelle `docs/assets/zuordnung-objekte.md`, Test `src/tools/zuordnung.test.ts`); AC-02: Ergebnis GR1.3 (`docs/assets/zuordnung-welt.md`); AC-05 Kopf: Workshop GR1.1 mit 🧑; AC-06: `task check` grün. Schwere Befunde: keine. Neue Tickets: B-251 (Figuren-Skalierung). B-161 archiviert; 24 Lücken gehen an GR2 (B-162), Skill-Icons an B-124, Bosse an B-130. Version: v0.6.1 vorgeschlagen (Patch: nur Doku und Test, keine Spieländerung); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `docs/assets/zuordnung.md` hat zu jeder Objekt-ID aus `data/buildings.json`, `data/enemies … |
| AC-02 | – | Weitere Objekte (Hub-Stufen, Materialstufen, Materialien, Adern, Plantage, Truhen, Portale … |
| AC-03 | – | Jedes zugeordnete Asset hat einen Credit-Eintrag (B-161/AC-03). |
| AC-04 | – | Jede Lücke ist fett markiert und verweist auf ein Ticket (B-161/AC-04). |
| AC-05 | – | Jede Zeile nennt Stil und Lizenz, der Stilbeschluss von 🧑 steht im Kopf (B-161/AC-05). |
| AC-06 | – | `task check` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-161 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 – | Jedes Spielobjekt hat eine Zuordnung zu Asset und Lizenz oder eine dokumentierte … |
