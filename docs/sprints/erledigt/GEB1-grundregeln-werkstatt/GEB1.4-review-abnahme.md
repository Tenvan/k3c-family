# GEB1.4 · Review und Abnahme am Bildschirm

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** geb1/4-review
- **Abhängig von:** GEB1.2, GEB1.3
- **Tickets:** B-435
- **Kriterien:** AC-04

## Ziel

Diff von GEB1 ist geprüft, 🧑 hat Werkstatt und Bäume am Bildschirm abgenommen und die Spiel-Dichte (32 px ×1 oder 16 px ×2) festgelegt; der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff. Der Diff berührt `tools/gebaeude/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, `Taskfile.yml`, `docs/standards/gebaeude-grafik.md`. Besonders prüfen: kein `math/rand`/`Math.random()`; zweiter `task gebaeude`-Lauf ändert nichts; Test 64→32 (AC-06) prüft wirklich Pixelgleichheit; erlaubte Dichten nur 16/32/64/128; `data/assets.json` und `src/scenes/` unberührt; Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-04, AC-05, AC-06 Vorschau):** Diese Session nennt die Browser-Prüfung ausdrücklich; sie läuft nur, wenn 🧑 sie für diesen Lauf freigibt. Vite über `svc_start`, Seite `assetvorschau.html?szene=gebaeude` (Adresse über `get_urls`), Screenshot der Streifen Werkstatt und Bäume in 32 px ×1 und 16 px ×2. 🧑 entscheidet: abgenommen je Dichte, und welche Dichte das Spiel zeigt (Workshop GEB1.1, Frage 3).
- Die Spiel-Dichte wird in `docs/standards/gebaeude-grafik.md` eingetragen. Die Zuordnung ins Spiel (`data/assets.json`, Slot `workshop`, `node:tree`) ist ein neues Ticket für einen Folgesprint im Projekt GEB.
- Der Skalierungsfaktor kommt aus GR7.4 (B-251). Ist GR7.4 offen, führt die Abnahme den Faktor in AC-01 als `angenommen, Validierung offen (GR7.4)`.

## Erlaubte Dateien

- `tools/gebaeude/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts` nur für schwere Befunde
- `docs/standards/gebaeude-grafik.md` (Spiel-Dichte nach Entscheidung 🧑)
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit über schwere Befunde hinaus, Zuordnung ins Spiel, TV-Test (GR7.4), weitere Gebäude.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Mit Freigabe von 🧑: Vorschau öffnen, Screenshot vorlegen; Abnahme je Dichte und Entscheidung zur Spiel-Dichte im Ergebnis festhalten, Spiel-Dichte in die Regeldatei eintragen.
4. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen GEB1.1 bis GEB1.3 und Schritt 3 prüfen.
5. Ticket für die Zuordnung ins Spiel anlegen (Projekt GEB); B-435 bleibt `eingeplant`, solange das Projekt weitere Sprints hat.
6. Abnahme (höchstens fünf Zeilen) in die Sprint-README; Sprint nach `erledigt`, wenn AC-04 abgenommen ist, sonst `aktiv` lassen; `git merge origin/develop`, pushen, den einen PR des Sprints öffnen und `gh pr merge --auto --squash` einschalten.

## Fertig, wenn

- [ ] AC-01 bis AC-03, AC-05, AC-06 haben einen Nachweis im Ergebnis von GEB1.1 bis GEB1.3.
- [ ] AC-04: Abnahme durch 🧑 je verwendeter Dichte und Spiel-Dichte im Ergebnis und in der Regeldatei.
- [ ] Schwere Befunde behoben oder als Ticket angelegt; `task check` grün; PR geöffnet.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung: Asset-Vorschau im Browser (Schritt 3), nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

2026-10-16, Claude (Review per Sonnet-Agent in eigenem Kontext), Sichtprüfung von Ralf.
- Checks: `task check` und `task check:go` grün. Diff-Review `origin/develop...origin/sprint/geb1`: keine schweren Befunde.
- AC-01 geprüft (Regeldatei vollständig; Skalierungsfaktor `angenommen, Validierung offen (GR7.4)`), AC-02, AC-03, AC-05, AC-06 geprüft laut GEB1.2/GEB1.3.
- AC-04 abgenommen: Ralf hat Werkstatt und Bäume in der Asset-Vorschau (Browser-Pane, `127.0.0.1:5173/assetvorschau.html?szene=gebaeude`) in 32 px ×1 und 16 px ×2 abgenommen; Spiel-Dichte **32 px ×1**, in der Regeldatei eingetragen.
- Neues Ticket B-438 (Zuordnung ins Spiel). Abweichung: Review-Session lief im selben Gespräch wie die Umsetzung; das Diff-Review kam deshalb von einem frischen Agenten.
- Hinweis: `localhost:5173` lieferte im Pane 404 (vermutet IPv6 auf einem anderen Dienst), `127.0.0.1` ging.
