# WZ6.6 · assetvorschau.html samt Skript, Kachel und Verweisen entfernen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz6/6-assetvorschau-entfernen
- **Abhängig von:** WZ6.3, WZ6.4
- **Tickets:** B-459
- **Kriterien:** AC-06

## Ziel

`assetvorschau.html` ist mit Skript, Logik, Test, Kachel, Texten und Verweisen vollständig entfernt; ihr Inhalt liegt auf der Grafik- und der Sound-Seite des Asset-Studios (WZ6.3/WZ6.4). `grafiken.html` und `soundtest.html` bleiben unverändert.

## Kontext

Nach WZ6.3 (Grafik-Szenen, Gebäude-Stufenreihen, Hintergründe) und WZ6.4 (Ereignis-SFX, Musik mit Wechsel, Umgebungsgeräusche) liegt die Logik von `src/tools/assetvorschauLogic.ts` im Studio-Frontend (Sprint-README › Anforderungen). Damit hat `src/tools/assetvorschau*.ts` keinen Nutzer mehr und fällt **vollständig** weg. Importiert `tools/k3c-studio/` noch etwas aus `src/tools/assetvorschau*`, ist der Umzug unvollständig. Dann setzt die Session `Status: blockiert` (Ablauf laut `docs/arbeitsweise.md`); die Logik bleibt nicht als Rest liegen.

**Domäne DEV:** `src/tools/` gehört zu DEV. `assetvorschau.html`, `vite.config.ts` und `tests/projectRules.test.ts` laufen als Grenzfall in DEV, so wie B-299 die Seite in DEV eingeführt hat (B-299 › Regeln und Einschränkungen).

**Fundstellen** (Stand 2026-10-17, Zeilen ungefähr):

- `assetvorschau.html`: Marker `<meta name="k3c-dev-only">`, Texte `av.*`, Script `/src/tools/assetvorschau.ts`
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `src/tools/devTiles.ts`: Kachel `tile('assetvorschau', …)` mit `devOnly: true` (Z. ~65); das Feld `devOnly?` (Z. ~18) ist danach tot
- `src/tools/dev.ts` (Z. ~50): Filter `import.meta.env.DEV || !t.devOnly`
- `src/tools/devTiles.test.ts`: `TOOLS`-Array (Z. 5) und Test „nur die Asset-Vorschau gibt es nur im Dev-Build (B-299)“
- `src/tools/texts.de.ts`, `src/tools/texts.en.ts`: je 22 Schlüssel `av.*` sowie `tile.assetvorschau.title` und `tile.assetvorschau.desc`
- `src/tools/codemap.md`: Satz zu `assetvorschau.ts`/`assetvorschauLogic.ts`
- `tests/projectRules.test.ts` (Z. 38–42): Test „Dev-Seiten (Marker k3c-dev-only) …“. Er vergleicht die Seiten mit dem Marker mit den `devOnly`-Kacheln und prüft den Filter in `vite.config.ts`.
- `vite.config.ts` (Z. ~51–54): Filter für Seiten mit Marker `k3c-dev-only`. Danach trägt keine Seite den Marker mehr; der Mechanismus fällt ganz weg (Filter, Projektregel, `devOnly`), damit kein toter Code bleibt.
- `docs/standards/gebaeude-grafik.md` § 9, Schritte 3–4 (Z. 187–188): Vorschau und Hintergründe auf `assetvorschau.html?szene=gebaeude`
- `data/assets.json`: nur die Zeile `_comment` („liest assetvorschau.html“)
- `docs/glossar.md`: Eintrag „Mini-Szene“ nennt `assetvorschau.html` als Ort; der Begriff bleibt (B-459/AC-02), der Ort wird die Grafik-Seite des Asset-Studios

