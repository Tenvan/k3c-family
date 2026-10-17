# Release Notes – Tenvan CC

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
