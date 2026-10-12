# Spec: tenvan-cc

Agenten: vor jeder Arbeit an diesem Plugin zuerst [DEVELOPMENT.md](DEVELOPMENT.md) lesen (Prüfbefehle, API-Fallstricke, Test-Muster).

Lebende Spezifikation. Bei jeder Feature-Änderung mitpflegen: Abschnitt „Features“ und „Akzeptanzkriterien“ anpassen, erledigte Backlog-Punkte streichen, Entscheidungen im Log ergänzen.

## Ziel

Ein einziges, über den orga-marketplace installierbares Mod für **Claude Desktop**, das nützliche Community-Mods bündelt und vereinfacht. Zwei Anzeigeflächen:

- **Band** über der Prompt-Eingabe (`ui.render` auf `AbovePrompt`): kompakt, immer sichtbar
- **Panel** rechts (`ui.render` auf `Pane`, `requestId: 'tenvan-cc'`): ausführlich, auf Abruf

## Leitplanken

| Regel | Begründung |
| --- | --- |
| Logik oberflächenneutral, Zeichnen je Oberfläche: Daten und Formatierung in `core.ts`, Texte in `i18n.ts`, Farben in `THEME`; Svg, Tooltips und Pixelmaße nur in den Zeichenfunktionen hinter `e.surface` | Später folgt eine Claude-Code-CLI-Version mit eigenen Renderings für das Terminal; sie soll dieselbe Logik nutzen |
| Kein Code aus Vorlagen übernehmen, nur Feature-Ideen | Lizenz- und Wartungshoheit bleiben bei uns |
| Nur `desktop`-Surface zeichnet, sonst `next(e)` | Entscheidung Ralf; Terminal bleibt unverändert |
| Kein Netzwerk, keine Telemetrie; Transcript nur für die Zusammenfassung | Datenschutz; Daten aus Engine-Events und `$.session.usage()`. Ausnahme: die Zusammenfassung liest `$.session.messages()` und sendet Auszüge samt Git-Stand an Haiku; abschaltbar (Einstellung „Automatisch aktualisieren“), manuell weiter möglich |
| Model-Calls nur über `$.model.complete` (Folge-Schritte, Zusammenfassung), jeweils in den Einstellungen abschaltbar | Kosten bleiben steuerbar |
| Weitere Community-Mods werden Abschnitte in Band oder Panel, kein eigenes Plugin | Ein Mod, ein Install |
| Beobachtende Hooks greifen nie ein (kein `deny`, kein Rewrite); einzige Ausnahme sind die Hover-Infos, die nur gezeichnete Elemente umhüllen | Mod darf das Verhalten der Session nicht ändern |
| UI-Texte auf Deutsch mit Umlauten (Standard), umschaltbar auf Englisch; alle Texte in `hooks/i18n.ts` | Zielgruppe |

## Quellen der Features

