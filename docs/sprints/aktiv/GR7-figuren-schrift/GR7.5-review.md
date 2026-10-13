# GR7.5 · Review des Sprints GR7

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr7/5-review
- **Abhängig von:** GR7.1, GR7.2
- **Tickets:** B-193, B-251, B-198, B-197, B-320
- **Kriterien:** alle

## Ziel

Der Sprint GR7 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, der eine PR des Sprints ist geöffnet (Auto-Merge an); erledigte Tickets sind archiviert, der Sprint bleibt `aktiv`, solange GR7.3 oder GR7.4 (🧑) offen sind.

## Kontext

Leichter Review: nur schwere Befunde im Diff, günstiges Modell. Der Diff berührt `src/scenes/` (Schrift, Platzhalter), `data/sprites.json` (Sattelpunkte) und Doku. Besonders prüfen: `placeholder` steht in `FONTS`, `stageView.ts` hat keine lose Schriftgröße mehr, `fontRules.test.ts` grün; `bedienung.md` § 2 und `minFontPx` nennen dieselbe Regel für die Mitspieler-Zelle (Beschluss 🧑 2026-10-12); Sattelpunkt-Test (≤ 4 px) belegt AC-05 ohne umformulierte Grenze; kein `Math.random()`; keine Datei über 400, keine Funktion über 60 Zeilen. **GR7.3 und GR7.4 (Agent: Mensch)** sind keine Abhängigkeit: Sind sie offen, führt die Abnahme AC-01 und AC-02 als `angenommen, Validierung offen (GR7.3 bzw. GR7.4)`; B-193 und B-251 bleiben dann im Backlog (`eingeplant`), ebenso B-320 bis zur Sichtprüfung (AC-02 des Tickets) durch 🧑. Der Sprint bleibt `aktiv`, die offenen Sessions stehen im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `src/scenes/`, `data/sprites.json` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umgebungsgrafik, Server und Protokoll, Sichtprüfungen am Gerät.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr7` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-05 aus den Ergebnissen von GR7.1 bis GR7.4 prüfen.
4. Abnahme (höchstens fünf Zeilen, mit Versionsvorschlag) in die Sprint-README schreiben.
5. Erledigte Tickets (B-198, B-197, B-320 nur nach der Sichtprüfung) auf `erledigt` setzen und archivieren.
6. Sprint bleibt `aktiv`, solange GR7.3 oder GR7.4 offen sind (Fahrplan „Offen am Gerät“), sonst nach `docs/sprints/erledigt/`; `git merge origin/develop`, pushen, den einen PR des Sprints öffnen und `gh pr merge --auto --squash` einschalten.

## Fertig, wenn

- [ ] AC-03, AC-04, AC-05 haben einen Nachweis im Ergebnis von GR7.1 bzw. GR7.2.
- [ ] AC-01 und AC-02 sind abgenommen oder als `angenommen, Validierung offen (GR7.3 / GR7.4)` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; `task check` grün; PR geöffnet.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

**Fertig (2026-10-13).** Abweichung vom Plan: Der Sprint-PR [#279](https://github.com/tenvan/k3c-family/pull/279) war schon vor dem Review in `develop` gemergt (Squash `79992b24`); es gab keinen Branch `origin/sprint/gr7`. Geprüft wurde deshalb `git diff 79992b24^ 79992b24`, der Abnahme-Commit geht als kleiner Folge-PR auf `develop`.

- **Prüfläufe:** `task check:go` grün. `task typecheck` grün. `npx vitest run`: 2171 Tests grün, 6 Testdateien rot, **alle** unter `tools/cockpit/hooks/` (Paket `claude-code/testing` fehlt). `task lint`: 22 Fehler, **keine** in `src/scenes/`, `data/` oder `tools/k3c-dev/`, sondern in `tools/cockpit/` und fünf Werkzeug-Seiten unter `src/tools/`. Ursache aus PR #278, nicht aus GR7; im Folgecommit mit B-415 behoben (`tools/cockpit` aus Lint und Vitest ausgenommen, `task check` grün).
- **Diff-Befunde:** keine schweren. `cameraFilter` der Stufen-Texte wird nur auf Text-Objekten gesetzt, das Ausblenden je Kamera läuft weiter über die Layer (`showOnly`); `Math.random()` nirgends; `GameScene.ts` bleibt unter der Grenze (409 Zeilen mit Kommentaren, 400 ohne); `proc.Command` ersetzt `exec.Command("git")`, Test sichert das.
- **AC-03 geprüft:** Rolle `placeholder` in `FONTS`, `stageView.ts` ohne lose Größe, `fontRules.test.ts` grün (GR7.1).
- **AC-04 geprüft:** `bedienung.md` § 2 und `minFontPx` (`null` für `partner`) nennen dieselbe Regel (Beschluss 🧑 2026-10-12), Test grün (GR7.1).
- **AC-05 geprüft (Test):** `mountData.test.ts` mit Grenze 4 px, nur `run[2]` (Sammelphase, +2 px) begründet ausgenommen, wie B-320/AC-01 es zulässt. Sichtprüfung B-320/AC-02 bleibt bei 🧑.
- **AC-01 und AC-02:** angenommen, Validierung offen (GR7.3 bzw. GR7.4, Agent: Mensch).
- **Neue Tickets:** keine.
