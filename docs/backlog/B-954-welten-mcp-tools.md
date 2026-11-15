# B-954 · Der Welten-Editor hat MCP-Tools und kann neue Welten anlegen

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WE2
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Test des Welten-Editors (Auftrag 🧑 über Schaltzentrale, 2026-11-15): Vier Welten (Germania, Wikinger in Amerika, Hanse, Römisches Reich) sollten nur über MCP entstehen. Befund: Der MCP-Server `k3c-workbench` hat kein Welt-Tool. Die Wails-Bindings (`tools/k3c-workbench/app_world.go`) kennen nur `WorldList`, `WorldDetail`, `WorldLevel`, `WorldValues`, `WorldOverrideSet`, `WorldNatureSet` – eine neue Welt anlegen, Inseln/Tiefen/Biome setzen oder `plannedChunks` bearbeiten geht weder per MCP noch per Oberfläche.

## Ziel

MCP-Tools `world_list`, `world_get`, `world_create {id, name, extends?}`, `world_set` (Kopf, Inseln, Tiefen, Biome, `combat`, Start-Handwerker), `world_override_set`, `world_chunks_set` (`plannedChunks`), `world_check`; Oberfläche bekommt „Welt anlegen“. Schreiben format-stabil nach `data/worlds/<id>.json`, Prüfung wie beim Laden (`engine/sim/island_worlds.go`).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/**`, `engine/sim/island_worlds.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Welt per MCP anlegen, setzen, prüfen; Roundtrip-Test.
- **AC-02** Oberfläche: Welt anlegen.
- **AC-03** Die vier Entwürfe (Notiz in diesem Ticket) lassen sich damit anlegen.

## Offene Fragen

keine

## Notizen

Entwürfe (Konzept, nicht angelegt):
- **germania** – Germania zur Römerzeit: Wald- und Moorinseln, Gegner „Legion“ dringt über Limes-Portale ein; Biome forest→cave; combat groups; Overrides: mehr Holz-Gold (B-952), dichte Natur.
- **vinland** – Wikinger in Amerika: Küsteninseln mit Schiff als Inselwechsel, raue Winter; Biome forest→crystal (Eis); combat general (Anführer mit Banner); Overrides: weniger Startgold, Truhen häufiger.
- **hanse** – Hanse-Zeitalter: Handelsstädte, starke Händler, Überfälle von Piraten; Biome forest→mine; combat groups; Overrides: Händler-Rabatt, mehr Händler-Überfälle.
- **roma** – Römisches Reich: Steinbauten, Legionäre als eigene Truppen, Barbaren als Gegner; Biome forest→ironhold; combat general; Overrides: Stein günstiger, höhere Burgstufen früher.
