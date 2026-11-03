# Release Notes – Tenvan CC

## 2.4.0 – 2026-11-03

### Neu

- **CI/CD: Backend in der Kopfzeile.** Der Reiter nennt, gegen welches Backend er läuft: Badge GitLab, GitHub oder Bitbucket plus `Host/Projekt` aus dem Remote-URL. Der Reiter ist auch bei Bitbucket über den Git-Reiter erreichbar.
- **CI/CD: Zeiten wie im Browser.** Pipeline/Run und Jobs zeigen Start und Ende im lokalen Format (`02.11.2026 10:00 – 10:07`, Ende mit Datum nur an einem anderen Tag), laufende Pipelines und Jobs eine mitlaufende Dauer (jetzt − Start), dazu weiter „vor 5 min“.
- **CI/CD: Auslöser und Ersteller.** Die Pipeline-Karte nennt den Auslöser (Push, Tag, Merge/Pull Request, manuell, Zeitplan, API/Trigger) und den Ersteller (GitLab mit Anzeigename: `Ralf Stich (@ralf)`, GitHub mit Login); die Liste zeigt `@login` je Zeile.
- **CI/CD: Artefakte.** GitLab-Job-Artefakte (Archiv je Job) und GitHub-Actions-Artefakte (je Run): Name, Größe, abgelaufen markiert, Link „Im Browser laden“. **Das Cockpit lädt selbst nichts herunter** (Begründung in der SPEC: Mod-API ist textbasiert, `gh run download`/`glab job artifact` entpacken, ein Token-Weg widerspricht der Regel „kein Token im Cockpit“).
- **Sessions: Prompt-Angebote.** Fehlt die Status-Datei (`sessions.json`) oder ist sie leer, bietet der Reiter „Schaltzentrale einrichten (Prompt)“: ein Prompt, der Claude eine Schaltzentrale samt Datei einrichten lässt. Ist die Datei unlesbar (kein JSON, falsche Form) oder hat sie Hinweise, bietet er „Status-Datei anpassen (Prompt)“. Der Prompt landet in der Eingabezeile (nichts wird gesendet), das Cockpit schreibt nie selbst in die Datei.
- **Bitbucket-Panel im Reiter CI/CD (nur lesen).** Bitbucket Server/Data Center hat kein eigenes CI; das Panel zeigt deshalb die letzten 15 Commits des ausgecheckten Branches mit dem Gesamtstand (grün, rot, läuft, kein Build) und zu dem gewählten Commit die Builds, die Bamboo/Jenkins gemeldet haben: Status, Name und Key, Beschreibung, Zeitpunkt, Dauer (wo die Instanz sie meldet) und der Link ins Build-System. Erkannt wird Bitbucket am https-Pfad `/scm/`, am SSH-Port 7999 oder am Hostnamen (`git.deltra.de`). Zugang aus dem Git-Credential-Store wie bei den MR-Funktionen (nie angezeigt, nie geloggt); ohne Zugang steht ein Hinweis im Reiter. Aktualisierung jede Minute, solange der Reiter offen ist, und per „↻ Aktualisieren“. Was Bitbucket nicht hergibt, fehlt bewusst: Wiederholen und Abbrechen, Logs, Artefakte, Ersteller des Builds, Hinweise bei Statuswechseln (Begründung in der SPEC).

## 2.3.0 – 2026-11-02

### Neu

- **CI/CD auch für GitHub:** Der Reiter CI/CD erkennt die Plattform aus dem Remote-URL des Repos (`gitlab.deltra.de` → GitLab, `github.com` → GitHub) und zeigt auf GitHub dieselbe Oberfläche für GitHub-Actions-Workflow-Runs: Status, Branch, Commit-Titel, Alter und Dauer, die Jobs unter dem Workflow, bei laufenden und fehlgeschlagenen Jobs die Schritte, dazu das Job-Log (letzte 60 Zeilen, ohne Zeitstempel und Workflow-Befehle).
  - **Steuerung:** Job wiederholen, fehlgeschlagene Jobs eines Runs neu starten (fällt auf alle Jobs zurück, wenn GitHub das ablehnt), Run abbrechen (mit Rückfrage). „Starten“ manueller Jobs, Abbrechen einzelner Jobs und „Neue Pipeline“ gibt es bei GitHub nicht und erscheinen dort nicht.
  - **Aufruf und Takt wie bei GitLab:** Git-Reiter, `/cockpit-ci`, Polling (15 s bei laufendem Run, sonst 1 min), Toasts bei Statuswechseln und fehlgeschlagenen Jobs, Reiter öffnet sich bei Fehlschlag.
  - **Anmeldung:** über die vorhandene `gh`-Konfiguration, kein Token im Cockpit. Escape-Folgen und Steuerzeichen aus GitHub-Text werden wie bei GitLab entfernt.
  - **Intern:** gemeinsame Datenform, ein Adapter je Plattform (`forge.ts`), Übersetzung in `ci-github.ts`; das GitLab-Verhalten ist unverändert.
  - **Hinweis:** gegen GitHub noch nicht live geprüft; die Tests laufen gegen simulierte `gh`-Antworten.

