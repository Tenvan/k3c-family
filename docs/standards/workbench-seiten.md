# Workbench-Seiten – gemeinsame Spec

**Version 1.2 · Stand 2026-10-08 · Vorlage: ErpApi (`tools/go/dev-workbench/frontend`)**

Diese Datei liegt in jedem Projekt mit Workbench (ErpApi, k3c, …) wortgleich unter
`docs/standards/workbench-seiten.md`. Sie beschreibt Aufbau und Verhalten der Seiten,
die jede Workbench hat. Die Spec gibt den Stand vor, nicht der Code eines einzelnen Projekts:
Wer eine Seite in einem Projekt ändert, ändert erst diese Datei (Version hochzählen) und
überträgt sie dann in die anderen Projekte.

Projektspezifisch bleiben Inhalte (welche Dienste, welche Tools, welche Tasks), Datenquellen
und CSS-Präfixe. Gleich sind Begriffe, Aufteilung, Ebenen, Bedienung und gemerkter Zustand.

## Gemeinsame Regeln

| Regel | Inhalt |
| --- | --- |
| Höhe | Jede Seite füllt die Inhaltsfläche; Kopf- und Fußzeilen stehen fest, nur Listen scrollen. |
| Zwei Spalten | Liste links, Ausgabe/Detail rechts. Unter **1100 px** stehen sie untereinander (Container Query, wo die Seite eine eigene Fläche hat). |
| Gemerkter Zustand | `localStorage` über einen Helfer, der Lese-/Schreibfehler schluckt und veraltete Werte verwirft (ErpApi: `lib/persist.ts`). Schlüssel `<präfix>.<seite>.<was>`. Freitext-Filter werden **nicht** gemerkt, Chips/Toggles schon. |
| Status-Marken | Rund mit Farbpunkt (Pill): grün fertig, blau in Arbeit, amber Freigabe/Warnung, rot Fehler/geändert, violett Worktree, grau offen. |
| Prompt-Knopf | Überall, wo Arbeit für einen Agenten entsteht: ⧉ kopiert den Prompt, der Claude-Funke öffnet ihn als neue Code-Session (Deep Link, schickt nicht ab). |
| Keine Intervalle | Nachgeladen wird über Ereignisse der Go-Seite und einen „Aktualisieren“-/„Neu laden“-Knopf. Ausnahmen: Tailing einer Logdatei und der Planungsstand (Takt 2 s; der nächste Abruf startet erst nach der Antwort, eigene Änderungen laden sofort neu; teure Abfragen wie Git je Worktree werden bis zur nächsten Änderung zwischengespeichert). |

## MCP-Tools (Regeln für alle Seiten)

Jede Seite hat ihre MCP-Tools; Agenten sehen damit dasselbe wie die Oberfläche und
schreiben über dieselben Funktionen. Namen, Parameter und Verhalten sind in allen
Workbenches gleich, damit Skills und Policies projektübergreifend funktionieren.

| Regel | Inhalt |
| --- | --- |
| Name | `<bereich>_<verb>` in Kleinbuchstaben (`plan_session`, `svc_start`, `logs_query`). Ein Tool je Ebene mit `action` (`list`/`get`/`create`/`update`/`delete`/`done`), wo es um Pflege geht. |
| Antwort | Verdichteter Text (keine Rohausgabe); große Ausgaben über Kappe (`limit`) und Cursor (`since`). |
| Schreiben | `delete` und Neustarts verlangen `confirm=true`; abgelehnte Aufrufe nennen den gültigen Wertebereich. |
| Beschreibung | Jede Tool-Beschreibung endet mit „Nutze es bei: … Statt: …“ (wofür, und welchen Shell-Weg es ersetzt). |
| Systemprompt | Die MCP-`instructions` der Workbench nennen die Tools je Seite in derselben Gliederung wie diese Spec. |

### Übergreifend (keiner Seite zugeordnet)

| Tool | Zweck | Parameter |
| --- | --- | --- |
| `workbench_status` | Umgebung selbst: Modus, Repo-Wurzel, Config-Fehler, MCP-Statistik, Versionen | – |
| `notify_ui` | Hinweis an den Nutzer in der Oberfläche | `level`, `title`, `text?` |

