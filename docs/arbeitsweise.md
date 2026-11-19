# Arbeitsweise

**Projekt** (Thema mit Rang) → **Sprint** (3–6 Sessions, ein PR) → **Session** (eine Domäne, ein Commit).
**Tickets** ([`backlog/`](backlog/)) gehören zu einem Projekt und werden in Sprints eingeplant. **Ein PR je Sprint.**
Ein Code-Sprint ist erst fertig, wenn seine **Review-Session** ihn abgenommen hat (leicht, siehe unten).
Übersicht der Projekte: [`projekte/README.md`](projekte/README.md), aller Sprints: [`sprints/README.md`](sprints/README.md).
Zielbild: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).
**Parallel statt seriell:** Gearbeitet wird in **Arbeitswellen**: Sessions, die sich weder über Abhängigkeit noch **Bereich** noch **Sperre** berühren, laufen gleichzeitig.
Was 🧑 braucht, steht in der **Ralf-Queue** und hält nur auf, was wirklich davon abhängt (Abschnitt „Parallelität und Wellen“).

## Ablage

```text
docs/
  arbeitsweise.md             diese Datei (einzige Prozess-Beschreibung)
  vorlagen/                   Pflicht-Vorlagen: projekt.md, ticket.md, sprint.md, session.md
  projekte/README.md          Rangliste aller Projekte
  projekte/XXX-name.md        ein Projekt: Ziel, Status, Rang, Sprints in Reihenfolge
  backlog/README.md           Index der offenen und eingeplanten Tickets (eine Zeile pro Ticket)
  backlog/B-NNN-kurzname.md   ein offenes oder eingeplantes Ticket pro Datei
  backlog/archiv/README.md    Index der erledigten und verworfenen Tickets (B-696)
  backlog/archiv/B-NNN-…      erledigte und verworfene Tickets (gleicher Dateiname, per `git mv`)
  sprints/README.md           Fahrplan: alle Sprints mit Ordner und Status
  sprints/aktiv/SPnn-name/    der laufende Sprint: README.md (Sprint) + SPnn.m-name.md (Sessions)
  sprints/geplant/…           kommende Sprints, gleicher Aufbau
  sprints/erledigt/…          abgeschlossene Sprints
```

**Lesen:** `projekte/README.md` und `sprints/aktiv/` immer, `sprints/geplant/` nur beim Planen, `sprints/erledigt/` und `backlog/archiv/` nur auf ausdrückliche Nachfrage.
Setzt eine Session ein Ticket auf `erledigt` oder `verworfen`, verschiebt `plan_set` es nach `backlog/archiv/` und die Index-Zeile nach `backlog/archiv/README.md` (zurück beim Wiederöffnen); `backlog/README.md` führt nur offene und eingeplante Tickets.
Duplikate und Ersatz stehen im neueren Ticket als `Duplikat von:` (setzt `verworfen`) und `Ersetzt:`; die Gegenrichtung zeigt `plan_get` berechnet.

**Vorlagen sind Pflicht.** Jedes Ticket, jeder Sprint und jede Session entsteht als Kopie der Vorlage aus
`docs/vorlagen/`. `tests/planning.test.ts` prüft Felder, Überschriften, Status passend zum Ordner, Index und Verweise.
Eine Abweichung lässt `npm test` scheitern.

