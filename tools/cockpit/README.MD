# Tenvan CC 🛩️ (`tenvan-cc`)

Universal-Mod für **Claude Desktop** und die **Claude-Code-CLI**: ein schmales Band über der Prompt-Eingabe und ein ausführliches Panel rechts. Eigene Implementierung, die den Funktionsumfang mehrerer Community-Mods bündelt und verschlankt:

- [scasella/claude-flightdeck](https://github.com/scasella/claude-flightdeck) – Panel mit Turn-Beleg, Gate, Agents, Log
- [augiefra/token-weather-usage](https://github.com/augiefra/claude-mods/tree/main/plugins/token-weather-usage) – Band mit Kontext, Limits, Cache, Kosten
- [pawandeepdhall/next-steps](https://github.com/pawandeepdhall/claude-mods/tree/main/plugins/next-steps) – Folge-Schritte als Buttons
- [Antreas-Strb/glanceflow](https://github.com/Antreas-Strb/glanceflow) – Checkliste mit Status, Fortschritt und Verlauf
- [YeonwooSung/my-claude-code-mods](https://github.com/YeonwooSung/my-claude-code-mods) – Diagnose: Kontext-Inspektor, Agent-Profiler, Blast-Radius

Kein Code aus diesen Projekten wurde übernommen. Kein Netzwerk, keine Telemetrie, kein Lesen von Transcripts – alle Werte kommen aus den Mod-Events der Engine und `$.session.usage()`. Einzige Ausnahme sind die Folge-Schritte: Sie kosten einen kurzen Haiku-Call pro Antwort über die eigene Claude-Anbindung.

## Anzeige

**Band über der Prompt-Eingabe**

| Teil | Inhalt |
| --- | --- |
| Checkliste | Ganz oben im Band: Claudes Aufgabenliste mit Status, Fortschritt, Restzeit und Knöpfen (siehe Abschnitt „Checkliste“) |
| Titelleiste | `Tenvan CC (Version x, Build: Datum Uhrzeit)`, auch erste Zeile im Panel |
| Features | Knopf hinter der Legende: gibt alle Funktionen mit aktuellem Einstellungsstand als Bericht im Chat aus (`/cockpit-features`) |
| Kontext | Füllstand des Kontextfensters in Prozent; ab 85 % gelb mit Button „Kompaktieren“ |
| `5h` / `7d` | Verbrauch der Rate-Limit-Fenster; gelb, wenn mehr als 10 Punkte vor dem Takt, rot ab 90 % |
| Cache | Restzeit des Prompt-Caches seit dem letzten Request (Schätzung: 1 h mit Subscription, sonst 5 min) und Trefferquote des letzten Requests |
| Kosten | Bisherige Session-Kosten |
| Agents | Anzahl laufender Subagents, nur wenn > 0 |
| Details | Öffnet das Panel |
| Weiter: … | Nach jeder Antwort bis zu 3 Folge-Schritte als Buttons; ein Klick schreibt den Schritt ins Eingabefeld, Enter sendet ihn |

**Panel rechts** (`/cockpit` oder „Details“)

- Übersicht: Kontext in Tokens, Limits mit Takt und Reset-Uhrzeit, Kosten, Prompt-Cache
- Letzter Turn: Dauer, Tool-Calls, Edits, Fehler, Kosten-Zuwachs
- Tool-Freigaben: erlaubt / gefragt / abgelehnt
- Subagents: Typ, Aufgabe, Status, Laufzeit
- Log: die letzten Ereignisse der Session

**Sitzungsleiste der Desktop-App** (Einstellungen → „Sitzungsleiste (Desktop)“, alle Schalter standardmäßig aus)

| Schalter | Wirkung |
| --- | --- |
| Gemergt → erledigt | Ist der MR des Branches gemergt, wird der Punkt der Session quittiert („Als erledigt markieren“) |
| Rot oder Konflikt → ungelesen | Pipeline rot oder neuer Konflikt, während die Session ruht: blauer Punkt |
| Offener MR → angepinnt | Session steht unter „Angepinnt“, solange der MR offen ist; danach löst das Cockpit nur die eigene Nadel |

„Takt“ = Verbrauch minus verstrichener Anteil des Fensters, in Prozentpunkten. `+12` heißt: 12 Punkte schneller als gleichmäßig.

## Checkliste

Oben im Band steht Claudes Aufgabenliste als Checkliste. Die gleichbleibenden Cockpit-Zeilen bleiben direkt über der Prompt-Eingabe, die Checkliste wächst nach oben. Sie liest nur mit: Das Cockpit schiebt Claude keine Anweisungen unter, legt keine eigenen Werkzeuge an und sperrt nichts.

| Teil | Inhalt |
| --- | --- |
| Schritte | Nur was Claude selbst als Aufgabenliste anlegt (`TodoWrite`, Aufgabenliste der Desktop-App, `TaskCreate`/`TaskUpdate`). Legt Claude keine an, steht dort „ohne Aufgabenliste“. Ab 5 Schritten faltet das Band erledigte und spätere Schritte in je eine Zeile |
| Status | Arbeitet · Braucht dich (Frage in Claudes Antwort, Abschlussfrage der Desktop-App oder Freigabe, die auf dich wartet) · Hängt? (3 Minuten ohne Lebenszeichen) · Pausiert · Gestoppt · Fertig |
| Fortschritt, Restzeit | „n von m erledigt“ und „~Dauer übrig“, gerechnet aus den eigenen erledigten Schritten, sonst aus dem Verlauf des Projekts; „dauert länger“, wenn ein Schritt doppelt so lange läuft wie üblich |
| Jetzt | Was Claude gerade tut (Dateien lesen, suchen, ändern, Befehle, Web, Helfer, Planen, Frage an dich); die laufenden Helfer stehen in derselben Zeile, gruppiert als `↳ 3× planner · Opus 5.5` |
| ‖ Pause / ▶ Weiter | Pause stoppt den laufenden Turn; Weiter schickt Claude den Auftrag mit den noch offenen Schritten erneut. Geht das Senden nicht, steht der Auftrag im Eingabefeld |
| ≣ Verlauf | `/cockpit-verlauf`: Aufgaben je Tag (Ergebnis, Schritte, Dauer) mit Vortag/Folgetag und zwei Knöpfen, die einen Bericht in die Zwischenablage kopieren: **Tagesbericht** (dieses Projekt: erledigt, offen, braucht eine Entscheidung, Zeit) und **Wochenbericht** (alle Projekte, sieben Tage) |

Einstellungen (Gruppe „Checkliste“ unter `/cockpit-einstellungen`):

| Schalter | Standard | Wirkung |
| --- | --- | --- |
| Checkliste im Band | an | Zeigt die Checkliste oben im Band |
| Plan-Karte im Panel | an | Karte „Plan“ mit allen Schritten |
| Verlauf speichern | an | Jede Aufgabe mit Ergebnis, Schritten, Dauer und Dateinamen 30 Tage lokal speichern; Grundlage der Berichte |

Im Verlauf stehen Aufgabentitel (die ersten 70 Zeichen deines Prompts), Schrittnamen, die letzte Rückfrage und Dateinamen, nur lokal im Speicher des Mods. Die Berichte enthalten weder Tokens noch Modelle noch Dateinamen.

## Diagnose

Drei Kurzinfos im Band, jede ein Knopf, der das Panel „Profiling“ öffnet (`/cockpit-profiling`): ein Panel im Schema des Cockpit-Panels mit einer einklappbaren Karte je Mod. Alle lesen nur mit (kein `deny`, kein Rewrite, kein Netzwerk) und sind einzeln abschaltbar (Gruppe „Diagnose“ unter `/cockpit-einstellungen`).

| Mod | Band | Bericht im Chat | Inhalt |
| --- | --- | --- | --- |
| Kontext-Inspektor | `Kontext 62 % · Verschwendung ≈8,1k` | `/cockpit-kontext text` | Belegung des Fensters nach Kategorie und Ziel, erneut gelesene Dateien (auch „unverändert, noch im Fenster“ = reine Verschwendung), ähnliche Ausgaben (Hash und SimHash), was Kompaktieren behalten und verloren hat, Zeilenänderungen gegen Kontextwachstum, Hinweise mit geschätzter Ersparnis. Tokenwerte sind Schätzungen (`≈`), kalibriert am gemessenen Input |
| Agent-Profiler | `Modell 61 % · Tools 28 % · läuft: Bash npm test 14s` | `/cockpit-profil text` oder `turn` | Zeitverteilung Modell (erster Chunk, Thinking, Ausgabe), Tools (Ausführung gemessen, Wartezeit abgeleitet, Freigabewartezeit), Subagents und Engine-Overhead; langsamste Tools, Testläufe, wiederholte Reads. Jeder Wert trägt seinen Beobachtungsgrad |
| Blast-Radius | `Blast: 5 betroffen · 2 Tests` | `/cockpit-blast [watch <Pfad> \| unwatch <Pfad> \| <Pfad> \| reset]` | Rückwärts-Importe geänderter (git status) und beobachteter Dateien, Testkandidaten, unaufgelöste Aliase. Nur statische relative JS/TS-Importe; höchstens 2.000 Dateien und 2 MiB, Listen auf 200 Einträge gekappt, sonst „unvollständig“ |

Grenzen: Der Zustand liegt im Speicher und beginnt nach einem Reload von vorn (beobachtete Blast-Pfade bleiben im `$.store`). Der Profiler exportiert bewusst keine OTLP-Traces (kein Netzwerk). Blast-Radius ruft einmal je Turn `git` lesend auf und bleibt in Repos mit riskanter Git-Konfiguration gesperrt, bis das Repo im Git-Teil des Panels entsperrt ist.

## Grenzen

- Zeichnet auf Desktop und im Terminal. Im Terminal gibt es kein Svg: Ringe werden Textbalken, Turn-Bilanz Blockzeichen, Formeln Unicode-Text; Tooltips, Atemkreis, Logo, Tool-Symbole und Diff-Karte entfallen dort (Mermaid und Diffs zeichnet die Engine).
- Die Cache-Restzeit ist geschätzt; ob der Cache wirklich traf, zeigt erst die Trefferquote des nächsten Requests.
- Ohne Subscription liefert die Engine keine Rate-Limits – der Limit-Teil entfällt dann.
- Tool-Freigaben werden nur gezählt, das Mod lehnt selbst nie etwas ab.
- Die Sitzungsleiste gibt es nur in der Desktop-App; im Terminal bleiben die Schalter wirkungslos. Sie folgt dem MR-Abruf (jede Minute, bei laufender Pipeline alle 15 s). Im Auto-Modus lehnt der Classifier die Aufrufe ab (das Log nennt den Grund einmal je Session).
- Folge-Schritte sind Einzelklicks (keine Mehrfachauswahl) und lassen sich abschalten.
- Die Checkliste kennt nur Schritte, die Claude selbst anlegt, und erfindet keine. Die Restzeit ist eine Schätzung aus der bisherigen Schrittdauer; ohne mindestens zwei erledigte Schritte oder drei Aufgaben im Verlauf des Projekts steht keine. „Hängt?“ heißt nur: seit 3 Minuten kein Tool-Aufruf und kein Request. Pause stoppt den Turn wie Esc, Weiter setzt mit einem neuen Auftrag fort (kein Weiterlaufen an derselben Stelle).

## Optionen

| Option | Standard | Wirkung |
| --- | --- | --- |
| `nextSteps` | `true` | Folge-Schritte nach jeder Antwort vorschlagen (ein Haiku-Call pro Antwort) |

Gesetzt über `/config` oder in den Settings unter `pluginConfigs.tenvan-cc`.

## Installation

Im Terminal (der Desktop-Code-Tab kennt `/plugin` nicht, lädt das Mod aber mit):

```shell
claude --plugin-dir tools/cockpit
```

## Entwicklung

Spezifikation, Backlog und Entscheidungen: [SPEC.md](SPEC.md). Arbeitsanleitung für Agenten (Prüfbefehle, API-Fallstricke, Test-Muster): [DEVELOPMENT.md](DEVELOPMENT.md).

Mod-API: Claude Code Function Hooks (Engine ≥ 2.1.295, Early Access). Struktur:

| Datei | Zweck |
| --- | --- |
| `hooks/register.tsx` | Hooks, Band und Panel |
| `hooks/core.ts` | Reine Rechen- und Formatfunktionen |
| `types/index.d.ts` | `$.state`-Vertrag |
| `hooks/register.test.ts` | Tests |

```shell
claude plugin validate tools/cockpit
claude plugin test tools/cockpit
```

Live ausprobieren ohne Installation: `claude --plugin-dir tools/cockpit`, oder den Pfad in `CLAUDE_CODE_PLUGIN_DIRS` (env-Block der `~/.claude/settings.json`) eintragen, damit die Desktop-App das Mod lädt.

Weitere Community-Mods werden als zusätzliche Abschnitte in Band oder Panel ergänzt, nicht als eigene Plugins.
