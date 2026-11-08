# B-748 · gen:paper rendert, wenn Grafiken fehlen, obwohl Specs unverändert sind

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task dev` (Dienst Vite der Workbench) hängt von `gen:paper` ab. Der Task nutzt `sources`/`generates`; `golden.json` und `index.paper.json` sind eingecheckt und erfüllen `generates`, auch wenn PNGs und Manifeste fehlen. Fehlen die Asset-Ordner bei unveränderten Specs (frischer Checkout, gelöschter Ordner), gilt der Task als aktuell und rendert nicht (`task gen:paper --status` Exit 0).

## Ziel

Fehlen Grafiken in `public/grafik`, kopiert der Start des Clients sie aus `.work/grafik`, ohne neu zu rendern; fehlt auch `.work`, wird gerendert.

## Beteiligte und Zielgruppen

Entwickler (🧑), Workbench-Dienst Vite.

## Anforderungen

- Neuer Teil-Task `gen:paper-sync` (kopiert nur) mit `status`: fehlt zu einer Spec das Manifest in `public/grafik`, läuft er.
- `gen:paper-render` läuft zusätzlich, wenn in `.work/grafik` Manifeste fehlen.
- `gen:paper` ruft beide nacheinander; Aufrufer (`dev`, `test`, `build`, `check:diff`) bleiben unverändert.
- Sonst unverändertes Verhalten (kein Neurendern bei aktuellem Stand).

## Nicht-Ziele

Kein schnelleres Rendern, keine Änderung an gen.ts.

## Regeln und Einschränkungen

Tasks nur über `task`; Eintrag `status:` in `tools/Taskfile.yml`.

## Beispiele

Ordner `public/grafik/figuren/monarch/` gelöscht, `.work` vollständig → `task dev` kopiert in ca. 1 s, kein Rendern.

## Ausnahme- und Fehlerfälle

nicht relevant: Fehlerfälle des Renderers bleiben wie bisher.

## Akzeptanzkriterien

- **AC-01** Fehlt in `public/grafik` ein Manifest, meldet `task gen:paper-sync --status` Exit 1; danach (nach dem Kopieren) Exit 0.
- **AC-02** `bun tools/paper/bin/gen.ts --nur-kopieren` kopiert aus `.work/grafik`, ohne zu rendern (Test in `tools/paper/cli/gen.test.ts`).
- **AC-03** `bun tools/paper/bin/gen.ts --fehlt public|work` liefert Exit 1 mit den fehlenden Assets, sonst 0.

## Offene Fragen

keine

## Notizen

Gefunden beim Start des Clients in der Workbench.
