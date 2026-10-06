# B-063 · k3c-dev macht Xbox-Berichte und Spielstände für Agenten lesbar

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M2
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M2 Revision 1 mit B-062 und B-063)

## Ausgangslage

Die Gamepad-Testseite schickt Berichte nach `reports/*.json` (`server/reports.mjs`), Spielstände liegen in `saves/`
(`server/saves.mjs`). Agenten lesen beides als rohes JSON. Der MCP-Server aus B-046 kennt beides nicht.

## Ziel

`k3c-dev` macht Xbox-Berichte und Spielstände für Agenten lesbar. Nutzen: Xbox-Ergebnisse und Spielstände ohne
JSON-Wühlen und ohne freien Dateizugriff.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; Auswertung der Xbox-Tests (X1).

## Anforderungen

- Paket `tools/k3c-dev/internal/gamedata`, Tools im Katalog aus B-046, alle `readOnlyHint`.
- **Berichte** (Felder aus `src/tools/gamepadTest.ts`: `createdAt`, `receivedAt`, `userAgent`, `pads` als Map Index →
  `id`, `mapping`, `buttonsSeen`; `maxSimultaneousPads`, `perf` mit `sprites`, `avgFps`, `minFps`; `fullscreenAttempts`,
  `backNavigations`, `keyEvents`). Tastennamen `A B X Y LB RB LT RT View Menu LS RS ↑ ↓ ← → Xbox` (Index 0–16).
  - `reports_list`: neueste zuerst (nach `receivedAt`, sonst Dateiname), eine Zeile je Bericht: Datum, Gerät (Inhalt
    der ersten Klammer im `userAgent`), Anzahl Controller, FPS der größten Sprite-Stufe.
  - `report_read {name}`: Datum, Gerät, je Controller `id`, `mapping` und gesehene Tasten als Namen, min/Ø FPS je
    Sprite-Stufe, Vollbild-Versuche (`via` → ok/Fehler), Zurück-Navigationen, Anzahl Tasten-Ereignisse.
- **Spielstände** (Format `SaveGame` in `src/world/sim/campaign.ts`; Ordner `saves/` oder `K3C_SAVES_DIR` wie
  `server/saves.mjs`; Sicherungen heißen `<slot>-<savedAt>.json`): `saves_list` → eine Zeile je Datei: Slot, Stufe
  (`depth`, freigeschaltet `unlockedDepth`), Tag, Spieler (`players.length`), `savedAt`. Tag =
  `floor(time / Zykluslänge) + 1`; Zykluslänge in Sekunden = `(dayMinutes + twilightMinutes + nightMinutes) × 60` aus
  `data/biomes/forest.json` › `cycle`, zur Laufzeit aus der Repo-Wurzel gelesen, nicht im Code wiederholt.
- Instructions (B-046) nennen die drei Tools.

## Nicht-Ziele

Berichte oder Spielstände schreiben, löschen, umbenennen; Anzeige in der Oberfläche; Änderungen an `server/`, `src/`, `data/`.

## Regeln und Einschränkungen

Nur Standardbibliothek. Pfade nur innerhalb von `reports/` und des Spielstand-Ordners. Beispieldaten für Tests
erfinden, keine echten Berichte einchecken (`reports/`, `saves/` stehen in `.gitignore`).

## Beispiele

- `reports_list` → `2026-09-30 21:14 · Xbox; Xbox One · 2 Controller · 58 FPS bei 4000 Sprites`.
- `saves_list` → `autosave · Stufe 2/3 · Tag 5 · 2 Spieler · 2026-09-30 20:02`.

## Ausnahme- und Fehlerfälle

- `report_read` mit `..`, absolutem Pfad, Unterordner oder anderer Endung als `.json` (Name nur
  `^[A-Za-z0-9_.-]+\.json$`) → Ablehnung; der aufgelöste Pfad muss in `reports/` liegen.
- Kaputte Datei → Zeile `<name> · nicht lesbar`, die anderen bleiben sichtbar.
- Fehlender Ordner → `keine Berichte` bzw. `keine Spielstände`, kein Fehler.

## Akzeptanzkriterien

- **AC-01** `reports_list` und `report_read` liefern die verdichteten Zeilen an Beispieldateien in `testdata/` (Tests).
- **AC-02** `saves_list` rechnet den Tag aus `forest.json` und beachtet `K3C_SAVES_DIR` (Tests).
- **AC-03** Pfade außerhalb von `reports/` werden abgelehnt, kaputte Dateien erscheinen als `nicht lesbar` (Tests).

## Offene Fragen

keine

## Notizen

Aus B-046 Revision 1 übernommen (2026-09-30).