## 2.2.0 – 2026-11-02

### Neu

- **Reiter CI/CD** (`/cockpit-ci`, achter Reiter nach Git): zeigt die letzten 15 GitLab-Pipelines des Projekts (Status, Branch, SHA, Alter, Dauer) und die Jobs der gewählten Pipeline je Stage mit Status und Dauer. Nur für GitLab-Remotes; sonst steht dort ein Hinweis.
  - **Steuerung:** Job wiederholen, abbrechen, manuellen Job starten, Job-Log ansehen (letzte 60 Zeilen, ohne ANSI-Farben); Pipeline neu starten, abbrechen (mit Rückfrage), neue Pipeline für den ausgecheckten Branch (mit Rückfrage). Schreibend nur per Klick.
  - **Einstieg aus dem Git-Panel:** Link „CI/CD“ in der Karte Merge Request.
  - **Automatisch:** Das Cockpit fragt GitLab im Takt ab (15 s bei laufender Pipeline, sonst 1 min) und meldet Statuswechsel der Pipelines des Branches (grün, fehlgeschlagen, abgebrochen, wartet auf manuellen Job) und fehlgeschlagene Jobs als Toast; bei Fehlschlag öffnet es den Reiter von selbst, ohne den Eingabefokus zu nehmen. Neue Einstellungen „CI/CD-Hinweise“ und „CI/CD-Reiter bei Fehlschlag öffnen“ (beide an).
  - **Mechanik:** Polling über `glab api` statt Webhook (kein erreichbarer Empfänger nötig); Anmeldung vollständig über die vorhandene glab-Konfiguration, kein Token im Cockpit. Begründung in der README (Abschnitt CI/CD) und in SPEC.md.
  - **Hinweis:** gegen `gitlab.deltra.de` noch nicht live geprüft; die Tests laufen gegen simulierte `glab`-Antworten.

### Behoben

- **Abschluss-Box:** Rückblick und Frage der Desktop-Abschlussbox (`wrap_up`) brechen um, statt am rechten Rand abgeschnitten zu werden.

## 2.1.2 – 2026-11-02

### Geändert

- **Weiter-Zeile bricht nicht mehr um:** Die Knopftexte der Vorschläge sind auf höchstens 28 Zeichen begrenzt (Ellipsis); der volle Text steht im Tooltip am ⓘ. Die ausführliche Fassung bleibt der Prompt, der beim Klick ins Eingabefeld geschrieben wird – der Vorschlags-Prompt an Haiku verlangt jetzt kurze Labels und separate, ausführlichere Aufträge. Bei schmalem Band entfallen die hinteren Vorschläge, statt in eine zweite Zeile zu rutschen.
- **Neue Vorschläge als Icon-Knopf:** „↻ Neue“ ist jetzt ein reiner ↻-Knopf (beim Laden ◌) in der Höhe der Vorschläge.

## 2.1.1 – 2026-11-02

### Geändert

- **Reiter Sessions: neuer Standardpfad.** Das Cockpit liest die Status-Datei jetzt aus `<Benutzerordner>/.deltra/claude/sessions.json` (unter Windows `C:/Users/<Name>/.deltra/claude/sessions.json`). Der Benutzerordner wird zur Laufzeit aus `USERPROFILE`, sonst `HOME` aufgelöst, nichts ist hartcodiert. Das Schema der Datei ändert sich nicht.
- **Einstellung „Status-Datei der Sessions“ (`sessionsFile`):** Standard ist jetzt leer (= Standardpfad); ein eingetragener Pfad hat Vorrang. Wer in 2.1.0 einen Pfad in den Einstellungen gespeichert hatte (auch den früheren Standard), muss ihn dort leeren, sonst gilt er weiter als Override.

