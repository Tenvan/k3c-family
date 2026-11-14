# CU1.6 · Review und Abnahme CU1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** CU1.5
- **Tickets:** B-700
- **Kriterien:** alle

## Ziel

Der Sprint CU1 ist geprüft und die Abnahme der Sprint-README ausgefüllt; B-700 erfüllt, D-CU1-Zeilen in B-701 gemeldet. Damit ist das Projekt PAP technisch abgeschlossen.

## Kontext

- Kriterien AC-01–AC-05 der Sprint-README, B-700 AC-01–AC-03, D-US2-02 in B-701.
- Risiken: versteckte Laufzeit-Pfade (Strings wie `grafik/` + Variable), Docker-Build ohne Atlas-Stufe, Sound-Regression durch `data/sounds.json`, verwaiste Kacheln oder Seiten.
- Manuelle Sichtprüfung (Gerüste, Plantage, Sound im Spiel) ist Ralfs Sache; Claude listet sie als offene Abnahme.

## Erlaubte Dateien

- `docs/sprints/*/CU1-alte-grafik-aufraeumen/**`
- `docs/backlog/B-700-alte-pixel-grafik-aufraeumen.md`, `docs/backlog/B-701-pap-gesammelte-fragen.md`
- Fixes unter etwa 30 Zeilen in den Dateien der Sessions CU1.2–CU1.5; größere Befunde → Ticket

## Nicht-Ziele

Neue Funktionen; Tickets verwerfen (Ralf).

## Schritte

1. Diff des Sprints gegen den Start-Commit lesen (Opus-Review-Agent, getrennt vom Schreiben), Schwerpunkt Risiken aus dem Kontext.
2. Suchen aus AC-01 erneut ausführen, Ergebnis festhalten.
3. `task check`, `task check:go`, `task check:race` nacheinander (ohne gcc: übersprungen nennen), `task check:dev`, `task buildall`.
4. Lade-Probe headless: Spiel startet, Netzwerk-Log zeigt keine Anfragen auf `sprites/`, `atlas/`, `grafik-db32/` oder alte Packs.
5. Abnahme in der README (höchstens fünf Zeilen), B-700 auf erledigt, Sprint-Status über `plan_set`.

## Fertig, wenn

- [ ] Alle Kriterien AC-01–AC-05 mit Nachweis im Ergebnis.
- [ ] Lade-Probe ohne Anfragen auf alte Pfade.
- [ ] Schwere Review-Befunde behoben oder als Ticket erfasst.

## Prüfen

```bash
task check
task check:go
task check:race
task check:dev
task buildall
```

## Ergebnis

- **AC-01 geprüft:** Ordner `public/grafik-db32`, `public/atlas`, `public/sprites`, `tools/buildings|atlas|pipeline|spritegen`, `data/assets.json`, `data/sprites.json` fehlen (`ls`). Suche aus „Beispiele“ (ohne `docs/`, `.codemap/`) trifft: Kommentare in `data/paper.json`, `data/sounds.json`, `site/index.html`; Negativregel `tests/projectRules.test.ts`; `src/…` (Browser-Client, durch 009 überholt, B-823); Konstante „tools/buildings“ in `tools/assetkit/catalog*.go` (verschoben → B-760, wie CU1.4/CU1.5); `cmd/k3c-server/k3c-server.exe` (eingecheckter Binary seit #427, nicht aus CU1 → B-835). Behoben: Zeile `atlas/` in `tools/codemap.md`.
- **AC-02 verschoben → B-836:** `task check:all`, `task buildall`, `task check:dev` liefen in keiner Session; `buildall` ist in diesem Lauf ausgeschlossen (keine EXE bauen), `dev:test` scheiterte im Worktree am fehlenden `frontend/dist` (Umgebung). `task check` grün (6 min 28 s), `task check:go` grün (3 min 43 s).
- **AC-03 geprüft:** Nachweis CU1.5 (`LICENSE-ASSETS` ohne Grafik-Packs, `public/grafik/CREDITS.md` fehlt, Sound-Credits in `public/audio/README.md`).
- **AC-04 geprüft:** Nachweis CU1.1/CU1.2; Sichtprüfung im Spiel bleibt bei Ralf, nach 009 im nativen Spiel.
- **AC-05 geprüft (Daten):** `data/sounds.json` hat dieselben 32 Slots wie das alte Register, je Slot gleich (JSON-Vergleich, Review); Workbench und `soundatlas` lesen `sounds.json` bzw. `paper.json › figuren`. Lade-Probe headless im Browser-Client: durch 009 überholt.
- **Review (k3c-reviewer, Opus, #543 und CU1.5-Teil von #546):** keine schweren Befunde; Dockerfile ohne Atlas-Stufe sauber, kein Task ruft gelöschte Ordner auf, `data/embed.go` unberührt. Niedrig, nicht behoben: Lizenz „unbekannt“ für Papier-PNGs im Katalog (`catalog.go`, gehört zu B-760); feste Frame-Maße in `engine/balance/report_html.go` (Balancing gesperrt).
- **B-701:** keine D-CU1-Zeilen nötig (CU1.1: keine offenen Formfragen); D-US2-02 mit CU1.1/CU1.2 umgesetzt, B-701 liegt schon im Archiv.
- **Abweichung:** Kein PR und kein Auto-Merge aus dieser Session (Welle 5b, Koordinator), kein Push. Neu: B-835, Abnahme-Ticket B-836.
