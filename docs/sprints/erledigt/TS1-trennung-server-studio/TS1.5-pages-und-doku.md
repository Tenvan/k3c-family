# TS1.5 · GitHub Pages nur site/, Entscheidung und Dokumentation

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** ts1/5-pages-doku
- **Abhängig von:** TS1.4
- **Tickets:** B-507
- **Kriterien:** AC-05, AC-06

## Ziel

`deploy-pages` baut nur `site/` (Bilder als Kopien in `site/`, kein Node), und die Entscheidung, `CLAUDE.md`, `codemap.md`, B-459/B-488/WZ19 sowie der Text von WZ19.2 zeigen die neue Trennung (AC-05, AC-06).

## Kontext

Spec `B-507 › Anforderungen › GitHub Pages` und `› Dokumentation`. Heute: `.github/workflows/deploy-pages.yml` führt `task pages` (voller Build, Spiel-Client unter `app/`); `site/index.html` lädt Bilder aus `app/…` (grafik, sprites); `tests/pagesSite.test.ts` prüft das. Neu: benötigte Bilder liegen als Kopien in `site/` (z. B. `site/img/`), Referenzen zeigen darauf, der Workflow lädt nur `site/` hoch. Doku: neue Datei in `docs/decisions/` (Studio vom Server getrennt, löst die Server-Studio-Seite aus B-459 ab; Watch-Entscheidung), `CLAUDE.md` (Struktur, Befehle, Tasks), `codemap.md`, Verweise in B-459/B-488/WZ19-README. WZ19.2 (Chat-Oberfläche) bekommt den Text: Chat global rechts in allen Studio-Seiten (Welt zuerst), Provider nur über CLI und ACP, automatisch erkannt und zum Verbinden vorgelegt; die Spec-Revision wird nicht still geändert, die Anpassung steht als Notiz in WZ19.2 und als Offene Frage in B-488 für 🧑.

## Erlaubte Dateien

- `.github/workflows/deploy-pages.yml`, `site/**`, `tests/pagesSite.test.ts`, `Taskfile.yml` (`pages`), `requirements.md`
- `docs/decisions/**`, `CLAUDE.md`, `**/codemap.md`, `docs/backlog/B-459*`, `docs/backlog/B-488*`, `docs/sprints/aktiv/WZ19-chat-und-prompt/WZ19.2-*.md`, `docs/arbeitsweise.md` nur falls Studio-Regeln dort stehen

## Nicht-Ziele

Neuer Seiteninhalt der Präsentation; Code von Server und Studio; Spec-Freigaben ändern oder Kriterien umformulieren (Notiz statt Änderung).

## Schritte

1. Test zuerst (rot): `pagesSite.test.ts` verlangt, dass jede Bildreferenz in `site/index.html` auf eine Datei in `site/` zeigt.
2. Bilder nach `site/` kopieren, Referenzen umstellen; `deploy-pages.yml` ohne Node, Task und Build, lädt nur `site/`; `task pages` entfällt oder kopiert nur.
3. Entscheidung in `docs/decisions/`, `CLAUDE.md`, `codemap.md`, B-459/B-488/WZ19, Text WZ19.2 anpassen.
4. `check_run` task:test mit Muster `planning` und `pagesSite` grün; Commit `docs(inf): Pages nur site/, Trennung dokumentiert (TS1.5)`.

## Fertig, wenn

- [x] AC-05: Workflow ohne Node und Build, Bilder in `site/`, `pagesSite.test.ts` grün.
- [x] AC-06: Entscheidung, `CLAUDE.md`, `codemap.md`, Verweise und WZ19.2-Text angepasst; `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
task test -- pagesSite
```

## Ergebnis

Siehe unten (Kurzfassung): AC-05 geprüft: `pagesSite`-Test rot→grün (Bildreferenzen auf `site/img`, Workflow ohne Node/Task/Build, `path: site/`); `task check` grün. AC-06 geprüft: Entscheidung `docs/decisions/004-studio-getrennt-datei-watch.md`, `CLAUDE.md`, Codemaps, `docs/arbeitsweise.md`, `docs/rules/welten.md`, `docs/glossar.md` angepasst; B-459 mit Vermerk „Abgelöst“; B-488 und WZ19 mit Notiz und offener Frage für 🧑 (Provider über CLI und ACP), WZ19.2 mit Notiz (Chat global rechts, Welt zuerst), Kriterien unverändert; `task test -- planning` grün.
Abweichungen: `site/index.html`: Links auf `app/` zeigen jetzt auf das GitHub-Repo; zusätzliche Dateien: `docs/glossar.md`, `docs/rules/welten.md`, Text-Dateien (ungenutzte `studio`-Schlüssel entfernt), `.github/workflows/ci.yml` (`task pages` → `task build`, im Lauf von Claude nachgezogen), `engine/codemap.md`. Offen/Ticket: B-499 nennt noch den alten Pfad `engine/studio` (Ticket-Text, Entscheidung bei 🧑); Folgetickets B-508 bis B-512 aus TS1.4.
