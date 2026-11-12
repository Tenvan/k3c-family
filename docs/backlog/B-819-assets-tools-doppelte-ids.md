# B-819 · Die assets_*-Tools erreichen auch Assets, deren ID in zwei Gruppen vorkommt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Die ID `wolf` gibt es zweimal: `public/grafik/reittiere/wolf.paper.json` und `public/grafik/gegner/wolf.paper.json`. `assets_get`/`assets_set` mit `id: wolf` treffen immer das Reittier, `gegner/wolf` wird als „Asset fehlt“ abgelehnt (belegt 2026-11-12 in NG17). Damit lässt sich der Gegner-Wolf nicht regelkonform ändern (Assets nur über `assets_*`, Entscheidung 005); B-804 AC-02 bleibt für ihn offen.

## Ziel

Jedes Asset ist über die MCP-Tools eindeutig erreichbar, auch bei gleicher ID in zwei Gruppen.

## Beteiligte und Zielgruppen

Claude und andere Agenten (Asset-Arbeit), 🧑 Ralf.

## Anforderungen

- `id` darf die Form `<gruppe>/<id>` haben; bei mehrdeutiger ID ohne Gruppe lehnt das Tool mit Liste der Treffer ab statt still das erste zu nehmen.
- Gilt für alle `assets_*`-Tools.

## Nicht-Ziele

IDs umbenennen.

## Regeln und Einschränkungen

Entscheidung 005. Neubau der Workbench-EXE nötig.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `tools/k3c-workbench/internal/**`, `tools/assetkit/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

`assets_set {id: "gegner/wolf", patch: …}` ändert `public/grafik/gegner/wolf.paper.json`; `assets_get {id: "wolf"}` meldet „mehrdeutig: reittiere/wolf, gegner/wolf“.

## Ausnahme- und Fehlerfälle

Unbekannte Gruppe → Ablehnung mit den vorhandenen Gruppen.

## Akzeptanzkriterien

- **AC-01** Test: `gegner/wolf` lesbar und änderbar über `assets_get`/`assets_set`.
- **AC-02** Test: mehrdeutige ID ohne Gruppe wird mit Trefferliste abgelehnt.
- **AC-03** Danach B-804 AC-02 für den Gegner-Wolf nachholen (`proportionen.fuss: pfote`).

## Offene Fragen

keine

## Notizen

Gefunden in NG17.1 (2026-11-12).