Projektspezifisch und nicht Teil dieser Spec: `check_run` (Build-/Test-Ziele), `git_commit`
(Seite Commit), `ui_snapshot` (Oberfläche der Anwendung, nicht der Workbench) und
`open_in_ui` (Seitenwechsel durch den Agenten; nicht gefordert, wird kaum gebraucht).

## Begriffe (verbindlich in allen Projekten)

| Ebene | Ablage | ID | MCP-Tool |
| --- | --- | --- | --- |
| **Projekt** (oberste Karte) | Ordner `TODOs/2-projekte-aktiv/PJ-NNN-<name>/` | `PJ-054` | `plan_project` |
| **Sprint** (Panel in der Karte) | Datei `NN-<name>.md`, `# Sprint N — <Titel>` | Kürzel `S14` | `plan_sprint` |
| **Session** (Zeile) | Zeile in Active/Done Sessions | `PJ-054#22` | `plan_session` |
| **Ticket** (Backlog) | `TODOs/1-backlog/<PRÄFIX>-NNN-<thema>.md` | `FE-009` | `plan_ticket` |

„Phase“, „Aufgabe“ und „Task“ sind für Planungsebenen nicht zulässig. Die Ablage im
Detail regelt der Skill `todo-planner` (orga-planning ≥ 3.0.0).

---

## 1. Planung

Reiter mit Marke **!**, solange der Radar ungesehene Änderungen hat.

### Aufbau

```text
┌ Filterleiste ─────────────────────────────────────────────────────────────────┐
│ Freitext · Chips Projekt-Zustand · Chips Session · | Backlog-Chips · Treffer   │
└───────────────────────────────────────────────────────────────────────────────┘
┌ Projekte ─────────────────────────────┐ ┌ Backlog ──────────────────────────┐
│ Kopf: „Projekte“ n von m · k erledigt │ │ Tickets je Domain, Einplan-Ziel   │
│       [alle einklappen][alle aufklappen]│ ├ Detail ───────────────────────────┤
│ Leiste markierter Sessions + Prompt   │ │ Projekt / Sprint / Session / Ticket│
│ Projekt-Karten (scrollen)             │ │ mit Prompt-Knopf                   │
└───────────────────────────────────────┘ └───────────────────────────────────┘
```

### Filterleiste

Ein Freitext für Projekte und Backlog zugleich. Chips: Projekt-Zustand (geplant, aktiv, erledigt),
Sessions (offen, Freigabe, geändert, autonom, worktree-tauglich), danach Backlog (Priorität,
eingeplant, Domains). Chips einer Gruppe wirken als UND, Domains untereinander als ODER.
Rechts die Trefferzahl.

### Reihenfolge

Offene Projekte mit gesetztem Rang stehen vorn (aufsteigend), danach die übrigen in automatischer
Ordnung (Abhängigkeit, Priorität, zuletzt bearbeitet); erledigte am Ende. Der Rang steht als
`│ Rang: N` in der Kopfzeile des `00-index.md`; „hoch/runter“ setzt ihn und nummeriert alle offenen
Projekte lückenlos neu (`plan_project update rank`).

### Ebenen der Projekt-Spalte (k3c-Look)

| Ebene | Darstellung |
| --- | --- |
| **Projekt-Karte** | Getönte Fläche (`accent-a2`, Rahmen `accent-a6`) mit **4 px Akzentkante links**, Radius 4. Erledigt: grau getönt, graue Kante. Ausgewählt: Ring in Akzentfarbe. |
| **Sprint-Panel** | Ruhige Fläche (`color-panel-solid`, Rahmen `gray-a5`) in der Karte; Sprint „in Arbeit“ mit Akzentrahmen, im Worktree mit violetter Innenkante. |
| **Session** | Zeile ohne eigene Fläche im Sprint-Panel; Hover grau, Auswahl akzentgetönt. |

### Projekt-Karte, von oben nach unten

