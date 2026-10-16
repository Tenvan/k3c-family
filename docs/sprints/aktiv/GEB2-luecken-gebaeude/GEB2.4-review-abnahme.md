# GEB2.4 · Review und Abnahme am Bildschirm

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb2/4-review
- **Abhängig von:** GEB2.3
- **Tickets:** B-435
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB2 ist geprüft, 🧑 hat die vier Gebäude in der Vorschau und gebaut im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff (`tools/gebaeude/`, `public/grafik/gebaeude/`, `docs/standards/gebaeude-grafik.md`, `src/scenes/buildingSprites*.ts`, `src/scenes/siteView.ts`, `data/assets.json`). Besonders: zweiter `task gebaeude`-Lauf ändert nichts; nur DB32; andere Gebäude weiter Maßstab 2; kein `Math.random()`; Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** Diese Session nennt die Browser-Prüfung ausdrücklich; sie läuft nur, wenn 🧑 sie für diesen Lauf freigibt. Vite und Spielserver über `svc_start`, Adressen über `get_urls`. (1) `assetvorschau.html?szene=gebaeude`: Screenshot des Streifens mit allen fünf Gebäuden in 32 px ×1 und 16 px ×2. (2) Spiel mit gebauten Gebäuden: über das Debug-Menü (Hub-Stufe hoch, Gold/Material) Taverne, Heilplatz, Schmiede und Rüstkammer bauen, Screenshot neben einer Pack-Grafik (Kaserne o. ä.).
- Nachbesserungen am Stil nur, wenn 🧑 sie in diesem Lauf verlangt; sonst als Ticket im Projekt GEB.

## Erlaubte Dateien

- Alle Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, Bäume ins Spiel (B-438), TV-Test (GR7.4), weitere Gebäude (GEB3 ff.).

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb2` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Mit Freigabe von 🧑: Vorschau und Spiel nach Kontext, Screenshots vorlegen; Abnahme im Ergebnis festhalten (wer, wann, was).
4. Nachweis je Kriterium AC-01 bis AC-03 aus den Ergebnissen GEB2.1 bis GEB2.3 und Schritt 3; Abschnitt „Abnahme“ der Sprint-README.
5. Sprint nach `erledigt/`, `Status: erledigt`; PR `sprint/geb2` → `develop` öffnen. B-435 bleibt `eingeplant`; B-438 behält nur die Bäume.

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 in Vorschau und Spiel steht im Ergebnis.
- [ ] AC-01, AC-02: Nachweise aus GEB2.1–GEB2.3 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane (Vorschau und Spiel) nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
