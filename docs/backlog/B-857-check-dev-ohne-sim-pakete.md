# B-857 · check:dev testet die Sim-Pakete der Workbench nicht, solange die Sim-Sperre gilt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

WZ48.4: `k3c-workbench:test` läuft lokal über `./...` und nimmt `internal/balance`, `simrun`, `simexe` und `botdev` trotz Sim-Sperre mit; nur die CI filtert sie über `WB_SIM_PKGS`.

## Ziel

`task check:dev` und `check_run dev:test` lassen die Sim-Pakete der Workbench aus, solange die Sperre gilt; `CHECK_SIM=1` nimmt sie wie bei `check:go` mit.

## Beteiligte und Zielgruppen

Claude (Umsetzung); 🧑 (Sperre).

## Anforderungen

- Eine Paketliste (Quelle wie `tools/checkdiff` › `simDirs` bzw. `WB_SIM_PKGS`).
- Scheitert die Liste, bricht der Lauf ab statt alles zu testen.

## Nicht-Ziele

Sim-Tests selbst.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/Taskfile.yml`, `tools/checkdiff/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task check:dev` → `internal/balance` erscheint nicht in der Paketliste.

## Ausnahme- und Fehlerfälle

Paketliste leer → Abbruch mit Meldung.

## Akzeptanzkriterien

- **AC-01** Test oder Nachweis: `k3c-workbench:test` ohne Sim-Pakete, mit `CHECK_SIM=1` mit.

## Offene Fragen

keine

## Notizen

Befund WZ48.4 (2026-11-14).