1. **Kopf** (Klick schaltet auf/zu und wählt das Projekt; Linie `accent-a5` darunter, eingeklappt ohne Linie):
   Pfeil ▾/▸ · **Rang-Kachel** (24 px; Position unter allen offenen Projekten, unabhängig vom
   Filter; gesetzter Rang `accent-9` mit Zahl in `accent-contrast`, automatischer Rang blasser
   `accent-a5`; erledigte ohne Kachel) · ID in Mono, fett · Titel fett, Größe 4, in `accent-11` ·
   `Sprints x/y` (fertige von allen) · Knöpfe **hoch/runter** (an den Enden gesperrt) · Pills: „geändert“ (Radar),
   Priorität, Abhängigkeits-Links zu anderen Projekten, Worktree-Pills, Zustand, Prompt-Knopf.
   Reicht die Breite nicht, brechen Zähler und Pills in eine eigene Zeile um.
2. **Sprint-Chips**: eckig (Radius 4 px, Mono 11 px, fett) `S<nr> · <zustand> · <fertig>/<alle>`;
   in Arbeit Akzent, Freigabe amber, fertig grün. Klick wählt den Sprint: Projekt und Sprint
   klappen auf, die Liste scrollt hin.
3. **Fortschrittsbalken** (4 px): grün erledigt, amber Freigabe nötig.
4. **„Nächste Session“** (nur aufgeklappt): `S<nr> #<id>` fett, Name, Prompt-Knopf. Erste offene Session in Planungsreihenfolge.
5. **Sprint-Panels** (nur aufgeklappt).
6. **Fußzeile**: `x/y Sessions erledigt · n Freigabe nötig · n Blocker/Befunde · <Pfad>`.

### Sprint-Panel

Kopfzeile (Klick schaltet auf/zu und wählt den Sprint): Pfeil · `S<nr>` fett · Priorität · Titel fett (mit roter `!`
bei Radar-Änderung) · rechts Zustands-Pill, Worktree-Pill, Abhängigkeits-Links `S…`,
`fertig/alle`, Prompt-Knopf. Darunter ein 3-px-Fortschrittsbalken. Aufgeklappt folgen die
Session-Zeilen: Checkbox zum Markieren (nur offene mit ID) · Zustands-Pill · `#<id>` (Mono, fett)
· Name · **Tickets** der Session (eckig, Mono; Klick zeigt das Ticket im Detail) · Worktree-Pills, Priorität, Modus, Abhängigkeits-Links `#…`, Datum · Prompt-Knopf.
Mit „nur offene“ verschwinden fertige Sprints; bei Suche verschwinden Sprints ohne Treffer.

### Einklappen

| Was | Regel |
| --- | --- |
| Kopf | Klick auf den Kopf von Projekt oder Sprint schaltet auf/zu und wählt aus; Knöpfe im Kopf (Prompt, hoch/runter, Links) reichen den Klick nicht weiter. |
| Pfeil | Klappt nur, wählt nichts aus (`stopPropagation`). `aria-expanded`, `aria-label` „… einklappen/aufklappen“. |
| Standard ohne gemerkten Wert | Projekt offen. Sprint zu, außer dem **ersten unfertigen Sprint eines aktiven Projekts**. |
| Gemerkt | Je Projekt (`pj:<ref>`) und Sprint (`sp:<ref>#<nr>`) ein Eintrag `offen: bool` unter `dwb.planning.fold`; nur gesetzte Werte, sonst Standard (ErpApi: `lib/useFold.ts`). |
| Sync | Jede Änderung schreibt die Ablage und feuert ein `window`-Ereignis; alle Karten lesen daraufhin neu. |
| Alle ein-/aufklappen | Im Spaltenkopf. Setzt alle **angezeigten** Projekte samt Sprints auf zu bzw. auf und ersetzt den gemerkten Stand. |
| Auswahl von außen | Nur Auswahl, die nicht vom Kopf der Karte selbst kommt — Detailbereich, Abhängigkeits-Link, Sprint-Chip: Projekt und Sprint klappen auf, die gewählte Zeile scrollt in Sicht. |
| Radar-Änderung | Projekt mit ungesehener Änderung klappt auf, ebenso die Sprints mit geänderten Sessions. |
| Suche | Während Freitext oder „nur Freigabe“ aktiv ist, ist alles mit Treffern offen, der gemerkte Stand bleibt unberührt. |

