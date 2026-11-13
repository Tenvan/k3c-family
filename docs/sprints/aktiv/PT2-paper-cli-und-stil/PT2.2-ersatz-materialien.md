# PT2.2 · Ersatz-Materialien anlegen und Assets umstellen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt2/2-ersatz-materialien
- **Abhängig von:** PT2.1
- **Tickets:** B-735
- **Kriterien:** AC-04, AC-05

## Ziel

Die Stil-Datei enthält `stroh`, `lava`, `geisterweiss` und `fell-grau` (dazu was B-735 › Ausgangslage sonst nennt), die genannten Assets nutzen sie, und der Grafik-Golden ist neu.

## Kontext

B-735 › Ausgangslage listet die Ersatz-Abbildungen je Asset (Strohhut Bauer D-MG3-04, Lava-Golem/Lavaschleim/Feuergeist D-MG4-05/D-MG5-02, Grubengeist/Alphawolf D-MG4-05, Wolf auf `stoff` D-MG2-01, Eisen und Kristall bei Gegnern). Specs unter `public/grafik/<gruppe>/*.paper.json`, Stil `public/grafik/stil.paper.json`, Schema `tools/paper/schema/`. Specs nur über `assets_set` bzw. `assets_style_set` ändern (Entscheidung 005), Golden nach `docs/arbeitsweise.md` › Golden aktualisieren; `gen:paper` und Grafik-Golden laufen hier am Feature (B-742).

## Erlaubte Dateien

- `public/grafik/stil.paper.json`
- `public/grafik/*/*.paper.json`
- `tools/paper/schema/**`
- `tools/paper/golden/**`

## Nicht-Ziele

Neugestaltung von Bossen oder Gebiet (MG10, MG11), Renderer-Änderungen.

## Schritte

1. Materialien mit `assets_style_set` anlegen, Schema-Test.
2. Betroffene Assets mit `assets_set` umstellen.
3. `task gen:paper`, Grafik-Golden aller Gruppen neu, Begründung in den Commit.
4. Ergebnis eintragen, B-735 auf `erledigt`; Sichtprüfung im Bereich Assets als Hinweis für 🧑 (nicht blockierend).

## Fertig, wenn

- [ ] AC-04: Materialien vorhanden, Schema-Test grün (`B-735/AC-01`).
- [ ] AC-05: keine Ersatz-Abbildung mehr, Golden neu und grün (`B-735/AC-02`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-04 geprüft: `stroh`, `lava`, `geisterweiss`, `fell-grau` per `assets_style_set` angelegt; `tools/paper/golden/ersatz.test.ts` › „Stil-Datei (B-735/AC-01)“ samt `checkStil` grün, `tools/paper/schema` grün.
- AC-05 geprüft: 19 Assets per `assets_set` umgestellt (Bauer-Hut → stroh; Lava-Golem, Lavaschleim, Feuergeist → lava; Grubengeist, Alphawolf → geisterweiss; Alphawolf-Latz, Wolf (Gegner) und Wolf (Reittier, vorher `stoff`) → fell-grau; Eisen für Eisenkäfer-Panzer, Skelett-Schwert/-Schild, Goblin-Dolch, Pfeilspitze, Keulen-Beschlag, Goblin-Anführer, Troll-König-Schild, Ratten-Königin-Armschienen; Kristall statt `edelstein`/`koenigsblau` bei Kristallspinne, Splitterwicht, Kristallwächter). `ersatz.test.ts` › „B-735/AC-02“ grün. `gen:paper` (`bun tools/paper/bin/gen.ts`) gelaufen: `golden.json` und `index.paper.json` aller Gruppen neu (neuer `stilHash` → neuer `renderKey`); Fixture-Golden `fixtures.golden.json` neu, nur `renderKey`, PNG-Hashes gleich. Grafik-Golden `tools/paper/golden` (alle Gruppen, `public.test.ts`) und `tests/paperAssets.test.ts` grün.
- `task check:diff` grün.
- Abweichung: Die Specs wurden über das MCP-Tool `assets_set` geändert, aber per Skript über die HTTP-Schnittstelle der Workbench aufgerufen (Merge-Patch auf `teile` je Asset); ein Teil-Patch fehlt noch (B-710).
- Hinweis für 🧑 (nicht blockierend): Sichtprüfung der 19 Assets im Bereich Assets; deutlich anders sehen Feuergeist, Lavaschleim, Grubengeist, Alphawolf und die Wölfe aus. Mehr umgestellt als genannt: Bosse-Waffen (Goblin-Anführer, Troll-König, Ratten-Königin) und Höhlentroll-Keule (Eisen bei Bossen und Gegnern).
- Neue Tickets: keine.
