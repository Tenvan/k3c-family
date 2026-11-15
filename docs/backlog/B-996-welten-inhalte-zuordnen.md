# B-996 · Sengoku, Nil und Britannien nutzen ihre Assets, Gegner-Varianten und Events

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WE8
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sengoku, Nil und Britannien (`data/worlds/*.json`, gitignored, Kopie im Spielpaket) nutzen nur vorhandene Assets, Gegner und Events. Mit WE5 bis WE7 gibt es Skins (B-984, B-994), Event-Overrides (B-982), die Events Überfall, Sandsturm, Nebel (B-991, B-992) und die Assets (B-988 bis B-990).

## Ziel

Jede der drei Welten sieht anders aus und spielt sich anders: eigene Objekte, Gegner-Varianten mit Namen und ihr Event.

## Beteiligte und Zielgruppen

Spieler (2 Spieler). Claude ordnet über `world_*` zu, Agent `k3c-welten` prüft, 🧑 nimmt am PC ab.

## Anforderungen

- Sengoku: Skins Torii, Pagode, Samurai-Lager, Ninja (für Wolf), Samurai (für Höhlentroll); Namen „Ninja“ und „Samurai“ über `overrides › enemies`; Event `raid` an (Ninja-Überfall).
- Nil: Skins Palme, Obelisk, Barke, Skorpion (für Wolf), Mumie (für Zombie) mit Namen; Event `sandstorm` an.
- Britannien: Skins Steinkreis, Rundhaus, römischer Späher (für Goblin-Bogenschütze), Moorgeist (für Grubengeist) mit Namen; Event `fog` an.
- Gesetzt nur über `world_set` und `world_override_set`; Werte der Varianten bleiben die der Basisart (kein Balancing).
- `world_check` für alle drei Welten ohne Fehler; die Standard-Welt bleibt unberührt.

## Nicht-Ziele

Neue Biome, Balancing der Varianten, eigene Endbosse je Welt (B-981 liefert nur das Feld), Geschichten je Welt (Sprint GS1).

## Regeln und Einschränkungen

Weltdateien nur über `world_*`. Keine Simulation, kein Balancing.

- **Bereich:** `data/worlds/sengoku.json`, `data/worlds/nil.json`, `data/worlds/britannien.json`
- **Abhängig von:** B-979, B-987, B-988, B-989, B-990, B-991, B-992, B-994, B-995
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku starten → Torii am Ausgang, Ninja statt Wölfe, in der 5. Nacht der Ninja-Überfall. Standard-Welt starten → alles wie vorher.

## Ausnahme- und Fehlerfälle

Eine Welt enthält die Basisart einer Variante nicht (z. B. kein Zombie, weil die Mine fehlt) → Skin trotzdem setzen, er wirkt nur, wo die Art vorkommt; im Ergebnis melden. Ein Asset wurde in WE7 verschoben → Skin weglassen, Folgeticket.

## Akzeptanzkriterien

- **AC-01** `world_get` zeigt je Welt die Skins, Namen und das eingeschaltete Event.
- **AC-02** `world_check sengoku`, `nil`, `britannien` ohne Fehler.
- **AC-03** Sichtprüfung 🧑 am PC: je Welt Objekte, Gegner-Varianten und Event-Nacht in beiden Bildhälften (deckt B-979/AC-03, B-994/AC-04, B-995/AC-03).

## Offene Fragen

- Zuordnung Variante → Basisart → Default: Ninja = Wolf, Samurai = Höhlentroll, Skorpion = Wolf, Mumie = Zombie, römischer Späher = Goblin-Bogenschütze, Moorgeist = Grubengeist (vorläufig, 🧑 bestätigt).
- „Samurai-Lager“ → Default: Aussehen des Portals in Sengoku, keine eigene Gegnerart (vorläufig, 🧑 bestätigt).
- Abstände der Events → Default: Werte der Basis-Vorschläge aus B-991/B-992 (vorläufig, Balancing später).

## Notizen

Zerlegt aus B-960 (AC-01 Zuordnung, AC-03 Sichtprüfung).