**Planung nur über k3c-workbench (B-210).** Anlegen, Felder ändern, Abschnitte füllen, archivieren, Sprints verschieben und
Entwürfe löschen laufen über die MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`,
`plan_delete`; sie ziehen Index, Session-Tabelle, Fahrplan und Ordner selbst nach. Handarbeit an diesen Dateien ist nur
der Rückfall, wenn k3c-workbench nicht läuft; dann gelten die Regeln dieses Abschnitts wörtlich.
**Fehlen die MCP-Tools, erst die Workbench prüfen.** Sind `mcp__k3c-workbench__*` in einer Session nicht verbunden, wird nicht sofort von Hand gearbeitet:
(1) Läuft sie? Prozess `k3c-workbench*`, Port 5180 hört, letzte Zeilen von `logs/k3c-workbench.jsonl` (`💥 start fehlgeschlagen` nennt den Grund, meist ein belegter Port).
(2) Läuft sie nicht: `bin/k3c-workbench.exe` starten; ist die EXE älter als `main`, vorher `task k3c-workbench:build`. (3) MCP der Session neu verbinden. Erst wenn das scheitert, gilt der Rückfall von Hand, und die Meldung an 🧑 nennt den Grund.
**Glossar ist verbindlich.** [`glossar.md`](glossar.md) legt die Begriffe fest und wird beim Schreiben von Specs und Texten gelesen. Ein neuer
Begriff kommt beim Planen zuerst ins Glossar, dann in Ticket, Sprint oder Regel. Reviews achten auf Begriffs-Treue: Abweichungen gleicht der Review-Commit an (kein schwerer Befund).

## Spec-Driven Development (SDD)

Ticket und Sprint-README **sind** die Spec, eine eigene Spec-Datei gibt es nicht. Das Wie steht in den Sessions.

- **Abschnitte:** Ausgangslage · Ziel · Beteiligte und Zielgruppen · Anforderungen · Nicht-Ziele · Regeln und
  Einschränkungen · Beispiele · Ausnahme- und Fehlerfälle · Akzeptanzkriterien · Offene Fragen. Kurz halten; passt ein
  Abschnitt nicht, `nicht relevant` mit Grund. Unbekanntes steht unter **Offene Fragen**, nie erfunden anderswo.
- **Kriterien** haben stabile IDs `AC-01`, `AC-02` … (lückenlos, nie umnummerieren). Ein Sprint verweist auf
  Ticket-Kriterien als `B-009/AC-01` statt sie zu kopieren. Jede Session nennt im Feld `Kriterien` die Sprint-Kriterien,
  die sie erfüllt (Review: `alle`); bei `Reife: bereit` hat jedes Kriterium mindestens eine Session.
- **Status der Spec:** `Entwurf` → `freigegeben` nur durch ausdrückliche Zustimmung von 🧑 zu genau dieser Revision
  (Feld `Freigabe`: Datum und Quelle). `rückwirkend` = aus erledigter Arbeit abgeleitet, ohne Freigabe.
  Die Freigabe eines Sprints umfasst seine Ticket-Specs in ihrer aktuellen Revision; die Tickets bekommen dieselbe Freigabe.
  **Ausnahme (Beschluss 🧑 2026-10-22):** Sagt 🧑 ausdrücklich „autonom abarbeiten“, gilt der Vorbehalt nicht: Claude darf
  die Specs der freigegebenen Arbeit selbst freigeben (Feld `Freigabe`: „autonom, Claude, <Datum>“), nach eigener
  Einschätzung und bisherigem Vorgehen. Offene Fragen entscheidet Claude mit einem begründeten Default und meldet sie im
  Nachgang; 🧑 ändert, was ihm nicht gefällt. Manuelle Abnahmen (siehe unten) bleiben bei 🧑.
  **Klasse A (Beschluss 🧑 2026-11-11):** Specs vom Typ `Problem` oder `Schuld` in den Domänen DEV, INF oder PAP, die weder Spielregeln noch Protokoll,
  Spielstand-Format oder Abhängigkeiten (`go.mod`, `package.json`) ändern, gibt Claude dauerhaft selbst frei (Feld `Freigabe`: „Klasse A, Claude, <Datum>“)
  und meldet sie gesammelt im Nachgang. Alles andere (Klasse B) gibt nur 🧑 frei.
- **Änderung:** Passt eine Anforderung während der Umsetzung nicht, erst die Spec ändern (`Revision` + 1, zurück auf
  `Entwurf`, neue Freigabe), dann den Code. Nie ein Kriterium umschreiben, damit es zum Code passt.
- **Nachweis:** Das **Ergebnis** der Session nennt je Kriterium `umgesetzt`, `geprüft` (womit), `verschoben` (Grund,
  Ticket) oder `blockiert`. Ein Kriterium wird nie still abgehakt.
- **Abnahme gehört 🧑:** Eine Spec-Freigabe ist keine Erlaubnis für manuelle Prüfungen. Browser-, Xbox- und TV-Prüfungen
  macht ein Agent nur, wenn die Session sie nennt und 🧑 sie für diesen Lauf freigegeben hat. Bestätigte Nachweise
  gelten weiter, solange ihre Grundlage unverändert ist; wer geprüft hat, steht im Ergebnis.

## Branches

- **`main`** ist der Arbeitsstand (kein `main` mehr, seit 2026-11-14). Jeder Sprint hat **einen** Branch `sprint/<präfix>` von `origin/main`; `<präfix>` ist
  der Teil vor `/` im Feld `Branch` seiner Sessions (z. B. `f5/1-…` → `sprint/f5`). Alle Sessions committen dort,
  am Ende steht **ein PR je Sprint** gegen `main`. Abgleich mit `main` per `git merge origin/main`, nie Rebase
  oder Force-Push auf einem geteilten Sprint-Branch.
- **Basis:** Nennt das Projekt einen Integrations-Branch (`feature/<projekt>`, Zeile `Basis:` in den Notizen des Projekts), basiert der Sprint-Branch darauf, sonst auf `origin/main`.
- **Parallele Sprints:** Sprint-PRs mergen in der Reihenfolge ihrer Fertigstellung. Wer später fertig wird, holt die Basis mit `git merge origin/<Basis>` und löst die Konflikte vor dem Push;
  für Index und Fahrplan gilt die Regel „Sammeldateien“ (Abschnitt „Parallelität und Wellen“). Eine Session, die von einem offenen Sprint abhängt, startet erst nach dessen Merge.
- **Aufräumen (B-302):** Lokal und auf `origin` liegen nur `main`, die Branches aktiver Sprints und ihre Worktrees. Vier Meilensteine:
  Session-Start (Autonomer Ablauf, Schritt 1: `task ops:aufraeumen`) · Sprint erledigt (Review, Schritt 5: `wip/<präfix>…`-Branches des Sprints prüfen; was im Sprint-Branch steckt, steht im PR-Text unter „Zum Löschen“) ·
  Sprint-PR gemergt (🧑 löscht die genannten `origin/wip/…`-Branches; auf `origin` löscht nur 🧑) · Release (Zeile „Aufgeräumt“ der Checkliste, nur Hinweis).
  `task ops:aufraeumen` löscht lokal nur Branches, deren Commits per `git cherry` in `origin/main` stecken oder deren Upstream weg ist und deren Pull Request laut `gh` gemergt ist, dazu saubere Worktrees dieser Branches.
  Nie gelöscht werden `main`, der ausgecheckte Branch und `sprint/*` mit Ordner in `docs/sprints/aktiv/`. Ohne `gh` oder Netz listet der Task nur.
- **`main`** ist geschützt und trägt nur Release-Stände. Nur 🧑 gleicht ihn bei einem neuen Major-Release ab,
  per Fast-Forward: `git fetch && git push origin origin/main:main`, danach Tag `v*` auf `main`
  (startet `release.yml`; GitHub Pages baut aus `main`). Nie direkt auf `main` committen oder einen PR gegen `main` öffnen.

## Autonomer Ablauf (Mensch oder Cloud-Agent)

Eine Session muss **ohne Rückfragen und ohne Planungs-Werkzeuge** abzuarbeiten sein. Deshalb:

1. **Aufräumen:** `git fetch --prune`, dann `task ops:aufraeumen` (löscht nur sichere lokale Branches und Worktrees, listet den Rest mit Grund; `task ops:aufraeumen -- --dry` zeigt nur an).
   `docs/sprints/aktiv/*/README.md` lesen. Liegt `sprint/<präfix>` schon auf `origin` (`git ls-remote --heads origin
   sprint/<präfix>`), gilt der Stand der Sprint-Dateien dort (`git show origin/sprint/<präfix>:<Pfad>`), sonst der von
   `main`. Die nächste Session ergibt sich aus dem **Rang** (`projekte/README.md`): aktives Projekt mit dem kleinsten
   Rang → seine aktiven Sprints in der Reihenfolge der Projekt-Tabelle → erste Session mit `Status: offen`,
   `Agent: autonom` und erledigten Abhängigkeiten. Gibt es dort keine, kommt das Projekt mit dem nächsten Rang. Die **Domäne** (bei gesetzter `Lane`: Domäne und Lane) der Session darf nicht gesperrt sein:
   Steht auf einem anderen Sprint-Branch (`git ls-remote --heads origin 'sprint/*'`) eine Session derselben Domäne auf
   `in Arbeit` und gleicher Lane oder ohne Lane (oder ist die eigene Session ohne Lane), die nächste passende Session nehmen
   (Sperr-Regel: Abschnitt „Domänen“). Gibt es keine: **nichts tun** und das melden.
   **Kurzweg:** `plan_next` (k3c-workbench) trifft diese Auswahl aus den lokalen Dateien und liefert Session, Kriterien und
   „Parallel möglich“; je aktivem Sprint überlagert es den Stand von `origin/sprint/<präfix>` (lokaler Ref nach `git fetch`,
   Schritt 1 oben; fehlt der Ref, gilt der lokale Stand und die Antwort sagt das, B-519). Sprint-Branches, die lokal noch nicht
   bekannt sind, zeigt `git fetch --prune`.
2. Sprint-Branch auschecken (fehlt er: von `origin/main` anlegen, die erste Session setzt `Start-Commit`).
   Session-Datei vollständig lesen, `Status: in Arbeit` setzen, committen und sofort `git push -u origin sprint/<präfix>`
   (beansprucht die Session; scheitert der Push, hat ein anderer Lauf sie: neu holen und bei 1. beginnen).
3. Nur die **Erlaubten Dateien** ändern. Die **Schritte** der Reihe nach ausführen, **Nicht-Ziele** einhalten.
4. Alles unter **Fertig, wenn** abhaken, die Befehle unter **Prüfen** müssen grün sein (Prüfstufen: „Tempo im Lauf“).
   Maßstab sind die Kriterien der Session in der Sprint-README (SDD), nicht eine eigene Auslegung.
5. **Ergebnis** mit Nachweis je Kriterium ausfüllen, `Status: fertig` setzen, auch in der Session-Tabelle der Sprint-README. Neue Ideen oder
   Probleme als Ticket anlegen (Vorlage!) und in `backlog/README.md` eintragen; jedes neue Ticket gleich mit Projekt und
   Sprint (keine ungeplanten Tickets, Regel 🧑 2026-11-14, `plan_create` erzwingt es). Gemeinsame Planungsdateien
   (`backlog/README.md`, `sprints/README.md`) erst am Ende ändern. Ein Commit mit Session im Titel
   (`feat(srv): Restore absichern (F4.3)`), `git fetch`, `git merge origin/main`, push auf den Sprint-Branch.
   **Kein PR**: den öffnet erst die letzte Session des Sprints (Review, im Doku-Sprint die letzte); sie schaltet
   auch Auto-Merge ein (siehe Review-Session, Schritt 5). Alle anderen Sessions mergen nie.

**Wenn etwas nicht passt** (Schritt unklar, Befehl scheitert unerklärlich, nötige Datei nicht erlaubt, Kriterium
widerspricht dem Code oder einer Regel):
`Status: blockiert`, im **Ergebnis** Grund und bisherigen Stand notieren, ein Ticket vom Typ `Frage` anlegen,
bisherigen Stand auf den Sprint-Branch pushen und einen **Entwurfs-PR** des Sprints öffnen (falls noch keiner offen ist), **aufhören**. Nicht raten, nicht um die Regeln herum arbeiten.

**Testläufe** (Balancing, Performance, Stabilität) sind **bis auf Widerruf gesperrt** (Beschluss 🧑 2026-11-11, Abschnitt „Sim, Balance und Gen“): kein `sim_test`, kein `sim_run`, kein Balancing, kein Lasttest, auch nicht bei der Feature-Arbeit oder im Abnahme-Ticket. Nach dem Widerruf startet und überwacht sie ein Agent nur über das MCP-Tool `sim_test` der Workbench (B-348), nie in der Shell.

Eine Session pro Lauf. Eine Review-Session nie im selben Lauf wie eine Umsetzung. Sessions mit `Agent: Mensch`
(Xbox-Test, Workshop, Spieleabend) nimmt ein Agent nicht; er darf sie nur vorbereiten, wenn die Datei das verlangt.
Eine Mensch-Session ist nie Abhängigkeit einer Agenten-Session, außer ihr Ergebnis ist deren Eingabe (Workshop-Beschluss, Spieleabend-Protokoll); dann steht sie in `Abhängig von`.

### Tempo im Lauf (B-518)

Wartezeit ist der größte Kostenblock eines Laufs: auf Prüfläufe, Reviews und Freigaben. Deshalb:

- **Prüfstufen:** Während der Arbeit nur gezielte Tests (`go test ./engine/sim/ -run TestX`, `check_run go:test` oder
  `task:test` mit `pattern`). Vor dem Commit **genau einmal** `task check:diff` (B-730, Werkzeug `tools/checkdiff`): Es prüft
  **nur, was die geänderten Dateien dieses Commits brauchen** (Beschluss 🧑 2026-11-11). Basis ist `HEAD` (geänderte und neue
  Dateien; bei sauberem Checkout der letzte Commit), nicht mehr der ganze Sprint; `CHECK_BASE=origin/main` nimmt bewusst mehr.
  **Stufe 1 (Sekunden):** Planungs-Tests nur bei Doku, Regel-Test, Oxlint, tsc, `go vet` der geänderten Pakete **und ihrer
  Abnehmer** (übersetzt sie samt Tests, führt nichts aus), golangci-lint der geänderten Pakete. **Stufe 2, nur nach grüner
  Stufe 1:** `go test -short` **nur der geänderten Pakete** und Vitest `--changed`. Tests der Abnehmer, volle Module, Sim,
  Balance, Gen und Race laufen hier nie; `go.mod` übersetzt nur das Modul, `Taskfile.yml` und `.github/` lösen keine Tests aus,
  `testdata/` an der Wurzel nur die Pakete, die es lesen. JS und Go laufen nebeneinander.
  **Teilaufgaben:** `task check:diff:plan` zeigt nur, was laufen würde; nach einem roten Lauf wiederholt `task check:diff:js`
  bzw. `task check:diff:go` nur die betroffene Bahn. Ein Lauf über mehr als etwa eine Minute ist ein Befund (Ticket), kein Normalfall.
  **Takt:** einmal je Session-Commit. Nach einem Merge der Basis in den Sprint-Branch kein neuer Lauf, wenn der eigene
  Diff unverändert ist (die Basis ist schon geprüft). Den vollen Satz (`task check`, `task check:go`) fährt nur
  die Review-Session, `task check:race` nur das Abnahme-Ticket, `task check:all` nur der Release. Ein roter Lauf wird behoben und einmal wiederholt, nicht
  mehrfach „zur Sicherheit“.
- **Lange Tests (B-619):** Ein Go-Test, der über etwa 3 s braucht (echte Git-Repos, `go build`, lange Lauf-Simulationen,
  Repo-Scans), beginnt mit `if testing.Short() { t.Skip(…) }`. `check:diff` lässt ihn aus, `check:go`, die
  Review-Session und die CI fahren ihn. Unveränderte Pakete liefert der Go-Testcache; er greift nur, weil das Go-Temp
  außerhalb des Repos liegt (`%USERPROFILE%\.k3c\tmp`, `requirements.md`). Für diesen Ordner braucht der Virenscanner (Norton) einmal eine Ausnahme, sonst prüft er jede Test-EXE (Einrichtung: `requirements.md`).
- **Review in der Umsetzung:** nur bei Domäne SIM oder SRV, bei Protokoll- oder Spielstand-Änderungen, bei Code mit
  Shell-Aufrufen, Pfaden oder Eingaben von außen oder ab etwa 150 geänderten Code-Zeilen. Ein Reviewer (Sonnet; Opus nur bei Protokoll, Spielstand oder Nebenläufigkeit), Maßstab wie in
  der Review-Session (nur schwere Befunde). Er startet im Hintergrund **parallel** zu `task check:diff`. Ein Fix unter
  etwa 30 Zeilen bekommt kein zweites Review. Doku-, Planungs- und reine Werte-Sessions haben kein eigenes Review.
- **Parallel statt nacheinander:** Unabhängige Teile einer Session laufen als parallele Agenten in getrennten Dateien
  (z. B. Kontext erheben ∥ Tests schreiben ∥ Umsetzung). Sessions verschiedener Sprints ohne gegenseitige Abhängigkeit, ohne gemeinsame Dateien
  und ohne gemeinsame Sperre laufen parallel, jede im eigenen Worktree (Abschnitt „Parallelität und Wellen“). Schwere Prüfläufe (Grafik-Golden,
  `check:go`, `check:race`, `buildall`) laufen höchstens zwei gleichzeitig; mehr bringt lastabhängige Tests zum Scheitern (B-704).
- **Kein Polling:** `check_run` wartet selbst bis zum Ende des Laufs (B-520); Shell-Läufe im Hintergrund mit
  Benachrichtigung (`run_in_background`); `sim_test status` nur einmal nach der erwarteten Dauer abfragen. Keine `sleep`-Schleifen; Messreihen höchstens drei Läufe;
  `sim_test` immer mit Dauer und Abbruchkriterium.
- **Planung in wenigen Aufrufen:** mehrere Abschnitte mit `plan_sections`. Das Ergebnis nennt je Kriterium höchstens
  zwei Zeilen; Einzelheiten stehen im Commit und im Code.
- **Lesen nach Bedarf:** Session und Kriterien liefert `plan_next`; das Glossar beim Schreiben von Specs und Texten,
  `codemap.md` nur für unbekannte Bereiche.

### Sim, Balance und Gen (B-742)

**Sperre (Beschluss 🧑 2026-11-11, bis auf Widerruf):** Simulationen und Balancing laufen **gar nicht**: kein `sim_test`,
kein `sim_run`, kein `task balance`, kein Lasttest (`k3c-load` ist entfallen, B-850), weder bei der Feature-Arbeit noch im Abnahme-Ticket. Die
Kriterien AC-01 Sim und AC-02 Balance eines Abnahme-Tickets heißen so lange `verschoben (Sperre 2026-11-11)`. `check:diff`
lässt die Tests der Sim-, Balance- und Lasttest-Pakete aus (`tools/checkdiff/steps.go` › `simDirs`; `go vet` und Lint bleiben).
Gen und Race bleiben wie unten beschrieben.

Lange Läufe gehören nicht in den Abschluss. **Sessions, Sprints und Projekte schließen ohne** Sim-, Balance- und
Gen-Läufe ab; auch `task check:diff` fährt sie nicht (kein `gen:paper`, kein Grafik-Golden, kein `sim_test`, kein
Balancing, kein `check:race`; Vitest lässt dort die Tests aus, die gerenderte Papierschnitt-PNGs brauchen).

- **Bei der Feature-Arbeit:** Nur die Session, die ein betroffenes Feature entwickelt, fährt die passenden Läufe gezielt
  (z. B. `sim_test` nach einer Balancing-Änderung, `task gen:paper` und Golden nach einer Spec- oder Renderer-Änderung).
  `task test` und `task build` rufen `gen:paper` weiter selbst auf.
- **Abnahme-Ticket:** Zum Abschluss eines Sprints legt die Review-Session (Doku-Sprint: letzte Session) **genau ein**
  Ticket für diese Läufe an; zum Abschluss eines Projekts ebenso genau eins, das noch offene Abnahme-Tickets seiner
  Sprints aufnimmt (diese `verworfen` mit Verweis). Claude arbeitet es autonom ab, gedacht für nachts; der Sprint wartet
  nicht darauf und wird trotzdem `erledigt`.
- **Anlegen:** `plan_create` (Vorlage `ticket.md`), Titel „Abnahme-Läufe <Sprint oder Projekt>“, Domäne der letzten
  Umsetzungs-Session, Typ `Schuld`, Prio `mittel`, Umgebung `live`, Felder `Sprint` und `Projekt` gesetzt,
  `Spec: freigegeben`, Freigabe „Regel B-742 (Abnahme-Ticket)“. Die Kriterien nennen nur die Läufe, die der Sprint
  braucht (was er nicht berührt, `nicht relevant` mit Grund):
  - **AC-01 Sim:** `sim_test` (Stabilität, Performance von Server und Client) mit Dauer und Abbruchkriterium, grün.
  - **AC-02 Balance:** Balancing-Tests über `sim_test` bzw. `sim_run` (vorläufig mit Cheats, B-698), grün.
  - **AC-03 Gen:** `check_run task:test` (ruft `gen:paper` auf, fährt Grafik-Golden `tests/paperAssets.test.ts`,
    `tools/paper/golden` und die Sprite-Tests), grün; eine gewollte Golden-Änderung nach „Golden aktualisieren“.
  - **AC-04 Race:** `check_run task:check:race`, grün (ohne C-Compiler übersprungen, im Ergebnis so nennen).
- **Ergebnis:** Rot heißt neues Ticket (Typ `Problem`) im Projekt, nicht Nachbessern im Abnahme-Ticket; danach
  `Status: erledigt` mit den Messwerten in den Notizen.

### Ausführung mit Superpowers

Verbindlich ab PV1 für jeden Sprint (Vorgabe 🧑, 2026-11-06): Agenten arbeiten mit den Superpowers-Skills.

- **Sprint:** `superpowers:executing-plans`; der Plan sind Sprint-README und Session-Dateien.
- **Session:** `superpowers:test-driven-development` (Red → Green → Refactor): erst der fehlschlagende Test, dann der Code.
- **Vor jedem Commit:** `superpowers:verification-before-completion` – die Befehle unter **Prüfen** frisch laufen lassen
  und das Ergebnis belegen; keine Erfolgsmeldung ohne Nachweis.
- **Review vor dem Commit:** `superpowers:requesting-code-review` statt eines freien Reviewer-Agenten. Wann reviewt wird,
  regelt weiter „Review in der Umsetzung“ oben (Risiko-Regel aus `CLAUDE.md`).

## Domänen

Jede **Session** gehört zu **genau einer Domäne** (Feld `Domäne`) und ändert nur deren Dateien (plus Tests und Doku dazu).
Ein Sprint darf Sessions mehrerer Domänen **nacheinander** enthalten (z. B. SIM → SRV → CLI); sein Feld `Domäne` nennt
sie in der Reihenfolge der Sessions. Was eine Domäne außerhalb des Sprints braucht, wird ein Ticket.
**Sperre:** Maßgeblich sind die Dateien: Zwei Sessions kollidieren, wenn sie zum **selben Sprint** gehören, ihre **Erlaubten Dateien** überlappen oder sie
dieselbe **Sperre** tragen (Abschnitt „Parallelität und Wellen“). Die Domäne ordnet Zuständigkeit und Commit-Titel.
`plan_next` und `plan_waves` prüfen das aus den Erlaubten Dateien und der Sperre der Sprint-README (B-783). Nur wenn einer Session die Erlaubten Dateien
fehlen, sperrt `plan_next` noch nach Domäne und `Lane` (PV1.1).
Sessions **eines** Sprints laufen immer nacheinander (ein Sprint-Branch, zwei Worktrees checken ihn nicht zugleich aus; entschieden 2026-11-14, B-838):
`plan_waves` setzt sie in Folgewellen, `plan_next` schlägt sie nicht als „Parallel möglich“ vor und sperrt den Sprint, solange eine seiner Sessions in
Arbeit ist. Parallelität entsteht zwischen Sprints. Siehe Autonomer Ablauf, Schritt 1.

| Kürzel | Domäne | Dateien |
|---|---|---|
| **REG** | Regelwerk & Balancing | `docs/game-design.md`, `docs/rules/`, `docs/playtests/`, **Werte** in `data/*.json` |
| **SIM** | Spiel-Logik (Go) | `engine/sim/`, `engine/level/`, **neue Felder** in `data/*.json` |
| **SRV** | Räume und Spielstände (Go) | `engine/room/`, `engine/store/` (Spielserver, `engine/net/`, `cmd/k3c-server/` und Docker sind entfallen, B-850) |
| **CLI** | Natives Spiel (Rust): Darstellung, HUD, Menü, Grafik, Audio | `rust/crates/k3c-game/`, `rust/crates/k3c-rig/` (Go-Spiel seit RU37 entfernt, Browser-Client entfallen, B-848) |
| **PLAT** | Plattform: Eingabe, Fenster, Pages-Präsentation | `rust/crates/k3c-game/src/input/`, Pages-Präsentation `site/` |
| **INF** | Frameworks, Tooling, CI, Repo-Aufbau, Arbeitsweise | `package.json`, `go.mod`, `vitest.config.ts`, `tsconfig.json`, Lint-Konfiguration, `.github/`, `tests/projectRules.test.ts`, `tests/planning.test.ts`, `docs/arbeitsweise.md`, `docs/vorlagen/` |
| **PAP** | Papierschnitt-Grafik (Entscheidung 005) | `tools/paper/`, `public/grafik/stil.paper.json`, `public/grafik/**/*.paper.json` samt den daneben gerenderten Ausgaben (PNG, `manifest.json`, Gruppen-Index), `data/asset-prompts.json`, `docs/standards/paper-*.md` |
| **DEV** | Entwickler-Werkzeug | `tools/k3c-workbench/` (eigenes Go-Modul mit Oberfläche – Bereiche Development und Assets, Entscheidung 005 › 5 – und MCP-Server), `tools/assetkit/` (Asset-Logik der Workbench, eigenes Modul), `cmd/k3c-sim/` (Engine-CLI der Workbench) |

**Modell-Wahl** (Feld `Modell` der Session, Empfehlung für den ausführenden Agenten): `haiku` für reine Nachschlage-
und Doku-Sessions, `sonnet` Standard für Umsetzung und Migration, `opus` für Verträge, Rig-Mathematik, Bild-Diff und
Sprint-Reviews, `fable` nur auf Anweisung 🧑. Ohne Feld (oder `–`) gilt `sonnet`. Höchstens **vier Agenten** arbeiten
gleichzeitig (vier Plätze über alle Checkouts); `plan_next` schlägt neben der gewählten Session höchstens drei parallele vor.

Grenzfälle:

- **Planungs-Dateien** (`docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`) darf jede Session für Status und Tickets ändern.
- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Protokoll** (`docs/protocol.md`) ist seit dem Rückbau des Spielservers Historie (B-850, B-897); Verträge sind das
  Spielstand-Format und die Schnittstelle von `cmd/k3c-sim` (Kopfkommentar `main.go`).
- **Feature-Kette:** Ein neues Spielelement läuft als REG → SIM → CLI, als Sessions eines Sprints oder als
  aufeinanderfolgende Sprints desselben Projekts.

## Parallelität und Wellen

Ziel: Was ohne 🧑 und ohne Kollision gleichzeitig gehen kann, läuft gleichzeitig (Beschluss 🧑 2026-11-11). Eine **Arbeitswelle** (kurz Welle; nicht die
Gegner-Welle des Spiels) ist die Menge der Sessions, die sich weder über Abhängigkeit noch Bereich noch Sperre berühren.

### Planungsangaben

Jedes Ticket und jede Sprint-README trägt im Abschnitt „Regeln und Einschränkungen“ je eine Zeile (Vorlagen), Pflicht ab `Spec: freigegeben`:

| Angabe | Werte | Bedeutung |
|---|---|---|
| **Bereich** | Pfad-Globs, kommagetrennt (z. B. `tools/paper/cli/**`) | alle Dateien, die die Arbeit anfasst, ohne Sammeldateien; bei Sessions sind es die Erlaubten Dateien |
| **Abhängig von** | `B-…`, `SPnn`, `SPnn.m` oder `–` | nur IDs, kein Freitext: erst starten, wenn der andere Stand erledigt **und gemergt** ist |
| **Braucht 🧑** | `–`, Freigabe, Entscheidung, Abnahme, Gerät | `–` heißt vollständig autonom; alles andere steht in der Ralf-Queue |
| **Größe** (Ticket) | S, M, L | S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen |
| **Sperre** | `–` oder Name aus der Tabelle unten | Ressource, die nur ein Sprint zugleich ändert |

`Umgebung: ?` ist ab `Spec: freigegeben` nicht mehr erlaubt. Worktree-tauglich ist, was `offline` ist und 🧑 nicht braucht.

### Sperren

| Name | Gilt für |
|---|---|
| `golden` | `testdata/golden/`, Grafik-Golden unter `tools/paper/golden/` (Abschnitt „Golden aktualisieren“) |
| `protokoll` | `docs/protocol.md` (Historie seit B-850) |
| `spielstand` | Spielstand-Format, `IslandSaveVersion`, Fixtures |
| `deps` | `go.mod`, `package.json`, Lockfiles, jede neue Abhängigkeit |
| `taskfile` | dieselbe Taskfile- oder Workflow-Datei (`Taskfile.yml`, `.github/workflows/*.yml`) |
| `exe` | `task buildall`, Neubau oder Neustart der Workbench (laufende EXE, Port 5180) |

Zwei Sprints mit derselben Sperre laufen nie in derselben Welle. Eine neue Sperre braucht eine Zeile in dieser Tabelle.

### Sammeldateien

`docs/backlog/README.md`, `docs/sprints/README.md`, `docs/projekte/README.md` und `docs/glossar.md` berührt fast jede Session. Sie zählen nicht zum
Bereich und werden nur über `plan_*` oder im **letzten Commit** des Sprints nach `git merge origin/<Basis>` geändert. Bei einem Konflikt dort gilt
der Stand der Basis; die eigene Änderung wird mit `plan_*` oder dem kleinen Edit neu ausgeführt, Tabellenzeilen werden nicht von Hand zusammengeführt.

### Wellen berechnen

`plan_waves {slots?, project?, domain?}` rechnet die Wellen so (B-783) und nennt je zurückgestelltem Kandidaten den Grund:

1. **Kandidaten:** offene Sessions `Agent: autonom` aktiver Sprints, dazu Tickets und Sprints mit `Spec: freigegeben` und `Braucht 🧑: –`.
2. **Ralf-Queue zuerst:** Alles mit `Braucht 🧑` ≠ `–` wird 🧑 vorab und gesammelt vorgelegt. Es hält nur auf, was es in `Abhängig von` nennt.
3. **Ebene:** ohne Abhängigkeit Ebene 1, sonst eins mehr als die höchste Ebene der Abhängigkeiten. Werkzeug-Änderungen sind erst nach Merge, Pull und
   Neubau nutzbar; wer sie braucht, nennt sie in `Abhängig von`.
4. **Kollision:** Zwei Kandidaten einer Ebene kollidieren, wenn sie Sessions desselben Sprints sind (B-838), ihre Bereiche überlappen (ein Pfad ist
   Präfix des anderen) oder sie dieselbe Sperre tragen. Der Kandidat aus dem Projekt mit dem kleineren Rang bleibt (danach Ticket-Prio, dann kleinere Nummer), der andere rückt in die nächste Welle.
5. **Plätze:** höchstens vier Agenten gleichzeitig; Sessions `in Arbeit` zählen mit, gleich in welchem Checkout.
6. **Folgewelle:** Sie startet, wenn die Sprint-PRs gemergt sind, von denen sie abhängt. Ihre Specs werden 🧑 schon während der laufenden Welle vorgelegt.

### Zuschnitt

- **Ein Ticket, eine Absicht, eine Dateimenge.** Tickets mit überlappendem Bereich werden ein Ticket (Kriterien je Teil) oder ein Sprint mit
  Sessions nacheinander, nie mehrere Einzel-Tickets nebeneinander.
- **Ein Sprint, eine Dateimenge.** Was eine andere Dateimenge braucht, wird ein eigener Sprint und kann parallel laufen.
- **Abhängigkeiten eines Sprints** stehen als IDs in seiner README, nicht als Prosa in den Sessions.
- **Bereit heißt aktuell:** Ein Sprint mit `Reife: bereit`, der 14 Tage nicht aktiviert wurde, bekommt vor dem Aktivieren einen Spec-Check (Pfade,
  Abhängigkeiten, Begriffe) oder fällt auf `Reife: Entwurf` zurück. Ein Sprint ohne offene Session wandert sofort nach `erledigt/`.

### Ralf-Queue

Was 🧑 braucht, wird gebündelt statt je Sprint einzeln vorgelegt; `plan_queue {device?}` stellt es zusammen (B-783). Das Gerät einer Mensch-Session
steht in einer Zeile `Gerät: …` im Abschnitt Kontext, sonst gilt „unbekannt“:

1. **Freigaben** (Klasse B, Abschnitt SDD) mit Default, sodass ein „ja, alle“ genügt.
2. **Entscheidungen:** Offene Fragen mit Vorschlag.
3. **Abnahmen je Gerät:** PC · Xbox/TV · Pi · Mini-PC · Workshop · Veröffentlichung. Ein Termin je Gerät; die Mensch-Sessions bleiben in ihren
   Sprints und stehen im Fahrplan unter „Offen am Gerät“ nach Gerät geordnet.

### Worktrees

- Ein **Wellen-Auftrag** von 🧑 („Welle starten“, „parallel abarbeiten“) ist die ausdrückliche Anweisung für Worktrees (`CLAUDE.md`) und gilt für
  alle Sessions dieser Welle. Ohne Auftrag bleibt ein Agent im aktuellen Checkout.
- Jede Session einer Welle arbeitet im eigenen Worktree auf ihrem Sprint-Branch; schreibende MCP-Aufrufe nennen `checkout`.
- In einem Checkout, in dem eine andere Session arbeitet, wechselt niemand den Branch, stasht oder setzt zurück.
- **Sim-Code (B-925):** Änderungen an `engine/sim`, `engine/level`, `engine/room`, `cmd/k3c-sim` und Daten in `data/`, die Sim, Level oder Balancing betreffen, laufen in einem eigenen Worktree mit eigener
  Dev-Instanz der Workbench, denn die Workbench rechnet die Engine im eigenen Prozess (B-924); eine Änderung in der Wurzel träfe die laufende Instanz.
  Im Worktree startet `task k3c-workbench:dev:worktree` Backend und Vite auf eigenen Ports (Versatz aus dem Worktree-Pfad, MCP 6001–6399, API 8601–8999,
  Vite 7001–7399, nie 5180, 5190, 5191; der Task nennt `K3C_DEV_PORT`). Geprüft wird gegen diese Instanz (`checkout`-Argument der MCP-Tools), am Ende
  wird sie gestoppt. Das gilt als ausdrückliche Anweisung für den Worktree (`CLAUDE.md`); die Wurzel-Workbench bleibt unberührt. Den Ablauf führt der
  Agent `k3c-sim-dev`. `sim_test start`, `sim_run`, `replay_run` und `level_generate` lehnen einen fremden `checkout` ab (Hinweis auf den Task); der Task setzt `K3C_DATA_DIR=<Worktree>/data`. Sim und Balancing bleiben gesperrt.
- Ticketnummern: `plan_create` kennt nur den eigenen Checkout. Vor dem Anlegen `git fetch` und die höchste Nummer aller offenen Branches prüfen
  (`git ls-tree -r --name-only <branch> docs/backlog`), sonst vergeben zwei Branches dieselbe Nummer.

### Temporäre Dateien

Regel (🧑 2026-11-18): Jeder Ordner außerhalb des Repos ist für Schreibarbeiten verboten; kein Temp, `%TEMP%`, `/tmp` oder Scratchpad. Temporäre Dateien und Worktrees liegen in `.work/` (ignoriert) und werden nach der Nutzung wieder gelöscht.
Gilt für Agenten, Skills und Werkzeuge; der Code der Produkte (z. B. `t.TempDir()` in Tests) bleibt davon unberührt.

### Second Worker (zweiter Rechner, Lane `zweit`)

Ein zweiter Rechner mit kleinerem Account arbeitet einfache autonome Sprints ab; Abstimmung nur über `origin` (🧑 2026-11-18).

- **Zuteilung:** Sessions mit `Lane: zweit` gehören dem Second Worker, alle anderen dem Hauptrechner. Der Hauptrechner
  vergibt die Lane; der Second Worker nimmt nie Sessions ohne `Lane: zweit`. Geeignet sind Oberflächen-Ports mit eigenem
  Ordner (`menu/`, `hud/`, `devtools/`), nicht Sim, Level, Plattform oder Planung über mehrere Sprints.
- **Vorrang (Ralf 2026-11-19):** Die Sprints des Go-Abbaus **RU58, RU37 und RU59** (Projekt RSU, in dieser Reihenfolge)
  gehen vor jeder anderen `zweit`-Session, auch vor RU51 und RU52. Der Second Worker nimmt zuerst deren freie Sessions
  (Abhängigkeiten beachten: RU58 vor RU37 vor RU59, die Sessions in Tabellen-Reihenfolge); erst wenn dort keine frei ist
  oder alle erledigt sind, folgen die übrigen. Technisch ordnet die Sprint-Reihenfolge in `docs/projekte/RSU-rust-umstellung.md`
  (`plan_next` liest sie); diese Regel gilt, bis die drei Sprints erledigt sind, danach entfällt sie.
- **Start je Session:** `git fetch`, Branch `sprint/<präfix>` von `origin/sprint/<präfix>` oder `origin/main`, nächste offene Session
  mit `Lane: zweit` und erledigten Abhängigkeiten (`plan_list {projekt: "RSU", status: "aktiv"}`). Ein Commit je Session, Push nach jeder Session.
- **Abschluss:** Review-Session des Sprints öffnet den PR gegen `main` und mergt ihn nach grüner Prüfung (`gh pr merge --squash --auto`).
- **Sim, Balancing und Messläufe (🧑 2026-11-18):** Die Sperre gilt nur für den Hauptrechner. Der Second Worker hat die bessere Hardware
  (Grafikkarte, CPU, Speicher) und darf `sim_test`, `sim_run`, Balancing, `gen:paper` und `check:race` in Sessions mit `Lane: zweit` fahren; die
  langen Läufe gehören dorthin: Der Hauptrechner ist Ralfs Arbeitsplatz, Sim- und Messläufe stören dort; der Second Worker ist sonst idle
  (🧑 2026-11-18). Rechenlastige Sessions (Sim, Balancing, Messung, `check:race`) bekommen deshalb `Lane: zweit`. Läufe nur über `sim_test`, blockierend gewartet. Sim-Code ändert er nur im eigenen Worktree (B-925).
- **Keine Tickets und Sprints anlegen:** Befunde schreibt der Second Worker ins Ergebnis seiner Session (Abschnitt „Für den
  Hauptrechner“); der Hauptrechner legt daraus Tickets an. So kollidieren keine Nummern.
- **Gemeinsame Dateien** (`Cargo.lock`, `lib.rs`-Modulzeilen, Indizes): vor dem Push `git merge origin/main`; Index-Konflikte über `plan_index` lösen.

## Projekte und Rang

Ein **Projekt** bündelt die Sprints eines Themas (z. B. Grafik, Sound, Leistung & Stabilität) in fester Reihenfolge.
Datei `projekte/XXX-name.md` nach Vorlage, `XXX` = Kürzel aus drei Großbuchstaben.

- **Status:** `aktiv` (wird abgearbeitet), `ruht` (zurückgestellt, wird nie gewählt) oder `erledigt`.
- **Rang:** Nur aktive Projekte haben einen Rang, eindeutig und lückenlos ab 1; er ist die **einzige Reihenfolge** der
  Arbeit. Den Rang setzt 🧑. `ruht` und `erledigt` tragen `Rang: –`.
  Der Rang entscheidet bei Kollision und bei zu wenigen Plätzen, wer zuerst läuft; er hält keine Arbeit zurück, die nicht kollidiert.
- **Sprints:** Die Tabelle im Projekt nennt seine Sprints in Abarbeitungs-Reihenfolge; jeder Sprint nennt sein Projekt
  im Feld `Projekt`. Je Projekt dürfen **mehrere Sprints gleichzeitig aktiv** sein; was sie trennt, sind nur
  Abhängigkeiten und die Sperre je Domäne (Abschnitt „Domänen“).
- **Tickets** tragen ihr Projekt im Feld `Projekt`, auch ohne Sprint. Die **Ticket-Prio** ordnet Tickets innerhalb eines
  Projekts beim Einplanen. `Ziel-Tickets` eines Projekts beschreiben das Thema als Ganzes (z. B. B-011).
- **Abnahmen am Gerät** sammelt das Projekt `ABN` ohne Rang; es läuft neben der Rangfolge, wann immer ein Gerät da ist.

## Sprint-Lebenslauf

1. **Geplant:** Ordner `sprints/geplant/SPnn-name/` mit `README.md` nach Vorlage, `Spec: Entwurf`. `Reife: Entwurf`
   erlaubt Stichpunkte für die Sessions, die Kriterien stehen trotzdem schon fest.
2. **Bereit machen** (Planung, meist am Ende des vorigen Reviews): Tickets sichten und bewerten, Offene Fragen klären,
   jede Session als Datei nach Vorlage schreiben (Felder `Domäne`, `Kriterien`), `Reife: bereit`. Nur der **nächste** Sprint je
   Projekt wird so detailliert. Danach 🧑 um Freigabe der Spec bitten.
3. **Aktivieren** (meist in der Review-Session des vorigen Sprints), nur mit `Spec: freigegeben`:
   `git mv docs/sprints/geplant/SPnn-name docs/sprints/aktiv/`,
   `Status: aktiv`, Fahrplan in `sprints/README.md` anpassen. Es gibt keine Obergrenze für aktive Sprints, weder je Projekt
   noch insgesamt: **Aktiviert werden darf jeder Sprint ohne offene Abhängigkeit** (Sprints, Tickets oder Sessions, die
   er laut „Regeln und Einschränkungen“ voraussetzt, sind erledigt). Hat ein Sprint eine offene Abhängigkeit, wartet er.
   Braucht er 🧑 (`Braucht 🧑` ≠ `–`), steht er bis dahin in der Ralf-Queue.
   Die Aktivierung ist der erste Commit auf `sprint/<präfix>`; ein Sprint mit Branch auf `origin`
   (`git ls-remote --heads origin 'sprint/*'`) gilt als aktiv, auch solange sein PR noch offen ist.
   Das Feld `Start-Commit` setzt die **erste Session** des Sprints: `git rev-parse --short origin/main` beim Anlegen des Sprint-Branchs.
4. **Abschließen:** Erledigt ist ein Sprint erst, wenn **alle** Sessions `fertig` oder `verworfen` sind. Die letzte
   davon (meist das Review, sonst die letzte Hardware-Session) verschiebt den Ordner nach `sprints/erledigt/` und setzt `Status: erledigt`.
   Das Abnahme-Ticket für Sim, Balance und Gen (B-742) ist keine Session und hält den Abschluss nicht auf.

- **Reihenfolge:** Abgearbeitet wird nach dem **Rang** der Projekte und der Sprint-Tabelle im Projekt
  (Abschnitt „Projekte und Rang“); aktiviert werden darf jeder Sprint ohne offene Abhängigkeit (Schritt 3). Eine Sprint-Prio gibt es nicht mehr; die Prio gehört zum Ticket.
- **Umgebung:** Ticket und Session tragen `offline` (ohne laufende Dienste prüfbar: Code, Unit-/Mock-Tests, Werkzeuge
  ohne Serverzugriff) oder `live` (braucht laufenden Server, Browser oder Gerät); `?` nur bis zur Einordnung beim
  Einplanen. Sessions mit `Agent: autonom` und `Umgebung: offline` sind worktree-tauglich; `live` ist es mit den eigenen Ports der Workbench (`get_urls`), solange kein Gerät nötig ist.
  In einen Worktree wechselt ein Agent nur auf Anweisung von 🧑; der Wellen-Auftrag ist eine solche (Abschnitt „Parallelität und Wellen“ › Worktrees).
- **Klein:** 3–6 Sessions auf einer zusammenhängenden Dateimenge (Zuschnitt: „Parallelität und Wellen“); Mensch-Sessions am Gerät zählen nicht mit. In Code-Sprints ist die letzte das **Review**; Doku- und Planungs-Sprints (nur `docs/`)
  haben keins, ihre letzte Session schließt den Sprint ab (Schritte 4–5 der Review-Session). Mehr Arbeit → nächster
  Sprint im selben Projekt.
- **Blockade** (🧑 fehlt): Sprint bleibt aktiv, blockierte Session `Status: blockiert`. Die Arbeit geht im Projekt mit
  dem nächsten Rang weiter (Autonomer Ablauf, Schritt 1).
- **Verworfen:** Eine Session, die nicht mehr durchgeführt wird, bekommt `Status: verworfen` mit Grund im Ergebnis
  (z. B. in ein anderes Projekt verschoben); sie zählt beim Abschließen wie `fertig` (B-338).
- **Hardware entkoppelt** (Beschluss 🧑 2026-10-03): Alles, was ein Gerät braucht (Xbox, TV, Pi, Handy, Controller),
  wartet nicht auf die App und die App wartet nicht darauf. Bis zur Validierung gelten die **angenommenen Werte**
  (`plan-weiterentwicklung.md` § 11.6); Code und Doku nennen sie „angenommen (Quelle)“. Eine Hardware-Session
  (`Agent: Mensch`, Test oder Abnahme am Gerät) ist **keine Abhängigkeit** einer App-Session und auch nicht des
  Reviews: Das Review nimmt ab, führt das Kriterium als `angenommen, Validierung offen (Session)` und öffnet den PR,
  der Sprint bleibt aber **aktiv**, bis die Hardware-Session fertig ist (entkoppelt heißt: weiter zum nächsten Sprint,
  nicht erledigt). Offene Hardware-Sessions stehen im Fahrplan unter „Offen am Gerät“ und werden erledigt, wenn das
  Gerät da ist; weicht das Ergebnis von der Annahme ab, entsteht ein Ticket (die Arbeit dahinter läuft weiter).
  Abnahmen mehrerer Sprints am selben Gerät fasst die Ralf-Queue zu einem Termin zusammen.
- **Richtwert Session:** ein Commit mit ≤ ~400 geänderten Code-Zeilen (ohne Bilder, Daten-JSON, Lockfiles).
- **Übergang:** Sprints, die vor dieser Regel (2026-10-03) schon Session-PRs hatten (F4), schließen nach altem Ablauf ab
  (Review-Session mit eigenem PR). Ab dem nächsten aktivierten Sprint gilt ein PR je Sprint.
- **Projekt Pflicht (seit PJ3, B-359):** Jeder aktive und geplante Sprint und jedes offene Ticket trägt ein Projekt
  (`tests/planning.test.ts`). Die Sprint-Felder `Prio` und `Einschiebbar` sind entfallen (B-361).
- **Commit-Titel** mit der Domäne der Session: `feat(sim): Taunt`, `fix(srv): Raum aufräumen`, `docs(reg): Wirtschaft v1`.

## Review-Session (Sprint-Abnahme) 🔍

Leicht und billig: Die Automatik prüft die Komplexität, das Review sucht nur **schwere Fehler**. Ein günstiges
Modell (z. B. Sonnet) reicht.

1. `task check` und `task check:go` → grün. Damit gelten die Grenzen aus dem Komplexitäts-Budget als geprüft.
   Sim, Balance, Gen und `check:race` laufen nicht hier, sondern im Abnahme-Ticket (Schritt 4, B-742).
2. `git fetch && git diff origin/main...origin/sprint/<präfix>` lesen (alles, was der Sprint ändert, in **allen**
   Domänen seiner Sessions), **nur den Diff**, nicht jede Datei vollständig.
3. Nur diese Befunde zählen:
   - falsches Verhalten oder Datenverlust (Spielstände, Berichte, Dateien)
   - Sicherheit: Pfade, Shell-Aufrufe, ungeprüfte Eingaben von außen
   - Spiel-Logik nicht deterministisch (`Math.random()`), mit 2 Spielern kaputt
   - Regeln aus `CLAUDE.md` verletzt (Seiten, Vollbild, B-Taste)
   - ein Kriterium der Spec ohne Nachweis oder umformuliert, damit es zum Code passt

   Stil, Doku, Benennung, mögliche Vereinfachungen sind **kein** Befund. Schwere Befunde in den Domänen des Sprints
   im Review-Commit beheben, außerhalb → Ticket. Die Review-Session trägt die Domäne der letzten Umsetzungs-Session. Das Review läuft auf dem Sprint-Branch, nie im selben Lauf wie eine Umsetzung.
4. **Abnahme** in der Sprint-README, höchstens fünf Zeilen: Datum, Kriterien (Verweis auf die Session-Ergebnisse,
   `verschoben` mit Ticket), behobene Befunde, neue Tickets und das **Abnahme-Ticket** des Sprints (genau eins, anlegen
   nach „Sim, Balance und Gen“; bei Projektabschluss das des Projekts).
5. **Wip-Branches prüfen** (B-302): Was inhaltlich von `wip/<präfix>…` im Sprint-Branch steckt, kommt in den PR-Text unter „Zum Löschen“.
   Sind alle anderen Sessions fertig: Sprint-Ordner nach `sprints/erledigt/` verschieben, `Status: erledigt`; sonst bleibt
   er in `aktiv/` (Hardware entkoppelt). Fahrplan anpassen, committen, `git merge origin/main`,
   pushen und **den einen PR des Sprints** öffnen (Vorlage, eine Zeile je Session), danach **Auto-Merge einschalten**:
   `gh pr merge --auto --squash`. Ist der PR schon `CLEAN` (Konflikt-Check fertig), lehnt GitHub `--auto` ab: dann
   `gh pr merge --squash`. Im PR läuft nur der Konflikt-Check, die Tests laufen vorher lokal und nach dem Merge auf
   `main` (B-397). Ein Entwurfs-PR einer blockierten Session bekommt kein Auto-Merge.
6. **Version vorschlagen** (siehe „Entscheidungen und Versionen“): eine Zeile `Version: v… vorgeschlagen (Grund)` in der Abnahme; gesetzt wird sie erst nach Bestätigung durch 🧑.

## Komplexitäts-Budget

Niedrige Komplexität ist in **jeder** Session Pflicht, nicht erst im Review.

| Regel | Grenze | Gilt für |
|---|---|---|
| Zeilen pro Datei | 400 | Code und Tests |
| Zeilen pro Funktion | 60 | Code, nicht Tests |
| Verschachtelung | 4 | alles |
| Zyklomatische Komplexität | 15 | Code |

- Nur diese Grenzen gelten, geprüft von der Automatik. Kürzer ist besser, aber kein Befund.
- **Ausnahmen für Bestandscode** stehen mit ihrem Wert in `.oxlintrc.json`; der Wert darf nicht steigen.
- Keine neue Abhängigkeit ohne Ticket und Zustimmung von 🧑 (mit der Spec-Freigabe). Keine Abstraktion für nur einen Fall.
- Schichtgrenzen: `engine/sim` und `engine/level` importieren nichts aus `engine/room` und `cmd/`; `engine/` nichts aus
  `cmd/` und `tools/`. Das Rust-Spiel zeichnet nur und ruft die Simulation in `rust/crates/k3c-sim`/`k3c-room`; die Go-Engine `engine/` dient der Workbench (Entscheidung 013, Entwurf: keine neuen Mechaniken in Go).
- Werkzeuge: Oxlint (TypeScript), `golangci-lint` mit `funlen`, `gocyclo`, `depguard` (Go);
  Go-Verschachtelung: `engine/internal/rules/nesting_test.go`; Dateilänge beider Sprachen: `tests/projectRules.test.ts`.

### Namen und Ablage im Code

- **Datei- und Ordnernamen im Code sind englisch** (`engine/`, `cmd/`, `rust/`, `tools/`), camelCase in TypeScript,
  snake_case in Go, React-Komponenten wie ihre Komponente. Deutsch bleiben Doku, Spieltexte, Task-Namen, Seiten
  (`*.html`) und Begriffe aus dem Glossar in Kommentaren und Texten. Bestand wird angepasst, wenn eine Session die Datei
  ohnehin ändert.
- **Gruppen statt flacher Ordner:** In `engine/sim` beginnt jede Datei mit ihrer Gruppe (`building_`, `unit_`,
  `monarch_`, `enemies_`, `boss_`, `skills_`, `events_`, `island_`, `hub_`, `world_`, `save_`); Kern-Dateien
  (`world.go`, `types.go`, `data.go`, `common.go` …) haben keine. Ab etwa 30 Dateien bekommt ein Ordner Gruppen.
- Tests liegen neben dem Code; `tests/` nur für Regeln über das ganze Repo (TypeScript), Go-Regeln in
  `engine/internal/rules`. Gemeinsame Oberflächen-Bausteine der Wails-Programme liegen in `tools/webkit` (nächster Abschnitt).

### Oberflächen der Wails-Programme (Atomic Design)

Gilt verbindlich für jedes Wails-Programm mit Oberfläche (`tools/*/frontend`: k3c-workbench, Starter) und jedes neue.
Die Oberfläche ist React mit **Radix Themes**, aufgebaut nach Atomic Design; die gemeinsamen Ebenen liegen in `tools/webkit`:

| Ebene | Ort | Inhalt |
|---|---|---|
| lib | `tools/webkit/lib/` | ohne Oberfläche: Fehlertext, gemerkte Einstellungen, Debounce, Splitter-Rechnung |
| Atome | `tools/webkit/atoms/` (`@webkit/atoms`) | Radix-Grund-Atome (Layout, Text, Controls) und eigene wie `ActionButton`, `SelectField`, `StatusBadge`, `Splitter` |
| Moleküle | `tools/webkit/molecules/` (`@webkit/molecules`) | kleine Gruppen: `Card`, `Callout`, `NoticeCard`, `CheckField`, `SwitchField` |
| Organismen | `tools/webkit/organisms/` (`@webkit/organisms`) | Bereiche: `Dialog`, `DropdownMenu`, `Tabs`, `Table`, `AppHeader` |
| Templates | `tools/webkit/templates/` (`@webkit/templates`) | Rahmen: `Theme`, `AppFrame` (Kopfzeile, Reiter, Seite) |
| Seiten | Feature-Ordner der App (`src/<feature>/`) | setzen alles zusammen |

- **Importe nur nach unten:** lib ← Atome ← Moleküle ← Organismen ← Templates ← Seiten; `tools/webkit` importiert nichts aus einer App.
- **Radix nur über webkit:** App-Code importiert Radix Themes nie direkt, sondern über `@webkit/<ebene>` (außer `@radix-ui/themes/styles.css` in `main.tsx`; Tests ausgenommen).
- **Keine rohen Controls:** `<button>`, `<select>`, `<input>`, `<textarea>` stehen nur in `tools/webkit/atoms`. Eigenes CSS für Controls gibt es nicht, Aussehen kommt aus Radix-Props und `tools/webkit/theme.css`; App-CSS regelt nur das Layout.
- **App-eigene Teile** in `src/ui/{molecules,organisms,templates}` (Atome nur in webkit), sonst im Feature-Ordner. Was zwei Programme brauchen, wandert nach webkit.
- Geprüft von `tests/atomicDesign.test.ts` (Ebenen, Importrichtung, Radix-Importe, rohe Controls); webkit-Tests laufen im Vitest von k3c-workbench.

## Golden aktualisieren

Die Golden-Daten (`testdata/golden/`) halten fest, was Simulation und Level-Generator rechnen; die Go-Tests vergleichen Feld für Feld.

- **Erlaubt** nur bei einer gewollten Regel- oder Wertänderung (Beschluss, Ticket, Daten in `data/`). Bricht ein Golden-Test ohne solche Änderung, ist das ein Fehler im Code, kein Anlass zum Update.
- **Befehl:** `task go:golden` schreibt `sim-*.json.gz`, `campaign-*.json.gz` und `level-*.json` aus dem Ist-Zustand neu. Unveränderte Einträge behalten ihre Bytes; der Diff zeigt nur, was sich wirklich geändert hat.
- **Gepackt (GD1, B-624):** Die großen Läufe (`sim-*`, `campaign-*`) liegen als `.json.gz`, Inhalt und `snapshotEvery` wie vorher. `engine/internal/golden` liest und schreibt sie anhand der Endung; `-update` packt mit festem Kopf ohne Zeitstempel, gleicher Inhalt ergibt dieselben Bytes. Die `level-*.json` bleiben JSON, weil TS-Tests sie direkt importieren.
- **Lesbare Diffs:** `.gitattributes` setzt `*.json.gz diff=gzip`. Einmal je Klon `task go:golden-setup` (setzt `git config diff.gzip.textconv "gzip -dc"`), dann zeigen `git diff` und `git log -p` den entpackten JSON-Text.
- **Prüfen:** `git diff --stat testdata/golden` lesen. Ändert sich mehr als die Regel erklärt, Ursache klären und nicht einchecken.
- **Begründung im Commit-Text:** welche Regel, welcher Wert, welche Kennzahl sich dadurch ändert. Golden-Änderungen in eigenem Commit.
- **Bestätigung:** Kein Freigabe-Zwang durch 🧑 (Beschluss Q09); die Review-Session des Sprints prüft Diff und Begründung.
- **`rng.json` nie per Task ändern:** Sie ist die Referenz der Zufallsfolge (`engine/rng`); der Task fasst sie nicht an.
- Nur ein aktiver Sprint ändert Golden-Daten (kein zweiter parallel; `plan-weiterentwicklung.md` § 11.5).

## Spielstand-Format ändern

**Beschluss 🧑 (2026-10-05, B-288): Bis auf Widerruf nimmt keine Formatänderung Rücksicht auf alte Spielstände** (es gibt noch keine relevanten).
Eine Session baut keine Migration, kein neues Fixture und keinen Ladetest nur für alte Stände. Die Regel aus B-137 unten bleibt als Stand der Technik für den Widerruf, ihr vorhandener Code in `engine/sim/` bleibt bis zu einem eigenen SIM-Ticket unangetastet.

- **Formatänderung** des Spielstands (Feld neu, entfernt, umbenannt oder mit neuer Bedeutung): `IslandSaveVersion` (`engine/sim/island_save.go`) darf steigen; ohne Widerruf ist eine Migration nicht nötig.
- **Fixture:** Steigt die Version, entsteht das Fixture der neuen Version aus dem Code neu (`TestJedeVersionHatFixture` bleibt bestehen, bis 🧑 das Löschen beschließt); für ältere Versionen wird nichts ergänzt.
- **Test „alter Stand lädt“:** `engine/sim/save_migration_test.go` bleibt, eine neue Version braucht dort ohne Widerruf keinen eigenen Fall.
- **Neuere Version** (Stand von einem neueren Server): `ParseIslandSave` meldet gefundene und unterstützte Versionen und ändert keine Datei.

## Entscheidungen und Versionen

- Größere Entscheidungen als `docs/decisions/NNN-titel.md`: **Kontext · Optionen · Entscheidung · Folgen**, höchstens eine Seite.
- **Version nach jedem Sprint:** Jeder abgeschlossene Sprint endet mit einem **Versionsvorschlag**. Die Review-Session (im Doku-Sprint die letzte Session) trägt in der Abnahme `Version: vX.Y.Z vorgeschlagen (Grund)` ein: **Minor** (`v0.<n+1>.0`) bei Sprints mit Wirkung im Spiel oder im Werkzeug, **Patch** (`v0.<n>.<m+1>`) bei reiner Doku, Planung oder Korrektur ohne neue Funktion. Der Agent legt den Vorschlag 🧑 vor und setzt den Tag **nur bei ausdrücklicher Bestätigung**: `task check:all` grün, `git tag <Version>`, `git push origin <Version>`; der Release-Workflow (`.github/workflows/release.yml`) baut das native Spiel für Windows als Zip am GitHub-Release (Server-Dateien und Docker-Image sind entfallen, B-850). Ohne Bestätigung entsteht kein Tag. Die Abnahme vermerkt danach `Version: vX.Y.Z gesetzt` oder `nicht gesetzt (Grund)`.
- Zusätzlich ein Release-Tag nach jedem Spieleabend, auch wenn kein Sprint abschließt; vor jedem Tag gilt die Checkliste im Abschnitt „Release“.

## Release

**Auslöser (Q20, 2026-10-03):** nach jedem fertigen Sprint (Versionsvorschlag der Abnahme, B-180) und nach jedem Spieleabend. Tag-Schema: „Entscheidungen und Versionen“. Ein Tag entsteht nur nach Bestätigung durch 🧑 (`git tag vX.Y.Z` auf `main` nach dem Fast-Forward, Abschnitt „Branches“); `release.yml` reagiert auf `v*`. Ein Punkt rot → **kein Tag**, der Befund wird ein Ticket; ein Dev-Rest blockiert den Release, bis er entfernt ist.

| Punkt | Prüfen mit | Erwartet |
|---|---|---|
| Golden amd64 | CI-Job `go` (`task go:test`) auf dem Stand, der getaggt wird | grün |
| Golden arm64 | entfällt (Job `go-arm64` seit RU37.2 entfernt; die Go-Engine dient nur noch der Workbench auf Windows amd64) | – |
| Alles grün | `task check:all` | Exit 0 (Client, Build, Go, k3c-workbench) |
| Spielstand-Migration (Beschluss 2026-10-05, B-288: bis auf Widerruf keine Rücksicht auf alte Stände) | `engine/sim/save_migration_test.go` | vorhandene Tests grün; neue Version ohne Migration zulässig (Abschnitt „Spielstand-Format ändern“) |
| Dev-Reste aus (B-098, B-107, B-080) | Release-Build des Rust-Spiels ohne Dev-Modus (`engine/room/dev.go` lehnt Dev-Aktionen ab); Prüfweg im nativen Spiel noch offen (Spielserver und Browser-Client entfallen, B-850) | Dev-Aktion wird abgelehnt |
| Version stimmt (B-141) | `VERSION` im `Taskfile.yml` (`git describe`) und die Version im Menü › Lizenzen des Rust-Spiels (B-808) | beide nennen den Tag |
| Credits vollständig (B-165) | Menü › Lizenzen des Rust-Spiels (`rust/crates/k3c-game/src/menu/licenses.rs`; der Browser-Test `src/tools/credits.test.ts` ist entfallen, B-850) | jede Grafik- und Audio-Quelle hat einen Eintrag |
| Aufgeräumt (B-302) | `task ops:aufraeumen -- --dry` | keine Reste, sonst Hinweis (blockiert den Tag nicht) |
| Tag-Schema | „Entscheidungen und Versionen“ | Minor bei Wirkung, Patch bei Doku oder Korrektur |
