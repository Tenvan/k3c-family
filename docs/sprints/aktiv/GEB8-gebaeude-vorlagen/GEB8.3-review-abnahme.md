# GEB8.3 · Review und Abnahme der Vorlagen

- **Status:** in Arbeit
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** geb8/3-review
- **Abhängig von:** GEB8.2
- **Tickets:** B-441
- **Kriterien:** AC-04

## Ziel

Der Diff von GEB8 ist geprüft, 🧑 hat die acht Vorlagen in der Vorschau gelesen und abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff (`tools/gebaeude/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, Regeldatei). Besonders: PNGs unverändert (md5), Pflichtfeld wirklich erzwungen, Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-04):** nur mit Freigabe von 🧑 für diesen Lauf. Vite über `svc_start`, Seite unter `http://127.0.0.1:5173/assetvorschau.html?szene=gebaeude` (auf Ralfs Rechner antwortet unter `localhost:5173` ein anderer Server, siehe GEB2.4), Screenshot der Vorlagen. Änderungswünsche an Texten setzt die Session direkt um (nur Feld `vorlage`).

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 gewünschte Textänderungen in `vorlage`
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Grafik-Änderungen, weitere Gebäude, KI-Erzeugung aus der Vorlage.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb8` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Mit Freigabe von 🧑: Vorschau öffnen, Screenshot vorlegen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-04; Abschnitt „Abnahme“ der Sprint-README.
5. Sprint nach `erledigt/`, `Status: erledigt`; B-441 `erledigt`; PR `sprint/geb8` → `develop` öffnen.

## Fertig, wenn

- [ ] AC-04: Abnahme der Vorlagen durch 🧑 steht im Ergebnis.
- [ ] AC-01 bis AC-03: Nachweise aus GEB8.1, GEB8.2 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane (Vorschau) nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
