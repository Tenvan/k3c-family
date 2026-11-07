# PV1.1 · Domäne PAP, Feld Lane für parallele Sessions, Glossar, resvg-Abhängigkeit

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pv1/1-domaene-pap-und-lanes
- **Abhängig von:** –
- **Tickets:** B-699
- **Kriterien:** AC-01

## Ziel

Auf `develop` gibt es die Domäne PAP, die optionalen Session-Felder `Lane` (Sperre je Lane) und `Modell` (welches Claude-Modell die Session ausführt), die neuen Begriffe im Glossar und `@resvg/resvg-js` sowie `ajv` als Abhängigkeiten; alle Sessions des Projekts PAP tragen ihr Modell.

## Kontext

- Entscheidung `docs/decisions/005-papierschnitt-assets-mcp.md` › Umsetzung: Lanes arbeiten parallel in getrennten Ordnern. Heute sperrt eine Session `in Arbeit` ihre ganze Domäne (`docs/arbeitsweise.md` › Domänen, › Autonomer Ablauf Schritt 1).
- Planungstests: `tests/planningDocs.ts` (`DOMAINS` Z. 25, `ALLOWED`, `checkTemplate` verlangt **exakt** die Kopf-Felder der Vorlage), `tests/planningProjects.test.ts:83` („höchstens eine Session je Domäne in Arbeit“, `busyDomains`), `tests/planning.test.ts` (Glossar-Dubletten).
- Im Glossar ist **Material** schon das Baumaterial der Insel (`docs/glossar.md`); der Papier-Begriff braucht einen eigenen Namen.
- Laufzeit des Renderers ist **bun** (das Projekt nutzt bun, bun führt TypeScript direkt aus); `tools/paper/` muss typgeprüft werden.

## Erlaubte Dateien

- `docs/arbeitsweise.md`
- `docs/vorlagen/session.md`, `docs/vorlagen/ticket.md`, `docs/vorlagen/sprint.md` (nur Domänen-Zeile)
- `tests/planning.test.ts`, `tests/planningDocs.ts`, `tests/planningProjects.test.ts`
- `docs/glossar.md`
- `package.json`, `bun.lock`, `tsconfig.json`
- `.gitignore` (Zeile `.cache/paper/`)
- `CLAUDE.md` (nur Abschnitt Struktur: eine Zeile zu `tools/paper/`)
- `docs/sprints/geplant/PV1-*/` (Felder `Lane`, `Modell`, `Domäne` der PV1-Sessions)

## Nicht-Ziele

Keine Änderung an `plan_next` (PV1.2), keine Dateien unter `tools/paper/` (PV1.3).

## Schritte

1. Tests zuerst (rot laufen lassen):
   - `tests/planningDocs.ts`: `DOMAINS` um `PAP`; `ALLOWED.session` um `Modell: ['–', 'haiku', 'sonnet', 'opus', 'fable']`; `checkTemplate` erlaubt das Fehlen der **optionalen** Felder `Lane` und `Modell` (alle anderen Felder weiter exakt wie die Vorlage), `Lane` muss `–` oder `^[a-z0-9-]+$` sein.
   - `tests/planningProjects.test.ts`: „höchstens eine Session je Domäne in Arbeit“ wird „höchstens eine Session je Domäne und Lane in Arbeit; eine Session ohne Lane schließt jede andere derselben Domäne aus“; `busyDomains` entsprechend mit Testfällen.
2. `docs/vorlagen/session.md`: nach `Domäne` die Zeilen `- **Lane:** – (oder Kurzname wie `mcp`; Sessions derselben Domäne mit verschiedener Lane laufen parallel)` und `- **Modell:** – | haiku | sonnet | opus | fable (Empfehlung für den ausführenden Agenten)`. In `session.md`, `ticket.md`, `sprint.md` PAP in der Domänen-Zeile.
3. `docs/arbeitsweise.md` › Domänen: Zeile **PAP** · Papierschnitt-Grafik · `tools/paper/`, `public/grafik/stil.paper.json`, `public/grafik/**/*.paper.json` samt den daneben gerenderten Ausgaben, `data/asset-prompts.json`, `docs/standards/paper-*.md`. Sperr-Regel: Session mit `Lane` sperrt nur Domäne **und** Lane; Session ohne Lane sperrt die ganze Domäne und wird von jeder Lane derselben Domäne gesperrt; verschiedene Lanes brauchen disjunkte **Erlaubte Dateien**. Sessions **eines** Sprints laufen nacheinander (ein Sprint-Branch). Autonomer Ablauf Schritt 1 anpassen. Neuer Absatz › Modell-Wahl: `haiku` für reine Nachschlage- und Doku-Sessions, `sonnet` Standard für Umsetzung und Migration, `opus` für Verträge, Rig-Mathematik, Bild-Diff und Sprint-Reviews, `fable` nur auf Anweisung 🧑; ohne Feld gilt `sonnet`; höchstens vier Agenten gleichzeitig.
4. `docs/glossar.md`: Papierschnitt, Paper-Spec, Stil-Datei, Rig-Typ, Teil, Lage, **Papier-Material** (Abgrenzung: nicht das Baumaterial „Material“), Ton (licht, basis, schatten), Lane.
5. `bun add -d --exact @resvg/resvg-js@2.6.2 ajv@8.17.1` (Freigabe der neuen Abhängigkeiten: PV1 › Offene Fragen); `.gitignore`: `.cache/paper/`; `tsconfig.json`: `tools/paper` in `include`.
6. `CLAUDE.md` › Struktur: `tools/paper/` – Papierschnitt-Renderer und Verträge (Entscheidung 005), Aufruf `bun tools/paper/bin/paper.ts`.
7. PV1-Sessions bekommen `Modell` laut Tabelle in `docs/projekte/PAP-papierschnitt.md` › Notizen und ihre endgültige Domäne (PV1.3, PV1.4, PV1.6 → PAP).
8. Tests grün, Commit `docs(inf): Domäne PAP, Lanes und Modell-Feld für parallele Sessions`.

## Fertig, wenn

- [ ] AC-01: `task test -- planning` grün mit den neuen Tests (inkl. der rund 1000 bestehenden Sessions ohne `Lane`/`Modell`); Domänentabelle, Sperr-Regel und Modell-Wahl stehen in `docs/arbeitsweise.md`.
- [ ] `bun -e "require('@resvg/resvg-js')"` läuft; `task install` und `task check` grün.
- [ ] Glossar enthält die neun Begriffe ohne Dublette.

## Prüfen

```bash
task check:diff
```

## Ergebnis

- AC-01 geprüft: `task test -- planning` grün (946 Tests, neu: optionale Felder `Lane`/`Modell` in `planning.test.ts`, Sperre je Domäne und Lane in `planningProjects.test.ts`); Domäne PAP, Sperr-Regel und Modell-Wahl in `docs/arbeitsweise.md` › Domänen und › Autonomer Ablauf Schritt 1.
- `bun -e "require('@resvg/resvg-js')"` läuft; Lockfile enthält die Linux-Binaries; `task check:diff` grün. Glossar: neun Begriffe (Lage, Lane, Papier-Material, Paper-Spec, Papierschnitt, Rig-Typ, Stil-Datei, Teil, Ton) ohne Dublette.
- Abweichung: Felder `Modell`/Domäne der PV1-Sessions von Hand gesetzt, weil die laufende Workbench das Feld erst nach PV1.2 kennt (plan_set lehnt ab: „Feld Modell fehlt in der Datei“). Sprint-Domäne jetzt `INF, DEV, PAP`. `tsconfig.json` nimmt `tools/paper` auf; Node-Typen dort klärt PV1.3.
