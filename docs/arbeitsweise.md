# Arbeitsweise

**Projekt** (Thema mit Rang) → **Sprint** (3–6 Sessions, ein PR) → **Session** (eine Domäne, ein Commit).
**Tickets** ([`backlog/`](backlog/)) gehören zu einem Projekt und werden in Sprints eingeplant. **Ein PR je Sprint.**
Ein Code-Sprint ist erst fertig, wenn seine **Review-Session** ihn abgenommen hat (leicht, siehe unten).
Übersicht der Projekte: [`projekte/README.md`](projekte/README.md), aller Sprints: [`sprints/README.md`](sprints/README.md).
Zielbild: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).

## Ablage

```text
docs/
  arbeitsweise.md             diese Datei (einzige Prozess-Beschreibung)
  vorlagen/                   Pflicht-Vorlagen: projekt.md, ticket.md, sprint.md, session.md
  projekte/README.md          Rangliste aller Projekte
  projekte/XXX-name.md        ein Projekt: Ziel, Status, Rang, Sprints in Reihenfolge
  backlog/README.md           Index aller Tickets (eine Zeile pro Ticket)
  backlog/B-NNN-kurzname.md   ein offenes oder eingeplantes Ticket pro Datei
  backlog/archiv/B-NNN-…      erledigte und verworfene Tickets (gleicher Dateiname, per `git mv`)
  sprints/README.md           Fahrplan: alle Sprints mit Ordner und Status
  sprints/aktiv/SPnn-name/    der laufende Sprint: README.md (Sprint) + SPnn.m-name.md (Sessions)
  sprints/geplant/…           kommende Sprints, gleicher Aufbau
  sprints/erledigt/…          abgeschlossene Sprints
```

**Lesen:** `projekte/README.md` und `sprints/aktiv/` immer, `sprints/geplant/` nur beim Planen, `sprints/erledigt/` und `backlog/archiv/` nur auf ausdrückliche Nachfrage.
Setzt eine Session ein Ticket auf `erledigt` oder `verworfen`, verschiebt sie es nach `backlog/archiv/` und die Index-Zeile in den Abschnitt „Archiv“.

**Vorlagen sind Pflicht.** Jedes Ticket, jeder Sprint und jede Session entsteht als Kopie der Vorlage aus
`docs/vorlagen/`. `tests/planning.test.ts` prüft Felder, Überschriften, Status passend zum Ordner, Index und Verweise.
Eine Abweichung lässt `npm test` scheitern.