### Backlog und Detail

Backlog: offene Tickets je Domain, Klick zeigt das Ticket im Detail, markierte Tickets plus
Einplan-Ziel (aktives Projekt oder „Neues Projekt“) ergeben einen Prompt.
Detail: zeigt die Auswahl (Projekt mit Blockern, Sprint mit Ziel und Sessions, Session, Ticket),
jeweils mit Fakten, Prompt-Knopf und Sprung-Links; ist die Auswahl verschwunden, ein Hinweis.
Ein Klick in ein Projekt quittiert dessen Radar-Änderungen.

### MCP-Tools

| Tool | Zweck | Parameter |
| --- | --- | --- |
| `plan_project` | Projekte: `list` (offen, auch Worktrees; `state=done/all`), `get` (Sprints, Sessions, Blocker), `create` (`PJ-NNN-<slug>` aus Vorlagen), `update` (Titel, Zustand, **Rang**), `delete` | `action`, `project?`, `state?`, `query?`, `title?`, `goal?`, `checkout?`, `rank?`, `confirm?` |
| `plan_sprint` | Sprints eines Projekts: `create` (nächste Datei, Zeile in der Sprint-Übersicht), `update` (Titel, Ziel), `delete` | `action`, `project`, `sprint?`, `title?`, `goal?`, `confirm?` |
| `plan_session` | Sessions: `create` (nächste Nummer), `update` (Beschreibung, Status, Priorität, Modus, Umgebung, Abhängigkeiten, **Tickets**), `done` (Active → Done mit Datum), `delete`; Index-Zähler werden nachgezogen | `action`, `project`, `sprint?`, `id?`, `desc?`, `status?`, `deps?`, `tickets?`, `prio?`, `mode?`, `env?`, `effort?`, `date?`, `note?`, `arch?`, `handbook?`, `confirm?` |
| `plan_ticket` | Tickets im Backlog: `list`, `create` (nächste ID zum Präfix), `update` (Frontmatter inkl. `status in-projekt` + `projekt`), `delete` | `action`, `id?`, `prefix?`, `title?`, `summary?`, `priority?`, `status?`, `statusNote?`, `project?`, `dependsOn?`, `mode?`, `env?`, `query?`, `confirm?` |

Oberfläche und Tools sind deckungsgleich: „hoch/runter“ = `plan_project update rank`,
Flags im Detail = `plan_session update`, Ticket-Flags = `plan_ticket update`.

---

## 2. Dienste & Logs

Reiter „Services & Logs“.

```text
┌ links ──────────────────────┐ ┌ rechts ─────────────────────────────────┐
│ Dienstkarten (Startreihen-  │ │ Log-Statistik der gewählten Quelle      │
│ folge), Klick wählt Quelle  │ ├─────────────────────────────────────────┤
│ ─────────────────────────── │ │ Tabs: Konsole · Log · Fehler (Digest)   │
│ Task-Läufe als Quellenliste │ │ Inhalt des Tabs                          │
└─────────────────────────────┘ └─────────────────────────────────────────┘
```

**Dienstkarte**: Name (fett) · `Port <n>` · Status-Badge rechts; darunter Health-Ziel
(Tooltip mit vollem Wert, sonst „process“), Kennzahlen, Hinweise, Aktionen Start/Stopp/Neustart.
Nur die ausgelöste Aktion zeigt den Ladezustand; **Stopp bleibt immer bedienbar**, auch während
eines hängenden Starts. Gewählte Karte hervorgehoben (`aria-current`). Ist Projektwurzel oder
Konfiguration nicht lesbar, steht statt der Karten eine Infokarte mit Pfad und Ursache.

**Quellenliste** unter den Karten: Task-Läufe mit Zustandspunkt, nach Gruppe überschrieben.

