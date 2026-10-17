# GEB6.6 · Review und Abnahme

- **Status:** blockiert
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb6/6-review-abnahme
- **Abhängig von:** GEB6.5
- **Tickets:** B-435, B-438
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB6 ist geprüft, 🧑 hat Natur und Objekte in der Vorschau je Biom und im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff; besonders ältere PNGs unverändert (`tanne-32.png`, `pappel-32.png`, `birke-32.png` und alle GEB1–GEB5-Gebäude), gelöschte Grafik nirgends mehr genutzt (Erz-Blatt und `icons/8.png` bleiben), Baumwahl deterministisch ohne `Math.random()`, Plantage-Pappel unverändert, Adern zeigen `vein:*`, Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** nur mit Freigabe von 🧑 für diesen Lauf. Vite und Spielserver über `svc_start`, Adressen über `get_urls`; Vorschau `assetvorschau.html?szene=gebaeude` (Bäume, Fels, Adern-Stufenreihe, Truhe, Portal, Ausgang, Lager), Spiel nur, wenn 🧑 auch das freigibt, sonst prüft 🧑 selbst (Wald, Höhle, Mine).
- **Basis:** GEB5 ist vorher auf `develop` gemergt; das Review liest `git diff origin/develop...origin/sprint/geb6`.

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, GEB7 (Hintergründe), Animationen, TV-Test.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb6` lesen, Befunde behandeln.
3. Mit Freigabe von 🧑: Vorschau (und ggf. Spiel) zeigen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-06; Abschnitt „Abnahme“ der Sprint-README (höchstens fünf Zeilen, Versionsvorschlag).
5. Fehlt die Abnahme von 🧑: `Status: blockiert` mit Grund im Ergebnis (Arbeitsweise › Wenn etwas nicht passt) und aufhören.
6. Sprint nach `erledigt/`, `Status: erledigt` (nur wenn alle Sessions fertig); PR `sprint/geb6` → `develop` öffnen, Auto-Merge einschalten (Arbeitsweise › Review-Session, Schritt 5).

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 steht im Ergebnis (Vorschau je Biom und Spiel).
- [ ] AC-01, AC-02, AC-04–AC-06: Nachweise aus GEB6.1–GEB6.5 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen, Auto-Merge eingeschaltet.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

- **AC-03 blockiert (🧑):** Abnahme der Vorschau je Biom und des Spiels steht aus; niemand außer Agenten hat Bilder gesehen, es lief keine Browser-Sichtprüfung. Nach Arbeitsweise › Wenn etwas nicht passt bleibt die Session `blockiert`; Abnahme-Ticket **B-451**, Liste unten. Der PR bekommt kein Auto-Merge.
- **Review 2026-10-17 (getrennter Reviewer-Agent, Diffs #319 und #323 auf `develop`):** keine schweren Befunde. Alle 28 `*-32.png` der Natur-Objekte und alle Einträge in `public/grafik/index.json` existieren; kein `Math.random` in `src/scenes`, Baumform nur aus Position und Biom (`treeTexture`, rein); keine Pfad- oder Shell-Eingaben; keine Seiten-, Vollbild- oder B-Tasten-Änderung; gelöschte Pack-Grafik nirgends mehr referenziert (Treffer nur in Doku); alte Gebäude-PNGs unverändert (nur `index.json` geändert), Plantage-Pappel unverändert. Hinweis, nicht schwer: `vein:*` in `data/assets.json` zeigt auf dieselben `ader-N-32.png`, der Renderer wählt aber über `veinSlot` und feste Texturschlüssel; beide Stellen müssen synchron bleiben.
- **Prüfläufe 2026-10-17:** `task check` grün (19 s, k3c-dev `check_run` im Worktree auf `develop` e29dcc8c); `task check:go` grün (3 min 44 s).
- **Nachweis je Kriterium:** AC-01 geprüft (JSON und PNG der Natur-Objekte vorhanden, Ergebnisse GEB6.1–GEB6.4, Reviewer-Check der Dateien); AC-02 geprüft (Slots in `data/assets.json` und `worldSprites.ts` auf neue PNGs, GEB6.5, Review); AC-03 blockiert (🧑); AC-04 geprüft (Maße und Byte-Gleichheit laut GEB6.1–GEB6.4, alte PNGs unverändert); AC-05 geprüft (Auswahl-Funktionen mit Vitest in `worldSprites.test.ts`, kein `Math.random`, Plantage bleibt Pappel); AC-06 geprüft (Suche nach gelöschten Namen ohne Treffer außerhalb Doku, `task check` und `task check:go` grün).
- **Doku-Stand:** Die Doku-Commits von `origin/sprint/geb6` (Abnahmeliste, Offene Frage 4 bestätigt) stehen über den Squash #323 schon auf `develop`; nichts nachzuziehen.

**Abnahmeliste AC-03 (🧑)** — Vorschau `assetvorschau.html?szene=gebaeude`:

- [ ] 1. Baum-Reihe Wald: alle neun Bäume (Tanne, Pappel, Birke je 3 Formen) unterscheidbar, überlappend, ohne Ausreißer.
- [ ] 2. Baum-Reihe „Look Höhle“: sechs Bäume (Formen 2 und 3), klar kühler und dunkler als Wald.
- [ ] 3. Busch, Fels in drei Looks (Wald, Höhle, Mine), Erzbrocken: Looks auf einen Blick unterscheidbar, Erzbrocken als Erz erkennbar.
- [ ] 4. Adern-Stufenreihe: Stein, Kupfer, Eisen, Kristall unterscheidbar, passend zu Mauer-Stufen 2–5.
- [ ] 5. Truhe (als Beute erkennbar), Portal und Ausgang (klar verschieden), Lager (Zelt und Feuerstelle lesbar).
- [ ] 6. Dichte 16 px ×2 neben 32 px ×1: nichts Wichtiges geht verloren.

Spiel (je Biom ein Durchgang, Maßstab 1):

- [ ] 7. Wald: Baumformen wechseln mit der Position; Busch, Fels, Truhe, Lager, Portal, Ausgang sichtbar; Plantage-Bäume bleiben Pappel.
- [ ] 8. Höhle: Bäume nur im Look Höhle, Fels bläulich, Stein-Ader.
- [ ] 9. Mine: Fels dunkel, Erzbrocken, Kupfer-Ader. Eisenstollen: Eisen-Ader, Fels wie Mine. Kristallhöhle: Kristall-Ader.
- [ ] 10. Fahne und Fortschrittsbalken sitzen über Bäumen, Fels und Adern; die Stauchung beim Abbau wirkt normal.
- [ ] 11. Ausgang-Beschriftung („Tiefer“/„Gesperrt“) liegt lesbar über dem Ausgang.
- [ ] 12. Zwei Spieler im Split-Screen: dieselben Objekte, nichts flackert oder fehlt.

Entscheidungen:

- [x] 13. Materialien (Offene Frage 4) von 🧑 bestätigt, Option 1, 2026-10-16: Fels = Stein, Zelt = „Stoff hell“, Erde = „Pfosten, Balken“, Gold = Glut.
- [ ] 14. Geschmack: Mine-Fels eher dunkelgrau als braun, Feuerstelle klein, Ausgang ohne Platz oben, Kristall-Ader ohne durchgehende Fuge; Animationen von Portal und Feuer entfallen (Nicht-Ziel).
- [ ] 15. Ergebnis: „abgenommen“ oder Nachbesserungen nennen. Danach setzt Claude GEB6.6 auf `fertig`, verschiebt den Sprint nach `erledigt/`, setzt B-451 auf `erledigt` und lässt die Version (v0.19.0 vorgeschlagen) von 🧑 bestätigen.