**Bleiben als Historie:** Tickets (`docs/backlog/`, auch `archiv/`), erledigte Sprints, die Planungsdateien von WZ6 selbst sowie `docs/sprints/geplant/GEB9-*/README.md`, die den Umzug beschreibt.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `assetvorschau.html` (löschen)
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts` (löschen)
- `src/tools/devTiles.ts`, `src/tools/devTiles.test.ts`, `src/tools/dev.ts` (Kachel, `devOnly`, Tests)
- `src/tools/texts.de.ts`, `src/tools/texts.en.ts` (nur Schlüssel `av.*` und `tile.assetvorschau.*`)
- `src/tools/codemap.md`
- `tests/projectRules.test.ts` (nur der Dev-only-Test, Grenzfall wie B-299)
- `vite.config.ts` (nur der Filter für `k3c-dev-only`, Grenzfall wie B-299)
- `docs/standards/gebaeude-grafik.md` (nur § 9, Schritte 3–4)
- `data/assets.json` (nur die Zeile `_comment`)
- `docs/glossar.md` (nur der Eintrag „Mini-Szene“)

## Nicht-Ziele

`grafiken.html`, `soundtest.html` (bleiben für Prüfungen am TV), `figuren.html`, `aufstellung.html` und `leveltest.html` ändern oder entfernen; `tools/k3c-studio/` ändern (Umzug gehört zu WZ6.3/WZ6.4); Logik aus `src/tools/assetvorschau*` behalten oder an anderer Stelle in `src/` neu anlegen; Tickets oder erledigte Sprints umschreiben (Historie); Slots oder Werte in `data/assets.json` ändern.

## Schritte

1. Vorbedingung: `git grep -n assetvorschau -- tools/k3c-studio` ohne Treffer, und die Ergebnisse von WZ6.3/WZ6.4 belegen die Grafik- und Sound-Szenen im Studio. Sonst `Status: blockiert` mit Frage-Ticket.
2. `git rm assetvorschau.html src/tools/assetvorschau.ts src/tools/assetvorschauLogic.ts src/tools/assetvorschauLogic.test.ts`.
3. Kachel und `devOnly` entfernen: Kachel und Feld in `devTiles.ts`, Filter in `dev.ts`, `assetvorschau.html` aus `TOOLS` und den Dev-only-Test in `devTiles.test.ts`. Dazu die Schlüssel `av.*` und `tile.assetvorschau.*` in `texts.de.ts` und `texts.en.ts`.
4. Dev-only-Mechanismus entfernen: Test in `tests/projectRules.test.ts` (Z. 38–42) und Filter in `vite.config.ts`, samt nicht mehr gebrauchtem Import.
5. Verweise umstellen: `docs/standards/gebaeude-grafik.md` § 9 Schritte 3–4 auf die Grafik-Seite des Asset-Studios (Reiter und Streifen so benennen, wie WZ6.3 sie gebaut hat); `_comment` in `data/assets.json` („liest das Asset-Studio“); Glossar „Mini-Szene“; `src/tools/codemap.md`.
6. Prüfung laut „Fertig, wenn“; `task check` und `task build` grün.

## Fertig, wenn

- [ ] AC-06: `assetvorschau.html`, `src/tools/assetvorschau.ts`, `assetvorschauLogic.ts` und `assetvorschauLogic.test.ts` gibt es nicht mehr. Kachel, Texte `av.*`/`tile.assetvorschau.*`, `devOnly` und der Filter `k3c-dev-only` sind entfernt.
- [ ] AC-06: `git grep -n assetvorschau -- . ':!docs/sprints/erledigt' ':!docs/backlog' ':!docs/sprints/*/WZ6-*' ':!docs/sprints/geplant/GEB9-*'` liefert keinen Treffer. `docs/backlog` schließt `docs/backlog/archiv` ein, Tickets bleiben als Historie. `.claude/worktrees` erfasst `git grep` nicht. Läuft der Sprint, liegt WZ6 unter `aktiv/`, daher `*`.
- [ ] AC-06: `git diff --quiet origin/develop -- grafiken.html soundtest.html` (unverändert).
- [ ] `task check` grün (Projektregeln, Kacheln); `task build` grün, `dist/` enthält keine `assetvorschau.html`.

## Prüfen

```bash
task check
task build
```

Manuelle Prüfungen (Browser, Xbox, TV) nennt diese Session nicht.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