**Rechte Seite**: Statistik nur für Dienste mit Logdatei. Tabs: *Konsole* (flüchtiger
Ringpuffer, ANSI-Farben, Schnellsuche mit Markierung, Kappe), *Log* (Logdatei, Filter, Tailing
im Takt) und *Fehler* (verdichtete Warn-/Fehlermeldungen); Log und Fehler sind ohne Logdatei
deaktiviert.

**Auswahl**: gemerkt wird die tatsächlich gezeigte Quelle. Fehlt sie, fällt die Anzeige auf den
ersten laufenden Dienst, dann den ersten Dienst zurück.
Umbruch auf eine Spalte unter 900 px.

### MCP-Tools

| Tool | Zweck | Parameter |
| --- | --- | --- |
| `svc_status` | Zustand aller Dienste (PID, CPU, Speicher, Laufzeit, Modus) – erster Aufruf vor jeder Arbeit an der laufenden Umgebung | – |
| `svc_health` | Health eines Dienstes, wenn er läuft, aber nicht antwortet | `service` |
| `svc_start` · `svc_stop` | Dienst starten/stoppen; Start wartet auf Health | `service`, `waitSeconds?` |
| `svc_restart` | Dienst neu starten | `service`, `waitSeconds?`, `confirm?` |
| `svc_start_all` · `svc_stop_all` | Alle Dienste in Startreihenfolge bzw. rückwärts | – |
| `ports_status` | Belegung der Dev-Ports, auch durch fremde Prozesse | – |
| `get_urls` | Erreichbare URLs je Dienst (vor jedem HTTP-Aufruf, jeder Browserprüfung) | `target?` |
| `console_tail` | Flüchtiger Konsolenpuffer eines Dienstes oder Laufs (Startfehler, Rohausgabe) = Tab *Konsole* | `service`, `limit?`, `since?` |
| `logs_services` | Dienste mit Logdatei | – |
| `logs_stats` | Kennzahlen je Level/Namensraum = Statistik über den Tabs | `service?`, `since?`, `groupBy?` |
| `logs_errors` | Verdichtete Fehler und Warnungen = Tab *Fehler* | `service?`, `since?`, `level?`, `limit?` |
| `logs_query` | Gefilterte Logzeilen = Tab *Log* | `service?`, `level?`, `ns?`, `pattern?`, `since?`, `limit?` |
| `logs_context` | Zeilen um einen Zeitpunkt | `service`, `ts`, `before?`, `after?`, `limit?` |
| `logs_since` | Neue Zeilen ab Cursor (Tailing) | `service?`, `cursor?`, `limit?` |

---

## 3. MCP (nur Layout)

Oben ein Umschalter **Widgets | Statistiken** (gemerkt).

**Widgets** in drei Bändern:

1. **Serverkarte**: Titel „MCP-Server“ · URL (Tooltip) · Status-Badge · Neustart (immer
   bedienbar, neben dem Badge) · Systemprompt-Knopf · „Aktualisieren“. Darunter zwei
   Kennzahlengruppen (Aufrufe/Fehler/Dauer; Verbindungen mit Tooltip). Fehler als Zeile darunter.
2. **Band**: links **Tool-Kacheln** (kompakt: Name, Anteilsleiste, eine Kennzahlenzeile;
   Beschreibung und letzter Aufruf nur im Tooltip und `aria-label`; aktivste zuerst, dann
   alphabetisch; jedes registrierte Tool, auch mit 0 Aufrufen), rechts **Live-Monitore**
   (Top-Tools als senkrechte Balken, Aufrufe- und Laufzeit-Linie über ein mitlaufendes Zeitfenster,
   Ausreißer rot).
3. **Aufruf-Log** bekommt die Resthöhe: Kopfzeile mit Titel und Filtern (Tool, Live/Alle) in
   einer Zeile; je Aufruf eine Start- und eine Endzeile mit Branch-Spalte wie `git log --graph`
   für parallele Aufrufe; Argumente aufklappbar und eingerückt.

Keine SectionCard-Rahmen um die Bänder: die Karten tragen ihre Titel selbst.

**Statistiken**: Kennzahlen, sortierbare Tabelle je Tool (Perzentile, Gesamtzeit, Ausreißer,
aufklappbar zu Argumenten und Fehlern), daneben jüngste Ausreißer, langsamste Aufrufe, häufigste
Fehler; umschaltbar zwischen laufender Sitzung und gesamter Aufzeichnung.

