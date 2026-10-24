# B-608 · Jede balancing-relevante Datei in data/ hat eine Beschreibung mit Aufbau und Werten

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

Ralf (2026-10-24): Für jede Konfig-JSON mit Einfluss auf das Balancing soll es eine Beschreibung (Markdown) geben, die Aufbau und Werte erklärt; angezeigt als Popup im Balancing-Dialog (Entscheidung Ralf: dort statt in der Planung). Heute erklären `docs/rules/*.md` die Regeln, aber nicht den Aufbau der Dateien; `goals.json` und `balance-targets.json` sind nirgends beschrieben, `guards.json`, `ruins.json`, `events.json` nur am Rand.

## Ziel

Unter `docs/data/` liegt je balancing-relevanter Datei eine Beschreibung mit Zweck, Aufbau (Felder, Typen, Einheiten), Wertebereichen, Wirkung im Spiel und Verweis auf die Regeldatei.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV/REG beim Ändern der Daten.

## Anforderungen

- Je Datei `docs/data/<name>.md` (`biomes.md`, `worlds.md` für die Ordner): Zweck, Gelesen von, Aufbau als Tabelle (Pfad, Typ, Einheit, Bedeutung, typische Werte), Wirkung auf das Balancing, Verweis auf `docs/rules/…`.
- Werte nicht kopieren, wo sie sich oft ändern: beschreiben, nicht spiegeln (Regeldateien bleiben die Quelle der Regeln).
- Ein Test prüft, dass jede balancing-relevante Datei eine Beschreibung hat und jeder Pfad der Tabelle in der Datei existiert.

## Nicht-Ziele

Regeln ändern; Client-Dateien (`sprites.json`, `assets*.json`).

## Regeln und Einschränkungen

Glossar-Begriffe (`docs/glossar.md`) verwenden; Dateinamen wie in `data/`.

## Beispiele

`docs/data/economy.md` › `purse.startGold` · Zahl · Gold · Startgold je Monarch · 100.

## Ausnahme- und Fehlerfälle

Neue Datei in `data/` ohne Beschreibung → Test rot mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Alle 17 Dateien haben eine Beschreibung (Test).
- **AC-02** Die Pfade in den Tabellen existieren in den Dateien (Test).

## Offene Fragen

keine. Default (Claude): Ablage `docs/data/`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
