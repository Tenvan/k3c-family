# B-1183 · Stimmung: Wolken, monumentaler Hintergrund und Wellenstriche fehlen noch

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA2
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-11-21, Auftrag von 🧑)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

LA1.5 hat Lichthöfe, Nachthimmel mit Sternen, Wasser-Spiegelung und Himmelsverlauf mit Dunst umgesetzt (`data/look.json` › sky, `world/sky.rs`). Offen aus `docs/messung/stimmung-abgleich.md` (Rang 2 und 3): Wolken am Himmel, die letzte feste Hintergrund-Ebene als monumentales Bild (großer Berg, Pyramide, Vulkan) und die Wellenstriche im Wasser (Shader der Plattform, `RIVER_WAVE`), die nachts zu hell und unruhig wirken.

## Ziel

Der Tageshimmel hat Wolken, jede feste Welt hat ein monumentähnliches Hintergrundbild, und die Wellenstriche des Wassers wirken ruhiger, nah am K2C-Vorbild.

## Beteiligte und Zielgruppen

🧑 nimmt am PC ab, nennt bei Bedarf das K2C-Vorbild der Wellenstriche. Agenten setzen um.

## Anforderungen

- Wolken als Bänder im oberen Himmel, tags sichtbar, nachts abgedunkelt, deterministisch, ohne Ausreißer im Frame-Budget.
- Monumentalbild je feste Welt als neues Asset über MCP `assets_*` (Entscheidung 005), dann im Rust-Spiel angebunden.
- Wellenstriche: Farbe oder Deckkraft im Shader bzw. in `RIVER_WAVE` nachts abdunkeln, ohne Go-Golden zu ändern.

## Nicht-Ziele

Neue Wetter-Mechanik, Regeln.

## Regeln und Einschränkungen

Assets nur über `assets_*`; Golden werden nie zugunsten von Rust neu geschrieben; Datei ≤ 400 Zeilen.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `data/look.json`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-platform/**`, `public/grafik/**`
- **Abhängig von:** LA1.5
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Tag am Hub: Wolkenbänder im oberen Himmel, ein großer Berg als Silhouette hinter den Kulissen; Nacht: Wellenstriche dünn und dunkel.

## Ausnahme- und Fehlerfälle

Biome ohne Himmel (`tint.noSky`): nichts. Ein Monumentalbild braucht neue Grafik: Asset-Session statt eigener Zeichnung.

## Akzeptanzkriterien

- **AC-01** Wolken: Aufnahme Tag und Abend zeigt Bänder im oberen Himmel; Test für feste Lage.
- **AC-02** Monumentalbild: je feste Welt ein Asset angebunden, Aufnahme neben K2C-Referenz, abgenommen von 🧑.
- **AC-03** Wellenstriche nachts ruhiger, abgenommen von 🧑 am PC.

## Offene Fragen

🧑: Welche Welt bekommt welches Monument (Berg, Pyramide, Vulkan)?

## Notizen

Herkunft: `docs/messung/stimmung-abgleich.md`, LA1.5.