Umbrüche: Band einspaltig unter 900 px, Statistik einspaltig unter 1000 px.

### MCP-Tools

Die Seite zeigt die Nutzung aller Tools; eigene Tools hat sie nicht. Kennzahlen des
Servers liefert auch `workbench_status`.

---

## 4. Tasks

```text
┌ Kopf: Filter (Esc leert) · 🔑 nur freigegebene · [n Schalter übernehmen] · Neu laden ┐
├ Baum (scrollt) ───────────┬─ Splitter ─┬ Ausgabe des gewählten Tasks ─────────────┤
│ ★ Favoriten               │            │ Kopf: Name, Zustand, Argumentfeld,        │
│ ▾ Namensraum              │            │       Start/Stopp (Enter startet)          │
│   🔓/🔒 📌 Task  Zustand   │            │ Konsole des Laufs                          │
│                           │            │ ▸ Letzte Läufe (Zeit, Ergebnis, Argumente)│
├ Fuß: n von m Tasks · Filter · k laufen · Zuletzt geladen hh:mm ───────────────────┤
```

- **Baum**: Namensräume einklappbar; gemerkt wird das *Eingeklappte*, damit neue Namensräume
  offen erscheinen. Favoriten sind eine Sicht auf dieselben Knoten (Pin), keine Kopie.
- **Freigabe-Schloss** je Task: ob ein Agent ihn über `task_start` starten darf. Vier Zustände
  aus Datei und Laufzeit-Schalter; gestrichelter Rahmen = nur bis zum Beenden. „Schalter
  übernehmen“ schreibt die Laufzeit-Schalter in die Freigabedatei.
- **🔑-Toggle** gemerkt, Freitext nicht.
- **Splitter** zwischen Baum und Ausgabe, Anteil gemerkt, Doppelklick setzt zurück; unter 1100 px
  untereinander, der Anteil gilt dann nicht.
- **Ausgabe**: dieselbe Konsolen-Komponente wie unter Dienste & Logs (Ringpuffer `task:<name>`).
  Zusatzargumente werden als `task <name> -- …` angehängt; Shell-Metazeichen lehnt die Go-Seite ab.
- **Fußzeile**: Zähler, laufende Tasks, Zustand des Katalogs; ein gescheiterter Reload steht
  hier, nicht über dem Baum.

### MCP-Tools

| Tool | Zweck | Parameter |
| --- | --- | --- |
| `task_list` | Taskfile-Baum; freigegebene Tasks mit `*` (= Schloss offen) | `namespace?`, `query?` |
| `task_start` | Freigegebenen Task starten (kehrt sofort zurück; Freigabe laut Schloss, nicht laut Name) | `task`, `args?` |
| `task_stop` | Laufenden Task stoppen | `task` |
| `task_status` | Läufe dieser Sitzung | `task?` |
| `task_output` | Konsole eines Laufs = rechte Hälfte | `task`, `limit?`, `since?` |

Wer auf das Ende warten will, nimmt das projektspezifische `check_run`.

---

## Änderungsprotokoll

| Version | Datum | Änderung |
| --- | --- | --- |
| 1.0 | 2026-10-08 | Erstfassung aus ErpApi; Planung mit Baumstruktur nach k3c B-364 (Sprint-Karte mit Akzentkante, Phasen-Panels, Phasen-Chips, gemerktes Einklappen). |
| 1.1 | 2026-10-08 | Begriffe in allen Projekten Projekt → Sprint → Session + Ticket (Tabelle je Projekt entfällt); Planung im k3c-Look: Rang-Kachel, Titel in Akzentfarbe, `Sprints x/y`, Sprint-Chips `S<nr>`, Sprint-Panels mit eigenem Balken. |
| 1.2 | 2026-10-08 | MCP-Tools je Seite als fester Teil der Spec (Namen, Parameter, Regeln); Planung: Rang (`hoch/runter`, `plan_project update rank`) und Spalte `Tickets` je Session. |