## 2.1.0 – 2026-11-02

### Neu

- **Reiter Sessions** (`/cockpit-sessions`, zweiter Reiter nach Jetzt): zeigt die Status-Datei der Schaltzentrale (`sessions.json`) als zwei Karten, nur lesend.
  - **Sessions-Übersicht:** eine Zeile je Session mit ID, Repo, Branch, Stand (Badge `läuft`, `wartet`, `fertig`, `archiviert`), „wartet auf“ und Notiz. Wer auf dich wartet, liegt gelb hinterlegt und steht oben.
  - **Freigabe-Inbox:** offene Freigaben mit ID, Aktion (Badge `commit`, `push`, `löschen`, `archivieren`, `sonstiges`), Detail und „seit“; ohne offene Freigaben steht dort eine klare Meldung.
  - **Robust:** fehlende oder leere Datei, kaputtes JSON und falsche Form zeigen eine lesbare Meldung samt Pfad; fehlende Felder erscheinen als Hinweis, die übrigen Zeilen bleiben sichtbar. Nach einem Lesefehler bleibt der letzte lesbare Stand stehen.
  - **Aktuell:** Solange der Reiter offen ist, prüft das Cockpit alle 5 s Änderungszeit und Größe der Datei und liest nur bei Änderung neu; „↻ Neu lesen“ erzwingt es. Oben steht das Feld `updated` der Datei.
  - **Pfad einstellbar:** Mod-Einstellung „Status-Datei der Sessions“ (`sessionsFile`), Standard ab 2.1.1 `<Benutzerordner>/.deltra/claude/sessions.json`.

## 2.0.1 – 2026-11-02

### Geändert

- **Arbeitsverzeichnis ganz oben:** Die Karte „Arbeitsverzeichnis“ steht als erste Karte im Reiter Jetzt statt im Reiter Git. Ohne Git-Teil zeigt der Reiter Git sie weiterhin mit Branch und Änderungen.
- **Repo-Label im Reiter Git:** Über dem Branch stehen Name und Wurzelordner des Repos, auf das sich der Git-Stand bezieht.

## 2.0.0 – 2026-11-02

### Behoben

- **Subagent öffnen (↗):** Auf dem Desktop passierte nichts, weil die Engine die Hauptansicht nur im Terminal umschalten kann (`no main view here`). Jetzt zeigt ↗ dort den Verlauf des Subagents (Auftrag, Antworten, Tool-Aufrufe) als Karte im Reiter Jetzt; ein zweiter Klick schließt sie, ist kein Verlauf lesbar, nennt die Karte den Grund.
- **Diagnose-Hinweise:** Lange Werte in Tipps, Wächter, Profil, Blast und Inspektor brechen um, statt abgeschnitten zu werden; lange Beschriftungen stehen in eigener Zeile.

### Geändert

- **Kacheln nur für Laufendes:** Subagents und Hintergrundaufgaben erscheinen im Reiter Jetzt nur noch als Kachel, solange sie laufen. Beendete stehen als Zeile unter „Verlauf“ (Status, Dauer, bei Aufgaben die erste Ergebniszeile); Subagents lassen sich dort weiter mit ↗ öffnen. Die Einstellung „Subagent-Kacheln“ begrenzt jetzt die laufenden Kacheln.
- **Ein Cockpit statt einer Sammlung:** Jede Information steht an genau einer Stelle.
  - **Band nur mit Links:** Alle Knöpfe im Band sind Links; Gelb und Rot liegen als getönter Grund darunter (hell und dunkel). Der Block mit den Folge-Schritten ist hellgelb hinterlegt. Kopf und Messwerte teilen sich eine Zeile; die Werte (Limits, Kontext, Cache) führen zum Reiter Verbrauch, der MR zum Reiter Git. Die Knöpfe „Zusammenfassung“ und „Features“ entfallen im Band.
  - **Eine Meldungszeile:** Schleifen-Warnung, Verzeichniswechsel und die sechs Kurzinfos der Diagnose zeigt das Band nur noch bei Gelb oder Rot, und dann nur die wichtigste mit „+n Hinweise“. Alles andere steht unter Diagnose → Hinweise.
  - **Checkliste:** Pause, Weiter und Plan als Links; Claudes Rückfragen stehen als Antworten (Tasten 4–6) direkt unter der Frage. Die Atemübung belegt keine eigene 56-px-Zeile mehr: der Statusring atmet, die ganze Übung steht im Reiter Jetzt.
  - **Ein Panel mit Reitern statt sechs Panels:** Jetzt · Verbrauch · Diagnose · Git · Rückblick · Einstellungen; Diagnose und Rückblick mit Unterreitern. Die Befehle `/cockpit-zusammenfassung`, `/cockpit-verlauf`, `/cockpit-einstellungen`, `/cockpit-git`, `/cockpit-profiling`, `/cockpit-kontext`, `/cockpit-profil`, `/cockpit-fehler`, `/cockpit-zeitreise` öffnen den passenden Reiter.
  - **Protokoll:** Log, Fehler-Eingang, Tool-Freigaben und Zeitreise sind eine Liste (neueste zuerst) mit Filter Alle · Fehler · Freigaben · Snapshots, über die `tool_use_id` verknüpft; Diff und Zurück stehen am Eintrag, die Fehlerzeile unter dem Aufruf. Aufrufe von Subagents stehen mit Quelle `agent`.
  - **Fußzeile:** nur noch „Tenvan“; der MR-Knopf entfällt, der MR steht im Band. Die Git-Karte des Panels ist im Reiter Git aufgegangen (ohne Git-Teil zeigt das Arbeitsverzeichnis Branch, Commit und Upstream).
  - **Einstellungen:** Die Schalter „Git“ (Karte), „Log“ (Karte) und „MR-Knopf unter dem Prompt“ entfallen mit ihren Anzeigen.

