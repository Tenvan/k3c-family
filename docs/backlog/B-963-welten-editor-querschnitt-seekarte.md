# B-963 · Welten-Editor zeigt alle Inseln als Querschnitt nach Etappen mit Seekarte zum Springen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE4
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15 (Mock „So umsetzen“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Welten-Seite der Workbench (`tools/k3c-workbench/frontend/src/world/`) zeigt eine Insel nach der anderen. 🧑 2026-11-15 hat das Mock „Querschnitt + Seekarte“ für Sengoku freigegeben.

## Ziel

Der Welten-Editor zeigt alle Inseln untereinander mit ihren Stufen als Streifen auf einer gemeinsamen Mittelachse, gruppiert nach Etappen, und links eine Seekarte, über die man zur Insel springt; die Etappe einer Insel ist im Editor und per MCP setzbar.

## Beteiligte und Zielgruppen

Welten-Autor (🧑, Claude über MCP).

## Anforderungen

- Querschnitt: je Insel eine Karte, jede Stufe ein Streifen (Breite ∝ Länge, Farben aus der Biom-Palette, Chunk-Raster), Streifen mittig ausgerichtet; Marker Burg, Portale mit HP, Treppe, Endboss, Hafen.
- Etappen-Bänder über den Inseln einer Etappe (Anzahl, „frei wählbar“, 🔓/🔒).
- Seekarte (beim Scrollen stehend): Etappen untereinander, Inseln als Knoten mit Mini-Querschnitt; Klick springt zur Insel und hebt sie kurz hervor.
- Etappe je Insel im Editor setzbar; MCP `world_set`/`world_get` kennen `etappe`; Prüfung wie beim Laden (B-961).
- Oberfläche über `@webkit` (Atomic Design), Datei ≤ 400 Zeilen.

## Nicht-Ziele

Drag & Drop zwischen Etappen (später), echte Level-Positionen der Marker (schematisch, später über `WorldLevel`).

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/**`, `tools/webkit/**`
- **Abhängig von:** B-961
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Sengoku: Seekarte zeigt Etappe 1 (Reisfeld-Ebene), Etappe 2 (Bambuswälder, Bergfestung); Klick auf Bergfestung scrollt zur Karte der Insel 3.

## Ausnahme- und Fehlerfälle

Welt ohne Etappen → jede Insel eigene Etappe (linear). Ungültige Etappe beim Speichern → Fehlermeldung der Prüfung, nichts geschrieben.

## Akzeptanzkriterien

- **AC-01** Vitest: Querschnitt rendert alle Inseln und Stufen nach Etappen gruppiert; Seekarten-Klick springt zur Insel.
- **AC-02** Go-Test: MCP `world_set` setzt `etappe` und lehnt Lücken ab.
- **AC-03** Seite im Browser-Pane mit Sengoku geprüft (Screenshot).

## Offene Fragen

keine

## Notizen

Mock: Scratchpad `welten-querschnitt.html` (Session 2026-11-15).
