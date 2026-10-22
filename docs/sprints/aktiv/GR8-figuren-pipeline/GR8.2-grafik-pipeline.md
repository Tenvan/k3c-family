# GR8.2 · Grafik-Pipeline mit Ziel-Palette und gleicher Pixeldichte

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** gr8/2-grafik-pipeline
- **Abhängig von:** GR8.1
- **Tickets:** B-331
- **Kriterien:** AC-02

## Ziel

Ein Befehl `task gen:pipeline` rechnet zugeordnete Grafiken deterministisch auf DB32 und 16-px-Pixeldichte um; Stil-Bibel und Regeln stehen in `docs/assets/`.

## Kontext

- Spec: B-331 (Ausgangslage, Anforderungen, Fehlerfälle). Palette DB32 und Dichte 16 px sind beschlossen (ð§ 2026-10-06).
- `tools/buildings/db32.go` enthält DB32 bereits; `tools/atlas` und `tools/buildings` sind Vorbilder für Go-CLIs mit `main.go`, Tests und Taskfile-Eintrag in `tools/Taskfile.yml` (Namespace `gen`).
- Entscheidungen (autonom, Claude, 2026-10-22): Ergebnis liegt getrennt von der Quelle (z. B. `public/grafik-db32/<pack>/…`, Spiegel der Quellstruktur); das Spiel lädt weiter die Quellen (Umschalten = Entscheidung 🧑, B-331/AC-04). Auswahl der Assets: Liste in `docs/assets/pipeline-liste.json` (Pfad → optionale Ausnahme); die Liste startet mit den acht Prototyp-Assets aus B-331 plus allen PNGs unter `public/grafik/gothicvania-town` und `public/grafik/opp2017-village-and-room`.
- Verfahren: größten gemeinsamen Teiler der Lauflängen erkennen (Hochskalierung), auf 16-px-Dichte verkleinern (je Block häufigste Farbe; 32-px-Packs halbieren), nearest in OKLab auf DB32, kein Dithering, Alpha hart (≥128 → 255). Byte-gleich: PNG deterministisch kodieren, Reihenfolge sortiert.
- Fehler: kaputte/fehlende Quelle → Abbruch mit Dateinamen, nichts schreiben (erst im Speicher, dann atomar schreiben).
- Credits: neuer Ordner braucht Eintrag, damit `src/tools/credits.test.ts` und `grafikPacks.test.ts` grün bleiben (prüfen und ggf. Ausnahme/Eintrag anlegen).

## Erlaubte Dateien

- `tools/pipeline/` (neu: Go, Tests), `tools/Taskfile.yml`
- `public/grafik-db32/` (Ergebnis), `docs/assets/stil-bibel.md`, `docs/assets/pipeline-liste.json`
- `src/tools/` nur für Credits-/Pack-Test-Anpassung und die Vorher/Nachher-Seite, wenn schlank machbar (`src/tools/devTiles.ts`, neue `*.html` mit `installPageChrome()`)

## Nicht-Ziele

Umschalten des Spiels auf die Ergebnisse, Bild-KI-Anbindung, Color-Grading-Shader, Gegner-Figuren (GR8.1).

## Schritte

1. Stil-Bibel schreiben (DB32-Hex, 16 px, Outline: 1 px dunkle DB32-Kontur sofern Quelle sie hat, Lichtrichtung oben links; Ausnahmeliste).
2. Go-CLI mit Paketen: Dichte-Erkennung, Verkleinern, OKLab-Mapping, Alpha, deterministischer Schreiber; Tests mit kleinen synthetischen PNGs (Palette-Treue, Byte-Gleichheit zweier Läufe, Fehlerfall ohne halbe Ergebnisse).
3. `task gen:pipeline`, Lauf über die Liste, Ergebnis einchecken.
4. Credits-Tests grün halten; Vorher/Nachher-Seite, falls schlank, sonst Ticket.
5. `task check:diff`; Ergebnis, Status `fertig`.

## Fertig, wenn

- [ ] AC-02: `task gen:pipeline` läuft; Go-Test prüft jedes Ergebnis-Pixel in DB32 und byte-gleiche Doppelläufe; `docs/assets/stil-bibel.md` nennt Palette, Dichte, Outline, Licht.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