### Sicherheit

- **Zeitreise ohne Filter:** Snapshots (`git add`) und Zurücksetzen (`git restore`) prüfen die Repo-Konfiguration direkt davor ohne Zwischenspeicher, leeren jeden konfigurierten Filter und lesen Attribute aus dem leeren Baum. Eine eben geänderte `.git/config` mit `.gitattributes` oder `.git/info/attributes` kann so keinen Clean- oder Smudge-Filter mehr ohne Freigabe starten.

### Neu

- **Band ganz abschaltbar:** Neue Schalter für Effort und Modell, Kontext (mit Kompaktieren/Handoff), Kosten, laufende Agents und die Meldungszeile; „Cache“ schaltet nur noch den Cache. Titel und Ordner/Branch bleiben.
- **Blast-Radius erklärt sich:** Abschnitt „Bewertung“ mit einem Satz je Ampel (Reichweite, Tests, Unaufgelöstes, Vollständigkeit), was sie bedeutet und was zu tun ist; jeder Abschnitt hat eine Erklärzeile. Unter Diagnose → Hinweise stehen die gelben und roten Sätze unter der Blast-Meldung.
- **Prompt kopieren:** Im Verlauf (Rückblick → Verlauf) kopiert ⧉ den getippten Prompt einer Aufgabe (ältere Einträge: den Titel).
- **Fehler-Eingang:** Teil des Protokolls (Filter Fehler, mit Bilanz), Meldung im Band ab Gelb und `/cockpit-fehler [text]`: die letzten 30 fehlgeschlagenen oder abgelehnten Tool-Aufrufe mit Ziel und erster Fehlerzeile, Wiederholungen als `×n`.
- **Nutzungswächter:** warnt bei 8+ gleichzeitigen Subagents, 10+ neuen je Minute, einem Prompt ab 85 % Kontext und demselben Aufruf 3× hintereinander. Neue Einstellung „Wächter hält Agents an“ (Standard aus) lehnt ab 12 laufenden Subagents den Start eines weiteren ab.
- **Tipps zu Cache, Kontext und Limits:** als Meldung im Band ab Gelb, alle unter Diagnose → Hinweise, mit der Zahl dahinter (Cache neu geschrieben, abgelaufen, vor Ablauf kompaktieren, Kontext, Limit).
- **Zeitreise:** Zeitleiste der Tool-Schritte mit Snapshots des Arbeitsbaums als git-Objekte (eigener Index, kein Branch, kein Commit). „Diff“ zeigt Änderungen seit einem Schritt, „Zurück“ (mit Rückfrage und Sicherung davor) setzt Dateien auf den Stand des Schritts; neue Dateien bleiben liegen. `/cockpit-zeitreise [text]`. Neue Einstellung „Zeitreise: Snapshots anlegen“.

## 1.9.1 – 2026-10-24

### Geändert

- **Rückfrage-Kopf ausgebaut:** Über dem unveränderten Dialog stehen jetzt die Aufgabe und der laufende Schritt aus der Checkliste, auf dem Desktop je Frage Kopf, Auswahlart (eine Wahl oder Mehrfachauswahl), Zahl der Optionen und die empfohlene Option (★). Im Terminal nur Kopf und Aufgabe.

