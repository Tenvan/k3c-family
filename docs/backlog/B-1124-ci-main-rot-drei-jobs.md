# B-1124 · Drei CI-Jobs sind auf main rot: Natives Spiel, Go arm64 und k3c-workbench

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU37
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Auf `main` (Lauf 44431405688, RU53) und auf `sprint/ru35` schlagen drei Jobs fehl, unabhängig von RU35: (1) `Natives Spiel` (Windows): `TestImportRule` in `native/k3c-game/game` meldet `props/rustref_test.go` importiert `world` und `world/rustref_test.go` importiert `actors`; dazu fehlt dort die gerenderte Grafik (`public/grafik/natur/baum/manifest.json`, der Job ruft `gen:paper` nicht). (2) `Go · Tests arm64`: Golden-Tests `TestGoldenLaeufe`, `TestGoldenKampagneAusbau`, `TestWriteVectors` rot (FMA-Abweichung vermutet, ungeprüft). (3) `k3c-workbench`: Vitest `expected ... not to contain '⎇'` (Frontend-Test).

Hinweis zum „skipping“ in PRs: Alle Jobs außer `merge-konflikte` haben `if: github.event_name != 'pull_request'` (bewusst, Kopfkommentar der ci.yml); Checks erscheinen im PR als „skipping“ und laufen erst nach dem Merge auf main. Das ist kein Pfadfilter-Fehler.

## Ziel

Die CI auf `main` ist grün.

## Beteiligte und Zielgruppen

Entwickler; 🧑 Ralf entscheidet, ob Rust-Checks im PR laufen sollen.

## Anforderungen

- Jeder der drei Jobs grün oder bewusst entfernt (Go-Job mit RU37).

## Nicht-Ziele

Golden zugunsten von Rust neu schreiben (verboten).

## Regeln und Einschränkungen

Golden nie neu schreiben, um Rust zu bedienen.

- **Bereich:** `.github/workflows/ci.yml`, `native/k3c-game/**`, `tools/k3c-workbench/frontend/**`, `engine/**`
- **Abhängig von:** RU35
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden

## Beispiele

`gh run list --branch main` zeigt alle Jobs success.

## Ausnahme- und Fehlerfälle

nicht relevant: CI-Befund.

## Akzeptanzkriterien

- **AC-01** Job `Natives Spiel` grün oder mit RU37 entfernt.
- **AC-02** Job `Go · Tests arm64` grün (Ursache der Golden-Abweichung geklärt).
- **AC-03** Job `k3c-workbench` grün.

## Offene Fragen

Sollen die Rust-Checks auch im Pull Request laufen (heute „skipping“)? Entscheidung 🧑 Ralf.

## Notizen

Gefunden in RU35.3 beim CI-Lauf per workflow_dispatch.
