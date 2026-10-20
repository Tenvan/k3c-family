# Release Notes – Tenvan CC

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