### Sicherheit

- **Git-Panel:** Auch `core.gitProxy`, `core.alternateRefsCommand` und `remote.*.vcs` in der lokalen Repo-Konfiguration sperren Aktionen.

## 1.9.0 – 2026-10-24

### Neu

- **Git-Panel (`/cockpit-git`, Knopf in der Git-Karte):** führt durch den Ablauf Commit → MR → Merge → Aufräumen. Oben eine Ablauf-Leiste, darunter die Status-Karte mit genau einer Hauptaktion je Lage: auf main mit Änderungen „Branch anlegen und mitnehmen“, sonst Committen, Pull bzw. Pull --rebase bei Divergenz, „Erst syncen“, wenn der Eltern-Branch vor dem MR weitergelaufen ist, MR entwerfen (Entwurf im MR-Panel), Push, und nach dem Merge „Aufräumen“ (fetch --prune, Wechsel auf den Eltern-Branch, Kinder in Git Town umhängen, Branch löschen). Karten für Fehler (Ausgabe des gescheiterten Befehls), Konflikte, Änderungen (je Datei stagen), Commit (Eingabefeld, Vorschlag von Haiku im Stil der letzten Commits), Eingehend, Sync, Branches (Baum nur mit Abhängigkeiten, Klick wechselt) und Verlauf. Verwerfen, Abbrechen, Force-with-lease, Löschen und „Letzten Commit zurück“ fragen beim ersten Klick nach. ✦-Knöpfe schreiben Aufträge an Claude ins Eingabefeld (committen, aufteilen, Review, eingehende Commits, Konflikte, MR, Aufräumen, Fehler). Neue Einstellungsgruppe „Git“. Das Panel startet git ohne Repo-Hooks; hat das Repo `pre-commit` oder `commit-msg`, ist die Hauptaktion „Mit Claude committen“, direkte Knöpfe tragen „(ohne Hooks)“. Force-Push nur mit `--force-with-lease --force-if-includes`.

### Geändert

- **MR-Panel im Git-Panel aufgegangen:** `/cockpit-mr` und das eigene MR-Panel entfallen; alle MR-Karten stehen im Git-Panel, Band, Fußzeile und Git-Karte öffnen es. Der Schalter „MR-Karten im Git-Panel“ blendet sie aus.
- **Branch-Klick zeigt Optionen** statt sofort zu wechseln. main, master und develop löscht das Panel nie.
- **Knöpfe nach Priorität:** Im Panel trägt nur die Hauptaktion die volle Akzentfläche, gewöhnliche Knöpfe sind gedämpft, Nebensachen ohne Fläche und gedimmt.

## 1.8.1 – 2026-10-22

### Geändert

- **„✦ Alle mergen (AI)“:** Der Auftrag besteht nur noch aus einer allgemeinen Anweisung („Merge alle n offenen Merge Requests inklusive der nötigen Konfliktlösungen“) und der Aufzählung der MRs mit ihrem bekannten Status. Das Regelwerk (Worktrees, Reihenfolge, Pipeline abwarten, Stopp-Bedingungen, CLI je Plattform) ist entfallen; wie Claude vorgeht, entscheidet Claude.

## 1.8.0 – 2026-10-22

### Neu

- **Subagents und Hintergrundaufgaben: Symbolknöpfe vor jedem Eintrag** (Kachel und kompakte Liste), nur was geht. **Stopp (■)** steht, solange der Eintrag läuft: beim Subagent bremst er ihn sofort (keine weiteren Tool-Aufrufe, wie bisher „Bremsen“, jetzt für jeden laufenden Subagent statt nur für teure); bei einer Hintergrundaufgabe schreibt er den Auftrag „TaskStop“ ins Eingabefeld, weil nur das Modell dieses Tool aufrufen kann (gesendet wird mit Enter). **Öffnen (↗)** gibt es nur bei Subagents und zeigt ihr Transcript in der Hauptansicht; Hintergrundaufgaben (Shells, Monitore) haben keinen. Ist nichts möglich, fehlt die Knopfzeile.

## 1.7.0 – 2026-10-21

### Neu

