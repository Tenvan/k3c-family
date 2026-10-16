# GEB3.5 · Review und Abnahme der Stufenreihe

- **Status:** in Arbeit
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb3/5-review
- **Abhängig von:** GEB3.4, GEB3.7
- **Tickets:** B-435
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB3 ist geprüft, 🧑 hat die Stufenreihe von Mauer und Turm in der Vorschau und im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff (`tools/gebaeude/`, `tools/k3c-dev/internal/assets/`, `public/grafik/`, `src/tools/assetvorschau*.ts`, `src/scenes/buildingSprites*.ts`, `src/scenes/siteView.ts`, `data/assets.json`). Besonders: PNGs der acht vorhandenen Gebäude unverändert; Stufen-Fehlerfälle getestet; gelöschte Grafik nirgends mehr genutzt; Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** nur mit Freigabe von 🧑 für diesen Lauf. Vite über `svc_start`, Vorschau unter `http://127.0.0.1:5173/assetvorschau.html?szene=gebaeude` (unter `localhost:5173` antwortet auf Ralfs Rechner ein anderer Server). Spiel: nur wenn 🧑 auch das freigibt, sonst prüft 🧑 selbst (Spielserver über `svc_start`).

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, Tor und Burg (GEB4), TV-Test (GR7.4).

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb3` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Mit Freigabe von 🧑: Vorschau (und ggf. Spiel) zeigen, Screenshots vorlegen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-06; Abschnitt „Abnahme“ der Sprint-README.
5. Sprint nach `erledigt/`, `Status: erledigt`; B-437 erledigt (falls nicht schon in GEB3.1); PR `sprint/geb3` → `develop` öffnen.

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 steht im Ergebnis.
- [ ] AC-01, AC-02, AC-04, AC-05, AC-06: Nachweise aus GEB3.1–GEB3.4 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
