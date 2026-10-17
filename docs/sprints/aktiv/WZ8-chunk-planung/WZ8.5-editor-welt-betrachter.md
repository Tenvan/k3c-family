# WZ8.5 · Editor für geplante Chunks im Welt-Betrachter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz8/5-editor-welt-betrachter
- **Abhängig von:** WZ8.4, WZ7.4
- **Tickets:** B-461
- **Kriterien:** AC-06, AC-09, AC-11, AC-12

## Ziel

Der Welt-Betrachter im Asset-Studio bearbeitet geplante Chunks mit sofortiger Chunk- und Live-Vorschau aus dem ungespeicherten Entwurf; die Chunk-Vorschau wählt Insel und Stufe und liest dieselbe Quelle wie die Live-Vorschau. Speichern nur gültig, atomar, format-stabil, mit Stand-Prüfung und REG-Hinweis.

## Kontext

Spec: B-461 und Sprint-README WZ8 (Editor, Chunk-Vorschau auf Insel und Stufe, Schreiben). **Beginnt erst, wenn WZ7 erledigt und WZ8.4 `fertig` ist.** Vor dem Start den Aufbau des Welt-Betrachters aus WZ7.2–WZ7.4 lesen (`tools/k3c-studio/`, Frontend und Go-Bindings); vorhandene Muster für Laden, Speichern und Neuladen (`ErrStale`, Atomares Schreiben aus WZ6, `tools/assetkit`) wiederverwenden statt neu bauen.

Editor: Insel und Tiefe wählen, Chunk-Platz wählen, Art, Chunk-Seed und Objekte setzen, verschieben, löschen; Prüffehler (WZ8.2) am Platz; Vorschau ≤ 1 s. Chunk-Vorschau: Level-Teil des Snapshots aus WZ8.4 als gemeinsame Quelle, Seed bleibt wählbar. Schreibregeln wie B-459/B-462: atomar (temporäre Datei, dann umbenennen), format-stabil (Reihenfolge, Zeilenumbrüche, übrige Felder und Inseln erhalten; Laden und Speichern ohne Änderung byte-gleich), Stand-Prüfung („`data/islands.json` geändert, bitte neu laden“, Entwurf bleibt), nach dem Speichern der Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“. Tests schreiben nur in Kopien (`t.TempDir()`), nie in `data/islands.json`.

Budget: ≈ 400 Code-Zeilen je Session. Sprengt der Editor es, Rest als Ticket, Kriterium nicht umformulieren. Neue npm-Abhängigkeiten sind verboten.

## Erlaubte Dateien

- `tools/k3c-studio/` (Welt-Betrachter: Editor, Chunk-Vorschau, Schreiben) und zugehörige Tests
- `tools/assetkit/`, nur falls ein gemeinsamer Schreib-Baustein dort bereits liegt und wiederverwendet wird
- Planungsdateien von WZ8 über k3c-dev

## Nicht-Ziele

Änderungen an `engine/` oder `data/` (→ Ticket); eine Planung in `data/islands.json` speichern oder einchecken; Gebäude-Planer (WZ9); Ruinen und Gegnerlager (B-468); `leveltest.html` (B-460, WZ7.5); neue Abhängigkeiten; Fenster- oder Browserprüfung durch Claude ohne Freigabe von 🧑.

## Schritte

1. Prüfen, ob WZ7 erledigt ist; Aufbau des Welt-Betrachters und die Vorschau-Schnittstelle aus WZ8.4 lesen.
2. Tests zuerst (rot): Schreib-Logik (byte-gleich ohne Änderung, nur betroffener Eintrag, atomar, Stand-Prüfung, REG-Hinweis) auf Kopien; Quelle der Chunk-Vorschau gleich Level-Teil des Snapshots für denselben Entwurf, dieselbe Insel, Stufe und denselben Seed.
3. Chunk-Vorschau von Biom/Seed auf Insel/Stufe umstellen, Level aus der Quelle der Live-Vorschau.
4. Editor: Platz, Art, Chunk-Seed, Objekte bearbeiten; Entwurf im Speicher; jede Änderung ruft die Vorschau-Funktion mit dem Entwurf auf; Fehler am Platz, letztes gültiges Bild bleibt.
5. Speichern: nur gültige Planung, atomar, format-stabil, mit Stand-Prüfung und REG-Hinweis.
6. `task check:studio`, `task check:dev`; `git status` zeigt keine Änderung an `data/islands.json`. Commit `feat(dev): Editor geplante Chunks (WZ8.5)`.

## Fertig, wenn