- **MR-Panel: „✦ Alle mergen (AI)“:** Ab zwei offenen MRs (Branch-MR und eigene, ohne Doppelte) steht in der Karte „Meine offenen MRs“ ein Knopf, der einen Sammelauftrag ins Eingabefeld schreibt (gesendet wird erst mit Enter). Claude arbeitet je MR in einem eigenen Worktree, löst Konflikte, pusht ohne Force-Push, wartet auf die Pipeline und mergt; nach jedem Merge prüft er die übrigen neu, weil sich der Ziel-Branch geändert hat. Drafts, rote Pipelines, fehlende Freigaben und echte inhaltliche Entscheidungen führen zum Stopp bei diesem MR; am Ende steht eine Ergebnistabelle. Die CLI (`glab`, `gh`, Bitbucket REST) richtet sich nach der Plattform.

- **Profiling: Ampel in vier Stufen** (sehr gut, gut, schlecht, sehr schlecht) überall, wo eine Aussage trägt: Engine-Overhead, Freigabe-Wartezeit, TTFT, Cache-Trefferquote und Tool-Dauer im Agent-Profiler; Verschwendung, Re-Reads, Kompaktier-Verluste und Hinweise im Kontext-Inspektor; Reichweite, Testabdeckung und Unaufgelöstes im Blast-Radius. Ein farbiger Punkt steht vor dem Wert, dazu eine Legende oben im Panel und je Karte der Abschnitt „Ampel-Schwellen“ mit den Grenzen (kommen aus derselben Tabelle wie die Bewertung). Die Kurzinfos im Band tragen den Punkt der schlechtesten Stufe, die Berichte (`/cockpit-kontext`, `/cockpit-profil`, `/cockpit-blast`) hängen die Stufe als Wort an. Rein beschreibende Werte (Anteile Modell/Tools, Tokenzahlen) bleiben ohne Ampel. Die Grenzen sind Startwerte (`hooks/rating.ts`), keine Messwahrheit.

## 1.6.0 – 2026-10-20

### Neu

