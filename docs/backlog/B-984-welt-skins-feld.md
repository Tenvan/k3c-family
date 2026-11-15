# B-984 · Die Welt-Datei ordnet mit skins Gegnern und Objekten eigene Assets zu (Gegner-Varianten je Welt)

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gegner und Objekte sehen in jeder Welt gleich aus: Die Zuordnung Art → Asset steht einmal in `data/paper.json` (`gegner`, `objekte`). Eine Welt kann über `overrides › enemies` Werte ändern, aber keine eigene Gegnerart mit eigenem Namen und Aussehen haben (Praxistest WE3). Die Welt-Datei wird streng gelesen (`engine/sim/island_worlds.go`), ein unbekanntes Feld wird abgelehnt.

## Ziel

Eine Welt hat Gegner-Varianten und eigene Objekte: Die Variante übernimmt die Werte einer Basisart, trägt einen eigenen Namen und ein eigenes Asset.

## Beteiligte und Zielgruppen

Spieler sehen je Welt eigene Gegner. Welten-Autoren (🧑, Claude über `world_set`). 🧑 gibt frei.

## Anforderungen

- Welt-Feld `skins` (optional): Objekt Basis-Asset → Ersatz-Asset, beide als `<gruppe>/<asset>` wie in `data/paper.json` (z. B. `"gegner/wolf": "gegner/ninja"`).
- Die Engine prüft nur die Form (Schlüssel und Werte nach dem Muster, Gruppe links und rechts gleich, kein Verweis auf sich selbst) und reicht `skins` über `WorldDef` an Spiel und Workbench weiter; sie rechnet nicht damit.
- Eine Gegner-Variante ist: Basisart (alle Werte), optional Werte und `name` über `overrides › enemies › <art>`, Aussehen über `skins`. Ein Test belegt, dass `name` übersteuerbar ist.
- `extends`: Skins der Eltern-Welt gelten, das Kind überschreibt je Schlüssel.
- Kein Einfluss auf Simulation, Level und Spielstand; `testdata/golden/` unverändert.
- `docs/rules/welten.md` § 2 und § 3 beschreiben Feld und Prüfung.

## Nicht-Ziele

Neue Gegnerarten mit eigenen IDs, Zeichnen im Spiel (B-994), Editor (B-987), die Assets selbst (B-988 bis B-990).

## Regeln und Einschränkungen

Sim-Code im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Die Engine liest keine Grafik-Dateien.

- **Bereich:** `engine/sim/**`, `docs/rules/welten.md`
- **Abhängig von:** B-983
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku: `skins: {"gegner/wolf": "gegner/ninja"}` und `overrides › enemies › wolf › name: "Ninja"` → der Wolf heißt Ninja, hat Wolf-Werte und das Ninja-Asset. Welt ohne `skins` → alles wie bisher.

## Ausnahme- und Fehlerfälle

Wert ohne Gruppe (`"ninja"`) oder Gruppenwechsel (`gegner/…` → `objekte/…`) → Welt lädt nicht, Meldung `skins › gegner/wolf: …`. Asset fehlt auf der Platte → kein Fehler der Engine; das Spiel fällt auf das Basis-Asset zurück (B-994).

## Akzeptanzkriterien

- **AC-01** Test: Welt mit `skins` lädt, `WorldDef` trägt die Zuordnung; mit `extends` gemischt.
- **AC-02** Tests der Fehlerfälle mit Meldung.
- **AC-03** Test: `overrides › enemies › <art> › name` ändert den Namen.
- **AC-04** `testdata/golden/` unverändert; `docs/rules/welten.md` nachgezogen.

## Offene Fragen

- Ablage → Default: `skins` in der Welt-Datei (die Welt bleibt als eine Datei weitergebbar), nicht je Welt in `data/paper.json` wie die Weltbilder (vorläufig, 🧑 bestätigt).
- Variante → Default: keine neuen Gegner-IDs; eine Variante ersetzt in ihrer Welt die Basisart ganz (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Gegner-Varianten je Welt).