| Vorlage | Übernommen | Bewusst weggelassen |
| --- | --- | --- |
| [scasella/claude-flightdeck](https://github.com/scasella/claude-flightdeck) | Panel: Turn-Beleg, Gate-Zähler, Agents, Log | Architect- und Loops-Panel, 4 Layouts, Paletten, Motion, Tastenkürzel, 12 Optionen, Statuszeile |
| [augiefra/token-weather-usage](https://github.com/augiefra/claude-mods/tree/main/plugins/token-weather-usage) | Band: Kontext, 5h/7d mit Takt, Cache-Countdown, Kosten, Agent-Zahl | Wetter-Icons, Hover-Cards, 3 Sprachen, Preistabelle, Ersparnis-Rechnungen, `$.store` über Sessions |
| [oliverow/catch-me-up](https://github.com/oliverow/catch-me-up) (MIT) | Fortlaufende Zusammenfassung der Session (rollierend, Haiku, Speicher je Session); Anweisungen als `assets/catchup.en.md` unverändert, auf Deutsch neu gefasst (`catchup.de.md`); eigene TypeScript-Umsetzung, siehe `THIRD_PARTY.md` | Eigenes Pane, Auto-Öffnen, Hotkeys |
| [pawandeepdhall/next-steps](https://github.com/pawandeepdhall/claude-mods/tree/main/plugins/next-steps) | Band: bis zu 3 Folge-Schritte als Buttons | Mehrfachauswahl, zweiter Model-Call zum Ausformulieren, Antwort-Cache, `/mod-bench` |

## Features

### Titelleiste (Band und Panel, jeweils erste Zeile)

`Tenvan CC (Version x, Build: JJJJ-MM-TT hh:mm)`. Version aus `.claude-plugin/plugin.json`, Build-Stempel = jüngste Änderungszeit von `hooks/register.tsx` und `hooks/core.ts` (kein Build-Schritt vorhanden). Gelesen bei `session.start`, also auch bei jedem Reload.

Das Logo steht vor der Titelzeile: `assets/logo.svg` (SVG mit eingebettetem PNG, 13 × 20 px); ohne Logo oder `Svg` bleibt nur der Text. Das Bild wird nie umgefärbt.

Darunter eine Infozeile zum Arbeitsverzeichnis (nur lesende git-Befehle, kein Netzwerk): Branch, Worktree-Name mit Art (Hauptcheckout oder verknüpft), Kurz-SHA, Anzahl geänderter Dateien, Abstand zum Upstream (↑ ↓, sonst „kein Upstream“), Ordner samt Unterordner. Gelesen bei `session.start` und nach `turn.complete`.

### Band (`AbovePrompt`)

Zeile 2, sobald eine Messung vorliegt:

- Titelzeile: Titel links, Farblegende rechts (`LEGEND` in `core.ts`: im Takt, zu schnell, knapp, Agents)
- Drei Karten (Rahmen `round`) nebeneinander, bei Platzmangel umbrechend: Kontext, Limits, Cache/Kosten/Agents
- Ringdiagramme (`Svg`, `ringSvg` in `core.ts`) für Kontext und jedes Limit statt Textbalken; ohne `Svg` auf der Oberfläche bleibt der Textbalken. Das Panel behält die Fortschrittsbalken
- Tooltips für Kontext, jedes Limit und Cache: der Ring ist ein interaktives `Svg` mit `<title>` (schwebender Tooltip), aber nur im Leerlauf: während der Agent arbeitet ist er ein einfaches Bild (interaktive Svg flackern bei jedem Neuzeichnen); Zeiten im Tooltip nur minutengenau. Kontext: Aufschlüsselung nach Kategorien aus `$.session.usage({ breakdown: 'summary' })` (lokal geschätzt, kein Request; nach `turn.complete` und höchstens alle 5 s nach `session.measure`, Atom `breakdown`); Limit: Verbrauch, Takt in Worten, Reset mit Wochentag; Cache: Restzeit, Gültigkeit, letzter Request, Trefferquote. Einstellung `ctxTip`. Reiner Klartext
- Kontext: Balken + Prozent; Button `compact` („Kompaktieren“, ruft `$.session.compact()`) immer sichtbar, ab `COMPACT_AT` (85 %) hervorgehoben und Zahl gelb
- je Rate-Limit-Fenster (`five_hour` → `5h`, `seven_day` → `7d`): Prozent, Farbe nach `limitColor`
- Cache: Restzeit oder „kalt“, dazu die Trefferquote des letzten Haupt-Requests
- Session-Kosten
- „Agents n“, nur bei laufenden Subagents
- Button `details` öffnet das Panel

Zeile 3, nur wenn Vorschläge vorliegen und kein Turn läuft:

- „Weiter:“ + bis zu 3 Buttons `step-0..2`; Klick schreibt `prompt` per `$.prompt.fill` ins Eingabefeld; erst Enter sendet ihn (Origin `composer`) und leert die Vorschläge

### Panel (`Pane`, `/cockpit` oder Button „Details“)

Legende mit Erklärung (Farben im Klartext; „Takt“ nur in der Erklärung: Verbrauch im Vergleich zur verstrichenen Zeit des 5h- bzw. 7d-Fensters, mit Rechenbeispiel), dann je Abschnitt eine Karte (Rahmen, Überschrift; „Tool-Freigaben“ mit Prüfungszahl, Kachelzeile der letzten 30 Entscheidungen und Aufschlüsselung Datei/Shell/Sonstige über `toolKind`): Übersicht, Letzter Turn, Tool-Freigaben, Subagents, Log (letzte 50 Einträge, sichtbar je nach Höhe).

Log im Flightdeck-Stil: letzte 10 Einträge, je `hh:mm`, Quelle (`you`, `main`, `gate`, `agent`) und Text. `you` = Prompt, einzeilig auf 80 Zeichen gekürzt; `main` = Tool-Call „Tool → Ziel“ (Pfade auf die letzten zwei Segmente, `✗` bei Fehler) und Turn-Ende; `gate` = abgelehnte Freigabe; `agent` = gestarteter Subagent.

Lade-Animation, solange ein Turn läuft: hervorgehobene Karte (Rahmen in Akzentfarbe) „Agent arbeitet · Dauer · Tool-Calls · Edits“ mit Spinner und letztem Log-Eintrag unter dem Titel und ein laufender Balken zwischen den Karten. Der Balken ist eine dünne Linie (Track `─` gedimmt, laufender Block `━` in Akzentfarbe) und füllt die Panelbreite (`bodyColumns`). Takt 250 ms über `$.clock.every`, nur während des Turns (Start bei `prompt.submit`, Stopp bei `turn.complete`), Bild im Atom `frame`.

Arbeitsverzeichnis: eigene Karte oben im Panel mit Branch, Worktree (Hauptcheckout oder verknüpft), Commit, Änderungen, Upstream, Ordner und ggf. Unterordner. Im Band steht davon nur der aktuelle Branch (`⎇ branch`) unter der Titelzeile.

Hover-Infos im Chat (Option `chatInfo`, Standard an): Tool-Zeilen, Tool-Ergebnisse und Command-Ausgaben bekommen oben rechts ein kleines ⓘ. Beim Überfahren erscheint ein Hinweis mit Art (Datei/Shell/Sonstige), Ziel, Status und Umfang einer Änderung. Antworten des Modells tragen keinen Hinweis (die frühere Zeile „Skinbar in dieser Nachricht“ ist seit 2026-10-12 entfernt). Technik: `ui.render` auf `ToolUse`, `ToolResult`, `CommandOutput` umhüllt das schon gezeichnete Element (`next(e)`) mit `position: absolute` und `hover.scope`. Das Skins-Mod bleibt als eigenes Plugin aktiv; tenvan-cc übernimmt keinen Code daraus.

Chat-Skins (Einstellungsgruppe „Chat-Skins“, nur Desktop, Logik in `hooks/skins.ts`): Tool-Symbole, ruhige Ergebnisse mit echter Fehlerzeile, Diff-Karte, Details aufklappen, Prompt-Rahmen, Code- und Tabellenkarten mit Kopier-Knopf, Mermaid-Karten (flowchart, sequenceDiagram; Logik in `hooks/mermaid.ts`), Formelkarten für `$$…$$`, Rückfrage-Kopf, Modus-Labels, Kennzeichen an der Hinweiszeile, Hervorhebungen (standardmäßig aus). Einzelheiten im Entscheidungslog.

Zusammenfassung (eigenes Panel rechts, `/cockpit-zusammenfassung` oder Knopf „Zusammenfassung“ in Band und Panel; Einstellung `cardCatchUp`; als einfacher Text gerendert, kein Markdown, damit nichts nachgeladen oder angeklickt wird): fünf Abschnitte (Warum diese Session, Was ich getan habe, Was dabei herauskam, Wo wir stehen, Wie es weitergeht) als kurze Stichpunkte, Sprache Deutsch (Standard) oder English. Aktualisiert nach jedem Turn mit Tool-Aufrufen (rollierend: vorige Zusammenfassung + neue Nachrichten + Git-Stand, ein Haiku-Call je ~100.000 Zeichen), Knöpfe „Aktualisieren“ und „Neu aufbauen“, Speicher je Session in `$.store` (100 neueste). Git-Stand ohne `status`/`diff` bei riskanter Repo-Konfiguration.

Einstellungsseite (`/cockpit-einstellungen` oder Knopf „⚙ Einstellungen“ in Band und Panel): Schalter für jedes Detail (Band: Ringe, Limits, Cache/Kosten, Kontext-Tooltip, Legende, Branch, Folge-Schritte; Panel: jede Karte und die Lade-Animation; Chat-Hover-Infos; automatische Zusammenfassung) und Sprache der Zusammenfassung. Gilt sofort, gespeichert in `$.store` für alle Sessions, „Alles einblenden“ setzt zurück. Die Plugin-Optionen `nextSteps` und `chatInfo` liefern nur noch den Startwert.

Subagents im Panel mit zweiter Zeile: Modell (resolved, bei Abweichung mit angefragtem), Hintergrund/Fork/Teammate, Rechte, Tokens ein/aus, Cache-Trefferquote, Dauer. Effort und Thinking liefert die Engine im Spawn-Event nicht.

Reset-Zeiten der Limits mit Wochentag („Di 21:00“, ab 7 Tagen Abstand mit Datum).

### Slash-Command

`/cockpit` öffnet das Panel. Kein Auto-Open bei Session-Start.

Kontextfenster: Karte im Panel mit allen Kategorien (Name, Token, Prozent, Balken; Einstellung `cardContext`); fehlen die Daten, nennt die Karte den Grund.

### Optionen (`userConfig`)

| Option | Typ | Standard | Wirkung |
| --- | --- | --- | --- |
| `nextSteps` | boolean | `true` | Folge-Schritte per Haiku-Call nach jeder Haupt-Antwort |

## Rechenregeln

| Größe | Regel | Ort |
| --- | --- | --- |
| Takt | `percentUsed − verstrichener Fensteranteil` in Prozentpunkten; nur für bekannte Fenster mit `resetsAt` | `core.ts` `pace` |
| Limit-Farbe | rot ab 90 %, gelb bei Takt > +10, sonst grün | `core.ts` `limitColor` |
| Cache-Restzeit | letzter Haupt-Request (`turn.step`, sonst `turn.complete`) + 60 min (Subscription, d. h. `rateLimits` nicht leer) bzw. + 5 min | `core.ts` `cacheLeftMs` |
| Cache-Trefferquote | `cache_read / (input + cache_read + cache_creation)` des letzten Haupt-Requests | `core.ts` `cacheHitPct` |
| Agent-Status | aus `agent.spawn` und `turn.complete` mit `agentId`; Abgleich über `$.agent.list()` nach jedem Haupt-Turn und im 30-s-Tick, solange Agents laufen | `register.tsx` `syncAgents` |
| Turn-Kosten | `cost.usd` nach dem Turn minus Wert bei `prompt.submit` | `register.tsx` `turn.complete` |
| Folge-Schritte | Haiku, `effort: 'low'`, 400 Tokens, 15 s; nur bei `reason: 'answer'` und nicht abgebrochen; Antwort wird verworfen, wenn inzwischen ein neuer Prompt kam | `register.tsx` `suggest` |

## Akzeptanzkriterien

Jedes Kriterium hat einen Test in `hooks/register.test.ts`, außer wo „manuell“ steht.

- [x] Takt- und Cache-Rechnung stimmen (Unit-Tests)
- [x] Band zeichnet auf `desktop`, auf `terminal` nichts Eigenes
- [x] Button „Details“ öffnet den Pane `tenvan-cc`
- [x] `tool.check` reicht ein `deny` von unten unverändert durch
- [x] Panel zeigt alle fünf Abschnitte inkl. Takt und Reset-Zeit
- [x] `parseSteps` liest JSON aus Freitext, max. 3, Label ≤ 30 Zeichen, Fallback `prompt = label`
- [x] Folge-Schritt-Button füllt das Eingabefeld; nach Enter verschwinden die Vorschläge
- [x] `nextSteps: false` löst keinen Model-Call aus
- [x] Abgebrochener Turn löst keinen Model-Call aus
- [x] Band zeigt „Kompaktieren“ immer (ab 85 % hervorgehoben) und ruft `session.compact`
- [x] Legende in Band und Panel, Abschnitte als Karten
- [x] Trefferquote aus `turn.step`-Usage erscheint im Panel
- [x] Agent-Status wird über `agent.list` abgeglichen
- [x] Titelleiste mit Version und Build-Stempel in Band und Panel
- [x] manuell: Band und Panel erscheinen in einer echten Desktop-Session (bestätigt 2026-10-12)
- [ ] manuell: Folge-Schritte erscheinen nach einer Antwort und senden korrekt
- [ ] manuell: Cache-Countdown und Agent-Laufzeiten aktualisieren sich ohne Turn (30-s-Tick)

## Backlog

Priorität von oben nach unten. Vor Umsetzung mit Ralf abstimmen, wenn ein Punkt eine Leitplanke berührt.

1. Weitere Community-Mods sichten und als Abschnitt integrieren (Ralf liefert Links)
2. manuell prüfen: Kompaktieren-Button in echter Session (Wirkung von `$.session.compact()` außerhalb eines Turns)

## Entscheidungslog

| Datum | Entscheidung |
| --- | --- |
| 2026-10-11 | Name `orga-mods`, Ort `plugins/orga-mods` (seit 2026-10-12 `tenvan-cc`), nur Desktop, Panel per Command + Button |
| 2026-10-11 | next-steps vereinfacht: Einzelklick, ein Model-Call, Option `nextSteps` |
| 2026-10-12 | v0.3.0: Titelleiste „Tenvan CC (Version, Build)“; Build = Datei-Änderungszeit, solange es keine Build-Pipeline gibt |
| 2026-10-12 | v0.2.0: Kompaktieren-Hinweis, Cache-Trefferquote über `turn.step`, Agent-Abgleich über `agent.list`, kein Vorschlag bei Abbruch |
| 2026-10-12 | Lokale Entwicklung über Junction `.claude/skills/tenvan-cc` → `plugins/tenvan-cc` im Hauptcheckout (projektlokal, kein globales `CLAUDE_CODE_PLUGIN_DIRS`) |
| 2026-10-12 | Hot-Reload über Junction im Watch-Ordner `~/.claude/dev-mods/<session>/cockpit`; `lastPrompt` als Atom (überlebt Reload), Band bricht um (`flexWrap`), Panel liest parallel und kürzt Zeilen |
| 2026-10-12 | Backlog „Cache-Miss → kalt“ gestrichen: `turn.step` setzt `lastTurnAt` bei jedem Haupt-Request, der Cache ist nach einem Miss neu aufgebaut; TTL-Aufschlüsselung (`ephemeral_1h/5m`) liefert die Engine nur im Subagent-Report, nicht in `turn.step` |
| 2026-10-12 | Kompaktieren-Button immer sichtbar (war unter 85 % unsichtbar); Legende und Karten-Layout nach Flightdeck-Vorbild; `plain` muss ein Literal sein, sonst überspringt die Engine den ganzen Render-Hook |
| 2026-10-12 | v0.3.1: Kompaktieren immer sichtbar, Legende, Karten-Layout, Freigaben-Karte mit Datei/Shell/Sonstige, `lastPrompt` als Atom |
| 2026-10-12 | Lokale Entwicklung jetzt global: Junction `~/.claude/skills/tenvan-cc` → `plugins/tenvan-cc`, Projekt-Junction entfernt (Doppelladung vermeiden); `$.state` überlebt Reloads, neue Felder deshalb mit Defaults auffüllen (`gateOf`) |
| 2026-10-12 | Log nach Flightdeck-Vorbild (Quelle, Prompt-Text, Tool-Ziel, 10 Einträge); Lade-Animation nur während eines Turns, damit kein Dauer-Redraw entsteht |
| 2026-10-12 | Ringe nur im Band (Idee aus dem Skins-Mod, eigene Umsetzung), Panel behält Fortschrittsbalken; Lade-Balken auf Panelbreite |
| 2026-10-12 | Kontext-Tooltip über Hover-Scope; offen: ob das Band das absolute Box über seine Region hinaus zeichnet (Clipping), sonst Aufschlüsselung als Karte im Panel |
| 2026-10-12 | Skins: nur Hover-Infos über dem laufenden Skins-Plugin (Variante A), kein Vendoring; Worktree-Infos: Band nur Branch, Panel eigene Karte |
| 2026-10-12 | Legende verständlicher: „im Takt“ → „im Plan“, „knapp“ → „fast leer“, Takt in Worten („60 Punkte schneller als die Zeit“); im Band Hover-Erklärung am Legendenzeichen ⓘ |
| 2026-10-12 | v0.4.0: Ringe und Kontext-Tooltip im Band, Lade-Animation, Log nach Flightdeck-Vorbild, Logo und Worktree-Infos, Hover-Infos im Chat, verständlichere Legende |
| 2026-10-12 | Sicherheit: `loadRepo` startet in fremden Repos keine Befehle der Repo-Konfiguration; `git status` entfällt bei Filter/fsmonitor/textconv/Alias-Schlüsseln in der lokalen Config (Anzeige „nicht geprüft“), alle git-Aufrufe mit `core.fsmonitor=false` und `--no-optional-locks` |
| 2026-10-12 | catch-me-up (MIT) als Zusammenfassung übernommen: eigener Code, deutsche Anweisungen, Sprache einstellbar; Einstellungsseite; Subagent-Details; Wochentag bei Reset-Zeiten |
| 2026-10-12 | Review-Funde behoben: Risiko-Prüfung für Git schlägt geschlossen fehl (nur Exit 1 = unbedenklich), prüft Includes, Worktree-Konfiguration, gpg, diff.external; `status`/`diff` mit `--ignore-submodules=all`, `git log --no-show-signature`; Zusammenfassung standardmäßig aus und nur mit sichtbarer Karte; Modell-Ausgabe ohne Bilder, HTML und Links (`safeMarkdown`); leere Antwort überschreibt nichts; Einstellungen speichern nur geänderte Schlüssel; Lade-Linie dünn (Variante A) |
| 2026-10-12 | v0.5.0: Einstellungsseite, Zusammenfassung nach catch-me-up (deutsch/englisch), Subagent-Details, Wochentag bei Reset-Zeiten, dünne Lade-Linie, Sicherheitsfixes aus dem Review |
| 2026-10-12 | Zusammenfassung in eigenes Panel verschoben und ohne Markdown-Rendering (Review: Umgehung des Sanitizers); Kontext-Tooltip im Band blieb leer (Clipping über die Bandregion) → kompakt im Band plus volle Karte „Kontextfenster“ im Panel |
| 2026-10-12 | Lade-Linie: Linienzeichen sind ca. 1,28 Zellen breit, deshalb läuft der Block über 78 % von `bodyColumns` (`GLYPH_FACTOR`), der Track ist länger und wird in einem Rahmen mit Höhe 1 und `overflow: hidden` abgeschnitten (kein Umbruch mehr); Karte „Kontextfenster“ nennt den Grund, wenn die Engine keine Aufteilung liefert |
| 2026-10-12 | Tooltips: ein Box mit `hover.scope` und `position: absolute` wurde auf dem Desktop nicht gezeichnet (weder am Kontext-Ring noch kompakt im Band) → Tooltips jetzt als `<title>` im interaktiven Svg an Kontext-, Limit- und Cache-Ring; die Legende behält ihren Hover-Hinweis |
| 2026-10-12 | v0.5.1: Zusammenfassung in eigenem Panel ohne Markdown, Tooltips als Svg-Titel an Kontext-, Limit- und Cache-Ring, Karte Kontextfenster, Lade-Linie ohne Umbruch |
| 2026-10-12 | Tooltips ohne interaktives Svg (flackerte): Ringe einfach, Hover über Ring oder Text blendet ein verstecktes Box im Band ein; Titel-Variante (`ringSvg` mit `tip`) bleibt ungenutzt als Reserve |
| 2026-10-12 | Tooltips: Hover-Blöcke im Band wuchsen im Fluss statt zu schweben → ⓘ-Symbole als kleine interaktive Svg (schwebender Tooltip, Flackern auf 16 px begrenzt); „Agent arbeitet“ als Karte mit Rahmen, Zähler und letztem Ereignis |
| 2026-10-12 | v0.5.2: ⓘ-Tooltips flackerten durch sekündlich wechselnde Svg-Quelle → Zeiten im Tooltip nur minutengenau; während der Agent arbeitet sind die Symbole nicht interaktiv (kein Tooltip, kein Frame-Neuaufbau) |
| 2026-10-12 | ⓘ-Symbole entfernt: Tooltips wieder am Ring selbst (interaktiv nur im Leerlauf) |
| 2026-10-12 | Kontext-Aufteilung absteigend sortiert (Puffer davor, Free space ganz unten); Karte „Kontextfenster“ im Panel mit gestapeltem Balken (`breakdownBarSvg`). Der Tooltip bleibt Text: ein `<title>` kann keine Grafik tragen |
| 2026-10-12 | Kontext-Tooltip grafisch: absolutes Box als Kind der Kontext-Karte (Hover-Gruppe `orga-ctx`), nach oben versetzt, weil absolute Boxen auf die Fläche des Bandes beschnitten werden (der frühere Versuch lag über dem Band und war deshalb unsichtbar); Titel, gestapelter Balken, Legende. Limit und Cache behalten den `<title>`-Tooltip |
| 2026-10-12 | Gate-Karte nach Flightdeck: Kachelzeile, erlaubt / von dir / offen / abgelehnt (Rückfrage gilt als entschieden, wenn ihr `tool.call` zurückkommt; `deny` = abgelehnt), Chips D/S/A, Rahmenfarbe nach Zustand; Abfragen ohne `tool_use_id` zählen nicht. Subagents als Kacheln (Nummer, Beschreibung, Typ · Modell, ctx/out/Schritte, Status mit Dauer). `/cockpit-demo` gibt `assets/skins-demo.md` als Markdown aus; Command-Ausgaben zeigen die Skinbar-Statistik im ⓘ |
| 2026-10-12 | Grafischer Kontext-Tooltip per absolutem Box im Band erscheint auf dem Desktop nicht (auch innerhalb der Bandfläche) → zurück zum `<title>`, Zeilen mit Blockzeichen-Balken. `/cockpit-demo`: Tabelle aller 15 `ui.render`-Komponenten mit Stand (`RENDER_COMPONENTS`, `demoTable`) plus vollständiger Markdown-Katalog |
| 2026-10-12 | Kontext-Tooltip farbig über Emoji-Quadrate (🟦🟥🟩…, je angefangene 10 %); Kategorie-Farben im Panel an die Emoji angeglichen, damit beide gleich aussehen. Subagent-Verlauf als Gantt (`timelineSvg`): Zeile je Agent, Balken auf gemeinsamer Achse vom ältesten Start bis jetzt |
| 2026-10-12 | v0.6.0: Gate-Karte und Subagent-Kacheln mit Verlauf nach Flightdeck, Tooltips am Ring (Kontext farbig per Emoji), sortierte Kontext-Aufteilung mit gestapeltem Balken im Panel, `/cockpit-demo`, Bildmarke mit farbiger Wortmarke |
| 2026-10-12 | Sprachumschaltung DE/EN für die ganze Oberfläche: alle sichtbaren Texte in `hooks/i18n.ts` (`de` ist Quelle, `en` muss per Typ dieselben Schlüssel haben, `t(lang, key, vars)` mit `{name}`-Platzhaltern), `CATCH_UI` aufgegangen; reine Funktionen in `core.ts` bekommen `lang`; Demo als `assets/skins-demo.{de,en}.md`. Log-Einträge, Kontext-Hinweis und Command-Beschreibungen behalten die Sprache beim Schreiben bzw. Session-Start. Einstellungsseite neu: Karte „Sprache · Language“ zuerst, je Gruppe eine Karte mit „n/m an“ und „Alle an/aus“, Zeilen mit Schalter-Button (Zustand im Label, der Desktop zeichnet keine Text-Kinder eines Buttons), fetter Beschriftung und Hinweis in eigener Zeile (schmale Panels); „Auf Standard zurücksetzen“ behält die Sprache |
| 2026-10-12 | Ring-Tooltips nur im Leerlauf: weder Turn noch laufender Subagent (interaktive Svg flackerten auch bei Neuzeichnen durch Agent-Updates) |
| 2026-10-12 | Hover-Infos im Chat waren nie sichtbar (absolute Boxen zeichnet der Desktop nicht) → ⓘ als kleines interaktives Svg mit `<title>` rechts neben dem Element, im Fluss. Modellzeile unter dem Titel (Panels) bzw. neben dem Branch (Band): Modell, Effort aus `turn.step`, Kontextfenster, Ausgabestil aus `prompt.compose`, Claude-Code-Version; Atom `modelInfo`. Mehr Abstand: Leerzeile nach dem Kopf, breitere Karten im Band |
| 2026-10-12 | Chat-Infos ohne Tooltip: statt ⓘ-Svg eine gedimmte, immer sichtbare Zeile „ⓘ …“ unter dem Element (`withInfo`), Einstellung heißt „Info-Zeilen“; `infoIconSvg` entfernt |
| 2026-10-12 | Info-Zeilen im Chat mit der Tenvan-Bildmarke (`assets/logo.svg`, 11×14 px, einfaches Bild) statt „ⓘ“; ohne Logo bleibt „ⓘ“ |
| 2026-10-12 | Kurze Legende im Band als ein Bild (`legendSvg`) mit der ganzen Legende als `<title>`-Tooltip; interaktiv nur im Leerlauf (kein Turn, kein laufender Subagent) wie die Ringe, Quelle ändert sich nur bei Sprachwechsel. Das absolute Hover-Box der Legende (nie gezeichnet) ist entfernt |
| 2026-10-12 | Legende-Karte im Panel und Einstellung `cardLegend` entfernt (Legende nur noch im Band-Tooltip); Tooltip mit farbigen Emoji-Kästchen 🟩🟨🟥🟦 passend zu den Farbmarken |
| 2026-10-12 | Folge-Schritte: genau 5 Vorschläge mit kurzer Begründung (`why`), je Zeile `plain`-Button mit `hotkey` 1–5 (wirkt auch als Ziffer im leeren Eingabefeld) und Begründung gedimmt daneben; maxTokens 1000 |
| 2026-10-12 | Chat-Skins (Ideen aus claude-skins und prismantis, MIT, eigener Code in `hooks/skins.ts`, siehe `THIRD_PARTY.md`): neue Einstellungsgruppe „Chat-Skins“ mit je einem Schalter; nur Desktop, andere Oberflächen behalten `next(e)`; alle Svg einfache Bilder mit stabiler Quelle, Knöpfe nur mit Label, keine absoluten Boxen; `withInfo` liegt weiter oben drauf |
| 2026-10-12 | Tool-Symbole (`skinIcons`): 16-px-Strich-Svg je Art (lesen, ändern, schreiben, Shell, Suche, Web, Agent, Sonstige) vor `ToolUse` und je Art vor zugeklappten `ToolGroup`; Farbe aus `THEME` (neu `violet`), rot bei Fehler |
| 2026-10-12 | Ruhige Ergebnisse (`skinQuiet`): `ToolResult` von Read/Grep/Glob und lesenden Shell-Befehlen (Positivliste, erkannt in `tool.call` vor dem Lauf, Atom `quietIds`) als eine Zeile; jeder Fehler als erste aussagekräftige Fehlerzeile statt „Exit code 1“ |
| 2026-10-12 | Diff-Karte (`skinDiff`): Edit/Write/MultiEdit aus `structuredPatch` als Svg-Karte mit Datei, +/−, Zeilennummern, getönten Zeilen, ab 40 Zeilen „… n weitere“; Quelle hängt nur vom Ergebnis ab (kein Flackern) |
| 2026-10-12 | Details aufklappen (`skinExpand`): `plain`-Button „▸/▾ Details“ unter `ToolUse`, Zustand im Atom `expanded` je `tool_use_id`; Eingabe und Ergebnis im nativen `Code` (Shell als Shell, Read mit Pfad, sonst JSON, ab 20.000 Zeichen gekürzt) |
| 2026-10-12 | Prompt-Rahmen (`skinPrompt`): nur getippte Prompts (`origin.kind` `composer`, ohne `from`/`task`) rechtsbündig im runden Rahmen in Akzentfarbe, Text als wörtliches `Markdown` (markierbar) |
| 2026-10-12 | Code- und Tabellenkarten (`skinCards`): `AssistantMessage` und `CommandOutput` werden nur selbst gezeichnet, wenn ein geschlossener Codeblock oder eine Tabelle vorkommt; Prosa bleibt natives `Markdown`, Code im `Code`-Element mit Kopf (Sprache, Zeilen), Tabellen als Markdown in der Karte, Kopier-Knopf darunter über `$.ui.copy`; Mermaid/Mathe und offene Fences (Streaming) bleiben Markdown |
| 2026-10-12 | Hervorhebungen (`skinInline`, standardmäßig aus, experimentell): GitHub-Alerts als farbige Karte, Aufgabenlisten mit Zähler, nur Absätze ganz ohne Markdown-Zeichen als farbiger Text (Pfade, in diesem Turn geänderte Dateien aus Atom `touched`, Versionen, Zahlen); alles andere bleibt Markdown |
| 2026-10-12 | Umbenannt in „Tenvan CC“: Plugin-ID `tenvan-cc` (vorher `orga-mods`), Ordner `plugins/tenvan-cc`, Junction `~/.claude/skills/tenvan-cc`, Commands `/cockpit`, `/cockpit-zusammenfassung`, `/cockpit-einstellungen`, `/cockpit-demo`, State- und Pane-IDs `tenvan-cc*`. Der Plugin-Store hängt an der ID; die alten Einstellungen werden einmalig von Hand übernommen |
| 2026-10-12 | Lade-Linien zwischen den Panel-Karten entfernt (Arbeitszustand zeigt die Karte „Agent arbeitet“), stattdessen eine Leerzeile; `sweep*`-Code gelöscht. Gate: je `tool_use_id` nur einmal gezählt (Mehrfachprüfungen blähten „von dir“/„offen“ auf), offene Rückfragen = Länge von `asked`, nach jedem Turn geleert |
| 2026-10-12 | Modellzeile nur noch in den Panels, nicht im Band. Folge-Schritte: Label erst beim Zeichnen gekürzt, nach Bandbreite (40 % von `bodyColumns`, mind. 24 Zeichen). Kennzeichen „hier greift ein Cockpit-Style“: kleine Tenvan-Bildmarke (9×11, einfaches Bild) an Karten, Diff-Karte, ruhigen Ergebnissen/Fehlerzeilen, Tool-Symbolen, Details-Knopf, Prompt-Rahmen und Arbeitszeile |
| 2026-10-12 | Folge-Schritte an Claude angeglichen: Claudes eigener Vorschlag (Event `prompt.suggest`, origin `suggestion`) steht wörtlich als Schritt 1 („Vorschlag von Claude“), Haiku-Duplikate fallen weg (`withClaude`); das Haiku-Prompt formuliert Schritte als knappen Nutzer-Prompt. Das Prompt hinter Claudes Vorschlag liefert die API nicht |
| 2026-10-12 | Style-Kennzeichen mit kurzem Style-Namen (gedimmt, kursiv, z. B. „Codekarte“, „Fehlerzeile“) neben der Tenvan-Marke; Symbol (`skinMarkIcon`) und Name (`skinMarkLabel`) getrennt in den Einstellungen der Gruppe Chat-Skins abschaltbar |
| 2026-10-12 | Karte „Hintergrundaufgaben“ (Einstellung `cardTasks`): Start, wenn ein Shell-Aufruf `backgroundTaskId` liefert (run_in_background oder Strg+B), Ende aus der `<task-notification>` im Prompt (origin `task-notification`, `parseTaskNotification`), `TaskStop` setzt „killed“; Liste mit Befehl und Kurztext, Verlauf wie bei den Subagents. Die API bietet keine Liste aller Hintergrundaufgaben; Aufgaben vor einem Reload fehlen |
| 2026-10-12 | Mermaid-Karten (`skinMermaid`): ```` ```mermaid ```` in `AssistantMessage`/`CommandOutput` als einfaches Svg in einer Karte „Mermaid · <Typ>“ mit Knopf „Quelle kopieren“ darunter; eigener Parser und eigenes Layout in `hooks/mermaid.ts` (flowchart/graph TD/LR/BT/RL mit Rang nach längstem Pfad und einer Baryzentrums-Runde, sequenceDiagram mit Lebenslinien, Notizen, Blöcken als Zeile). Andere Typen oder Parse-Fehler bleiben Markdown; nicht interaktiv (kein `<title>` je Knoten), Quelle nur aus dem Diagrammtext, keine Animation (Entrance-once erfüllt), Breite auf 720 px gekappt. Idee aus claude-skins, kein Code übernommen |
| 2026-10-12 | Formelkarten (`skinMath`): `$$…$$`-Blöcke und `math`/`latex`/`katex`-Fences als Karte; `texToText` setzt Symbole, `\frac`, `\sqrt`, `\text`, Hoch-/Tiefstellung in Unicode um und zeichnet sie als Svg-Text; bleibt ein unbekannter Befehl übrig, zeigt die Karte den LaTeX-Quelltext im `Code` („nicht umgesetzt“). Inline-`$…$` und offene `$$` (Streaming) bleiben Markdown |
| 2026-10-12 | Restliche Komponenten: `AskUserQuestion` bekommt nur ein Kopfband (Anzahl, Kurztitel, Kennzeichen „Rückfrage-Kopf“, `skinAsk`) über dem Dialog der Engine – die Engine verlangt genau einen eigenen Knoten im Baum, im Test-Host nicht montierbar; `SessionMode` zeichnet dieselben Labels in Akzentfarbe (`skinMode`); `PromptHint` behält die Engine-Zeile samt Pills und bekommt nur das Kennzeichen davor (`skinHint`). `ToolProgress` und `InfoNotice` löst die Engine nur im Terminal aus → nichts umgesetzt, `/cockpit-demo` zeigt „nur Terminal“ |
| 2026-10-12 | Panel-Karten mit ▾/▸ (zuklappen, nur Session, Atom `collapsed`) und ✕ (schaltet die Karte über `setSettings` ab, zurück nur über die Einstellungsseite) rechts im Kopf; auch die Gate-Karte, nicht „Agent arbeitet“. Die Zusammenfassung ist im Hauptpanel nur ein Knopf, deshalb ohne Kartenknöpfe |
| 2026-10-12 | Folge-Schritte: Claudes eigener Vorschlag (`fromClaude` am `Step`) mit Chip „CC“ (Chip-Stil wie Gate/Agents) vor dem Knopf; die Begründung „Vorschlag von Claude“ ist unser Text, die Engine liefert keine. Test löst `prompt.suggest` über den Test-`$` mit origin `suggestion` aus |
| 2026-10-12 | Band: Knopf „Skin-Demo“ neben Details/Zusammenfassung/Einstellungen, führt `/cockpit-demo` über `$.command.run` aus (wie getippt, läuft, sobald die Session frei ist) |
| 2026-10-12 | Tool-Namen lesbar: `mcp__server__tool` wird zu „server › tool“ (`splitTool`/`toolText`, Plugin-Präfix gekürzt), im Log und in „Zuletzt“ farbig (Server gedimmt, Tool in Akzentfarbe, Ziel nur mit „→“, wenn vorhanden, Fehler rot); Log-Einträge tragen dafür `tool`/`target`/`isError`. Auch in Info-Zeilen, ruhigen Ergebnissen und Gate-Ablehnungen |
| 2026-10-12 | v0.8.0: Claude-Vorschlag als Folge-Schritt 1 (CC), Style-Kennzeichen mit Namen, Karte Hintergrundaufgaben, Mermaid- und Formelkarten, Rückfrage-Kopf, Modus-Labels, Hinweiszeile, Panel-Karten ein-/ausklappbar und ausblendbar, Knopf Skin-Demo, lesbare Tool-Namen |
| 2026-10-12 | „Agent arbeitet“ fehlte nach einem Klick auf einen Folge-Schritt: `$.prompt.submit` aus dem eigenen Knopf durchläuft den eigenen `prompt.submit`-Hook nicht, der Turn wurde nie gestartet. Fix: `startTurn` (Zähler, Log, Spinner) gemeinsam für Hook und Knopf. Den Arbeitszustand aus `isWorking` des Bands zu übernehmen geht nicht: Render-Hooks dürfen keinen State schreiben |
| 2026-10-12 | Folge-Schritte per `$.prompt.fill` statt `$.prompt.submit`: ein Plugin-Prompt kommt mit Origin `plugin` an, effortless bewertet ihn dann nicht (Auto-Effort, Handoff-Prüfung). Enter der Person liefert Origin `composer`, `startTurn` läuft im eigenen `prompt.submit`-Hook. Kosten: ein Tastendruck mehr |
| 2026-10-12 | Panel-Karten: Abstand vor jeder Karte (auch „Agent arbeitet“, Arbeitsverzeichnis, Kontext) als halbe Zeile, 8-px-Svg-Spacer, da `Box` nur ganze Zeilen kennt; jede Karte trägt den Abstand selbst, abgeschaltete Karten lassen keine doppelte Lücke. Zugeklappte `ToolGroup` (`skinIcons`): in der Zählzeile je Tool (MCP: je Server) Symbol, Name und Zähler („Bash ×3“, höchstens 6); darunter nur MCP-Aufrufe einzeln als Server › Tool → Ziel (`toolTarget`), höchstens 8, Rest als „+n“ |
| 2026-10-12 | Arbeitshilfen: (1) Cache-Wächter im 30-s-Takt: ≤ 90 s vor Ablauf, im Leerlauf, einmal je Turn `$.prompt.suggest` mit kurzem Stand-Prompt; kalter Cache im Band als „kalt · n neu“ in Warnfarbe. (2) Kompaktieren rief `$.session.compact()` ohne sichtbare Wirkung → schreibt jetzt `/compact` mit Fokus (Ziel, geänderte Dateien, offene Schritte; `compactFocus`) ins Eingabefeld. (3) Kontext-Diät in der Kontextkarte: geladene MCP-Schemas nie aufgerufener Server ab 500 Tokens (`mcpDiet` aus `breakdown.mcpTools`), Knopf schreibt `/mcp`. (4) Schleifen-Warnung im Band: ≥ 3 Fehler desselben Aufrufs in den letzten 8 oder ≥ 5 Edits derselben Datei im Turn (`detectLoop`), Knopf „Anderen Ansatz“. (5) Turn-Bilanz: Balken je Turn (Kosten), gelb ab 0,25 $ ohne Edit. (7) Agent-Bremse: ab 40 Tool-Calls oder 10 min Knopf „Bremsen“; die API kann Subagents nicht abbrechen, darum lehnt `tool.call` deren weitere Aufrufe mit Aufforderung zum Abschluss ab. Schwellwerte als Konstanten in `core.ts` |
| 2026-10-12 | Knopf „Features“ hinter der Legende im Band (Legende und Knopf in eigener Zeilengruppe, sonst verteilt `space-between` sie): schreibt `/cockpit-features` per `$.prompt.fill` ins Eingabefeld (`$.command.run` aus dem Knopf kam im Desktop nicht an), Enter führt ihn aus; Übersicht in Schlagworten mit je einem Satz aus `assets/features.<lang>.md`. Verworfen: direkte Ausgabe per `$.session.append` (Typ `system` wird als `InfoNotice` gezeichnet, die der Desktop nicht zeigt; aus einem Knopf heraus hängt der Aufruf im Testhost) und die Einstellungstabelle im Bericht (zu lang) |
| 2026-10-12 | Erster Klick verpufft: jedes Neuzeichnen tauscht die nativen Buttons aus, auch den unter dem Zeiger (so auch effortless dokumentiert). Das Panel las das 250-ms-Animationsbild `frame` und wurde während eines Turns 4×/s neu gezeichnet → „Agent arbeitet“ im Panel mit festem ●, `frame` nur noch für die Arbeitszeile im Chat (ohne Buttons). Messwerte (`usage`, Breakdown, `mcpTools`, `modelInfo`, `cacheHit`) werden nur bei Änderung geschrieben (`same`, Vergleich am Aufrufort, weil der Modul-Scan nur Literal-Atome erlaubt) |
| 2026-10-12 | Restlicher Klickverlust nur in Panels: der erste Klick in ein Panel ohne Tastaturfokus holt den Fokus (`isFocused` wechselt), der Host zeichnet neu und tauscht den Button aus. Gegenmaßnahmen: Panel-Köpfe lesen `usage` nicht mehr (Kontextfenster steht in `modelInfo.window`), `repo` wird nur bei Änderung geschrieben, alle `$.ui.open` mit `focus: true` (wirkt nur bei leerem Eingabefeld mit Tastatur, nicht aus Knöpfen). Der Fokus-Wechsel selbst liegt beim Host. Skin-Demo-Knopf wie Features per `$.prompt.fill` |