- **Diagnose:** Drei Mods aus [YeonwooSung/my-claude-code-mods](https://github.com/YeonwooSung/my-claude-code-mods) als Funktionsumfang übernommen, als ein Panel „Profiling“ (`/cockpit-profiling`, eine Karte je Mod) mit Kurzinfos im Band. **Kontext-Inspektor** (`/cockpit-kontext`): Belegung des Fensters, erneut gelesene Dateien, ähnliche Ausgaben, Verluste beim Kompaktieren, Hinweise mit geschätzter Ersparnis. **Agent-Profiler** (`/cockpit-profil`): Zeit in Modell, Tools, Subagents und Overhead mit Beobachtungsgrad. **Blast-Radius** (`/cockpit-blast`): betroffene Dateien und Testkandidaten deiner Änderungen. Alle lesen nur mit und sind unter „Diagnose“ in den Einstellungen abschaltbar. Kein OTLP-Export, kein Netzwerk.

## 1.5.3 – 2026-10-19

### Behoben

- **Hintergrundaufgaben:** Das Ende einer Aufgabe wird auch erkannt, wenn die Benachrichtigung mitten im Turn als Anhang kommt. Bisher blieb die Aufgabe dann auf „läuft“, weil nur Prompts ausgewertet wurden. Tool-Ergebnisse zählen bewusst nicht, damit zitierter Text kein falsches Ende auslöst.

## 1.5.2 – 2026-10-19

### Behoben

- **Entsperren-Prompt gehärtet:** Der Git-Ordner stammt aus dem Repo und steht im Befehl des Prompts. Pfade mit Anführungszeichen, Backtick, `$`, `;`, `&`, `|`, `%` oder Steuerzeichen und Hashes ohne SHA-1/SHA-256-Form werden abgelehnt (Toast statt Prompt).

### Neu

- **Tool-Zeile:** `target` steht bei den kennzeichnenden Feldern, `timeoutSeconds` als „300s“ am Ende der Zeile.

## 1.5.1 – 2026-10-19

### Geändert

- **Entsperren an den Inhalt gebunden:** Die Freigabe eines Repos mit riskanter Konfiguration gilt nur, solange die riskanten Schlüssel samt Werten (lokal und Worktree) unverändert sind. Der Eintrag `cockpit.trustedRepo` lautet „Git-Ordner|Hash“; der Knopf „Entsperren“ liefert Claude den fertigen Befehl mit dem Hash. Ändert sich ein Eintrag, sperrt das Cockpit wieder. Freigaben der Version 1.5.0 (nur Pfad) gelten nicht mehr und müssen neu erteilt werden.
- **Tool-Zeile:** `title` steht am Ende der Zeile und überschreibt `note`, wenn beide vorhanden sind.

## 1.5.0 – 2026-10-19

### Neu

- **Panel „Git“:** Branch, Worktree, Commit, Änderungen, Upstream und der MR-Kurzstatus stehen in einer eigenen Karte; „Arbeitsverzeichnis“ behält Session-Ordner, Prozess-Ordner, Ordner und verknüpfte Sessions. Eigener Schalter `cardGit` in den Einstellungen (Gruppe Panel).
- **Entsperren:** Bei „nicht geprüft (Repo-Konfiguration enthält Befehle)“ schreibt der Knopf einen Prompt in die Eingabezeile. Claude zeigt die Treffer und trägt bei erkennbar eigenen Einträgen den Git-Ordner in die globale Git-Konfiguration ein (`cockpit.trustedRepo`). Die Marke liegt bewusst nicht im Repo, damit ein kopierter Ordner sich nicht selbst freigibt.
- **Einzelner MCP-Aufruf:** Dieselbe farbige Zeile „Server › Tool (Aktion) → Ziel“ wie in der Tool-Gruppe.
- **Hervorhebung:** Kennungen wie `PJ-013` oder `WB-025` zählen als ein Block in der Zahlenfarbe.
- **Details-Knopf** im Chat als flache orange Pille ohne Rundung, kleiner als bisher.

## 1.4.1 – 2026-10-19

### Neu

- **Tool-Zeile:** `action` steht grün in Klammern hinter dem Tool-Namen („plan_session (done) → …“), `project` und `note` (gekürzt) stehen im Ziel, auch bei fremden MCP-Tools; Ziel in Gelb, „→“ gedimmt.

### Behoben

- **Checkliste:** Die Abschluss-Box der Desktop-App (`wrap_up`) hakt die Schritte ab; vorher blieb das Band bei „Fertig“ auf „0 von 4“. Ein Turn-Ende allein hakt nichts ab, damit „Fertig“ nicht zu früh erscheint, wenn Claude auf Hintergrundarbeit wartet.

## 1.4.0 – 2026-10-17

### Neu

- **Checkliste ganz oben im Band** (Idee: glanceflow, kein Code übernommen): Claudes eigene Aufgabenliste (`TodoWrite`, Aufgabenliste der Desktop-App, `TaskCreate`/`TaskUpdate`) als Schritte, dazu Status, Fortschritt, Restzeit, was Claude gerade tut; die laufenden Helfer stehen gruppiert in derselben Zeile (`↳ 3× planner · Opus 5.5`). Die gleichbleibenden Cockpit-Zeilen bleiben direkt am Prompt, die Checkliste wächst nach oben.
  - Status: Arbeitet, Braucht dich (Frage aus Claudes Antwort, Abschlussfrage der Desktop-App oder Freigabe), Hängt? (3 Minuten ohne Lebenszeichen), Pausiert, Gestoppt, Fertig.
  - Restzeit aus den eigenen erledigten Schritten, sonst aus dem Verlauf des Projekts; „dauert länger“, wenn ein Schritt doppelt so lange läuft wie üblich.
  - Knöpfe „‖ Pause“ (stoppt den Turn) und „▶ Weiter“ (schickt den Auftrag mit den offenen Schritten).
  - Ab 5 Schritten faltet das Band erledigte und spätere Schritte.
- **Plan-Karte** im Panel: ganzer Plan mit Start, voraussichtlichem Ende, Dauer je Schritt und geänderten Dateien je Schritt.
- **Verlauf** (`/cockpit-verlauf`): Aufgaben je Tag mit Ergebnis, Schritten und Dauer, 30 Tage lokal im Speicher des Mods. Knöpfe kopieren einen Tagesbericht (Projekt) oder Wochenbericht (alle Projekte) in die Zwischenablage.
- **Einstellungen:** neue Gruppe „Checkliste“ mit den Schaltern Checkliste im Band, Plan-Karte, Verlauf speichern.
- **Slash-Befehle in der Checkliste:** ein getippter Befehl erscheint als Job „/name args“; startet er einen Turn (Skill), gehört der Turn zu diesem Job.
- **Agent-Meldungen lesbar:** Rückmeldungen von Subagents (↩), Nachrichten anderer Sessions (✉) und Hintergrund-Benachrichtigungen (⚙) bekommen einen Titel statt der XML-Hülle.
- **Merge Requests:** Haken „☐ Squash“ neben „Mergen“ (Branch-MR und eigene MRs), Standard aus und nach jedem Merge wieder aus (GitLab `squash`, GitHub `--squash`, Bitbucket `strategyId: squash`).

### Bewusst nicht übernommen

- Eingeschleuste Prompt-Anweisung, eigene Werkzeuge für Plan und Fortschritt und das Sperren von Tools bis zum Plan: Das Cockpit greift laut Leitplanke nie in die Session ein.
- Passwort-Wächter, „Ship it“, Datenschutz-Maskierung, Ausblenden von Tool-Zeilen, Aufräumen/Checkpoint, Handoff, Willkommenstour, andere Chats, Ruhemodus, Plan-Freigabe und KI-Jobnamen (der Titel ist der Prompt).
- Ton und Desktop-Hinweis (glanceflow): Die Hinweise starteten `powershell.exe` und lösten Berechtigungsabfragen aus; Entscheidung Ralf, 2026-10-17.

### Intern

- Neues Modul `hooks/glance.ts` (reine Funktionen ohne `$`), Typen `Job`, `JobStep`, `JobRecord` in `types/index.d.ts`, Texte unter `glance.*` in `hooks/i18n.ts`.
- Drittanbieter-Hinweis zu glanceflow (MIT) in `THIRD_PARTY.md`.

## 1.1.2 – 2026-10-16

Umfasst alle Änderungen seit 1.1.0 (`7a5479b`); 1.1.1 wurde nicht gesondert veröffentlicht.

### Neu

- **Verzeichniswächter:** Die Kachel „Arbeitsverzeichnis“ zeigt oben den Session-Ordner, darunter den Prozess-Ordner und den letzten Ordner außerhalb. Wechselt Claude per `cd`, `git -C`, Desktop-Ordnerwechsel oder `EnterWorktree` nach außerhalb oder schreibt dort Dateien, wird der Aufruf zur Rückfrage (Einstellung `dirGuard`). Zusätzlich erscheinen eine rote Warnzeile im Band, ein Toast und ein Log-Eintrag. Temp, AppData und Punkt-Ordner im Home sind ausgenommen.
- **Restzeit im Band:** Hinter jedem Limit steht die Zeit bis zum Reset.
- **Verknüpfte Desktop-Sessions** in der Kachel „Arbeitsverzeichnis“ mit Status, Name und Branch.
- **Folge-Schritte:** Vorschläge sind als Auftrag an Claude formuliert, die Begründung steht im Tooltip am ⓘ. Der Knopf „↻ Neue“ holt andere Vorschläge; die Zeile „Weiter:“ bleibt auch ohne Vorschläge sichtbar.
- **Einstellungen:** Die Karten sind nach Themen gruppiert, einzeln einklappbar und lassen sich über „Alle ein-/ausklappen“ steuern.

### Geändert

- **Atemübungen** haben jeweils eine eigene Farbe: HRV blau, Seufzer grün, Box violett, 4-7-8 amber (Band, Spinner-Phase, Einstellungen).
- Der Kompaktieren-Knopf ist bei 50–70 % Kontext gelb statt blau.
- MR-Aktionen zeigen „◌ …“, solange sie laufen. Text-Knöpfe sind blaue Pillen, Skin-Knöpfe gedämpft orange.
- Design-System mit Tokens je Bereich (pane, skin, sidebar, band) in `hooks/ui.tsx`.
- Die Verzeichnis-Warnung im Band hat keinen „Verstanden“-Knopf mehr; sie verschwindet mit dem nächsten Prompt.

### Behoben

- Die Kachel „Arbeitsverzeichnis“ verschwand, wenn Claude per `cd` in einen Ordner ohne Git-Repo (z. B. Temp) wechselte oder Git einen Fehler meldete. Git läuft jetzt im Session-Ordner, und die Kachel erscheint auch ohne Repo-Daten.
- Der Verzeichniswächter schlug zu oft an:
  - Pfade mit `.` und `..` werden aufgelöst.
  - Das Ziel wird vor dem Lauf mit aktuellen Ordnern berechnet.
  - `cd` mit Variablen oder Befehlsersetzung (`$VAR`, `$(…)`, `%TEMP%`) ergibt kein Ziel mehr.
  - `cd /d` wird erkannt.

### Intern

- Die Store-Stubs der Tests antworten mit `{ value }`. Alle 148 Tests sind grün.