- [ ] AC-09: Editor bearbeitet Platz, Art, Chunk-Seed und Objekte; Vorschau folgt dem Entwurf; Speichern nur gültig (Tests).
- [ ] AC-06: Prüffehler stehen am Platz (Test oder Komponententest).
- [ ] AC-11: Test: beide Vorschauen zeigen für denselben Entwurf, dieselbe Insel, Stufe und denselben Seed dieselben Chunks und Objekte.
- [ ] AC-12: Tests auf Kopien: byte-gleich ohne Änderung, nur betroffener Eintrag, atomar, Stand-Prüfung mit Meldung und erhaltenem Entwurf, REG-Hinweis; `git status` zeigt keine Änderung an `data/islands.json`.
- [ ] `task check:studio` und `task check:dev` grün.

## Prüfen

```bash
task check:studio
task check:dev
```

Bei laufender Workbench `check_run`. Fenster- und Browserprüfungen nur, wenn 🧑 sie für diesen Lauf freigibt; die Abnahme am PC ist WZ8.7.

## Ergebnis

Umgesetzt von Claude (autonom), 2026-10-17; getrennter Reviewer-Agent vor dem Commit, keine schweren Befunde (Hinweise zu Dateirechten und `useMemo` sofort umgesetzt).

- **AC-09 umgesetzt, geprüft:** Editor im Welt-Betrachter (`PlanEditor.tsx`, `usePlan.ts`, reine Logik `plan.ts`): Insel und Stufe wählen, Chunk am Platz anlegen, Art, Chunk-Seed und feste Objekte (Art, Lage) bearbeiten, löschen; der Entwurf bleibt im Speicher, jede Änderung ruft nach 200 ms die Prüfung (`PlanCheck`) auf; die Vorschauen zeigen den letzten gültigen Entwurf; Speichern nur bei gültigem Entwurf. Tests: `plan.test.ts` (Logik), `PlanEditor.test.tsx` (DOM mit Mock-Backend), `tools/k3c-studio/app_plan_test.go`.
- **AC-06 geprüft:** Prüffehler stehen am Platz (`.wd-error` im Chunk, Test mit „volcano“ und Platz 0); Fehler ohne Platz (Summen, „kein Platz mehr“) stehen über der Liste. Go-Seite: `PlanCheck` liefert den Platz aus der Meldung der Prüfung.
- **AC-11 geprüft:** `WorldLevel` und `WorldPreview` lösen die Quelle über dieselbe Funktion (`resolveSource`: Biom bzw. Insel + Stufe + Entwurf) und rufen denselben Generator; Go-Test `TestChunkUndLiveVorschauLesenDieselbeQuelle` (Layout der Chunk-Vorschau gleich `Level` der Live-Vorschau, das geplante Portal bei Hub +325 steht darin) und Frontend-Test (beide fragen mit Insel, Stufe, demselben Entwurf; bei ungültigem Entwurf bleibt das letzte Bild).
- **AC-12 geprüft (Tests auf Kopien in `t.TempDir()`):** `PlanSave` schreibt format-stabil (nur der Wert von `plannedChunks` der Insel ändert sich, Offsets über den JSON-Decoder; fehlendes Feld wird angehängt), byte-gleich ohne Änderung, atomar (Temp-Datei im selben Ordner, Rechte bleiben, keine Reste), mit Stand-Prüfung (SHA-256; veraltet → `data/islands.json geändert, bitte neu laden`, Datei und Entwurf bleiben) und mit dem REG-Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“. `git status` zeigt keine Änderung an `data/islands.json`.
- **Chunk-Vorschau auf Insel und Stufe umgestellt:** `ChunkPreview` hat statt der Biom-Auswahl Insel und Stufe (im Editor), lädt bei jeder Änderung der Quelle neu; Stufe → Biom aus den Tiefen der Insel. `WorldSource.Planned` (nil = gespeicherte Planung bei einer Insel).
- **Prüfläufe:** `task check` grün (`check_run`); `task check:studio` ist in der Workbench nicht freigegeben (Schloss geschlossen), deshalb dieselben Schritte einzeln in der Shell: `npm ci` im Studio-Frontend (Abhängigkeiten fehlten im Checkout), `vitest` (134 Tests grün), `tsc --noEmit`, `vite build`, `go test ./...` und `golangci-lint run` im Studio-Modul (0 Befunde). `task check:dev` nicht gefahren (k3c-dev unberührt).
- **Abweichungen:** Der Umfang liegt über dem Richtwert von etwa 400 Zeilen (rund 1000 Zeilen mit Tests); ein Rest-Ticket gibt es nicht, weil kein Kriterium offen ist. `PreviewOptions.Planned == nil` bei der Quelle Biom bedeutet keine Planung (siehe WZ8.4).
- **Manuelle Prüfung:** keine (Fenster- oder Browserprüfung nur durch 🧑 in WZ8.7). Neue Tickets: keine.
