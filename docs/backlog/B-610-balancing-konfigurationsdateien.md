# B-610 · Die Balancing-Seite listet die Konfigurationsdateien, die das Balancing beeinflussen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24): Er möchte sehen, welche Konfigurationsdateien (Welten u. a.) das Balancing beeinflussen. Laut Code lesen Simulation und Level-Generator aus `data/`: economy, buildings, hub, troops, enemies, waves, difficulty, bosses, guards, events, goals, island-sizes, ruins, monarch, biomes/*, worlds/*, balance-targets (Loader `engine/sim/data.go`, `data/embed.go`).

## Ziel

Die Balancing-Seite zeigt alle Dateien, die das Balancing beeinflussen, mit Zweck und den wichtigsten Werten, und öffnet je Datei ihre Beschreibung als Popup.

## Beteiligte und Zielgruppen

Ralf; Agent DEV.

## Anforderungen

- Liste „Konfiguration“ auf der Balancing-Seite: Datei, Zweck in einem Satz, wichtigste Werte, gelesen von (Sim, Level-Generator, nur Balancing-Tester), Hilfe-Knopf → Popup mit der Beschreibung (B-608).
- Die Liste kommt aus dem Backend (Datei-Liste mit Beschreibungen aus `docs/data/`), nicht fest im Frontend.
- Aus der Liste lässt sich ein Zahlenwert als Variante oder Abstimm-Parameter in die Konfiguration übernehmen.

## Nicht-Ziele

Dateien bearbeiten (macht das Studio bzw. die Hand); Client-Dateien wie `sprites.json`.

## Regeln und Einschränkungen

Nur Dateien, die die Simulation oder der Balancing-Tester liest; RPC liest nur `docs/data/*.md` (Pfadprüfung); Atomic Design.

## Beispiele

Zeile „economy.json · Börse, Steuer, Truhen, Händler · Sim“ → „?“ zeigt die Beschreibung.

## Ausnahme- und Fehlerfälle

Datei ohne Beschreibung → Hinweis in der Zeile.

## Akzeptanzkriterien

- **AC-01** Die Liste zeigt alle balancing-relevanten Dateien (Go-Test gegen die Loader-Liste, Vitest der Anzeige).
- **AC-02** Hilfe-Knopf öffnet die Beschreibung als Popup (Vitest).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
