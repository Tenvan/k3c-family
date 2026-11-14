# B-864 · Ein MCP-Tool plan_index baut Haupt- und Archiv-Index neu auf

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

WZ47.4: `RebuildIndex` gibt es nur als Go-Funktion und über einen Test mit Umgebungsvariable (`K3C_REBUILD_INDEX`).

## Ziel

Ein MCP-Tool `plan_index` baut Haupt- und Archiv-Index aus den Dateien neu auf.

## Beteiligte und Zielgruppen

🧑 (Freigabe, Idee); planende Agenten.

## Anforderungen

- `plan_index {checkout, dry?}`; Dry-Run nennt die Änderungen.

## Nicht-Ziele

Digest.

## Regeln und Einschränkungen

Typ Idee: Freigabe durch 🧑.

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`
- **Abhängig von:** WZ47
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** exe

## Beispiele

Index nach Merge-Konflikt kaputt → `plan_index` stellt ihn wieder her.

## Ausnahme- und Fehlerfälle

Dateien ungültig → Fehler mit Liste.

## Akzeptanzkriterien

- **AC-01** MCP-Test: `plan_index` baut beide Index-Dateien wie `RebuildIndex`.

## Offene Fragen

Freigabe durch 🧑.

## Notizen

Vorschlag WZ47.4 (2026-11-14).
