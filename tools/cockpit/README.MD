# Tenvan CC 🛩️ (`tenvan-cc`)

Universal-Mod für **Claude Desktop** und die **Claude-Code-CLI**: ein schmales Band über der Prompt-Eingabe (nur Links, keine Knöpfe) und **ein** Panel rechts mit Reitern. Eigene Implementierung, die den Funktionsumfang mehrerer Community-Mods bündelt und verschlankt:

- [scasella/claude-flightdeck](https://github.com/scasella/claude-flightdeck) – Panel mit Turn-Beleg, Gate, Agents, Log
- [augiefra/token-weather-usage](https://github.com/augiefra/claude-mods/tree/main/plugins/token-weather-usage) – Band mit Kontext, Limits, Cache, Kosten
- [pawandeepdhall/next-steps](https://github.com/pawandeepdhall/claude-mods/tree/main/plugins/next-steps) – Folge-Schritte als Buttons
- [Antreas-Strb/glanceflow](https://github.com/Antreas-Strb/glanceflow) – Checkliste mit Status, Fortschritt und Verlauf
- [YeonwooSung/my-claude-code-mods](https://github.com/YeonwooSung/my-claude-code-mods) – Diagnose: Kontext-Inspektor, Agent-Profiler, Blast-Radius

Kein Code aus diesen Projekten wurde übernommen. Kein Netzwerk, keine Telemetrie, kein Lesen von Transcripts – alle Werte kommen aus den Mod-Events der Engine und `$.session.usage()`. Einzige Ausnahme sind die Folge-Schritte: Sie kosten einen kurzen Haiku-Call pro Antwort über die eigene Claude-Anbindung.

## Anzeige

Ein Cockpit aus drei Teilen: Band über der Eingabe, ein Einstieg in der Fußzeile, ein Panel mit Reitern. Jede Information steht an genau einer Stelle; das Band zeigt nur, was jetzt zählt, und verweist per Link auf den passenden Reiter.

**Band über der Prompt-Eingabe** (2–4 Zeilen, alle Aktionen als Links; Gelb und Rot als getönter Grund aus dem Theme, hell und dunkel)

| Zeile | Inhalt |
| --- | --- |
| Checkliste | Nur wenn Claude an einer Aufgabe arbeitet: Status, Titel, Fortschritt, Restzeit, Links „Pause“, „Weiter“, „Plan“; laufender Schritt mit „Jetzt: …“ und Helfern. Solange Claude arbeitet, atmet der Statusring (Atemübung). Fragt Claude zurück, stehen die Antworten (Tasten 4–6) direkt unter der Frage |
| Kopf und Messwerte | Logo, Branch, MR des Branches (Link auf den Reiter Git) · `5h`/`7d` mit Reset · ◆ Effort · Modell mit „Auto“ · Kontext mit „Kompaktieren“ (ab 85 % gelb und fett) bzw. „✦ Handoff“ · Cache mit Trefferquote · Kosten. Die Werte sind Links auf den Reiter Verbrauch; die Zeile bricht in Gruppen um |
| Meldung | Nur bei Gelb oder Rot: die wichtigste Meldung aus Verzeichniswechsel, Schleife, Fehler-Eingang, Nutzungswächter, Tipps, Kontext-Inspektor, Profiler, Blast-Radius, dahinter „+n Hinweise“ zum Reiter Diagnose |
| Weiter | Hellgelb hinterlegt: nach jeder Antwort bis zu 3 Folge-Schritte (Tasten 1–3) und ein reiner ↻-Knopf für neue Vorschläge; Knopftexte höchstens 28 Zeichen (voller Text im Tooltip), die Zeile bricht nie um; ein Klick schreibt den Schritt ins Eingabefeld, Enter sendet ihn |

Bis auf Titel und Ordner/Branch ist jede Anzeige im Band abschaltbar (Gruppe „Band“ in den Einstellungen): Limits, Effort und Modell, Kontext, Cache, Kosten, laufende Agents, Meldungszeile, Folge-Schritte, Ringe; Checkliste und MR in ihren eigenen Gruppen.

**Fußzeile unter dem Prompt:** „Tenvan“ öffnet das Panel (einziger Einstieg neben `/cockpit`).

**Panel rechts** (`/cockpit`, „Tenvan“ oder ein Link im Band): ein Panel, acht Reiter, je Reiter höchstens vier Karten

| Reiter | Inhalt |
| --- | --- |
| Jetzt | Arbeitsverzeichnis mit verknüpften Sessions (ganz oben), Agent arbeitet (mit Atemübung), Plan mit Zeiten und Dateien, Subagents mit Bremse und Zeitachse (↗ öffnet den Verlauf: im Terminal in der Hauptansicht, auf dem Desktop als Karte im Panel), Hintergrundaufgaben, Letzter Turn mit Turn-Bilanz |
| Sessions | Sessions-Übersicht (ID, Repo, Branch, Stand, wartet auf, Notiz; wer auf dich wartet, steht oben und ist hervorgehoben) und Freigabe-Inbox (ID, Aktion, Detail, seit) aus der Status-Datei der Schaltzentrale (`sessions.json`, nur gelesen; Standardpfad `<Benutzerordner>/.deltra/claude/sessions.json`, änderbar in der Mod-Einstellung „Status-Datei der Sessions“). Aktualisiert sich bei Änderung der Datei, meldet fehlende oder kaputte Dateien im Klartext |
| Verbrauch | Übersicht (Kontext, Limits mit Takt und Reset, Kosten, Cache), Kontextfenster mit Kontext-Diät, Kontext-Inspektor |
| Diagnose | Unterreiter **Hinweise** (alle Meldungen mit Ampel, Tipps, Nutzungswächter), **Protokoll** (Log, Fehler-Eingang, Tool-Freigaben und Zeitreise in einer Liste, Filter Alle · Fehler · Freigaben · Snapshots), **Profil**, **Blast-Radius** |
| Git | Repo-Label mit Ordner, auf den sich der Stand bezieht, Ablauf Commit → MR → Merge → Aufräumen, Branches, MR-Karten |
| CI/CD | GitLab-Pipelines des Projekts (Status, Branch, Commit, Alter, Dauer) und die Jobs der gewählten Pipeline je Stage; Steuerung (Job wiederholen/abbrechen/starten, Log ansehen, Pipeline neu/abbrechen, neue Pipeline für den Branch), siehe Abschnitt CI/CD |
| Rückblick | **Diese Session** (Zusammenfassung) und **Verlauf** (Aufgaben je Tag, Tages- und Wochenbericht) |
| ⚙ Einstellungen | alle Schalter, Funktionsübersicht im Chat, Skin-Demo, Version |

Die Befehle bleiben und öffnen den passenden Reiter: `/cockpit-zusammenfassung`, `/cockpit-verlauf`, `/cockpit-einstellungen`, `/cockpit-git`, `/cockpit-sessions`, `/cockpit-ci`, `/cockpit-profiling`, `/cockpit-kontext`, `/cockpit-profil`, `/cockpit-fehler`, `/cockpit-zeitreise`.

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
| ‖ Pause / ▶ Weiter / ▤ Plan | Links: Pause stoppt den laufenden Turn; Weiter schickt Claude den Auftrag mit den noch offenen Schritten erneut (geht das Senden nicht, steht der Auftrag im Eingabefeld); Plan öffnet den Reiter Jetzt |
| ≣ Verlauf | Link in der Plan-Karte bzw. `/cockpit-verlauf` (Reiter Rückblick → Verlauf): Aufgaben je Tag (Ergebnis, Schritte, Dauer, ⧉ kopiert den Prompt) mit Vortag/Folgetag und zwei Knöpfen, die einen Bericht in die Zwischenablage kopieren: **Tagesbericht** (dieses Projekt: erledigt, offen, braucht eine Entscheidung, Zeit) und **Wochenbericht** (alle Projekte, sieben Tage) |

Einstellungen (Gruppe „Checkliste“ unter `/cockpit-einstellungen`):

| Schalter | Standard | Wirkung |
| --- | --- | --- |
| Checkliste im Band | an | Zeigt die Checkliste oben im Band |
| Plan-Karte im Panel | an | Karte „Plan“ mit allen Schritten |
| Verlauf speichern | an | Jede Aufgabe mit Ergebnis, Schritten, Dauer und Dateinamen 30 Tage lokal speichern; Grundlage der Berichte |

Im Verlauf stehen Aufgabentitel (die ersten 70 Zeichen deines Prompts), der getippte Prompt für den Kopier-Knopf (höchstens 2.000 Zeichen), Schrittnamen, die letzte Rückfrage und Dateinamen, nur lokal im Speicher des Mods. Die Berichte enthalten weder Tokens noch Modelle noch Dateinamen.

## Diagnose

Reiter Diagnose (`/cockpit-profiling`) mit den Unterreitern Hinweise, Protokoll, Profil und Blast-Radius; der Kontext-Inspektor steht im Reiter Verbrauch. Die Kurzinfos (Spalte „Band“) erscheinen im Band nur als Meldung, wenn ihre Ampel gelb oder rot ist, sonst unter Diagnose → Hinweise. Alle lesen nur mit (kein `deny`, kein Rewrite, kein Netzwerk) und sind einzeln abschaltbar (Gruppe „Diagnose“ unter `/cockpit-einstellungen`).

| Mod | Band | Bericht im Chat | Inhalt |
| --- | --- | --- | --- |
| Kontext-Inspektor | `Kontext 62 % · Verschwendung ≈8,1k` | `/cockpit-kontext text` | Belegung des Fensters nach Kategorie und Ziel, erneut gelesene Dateien (auch „unverändert, noch im Fenster“ = reine Verschwendung), ähnliche Ausgaben (Hash und SimHash), was Kompaktieren behalten und verloren hat, Zeilenänderungen gegen Kontextwachstum, Hinweise mit geschätzter Ersparnis. Tokenwerte sind Schätzungen (`≈`), kalibriert am gemessenen Input |
| Agent-Profiler | `Modell 61 % · Tools 28 % · läuft: Bash npm test 14s` | `/cockpit-profil text` oder `turn` | Zeitverteilung Modell (erster Chunk, Thinking, Ausgabe), Tools (Ausführung gemessen, Wartezeit abgeleitet, Freigabewartezeit), Subagents und Engine-Overhead; langsamste Tools, Testläufe, wiederholte Reads. Jeder Wert trägt seinen Beobachtungsgrad |
| Blast-Radius | `Blast: 5 betroffen · 2 Tests` | `/cockpit-blast [watch <Pfad> \| unwatch <Pfad> \| <Pfad> \| reset]` | Rückwärts-Importe geänderter (git status) und beobachteter Dateien, Testkandidaten, unaufgelöste Aliase. Nur statische relative JS/TS-Importe; höchstens 2.000 Dateien und 2 MiB, Listen auf 200 Einträge gekappt, sonst „unvollständig“ |

Vier weitere Teilmods (Ideen aus den Community-Mods Error Inbox, agent-usage-guard, Dividend und Agent Time Machine): Tipps und Wächter als Karten unter Hinweise, Fehler-Eingang und Zeitreise im Protokoll (Filter Fehler bzw. Snapshots, mit Diff und Zurück am Eintrag):

| Teilmod | Band | Bericht im Chat | Inhalt |
| --- | --- | --- | --- |
| Fehler-Eingang | `Fehler: 3 in 10 min` | `/cockpit-fehler text` | Die letzten 30 fehlgeschlagenen oder abgelehnten Tool-Aufrufe mit Uhrzeit, Tool, Ziel und erster Fehlerzeile; derselbe Fehler direkt hintereinander zählt als `×n`. Auch Subagent-Fehler (markiert) und vom Wächter abgelehnte Agents |
| Nutzungswächter | `Wächter: 9 Agents` | – | Warnt (Band, Hinweis, Log) bei **8+ Subagents gleichzeitig**, **10+ neuen Subagents je Minute**, **Prompt ab 85 % Kontext** und **demselben Aufruf 3× hintereinander**; jede Regel höchstens alle 5 Minuten. Mit der Einstellung „Wächter hält Agents an“ (Standard aus) lehnt er den Start eines Subagents ab, wenn schon 12 laufen. Nur das ist ein Eingriff, alles andere warnt |
| Tipps | `Tipp: Cache ~8 min` | – | Ein Tipp im Band, alle im Panel: Cache neu geschrieben (ab 40k Tokens in einer Antwort), Cache abgelaufen, „jetzt kompaktieren“ in den letzten 10 Minuten des 1-h-Caches ab 80k Tokens, Kontext ab 60 %/85 %, Limit ab 80 %/95 % oder zu schnell. Jeder Tipp nennt die Zahl dahinter. Nur lesend |
| Zeitreise | – | `/cockpit-zeitreise text` | Zeitleiste der letzten 30 Tool-Schritte; nach Datei-ändernden Aufrufen (Edit, Write, Shell, die nicht erkennbar nur liest) ein Snapshot des Arbeitsbaums. **Diff** zeigt die Dateien, die sich seit dem Schritt geändert haben, **Zurück** (zweistufig) setzt geänderte und gelöschte Dateien auf den Stand des Schritts, mit Sicherung davor, und schreibt Claude einen Hinweis ins Eingabefeld (Lesezugriffe sind veraltet). Seit damals neue Dateien bleiben liegen |

Die Zeitreise legt Snapshots als git-Tree-Objekte über einen eigenen Index (`<git-dir>/cockpit-tm.index`) an: kein Branch, kein Commit, Index und Arbeitsbaum des Nutzers bleiben unberührt. Das gilt nur in Repos ohne riskante Konfiguration (dieselbe Sperrliste wie im Git-Panel, geprüft direkt vor jedem Snapshot und jedem Zurücksetzen); dabei laufen nie Filter: alle konfigurierten Filter werden per `-c` geleert, Attribute kommen aus dem leeren Baum (`--attr-source`, git ≥ 2.40) und ist über „Zeitreise: Snapshots anlegen“ abschaltbar. Die Objekte räumt `git gc` nach der üblichen Frist ab.

Grenzen: Der Zustand liegt im Speicher und beginnt nach einem Reload von vorn (beobachtete Blast-Pfade bleiben im `$.store`). Der Profiler exportiert bewusst keine OTLP-Traces (kein Netzwerk). Blast-Radius ruft einmal je Turn `git` lesend auf und bleibt in Repos mit riskanter Git-Konfiguration gesperrt, bis das Repo im Git-Teil des Panels entsperrt ist.

## Git-Panel

Reiter Git des Cockpit-Panels (`/cockpit-git` oder der MR-Link im Band). Führt durch den Ablauf **Commit → MR → Merge → Aufräumen**: Eine Leiste oben zeigt, wo der Branch steht, und die Status-Karte bietet genau die nächste Aktion an.

| Karte | Inhalt |
| --- | --- |
| Status | Lage in einer Zeile, Upstream, MR des Branches, Hauptaktion: Branch anlegen und Änderungen mitnehmen (auf main), Committen, Pull, Erst syncen (Eltern-Branch weitergelaufen), MR entwerfen, Push, Mergen, Aufräumen (MR gemergt), Konflikte lösen |
| Fehler | Ausgabe des letzten gescheiterten Befehls (z. B. pre-commit-Hook, abgelehnter Push), ✦ „Mit Claude lösen“, Kopieren |
| Konflikte | Dateien eines angehaltenen Merge/Rebase, ✦ lösen lassen, Weiter, Abbrechen (mit Rückfrage) |
| Aufräumen | Die Schritte nach dem Merge: fetch --prune, auf den Eltern-Branch wechseln, Kinder in Git Town umhängen, Branch löschen |
| Änderungen | Gestaged, geändert, neu; je Datei stagen/entstagen, Alle stagen, Stash, Verwerfen (mit Rückfrage), ✦ Review |
| Commit | Nachricht als Eingabefeld, vorgefüllt von Haiku im Stil der letzten Commits (abschaltbar), Amend, ✦ Mit Claude committen, ✦ Aufteilen |
| Eingehend | Commits vom Upstream, ✦ „Was kommt rein?“ |
| Sync | Pull (wahlweise Rebase), Push, Force-with-lease nur bei Divergenz und mit Rückfrage, ✦ MR mit Claude |
| Merge Request | MR des Branches mit Pipeline, Freigaben, Beschreibung, offenen Threads und Aktionen (Mergen, Rebase, Squash, Anmerkungen an Claude), Entwurf zum Anlegen, deine offenen MRs und Reviews (früher eigenes MR-Panel) |
| Branches | Baum der lokalen Branches nur mit Abhängigkeiten (Eltern aus Git Town, sonst geschätzt), ↑↓ zum Eltern-Branch. Klick zeigt Optionen: Wechseln, Rebase darauf, Mergen, Pushen, Als Eltern, Löschen, ✦ Vergleichen. main, master und develop werden nie gelöscht |
| Verlauf | Letzte fünf Commits, ungepushte mit ↑; „Letzten Commit zurück“ (soft, mit Rückfrage) |

✦-Knöpfe schreiben einen Auftrag ins Eingabefeld, gesendet wird erst mit Enter. git läuft ohne Rückfragen (kein Passwort-Dialog, kein Editor) und ohne Repo-Hooks; hat das Repo `pre-commit` oder `commit-msg`, schlägt das Panel den Commit über Claude vor, direkte Knöpfe tragen „(ohne Hooks)“. Netzwerk nur auf Knopfdruck (Fetch, Pull, Push, Aufräumen). In Repos mit riskanter Git-Konfiguration bleibt das Panel gesperrt, bis das Repo entsperrt ist. Schalter unter „Git“ in `/cockpit-einstellungen`: Git-Panel, Commit-Nachricht vorschlagen. Knöpfe nach Priorität: die Hauptaktion in voller Akzentfarbe, gewöhnliche gedämpft, Nebensachen ohne Fläche.

## CI/CD

Reiter CI/CD (`/cockpit-ci` oder der Link „CI/CD“ in der Karte Merge Request des Git-Reiters). Nur für GitLab-Remotes (z. B. `gitlab.deltra.de`); bei anderen Plattformen sagt der Reiter das und bleibt leer.

| Karte | Inhalt |
| --- | --- |
| Pipelines | Neueste 15 Pipelines des Projekts: Nummer (Klick wählt sie), Status-Badge, Branch und Kurz-SHA (Branch des Checkouts fett), Alter, Dauer. „↻ Aktualisieren“, „Neue Pipeline“ (für den ausgecheckten Branch, mit Rückfrage), Ladefehler im Klartext (der letzte Stand bleibt stehen) |
| Pipeline #n | Status, Commit-Titel, Branch, SHA, Alter, Dauer, Auslöser; „Pipeline neu“ (fehlgeschlagen/abgebrochen), „Pipeline abbrechen“ (laufend, mit Rückfrage), Link nach GitLab; darunter die Jobs je Stage mit Status, Dauer und den Aktionen **Log**, **Starten** (manuell), **Wiederholen**, **Abbrechen** |
| Log | Die letzten 60 Zeilen eines Jobs, ANSI-Farben und GitLab-Abschnittsmarker entfernt; ein zweiter Klick auf „Log“ oder „Schließen“ blendet es aus |

**Aufrufwege:** (1) per Hand über den Git-Reiter, den Reiter selbst oder `/cockpit-ci`; (2) automatisch: das Cockpit fragt GitLab im Takt ab (alle 15 s, solange eine Pipeline läuft oder wartet, sonst jede Minute) und meldet Statuswechsel der Pipelines des Branches als Toast und Protokollzeile: grün, fehlgeschlagen, abgebrochen, wartet auf einen manuellen Job, dazu fehlgeschlagene Jobs (ohne `allow_failure`). Bei Fehlschlag öffnet es den Reiter CI/CD von selbst, ohne den Eingabefokus zu nehmen. Der erste Abruf einer Session meldet nichts. Beide Wege sind in den Einstellungen (Gruppe Merge Request · CI/CD) abschaltbar: „CI/CD-Hinweise“ und „CI/CD-Reiter bei Fehlschlag öffnen“; sind beide aus, fragt das Cockpit GitLab nur ab, solange der Reiter offen ist. Grün/rot der MR-Pipeline meldet bei eingeschalteten MR-Hinweisen nur der MR-Hinweis (kein Doppel).

**Mechanik und Entscheidung (Polling statt Webhook):** Ein GitLab-Webhook bräuchte einen vom Server erreichbaren Empfänger auf dem Arbeitsplatz (NAT, Firewall, wechselnde Netze) und ein Secret im Server; das ist für ein lokales Mod nicht zumutbar. `glab ci` ist auf interaktive Ansichten ausgelegt. Das Cockpit ruft deshalb `glab api` auf (gleiche Anbindung wie die MR-Karten, Projekt über `:id` aus dem Remote): Die Authentifizierung läuft vollständig über die vorhandene glab-Anmeldung, das Cockpit sieht, speichert und loggt kein Token. Lesend: `projects/:id/pipelines`, `…/pipelines/<id>`, `…/pipelines/<id>/jobs`, `…/jobs/<id>/trace`. Schreibend, nur per Klick: `POST …/jobs/<id>/retry|cancel|play`, `POST …/pipelines/<id>/retry|cancel`, `POST projects/:id/pipeline -f ref=<Branch>`.

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
claude plugin install tenvan-cc@orga-marketplace
```

## Entwicklung

Spezifikation, Backlog und Entscheidungen: [SPEC.md](SPEC.md). Arbeitsanleitung für Agenten (Prüfbefehle, API-Fallstricke, Test-Muster): [DEVELOPMENT.md](DEVELOPMENT.md).

Mod-API: Claude Code Function Hooks (Engine ≥ 2.1.295, Early Access). Struktur:

| Datei | Zweck |
| --- | --- |
| `hooks/register.tsx` | Hooks, Band und Panel |
| `hooks/core.ts` | Reine Rechen- und Formatfunktionen |
| `hooks/ci.ts`, `hooks/ci-ui.tsx` | CI/CD: Übersetzung der GitLab-Antworten, Ereignisse, Log-Aufbereitung (rein) bzw. Zeichnung des Reiters |
| `types/index.d.ts` | `$.state`-Vertrag |
| `hooks/register.test.ts` | Tests |

```shell
claude plugin validate tools/cockpit
claude plugin test tools/cockpit
```

Live ausprobieren ohne Installation: `claude --plugin-dir tools/cockpit`, oder den Pfad in `CLAUDE_CODE_PLUGIN_DIRS` (env-Block der `~/.claude/settings.json`) eintragen, damit die Desktop-App das Mod lädt.

Weitere Community-Mods werden als zusätzliche Abschnitte in Band oder Panel ergänzt, nicht als eigene Plugins.
