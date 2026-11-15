# B-987 · Welten-Editor und Welten-MCP bearbeiten Endboss, Skins, Events und Plätze einer Welt

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE6
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Welten-Editor (`tools/k3c-workbench/frontend/src/world/`, u. a. `Overview.tsx`, `CrossSection.tsx`, `WorldValues.tsx`, `worldOverrides.ts`) und Welten-MCP (`tools/k3c-workbench/internal/world/`, `internal/mcpsrv/tools_world.go`) kennen die Felder aus WE5 noch nicht: Insel-Feld `endBoss` (B-981), `overrides › events` (B-982), `hub › sites`/`islandSites` (B-983), Welt-Feld `skins` (B-984).

## Ziel

Alles, was das Welt-Format seit WE5 kann, lässt sich über MCP und im Editor setzen und ansehen.

## Beteiligte und Zielgruppen

🧑 im Welten-Editor, Claude über `world_*`. 🧑 gibt frei.

## Anforderungen

- `world_set`: Insel-Feld `endBoss` (ID oder `null`), Welt-Feld `skins` (Schlüssel setzen, `null` entfernt); `world_get` zeigt beides.
- `world_override_set` nimmt `events` und `hub › sites`/`islandSites` an (Listen-Merge wie die Engine).
- Editor: Endboss-Auswahl je Insel (Liste aus `data/bosses.json`), Events und Plätze in der Werte-Ansicht, Tabelle der Skins mit Vorschau von Basis- und Ersatz-Asset.
- Die Workbench schreibt unbekannte Felder nie weg: Eine Welt mit `endBoss` und `skins` übersteht jede Änderung über MCP und Editor (Test).
- Jede Änderung läuft durch die Prüfung der Engine; Fehler erscheinen mit der Meldung der Engine.
- Oberfläche nach Atomic Design (`@webkit`, `tests/atomicDesign.test.ts`).

## Nicht-Ziele

Neue Assets anlegen (nur `assets_*`), Drag & Drop der Plätze, Events anlegen (nur Werte ändern), Balancing.

## Regeln und Einschränkungen

Radix nur über `@webkit`, rohe Controls nur in `atoms/`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/frontend/src/world/**`
- **Abhängig von:** B-981, B-982, B-983, B-984, B-985, B-986
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

`world_set sengoku {skins: {"gegner/wolf": "gegner/ninja"}}` → `world_get` zeigt die Zuordnung, der Editor die Zeile mit beiden Bildern. Endboss-Auswahl einer Insel auf „Standard“ → Feld `endBoss` entfällt.

## Ausnahme- und Fehlerfälle

Skin auf ein fehlendes Asset → Editor zeigt eine Warnung, speichert aber (die Engine prüft nur die Form). Ungültiger Platz → Meldung der Engine, nichts gespeichert.

## Akzeptanzkriterien

- **AC-01** Go-Tests: `world_set` setzt und entfernt `endBoss` und `skins`; `world_override_set` setzt `events` und `hub › sites`.
- **AC-02** Go-Test: Rundlauf einer Welt mit allen neuen Feldern verliert nichts.
- **AC-03** Frontend-Tests (Vitest) für Endboss-Auswahl und Skin-Tabelle.
- **AC-04** `check_run task:check:dev` grün; Sichtprüfung 🧑 im Editor.

## Offene Fragen

- Skin-Vorschau → Default: Standbild aus dem Bereich Assets der Workbench, keine Animation (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Anteil Editor und MCP der Punkte Endboss, Overrides, Gegner-Varianten).
