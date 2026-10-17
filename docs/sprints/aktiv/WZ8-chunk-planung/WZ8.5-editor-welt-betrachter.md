# WZ8.5 · Editor für geplante Chunks im Welt-Betrachter

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
