# Golden-Fälle der Planungslogik

Orakel für `tools/k3c-workbench/internal/planning` (Go) und einen späteren Rust-Port (Sprint RU68, RU69). Ein Fall hält
das **heutige Verhalten** fest, auch wenn es ein Fehler ist; Fehler werden als Ticket erfasst, nicht im Golden korrigiert.
Das Format kennt keinen Go-Typ: Dateien, JSON und Text genügen für einen Rust-Test.

## Aufbau

```
testdata/planung/
  README.md              diese Datei
  _vorlagen/             feste Kopien von docs/vorlagen/{ticket,sprint,session,projekt}.md (gehören zu jedem Fall)
  <fall>/
    vorher/              Dateibaum vor dem Aufruf, Pfade ab der Repo-Wurzel (docs/backlog/…, docs/sprints/…, docs/projekte/…)
    aufruf.json          ein Aufruf: {"werkzeug": "plan_set", "args": {…}}
    antwort.txt          erwartete Antwort des Werkzeugs (Text)
    nachher/             erwarteter Dateibaum nach dem Aufruf (vollständig, ohne docs/vorlagen/)
```

Ordner mit `_` am Anfang sind keine Fälle. Fallnamen sind kleingeschrieben mit Bindestrichen, Gruppe zuerst
(`ticket-…`, `sprint-…`, `session-…`, `projekt-…`, `liste-…`, `fehler-…`).

## Ablauf eines Falls

1. Temporäre Wurzel anlegen, `vorher/` hineinkopieren, `_vorlagen/*.md` nach `docs/vorlagen/` kopieren und die leeren Ordner
   `docs/sprints/geplant`, `docs/sprints/aktiv`, `docs/sprints/erledigt` anlegen (Git hält keine leeren Ordner).
2. `werkzeug` mit `args` auf dieser Wurzel aufrufen. Kein Git-Repo, kein Netz, keine Worktrees.
3. Antwort mit `antwort.txt` vergleichen, alle Dateien außer `docs/vorlagen/` mit `nachher/` (gleiche Dateimenge, gleicher
   Inhalt; leere Ordner zählen nicht).

## Werkzeuge und Argumente

Namen und Argumente sind die der MCP-Tools der Workbench (ohne `checkout`):

| `werkzeug` | `args` |
|---|---|
| `plan_create` | `kind`, `id`, `slug`, `title`, `fields` |
| `plan_set` | `id`, `fields` |
| `plan_section` | `id`, `section`, `text` |
| `plan_sections` | `id`, `sections` (Name → Text) |
| `plan_delete`, `plan_get`, `plan_check` | `id` |
| `plan_list` | `kind`, `status`, `domain`, `sprint`, `projekt`, `archive` (alle optional) |
| `plan_next` | `domain` (optional) |
| `plan_waves` | `wave_count`, `project`, `domain` (alle optional) |
| `plan_queue` | `device` (optional) |
| `plan_index` | `dry` (optional, Standard `true`) |

Nicht Teil der Fälle sind Prüfungen der MCP-Schicht (`ticketPlanned` in `mcpsrv`: Ticket braucht Projekt und Sprint, das
Argument `checkout`) und alles, was Git braucht (Worktrees, `origin/sprint/…`, Ticket-Nummern aus anderen Checkouts).

## Maskierung beim Vergleich

- Zeilenenden: CRLF gilt als LF.
- Das heutige Datum in der Zeile `**Erstellt:**` neuer Tickets steht als `<HEUTE>`; andere Daten bleiben wörtlich.
- Der Pfad der temporären Wurzel steht als `<WURZEL>` (Antwort und Dateien).

## Fehler

Schlägt der Aufruf fehl, beginnt `antwort.txt` mit `FEHLER: ` und der Meldung. `nachher/` zeigt dann den Stand, den der
Fehler hinterlässt (meist gleich `vorher/`).

## Fälle schreiben und erneuern

`vorher/` und `aufruf.json` entstehen von Hand. Den Rest schreibt ein bewusster Lauf (Begründung in den Commit):

```powershell
$env:UPDATE_GOLDEN = "1"; go test ./internal/planning -run Golden   # im Ordner tools/k3c-workbench
```

Danach `git diff testdata/planung` lesen: Jede Änderung an `antwort.txt` oder `nachher/` ist eine Verhaltensänderung der
Planungslogik. Prüfen (ohne Schreiben): `check_run go:test` mit Muster `Golden`.
