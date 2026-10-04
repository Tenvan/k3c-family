# GR2.4 · Review und Abnahme des Sprints GR2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Branch:** gr2/4-review
- **Abhängig von:** GR2.3
- **Tickets:** B-162
- **Kriterien:** alle

## Ziel

Der Sprint GR2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-162 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `docs/funde/`, `public/grafik/`, Credits und `docs/assets/zuordnung.md`. Besonders prüfen: nur CC0/CC-BY, jede neue Datei mit Lizenzdatei und Credit (CC-BY mit Urheber und Quelle); keine Musik, kein Demo-Code, keine Quelldateien im Repo; nur gewählte Kandidaten zugeordnet, die übrigen nur als Gruppe `kandidaten` im Bestand (AC-06, Entscheidungen von 🧑 auf der Referenzseite); Tests nicht gelockert (Zahl der Packs angepasst, nicht entfernt).

## Erlaubte Dateien

- `public/grafik/`, Credits, `docs/assets/zuordnung.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Auswahl, Einbau.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr2` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von GR2.1 bis GR2.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-162 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Leichter Review am 2026-10-04. `task check` und `task check:go` grün. Diff `origin/develop...origin/sprint/gr2` (nur GR2.3 Teil 2): 93 PNG, 20 `LICENSE.txt`, `CREDITS.md`, `index.json`, `grafikPacks.ts/.test.ts`; keine Musik, kein Demo-Code, keine Quelldateien (nur PNG und TXT).

- **AC-01:** Ergebnis GR2.1 (38 Karten, 11 Abschnitte); abgeglichen.
- **AC-02:** Ergebnis GR2.2 (alle 11 Abschnitte mit Entscheidung 🧑); abgeglichen.
- **AC-03, AC-04:** Ergebnis GR2.3 Teil 1 (#126, `git show c345a9f --stat`: 59 Dateien, `grafikPacks.ts`, Zuordnungstabellen); abgeglichen.
- **AC-05:** `task check` und `task check:go` grün.
- **AC-06:** jede der 20 Pack-Ordner hat `LICENSE.txt` und Credit-Zeile mit Urheber und Quelle; CC BY 3.0 (`materials-pack`) und CC BY 4.0 (`plants-and-flowers-pixel-art`, `kyrises-free-16x16-rpg-icon-pack`) mit Namensnennung, sonst CC0; alle 93 Index-Einträge Gruppe `kandidaten`; kein Pack in `docs/assets/zuordnung-*.md`; Test-Zahl 21 → 41 angepasst, neuer Test ergänzt (nichts gelockert).
- Befunde: keine schweren. Nit ohne Ticket: fehlendes Leerzeichen in `grafikPacks.test.ts` (`=renderCredits`).
- Offen für 🧑: Ansicht `grafiken.html` im Browser-Pane (GR2.3 Schritt 5).
