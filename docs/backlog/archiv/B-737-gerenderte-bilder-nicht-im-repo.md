# B-737 · Gerenderte Papierschnitt-PNGs aus dem Repo nehmen, beim Build bzw. nach Änderung erzeugen

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-07, Chat ("das sollten wir jetzt umsetzen")

## Ausgangslage

`public/grafik/<gruppe>/<asset>/*.png` und `manifest.json` sind eingecheckt. Der Monarch allein belegt 6,9 MB, MG9.2 (Reittier-Gangarten, Reit-Idle, Zustände) fügt je Variante weitere ~30 Animationen hinzu (geschätzt +40 bis 50 MB). Die Specs (`*.paper.json`) sind die Quelle, die PNGs sind reproduzierbar (`task gen:paper`, Golden-Test).

## Ziel

Gerenderte PNGs und Manifeste liegen nicht mehr im Git-Verlauf, sondern werden beim Build, beim Start der Dev-Server und nach Spec-Änderungen erzeugt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Ablage (Ralf, 2026-11-07):** Der Renderer schreibt in einen ignorierten Arbeitsordner `.work/grafik/<gruppe>/<asset>/` (PNG, `manifest.json`, Cache nach renderKey). Von dort werden die Bilder in den ausgelieferten, ebenfalls ignorierten Pfad `public/grafik/<gruppe>/<asset>/` kopiert (nur geänderte Dateien). Specs, `index.paper.json`, `stil.paper.json` und `golden.json` bleiben eingecheckt.
- `.gitignore`: `.work/` und `public/grafik/*/*/` (die Asset-Ordner der Papierschnitt-Gruppen).
- `task build`, `task serve`, `task dev`, Workbench-Assets und CI rufen Rendern und Kopieren vor dem Bedarf auf (inkrementell über `sources`/`generates`, nur geänderte Assets).
- Golden-Test: `golden.json` bleibt eingecheckt (Hashes), der Test rendert neu (in den Temp-Ordner) und vergleicht; die Prüfung gegen eingecheckte PNGs entfällt.
- Go-Server (`public/` per Watch, `go:embed`), GitHub Pages (`site/`), Xbox-Hosting im Heimnetz: Pfad prüfen, dass nach dem Build alles da ist.
- Verlauf: Bilder aus dem Verlauf zu entfernen (History-Rewrite) ist nicht Teil des Tickets, nur die Entscheidung, ob sie bleiben.

## Nicht-Ziele

History-Rewrite; Änderungen an der Renderer-Logik.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Nach frischem Checkout erzeugt `task build` (bzw. `task gen:paper`) alle Bilder; ohne sie startet kein Dev-Server ohne Hinweis.
- **AC-02** Die Bilder sind in `.gitignore`, ein Commit nach `task gen:paper` enthält keine PNGs.
- **AC-03** `task check:diff` und CI sind grün, der Golden-Test vergleicht weiter die Hashes.
- **AC-04** Server und Client zeigen die Papierschnitt-Grafik nach `task serve` unverändert.

## Offene Fragen

keine (Entscheidungen von Claude als Default: `.work/` je Checkout, Rendern immer vollständig (rund 2 bis 5 Minuten, nicht inkrementell), übertragen wird nur Geändertes; `task` überspringt `gen:paper` bei unveränderten Quellen; Docker rendert im Build-Schritt).

## Notizen

Vorschlag und Ablage von Ralf (2026-11-07, Chat). Umgesetzt in `tools/paper/cli/work.ts` (`genMitWork`: spiegeln, rendern in `.work/grafik/`, übertragen, Veraltetes entfernen), `tools/paper/bin/gen.ts` (ohne Argument Arbeitsordner-Modus), `.gitignore` (`/.work/`, Asset-Ordner der neun Gruppen), `Taskfile.yml` (`build`, `test`, `dev`, `check:diff` rufen `gen:paper`), `Dockerfile`, `CLAUDE.md`. 416 Dateien aus dem Index genommen; der Git-Verlauf behält sie (kein History-Rewrite). Prüfung: `tools/paper/cli/work.test.ts`, volle Vitest-Suite grün (3319), `public.test.ts` Timeout auf 600 s (Render dauert 5 Minuten). Ungeprüft: Docker-Build (resvg-js auf alpine/musl) und CI. Folge: B-738 (Workbench-Go-Tests schon vorher rot).