**Planung nur über k3c-dev (B-210).** Anlegen, Felder ändern, Abschnitte füllen, archivieren, Sprints verschieben und
Entwürfe löschen laufen über die MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`,
`plan_delete`; sie ziehen Index, Session-Tabelle, Fahrplan und Ordner selbst nach. Handarbeit an diesen Dateien ist nur
der Rückfall, wenn k3c-dev nicht läuft; dann gelten die Regeln dieses Abschnitts wörtlich.
**Glossar ist verbindlich.** [`glossar.md`](glossar.md) legt die Begriffe fest und wird vor jeder Session gelesen. Ein neuer
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
- **Änderung:** Passt eine Anforderung während der Umsetzung nicht, erst die Spec ändern (`Revision` + 1, zurück auf
  `Entwurf`, neue Freigabe), dann den Code. Nie ein Kriterium umschreiben, damit es zum Code passt.
- **Nachweis:** Das **Ergebnis** der Session nennt je Kriterium `umgesetzt`, `geprüft` (womit), `verschoben` (Grund,
  Ticket) oder `blockiert`. Ein Kriterium wird nie still abgehakt.
- **Abnahme gehört 🧑:** Eine Spec-Freigabe ist keine Erlaubnis für manuelle Prüfungen. Browser-, Xbox- und TV-Prüfungen
  macht ein Agent nur, wenn die Session sie nennt und 🧑 sie für diesen Lauf freigegeben hat. Bestätigte Nachweise
  gelten weiter, solange ihre Grundlage unverändert ist; wer geprüft hat, steht im Ergebnis.

## Branches

- **`develop`** ist der Arbeitsstand. Jeder Sprint hat **einen** Branch `sprint/<präfix>` von `origin/develop`; `<präfix>` ist
  der Teil vor `/` im Feld `Branch` seiner Sessions (z. B. `f5/1-…` → `sprint/f5`). Alle Sessions committen dort,
  am Ende steht **ein PR je Sprint** gegen `develop`. Abgleich mit `develop` per `git merge origin/develop`, nie Rebase
  oder Force-Push auf einem geteilten Sprint-Branch.
- **Aufräumen (B-302):** Lokal und auf `origin` liegen nur `develop`, `main`, die Branches aktiver Sprints und ihre Worktrees. Vier Meilensteine:
  Session-Start (Autonomer Ablauf, Schritt 1: `task aufraeumen`) · Sprint erledigt (Review, Schritt 5: `wip/<präfix>…`-Branches des Sprints prüfen; was im Sprint-Branch steckt, steht im PR-Text unter „Zum Löschen“) ·
  Sprint-PR gemergt (🧑 löscht die genannten `origin/wip/…`-Branches; auf `origin` löscht nur 🧑) · Release (Zeile „Aufgeräumt“ der Checkliste, nur Hinweis).
  `task aufraeumen` löscht lokal nur Branches, deren Commits per `git cherry` in `origin/develop` stecken oder deren Upstream weg ist und deren Pull Request laut `gh` gemergt ist, dazu saubere Worktrees dieser Branches.
  Nie gelöscht werden `develop`, `main`, der ausgecheckte Branch und `sprint/*` mit Ordner in `docs/sprints/aktiv/`. Ohne `gh` oder Netz listet der Task nur.
- **`main`** ist geschützt und trägt nur Release-Stände. Nur 🧑 gleicht ihn bei einem neuen Major-Release ab,
  per Fast-Forward: `git fetch && git push origin origin/develop:main`, danach Tag `v*` auf `main`
  (startet `release.yml`; GitHub Pages baut aus `main`). Nie direkt auf `main` committen oder einen PR gegen `main` öffnen.

## Autonomer Ablauf (Mensch oder Cloud-Agent)

Eine Session muss **ohne Rückfragen und ohne Planungs-Werkzeuge** abzuarbeiten sein. Deshalb:

1. **Aufräumen:** `git fetch --prune`, dann `task aufraeumen` (löscht nur sichere lokale Branches und Worktrees, listet den Rest mit Grund; `task aufraeumen -- --dry` zeigt nur an).
   `docs/sprints/aktiv/*/README.md` lesen. Liegt `sprint/<präfix>` schon auf `origin` (`git ls-remote --heads origin
   sprint/<präfix>`), gilt der Stand der Sprint-Dateien dort (`git show origin/sprint/<präfix>:<Pfad>`), sonst der von
   `develop`. Die nächste Session ergibt sich aus dem **Rang** (`projekte/README.md`): aktives Projekt mit dem kleinsten
   Rang → seine aktiven Sprints in der Reihenfolge der Projekt-Tabelle → erste Session mit `Status: offen`,
   `Agent: autonom` und erledigten Abhängigkeiten. Gibt es dort keine, kommt das Projekt mit dem nächsten Rang. Die **Domäne** der Session darf nicht gesperrt sein:
   Steht auf einem anderen Sprint-Branch (`git ls-remote --heads origin 'sprint/*'`) eine Session derselben Domäne auf
   `in Arbeit`, die nächste passende Session nehmen. Gibt es keine: **nichts tun** und das melden.
2. Sprint-Branch auschecken (fehlt er: von `origin/develop` anlegen, die erste Session setzt `Start-Commit`).
   Session-Datei vollständig lesen, `Status: in Arbeit` setzen, committen und sofort `git push -u origin sprint/<präfix>`
   (beansprucht die Session; scheitert der Push, hat ein anderer Lauf sie: neu holen und bei 1. beginnen).
3. Nur die **Erlaubten Dateien** ändern. Die **Schritte** der Reihe nach ausführen, **Nicht-Ziele** einhalten.
4. Alles unter **Fertig, wenn** abhaken, die Befehle unter **Prüfen** müssen grün sein. Maßstab sind die Kriterien
   der Session in der Sprint-README (SDD), nicht eine eigene Auslegung.
5. **Ergebnis** mit Nachweis je Kriterium ausfüllen, `Status: fertig` setzen, auch in der Session-Tabelle der Sprint-README. Neue Ideen oder
   Probleme als Ticket anlegen (Vorlage!) und in `backlog/README.md` eintragen. Gemeinsame Planungsdateien
   (`backlog/README.md`, `sprints/README.md`) erst am Ende ändern. Ein Commit mit Session im Titel
   (`feat(srv): Restore absichern (F4.3)`), `git fetch`, `git merge origin/develop`, push auf den Sprint-Branch.
   **Kein PR**: den öffnet erst die letzte Session des Sprints (Review, im Doku-Sprint die letzte); sie schaltet
   auch Auto-Merge ein (siehe Review-Session, Schritt 5). Alle anderen Sessions mergen nie.

**Wenn etwas nicht passt** (Schritt unklar, Befehl scheitert unerklärlich, nötige Datei nicht erlaubt, Kriterium
widerspricht dem Code oder einer Regel):
`Status: blockiert`, im **Ergebnis** Grund und bisherigen Stand notieren, ein Ticket vom Typ `Frage` anlegen,
bisherigen Stand auf den Sprint-Branch pushen und einen **Entwurfs-PR** des Sprints öffnen (falls noch keiner offen ist), **aufhören**. Nicht raten, nicht um die Regeln herum arbeiten.

**Testläufe** (Balancing, Performance, Stabilität) startet und überwacht ein Agent nur über das MCP-Tool `sim_test` der Workbench (B-348), nie in der Shell; ohne laufende Workbench wartet der Testlauf.

Eine Session pro Lauf. Eine Review-Session nie im selben Lauf wie eine Umsetzung. Sessions mit `Agent: Mensch`
(Xbox-Test, Workshop, Spieleabend) nimmt ein Agent nicht; er darf sie nur vorbereiten, wenn die Datei das verlangt.

## Domänen

Jede **Session** gehört zu **genau einer Domäne** (Feld `Domäne`) und ändert nur deren Dateien (plus Tests und Doku dazu).
Ein Sprint darf Sessions mehrerer Domänen **nacheinander** enthalten (z. B. SIM → SRV → CLI); sein Feld `Domäne` nennt
sie in der Reihenfolge der Sessions. Was eine Domäne außerhalb des Sprints braucht, wird ein Ticket.
**Sperre:** Eine Session `in Arbeit` sperrt ihre Domäne; parallele Läufe arbeiten nur in verschiedenen Domänen
(siehe Autonomer Ablauf, Schritt 1).

| Kürzel | Domäne | Dateien |
|---|---|---|
| **REG** | Regelwerk & Balancing | `docs/game-design.md`, `docs/rules/`, `docs/playtests/`, **Werte** in `data/*.json` |
| **SIM** | Spiel-Logik (Go) | `engine/sim/`, `engine/level/`, **neue Felder** in `data/*.json` |
| **SRV** | Server & Betrieb (Go) | `engine/room/`, `engine/net/`, `engine/store/`, `cmd/k3c-server/`, Docker |
| **CLI** | Client: Darstellung, HUD, Grafik, Audio, Verbindung | `src/scenes/`, `public/`, `src/online/client.ts`, `src/core/saveStore.ts` |
| **PLAT** | Plattform: Eingabe, Shell, Seiten | `src/input/`, `src/core/shell.ts`, `src/core/fullscreen.ts`, `src/landing/`, `*.html`, Pages-Präsentation `site/` |
| **INF** | Frameworks, Tooling, CI, Repo-Aufbau, Arbeitsweise | `package.json`, `go.mod`, `vite*.ts`, `tsconfig.json`, Lint-Konfiguration, `.github/`, `tests/projectRules.test.ts`, `tests/planning.test.ts`, `docs/arbeitsweise.md`, `docs/vorlagen/` |
| **DEV** | Entwickler-Werkzeug | `tools/k3c-dev/` (eigenes Go-Modul mit Oberfläche, MCP-Server), `engine/studio/` und `tools/k3c-studio/` (Asset-Studio: Backend als Seite des Go-Servers, Frontend in `tools/k3c-studio/frontend`), `tools/assetkit/` (gemeinsame Asset-Logik von k3c-dev und Asset-Studio), `cmd/k3c-load/`, `cmd/k3c-tui/`, Werkzeug-Seiten `src/tools/` |

Grenzfälle:

- **Planungs-Dateien** (`docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`) darf jede Session für Status und Tickets ändern.
- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Protokoll** (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`, `testdata/protocol/`) betrifft
  Client und Server. Eine Änderung daran bekommt eine eigene Session, die nur das Protokoll und beide Enden anpasst.
- **Alt-Engine löschen (SP09, B-049):** INF löscht `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs`,
  `vite.server.config.ts` und zieht dafür Importe in `src/scenes/`, `src/online/`, `src/core/`, Dev-Server, `Taskfile.yml`, CI,
  `tools/k3c-dev/services.json` und die Landingpage-Hinweise für GitHub Pages (B-032) nach. Neue Funktionen gehören nicht dazu.
- **Feature-Kette:** Ein neues Spielelement läuft als REG → SIM → CLI, als Sessions eines Sprints oder als
  aufeinanderfolgende Sprints desselben Projekts.

## Projekte und Rang

Ein **Projekt** bündelt die Sprints eines Themas (z. B. Grafik, Sound, Leistung & Stabilität) in fester Reihenfolge.
Datei `projekte/XXX-name.md` nach Vorlage, `XXX` = Kürzel aus drei Großbuchstaben.

- **Status:** `aktiv` (wird abgearbeitet), `ruht` (zurückgestellt, wird nie gewählt) oder `erledigt`.
- **Rang:** Nur aktive Projekte haben einen Rang, eindeutig und lückenlos ab 1; er ist die **einzige Reihenfolge** der
  Arbeit. Den Rang setzt 🧑. `ruht` und `erledigt` tragen `Rang: –`.
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
   Die Aktivierung ist der erste Commit auf `sprint/<präfix>`; ein Sprint mit Branch auf `origin`
   (`git ls-remote --heads origin 'sprint/*'`) gilt als aktiv, auch solange sein PR noch offen ist.
   Das Feld `Start-Commit` setzt die **erste Session** des Sprints: `git rev-parse --short origin/develop` beim Anlegen des Sprint-Branchs.
4. **Abschließen:** Erledigt ist ein Sprint erst, wenn **alle** Sessions `fertig` oder `verworfen` sind. Die letzte
   davon (meist das Review, sonst die letzte Hardware-Session) verschiebt den Ordner nach `sprints/erledigt/` und setzt `Status: erledigt`.

- **Reihenfolge:** Abgearbeitet wird nach dem **Rang** der Projekte und der Sprint-Tabelle im Projekt
  (Abschnitt „Projekte und Rang“); aktiviert werden darf jeder Sprint ohne offene Abhängigkeit (Schritt 3). Eine Sprint-Prio gibt es nicht mehr; die Prio gehört zum Ticket.
- **Umgebung:** Ticket und Session tragen `offline` (ohne laufende Dienste prüfbar: Code, Unit-/Mock-Tests, Werkzeuge
  ohne Serverzugriff) oder `live` (braucht laufenden Server, Browser oder Gerät); `?` nur bis zur Einordnung beim
  Einplanen. Nur Sessions mit `Agent: autonom` und `Umgebung: offline` dürfen in einem eigenen Worktree ohne Rückfrage
  laufen; `live` arbeitet im Checkout mit den laufenden Diensten.
- **Klein:** 3–6 Sessions. In Code-Sprints ist die letzte das **Review**; Doku- und Planungs-Sprints (nur `docs/`)
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
2. `git fetch && git diff origin/develop...origin/sprint/<präfix>` lesen (alles, was der Sprint ändert, in **allen**
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
   `verschoben` mit Ticket), behobene Befunde, neue Tickets.
5. **Wip-Branches prüfen** (B-302): Was inhaltlich von `wip/<präfix>…` im Sprint-Branch steckt, kommt in den PR-Text unter „Zum Löschen“.
   Sind alle anderen Sessions fertig: Sprint-Ordner nach `sprints/erledigt/` verschieben, `Status: erledigt`; sonst bleibt
   er in `aktiv/` (Hardware entkoppelt). Fahrplan anpassen, committen, `git merge origin/develop`,
   pushen und **den einen PR des Sprints** öffnen (Vorlage, eine Zeile je Session), danach **Auto-Merge einschalten**:
   `gh pr merge --auto --squash`. Ist der PR schon `CLEAN` (Konflikt-Check fertig), lehnt GitHub `--auto` ab: dann
   `gh pr merge --squash`. Im PR läuft nur der Konflikt-Check, die Tests laufen vorher lokal und nach dem Merge auf
   `develop` (B-397). Ein Entwurfs-PR einer blockierten Session bekommt kein Auto-Merge.
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
- Schichtgrenzen: `engine/sim` und `engine/level` importieren nichts aus `engine/room`, `engine/net`, `cmd/`;
  `engine/` nichts aus `cmd/`. Im Client rechnet `src/scenes` nichts, es zeichnet Snapshots.
  `src/model` enthält keine Logik; der Client importiert Typen und Daten nur von dort.
- Werkzeuge: Oxlint (TypeScript), `golangci-lint` mit `funlen`, `gocyclo`, `depguard` (Go);
  Go-Verschachtelung: `engine/internal/rules/nesting_test.go`; Dateilänge beider Sprachen: `tests/projectRules.test.ts`.

### Namen und Ablage im Code

- **Datei- und Ordnernamen im Code sind englisch** (`engine/`, `cmd/`, `src/`, `tools/`), camelCase in TypeScript,
  snake_case in Go, React-Komponenten wie ihre Komponente. Deutsch bleiben Doku, Spieltexte, Task-Namen, Seiten
  (`*.html`) und Begriffe aus dem Glossar in Kommentaren und Texten. Bestand wird angepasst, wenn eine Session die Datei
  ohnehin ändert.
- **Gruppen statt flacher Ordner:** In `engine/sim` beginnt jede Datei mit ihrer Gruppe (`building_`, `unit_`,
  `monarch_`, `enemies_`, `boss_`, `skills_`, `events_`, `island_`, `hub_`, `world_`, `save_`); Kern-Dateien
  (`world.go`, `types.go`, `data.go`, `common.go` …) haben keine. `src/scenes` hält nur Szenen und ihre Logik, der Rest
  liegt in `render/`, `hud/`, `hints/`, `effects/`, `debug/`. Ab etwa 30 Dateien bekommt ein Ordner Gruppen.
- Tests liegen neben dem Code; `tests/` nur für Regeln über das ganze Repo (TypeScript), Go-Regeln in
  `engine/internal/rules`. Gemeinsame Oberflächen-Bausteine von k3c-dev und Asset-Studio liegen in `tools/webkit`.

## Golden aktualisieren

Die Golden-Daten (`testdata/golden/`) halten fest, was Simulation und Level-Generator rechnen; die Go-Tests vergleichen Feld für Feld.

- **Erlaubt** nur bei einer gewollten Regel- oder Wertänderung (Beschluss, Ticket, Daten in `data/`). Bricht ein Golden-Test ohne solche Änderung, ist das ein Fehler im Code, kein Anlass zum Update.
- **Befehl:** `task golden:update` schreibt `sim-*.json`, `campaign-abstieg.json` und `level-*.json` aus dem Ist-Zustand neu. Unveränderte Einträge behalten ihre Bytes; der Diff zeigt nur, was sich wirklich geändert hat.
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
- **Version nach jedem Sprint:** Jeder abgeschlossene Sprint endet mit einem **Versionsvorschlag**. Die Review-Session (im Doku-Sprint die letzte Session) trägt in der Abnahme `Version: vX.Y.Z vorgeschlagen (Grund)` ein: **Minor** (`v0.<n+1>.0`) bei Sprints mit Wirkung im Spiel, im Server oder im Werkzeug, **Patch** (`v0.<n>.<m+1>`) bei reiner Doku, Planung oder Korrektur ohne neue Funktion. Der Agent legt den Vorschlag 🧑 vor und setzt den Tag **nur bei ausdrücklicher Bestätigung**: `task check:all` grün, `git tag <Version>`, `git push origin <Version>`; der Release-Workflow (`.github/workflows/release.yml`) baut Zip, Server-Dateien und das Docker-Image für den Pi (`docker compose pull && docker compose up -d`). Ohne Bestätigung entsteht kein Tag. Die Abnahme vermerkt danach `Version: vX.Y.Z gesetzt` oder `nicht gesetzt (Grund)`.
- Zusätzlich ein Release-Tag nach jedem Spieleabend, auch wenn kein Sprint abschließt; vor jedem Tag gilt die Checkliste im Abschnitt „Release“.

## Release

**Auslöser (Q20, 2026-10-03):** nach jedem fertigen Sprint (Versionsvorschlag der Abnahme, B-180) und nach jedem Spieleabend. Tag-Schema: „Entscheidungen und Versionen“. Ein Tag entsteht nur nach Bestätigung durch 🧑 (`git tag vX.Y.Z` auf `main` nach dem Fast-Forward, Abschnitt „Branches“); `release.yml` reagiert auf `v*`. Ein Punkt rot → **kein Tag**, der Befund wird ein Ticket; ein Dev-Rest blockiert den Release, bis er entfernt ist.

| Punkt | Prüfen mit | Erwartet |
|---|---|---|
| Golden amd64 | CI-Job `go` (`task go:test`) auf dem Stand, der getaggt wird | grün |
| Golden arm64 | CI-Job `go-arm64` (nativer arm64-Runner, B-071) | grün |
| Alles grün | `task check:all` | Exit 0 (Client, Build, Go, k3c-dev) |
| Spielstand-Migration (Beschluss 2026-10-05, B-288: bis auf Widerruf keine Rücksicht auf alte Stände) | `engine/sim/save_migration_test.go` | vorhandene Tests grün; neue Version ohne Migration zulässig (Abschnitt „Spielstand-Format ändern“) |
| Dev-Reste aus (B-098, B-107, B-080) | Release-Server ohne `K3C_DEV` (`engine/room/dev.go` lehnt Dev-Aktionen ab); Client: Debug-Overlay und `window.game` nur mit `?dev=1` (`src/main.ts`) | Dev-Aktion wird mit „🚫 Dev-Aktion abgelehnt“ geloggt, ohne `?dev=1` kein Overlay |
| Pi-Image | CI-Job `docker` (amd64 und arm64); beim Tag schiebt `release.yml` das Image nach `ghcr.io/tenvan/k3c-family` | beide Plattformen gebaut, Image mit Tag und `latest` vorhanden |
| Version stimmt (B-141) | `VERSION` im `Taskfile.yml` (`git describe`), `/api/health` (Feld `version`), Versionszeile der Landingpage | alle drei nennen den Tag, kein Versionsunterschied zwischen Client und Server |
| Credits vollständig (B-165) | `src/tools/credits.ts` und `credits.test.ts` über `task test` | Test grün, jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Credits-Eintrag |
| Aufgeräumt (B-302) | `task aufraeumen -- --dry` | keine Reste, sonst Hinweis (blockiert den Tag nicht) |
| Tag-Schema | „Entscheidungen und Versionen“ | Minor bei Wirkung, Patch bei Doku oder Korrektur |
