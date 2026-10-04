# GR1.3 · Restliche Spielobjekte erfassen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr1/3-restliche-objekte
- **Abhängig von:** GR1.2
- **Tickets:** B-161
- **Kriterien:** AC-02, AC-06

## Ziel

Die Zuordnung deckt zusätzlich Hub-Stufen 1–5, Mauer-/Turm-Materialstufen, die fünf Materialien, Adern, Plantage, Truhen, Portale, UI-/Skill-Icons und Hintergründe je Biom (Wald, Höhle, Mine) ab, jeweils zugeordnet oder als Lücke mit Ticket.

## Kontext

- Regeln: `docs/rules/materialien-gebaeude.md` (§ 1 Materialien Holz, Stein, Kupfer, Eisen, Kristall, Adern, Plantage; § 2 Hub-Stufen; § 3.1 Mauer und Turm Stufe 1–5, Turm-Stufe 5 = Zaubertum). Diese Objekte gibt es teils noch nicht im Code (B-112, W1; B-114, W2); sie werden trotzdem als Zeile erfasst, Herkunft = Regel.
- Truhen: `data/economy.json` › `chestGold`; Portale und Münzen: Packs `portals-32-x-48`, `16x16-small-and-medium-coin-animation`, `gold-treasure-icons-16x16`; Erze: `various-stones-and-oregem-veins-16x16`; Hintergründe: `forest-background`, `blue-cave-background`, `sunnyland-*`, `gothicvania-*`, `warped-caves-pixel-art-pack`; Biome: `data/biomes/` (Wald, Höhle, Mine; Mine ohne Treffer laut B-161).
- Icons: Gruppe `icons` in `public/grafik/index.json` (11 Bilder); Skill-Icons gibt es noch nicht (Skills B-119) → Lücke, später.
- Bosse nur als „Lücke, später“ (B-161 › Nicht-Ziele). Reittiere (Q23) als zugeordnete Zeilen aus `data/sprites.json` › `mounts`.
- Stil je Pack aus GR1.1; Test aus GR1.2 prüft Credits und Lücken-Tickets auch für die neuen Zeilen.

## Erlaubte Dateien

- `docs/assets/` (Tabelle)
- `src/tools/` (Test aus GR1.2 erweitern: Pflichtgruppen aus AC-02 vorhanden)
- `docs/backlog/` (Status, neue Tickets für Lücken, falls B-162 sie nicht abdeckt)
- `docs/sprints/` (nur Status dieser Session)

## Nicht-Ziele

Neue Assets (GR2), Einbau (GR3), Atlas (GR4), Änderung von `data/` oder `docs/rules/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Zeilen je Gruppe aus AC-02 anlegen (Hub-Stufe 1–5, Mauer 1–5, Turm 1–5, fünf Materialien, Adern, Plantage, Truhen, Portale, Icons, Hintergründe Wald/Höhle/Mine).
3. Test erweitern: Jede Pflichtgruppe aus AC-02 hat mindestens eine Zeile (Hub-, Mauer- und Turm-Stufen je 1 bis 5, alle fünf Materialien, alle drei Biome).
4. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-02: Test belegt Zeilen für alle Gruppen aus AC-02, jede mit Status, Stil und Lizenz.
- [x] AC-06: `task check` grün; jede Datei ≤ 400 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

Neue Tabelle `docs/assets/zuordnung-welt.md` (ID-Konvention im Kopf: `hub:n`, `wall:n`, `tower:n`, `material:…`, `vein:…`, `node:…`, `pickup:…`, `mount:…`, `icon:…`, `bg:<Biom>` …), 64 Zeilen: 45 zugeordnet, 19 Lücken.

- **AC-02:** geprüft (Test `src/tools/zuordnung.test.ts`) – Hub-, Mauer-, Turm-Stufen je 1–5, fünf Materialien, Adern, Plantage, Truhe, Portal, Münze, 13 Reittiere (aus `data/sprites.json` › mounts), 11 Icons (aus `public/grafik/index.json`), Skill-Icons, Bosse, Hintergründe aller Biome (aus `data/biomes/`), zusätzlich Ressourcen (aus `data/economy.json` › gatherables und Biomen), Rekrutierungslager, Ausgang, Pickups. Rot gesehen mit fehlender Zeile `tower:5` bzw. `node:bush`.
- Lücken: `hub:2–5`, `wall:1`, `wall:3–5`, `tower:1`, `tower:3–5`, `material:wood`, `plantation`, `bg:mine`, `camp:recruit`, `pickup:skillPoint` (B-162); `skill-icon` (B-124, später); `boss` (B-130, später).
- Vermerk: `wall`/`tower` in `zuordnung-objekte.md` zeigen Stufe 2 (Stein); Stufe 1 (Holz) ist Lücke. Eisen-Erz = Zeile 3 im Erz-Sheet ist eine Deutung des Agenten (ungeprüft durch 🧑).
- **AC-06:** geprüft – `task check` grün (56 Dateien, 1046 Tests); alle Dateien ≤ 400 Zeilen.
