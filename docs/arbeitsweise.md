# Arbeitsweise

**Ticket** ([`backlog/`](backlog/)) → **Sprint** (eine Domäne, 2–4 Sessions) → **Session** (ein Commit). **Ein PR je Sprint.**
Ein Code-Sprint ist erst fertig, wenn seine **Review-Session** ihn abgenommen hat (leicht, siehe unten).
Übersicht aller Sprints: [`sprints/README.md`](sprints/README.md). Zielbild: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).

## Ablage

```text
docs/
  arbeitsweise.md             diese Datei (einzige Prozess-Beschreibung)
  vorlagen/                   Pflicht-Vorlagen: ticket.md, sprint.md, session.md
  backlog/README.md           Index aller Tickets (eine Zeile pro Ticket)
  backlog/B-NNN-kurzname.md   ein offenes oder eingeplantes Ticket pro Datei
  backlog/archiv/B-NNN-…      erledigte und verworfene Tickets (gleicher Dateiname, per `git mv`)
  sprints/README.md           Fahrplan: alle Sprints mit Ordner und Status
  sprints/aktiv/SPnn-name/    der laufende Sprint: README.md (Sprint) + SPnn.m-name.md (Sessions)
  sprints/geplant/…           kommende Sprints, gleicher Aufbau
  sprints/erledigt/…          abgeschlossene Sprints
```

**Lesen:** `sprints/aktiv/` immer, `sprints/geplant/` nur beim Planen, `sprints/erledigt/` und `backlog/archiv/` nur auf ausdrückliche Nachfrage.
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
- **`main`** ist geschützt und trägt nur Release-Stände. Nur 🧑 gleicht ihn bei einem neuen Major-Release ab,
  per Fast-Forward: `git fetch && git push origin origin/develop:main`, danach Tag `v*` auf `main`
  (startet `release.yml`; GitHub Pages baut aus `main`). Nie direkt auf `main` committen oder einen PR gegen `main` öffnen.

## Autonomer Ablauf (Mensch oder Cloud-Agent)

Eine Session muss **ohne Rückfragen und ohne Planungs-Werkzeuge** abzuarbeiten sein. Deshalb:

1. `docs/sprints/aktiv/*/README.md` lesen. Liegt `sprint/<präfix>` schon auf `origin` (`git ls-remote --heads origin
   sprint/<präfix>`), gilt der Stand der Sprint-Dateien dort (`git show origin/sprint/<präfix>:<Pfad>`), sonst der von
   `develop`. Die erste Session mit `Status: offen`, `Agent: autonom` und erledigten Abhängigkeiten nehmen, aus dem Sprint
   mit der höchsten **Prio** (hoch vor mittel vor niedrig vor `?`, bei Gleichstand Fahrplan-Reihenfolge).
   Gibt es keine: **nichts tun** und das melden.
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
   **Kein PR**: den öffnet erst die letzte Session des Sprints (Review, im Doku-Sprint die letzte), nicht selbst mergen.

**Wenn etwas nicht passt** (Schritt unklar, Befehl scheitert unerklärlich, nötige Datei nicht erlaubt, Kriterium
widerspricht dem Code oder einer Regel):
`Status: blockiert`, im **Ergebnis** Grund und bisherigen Stand notieren, ein Ticket vom Typ `Frage` anlegen,
bisherigen Stand auf den Sprint-Branch pushen und einen **Entwurfs-PR** des Sprints öffnen (falls noch keiner offen ist), **aufhören**. Nicht raten, nicht um die Regeln herum arbeiten.

Eine Session pro Lauf. Eine Review-Session nie im selben Lauf wie eine Umsetzung. Sessions mit `Agent: Mensch`
(Xbox-Test, Workshop, Spieleabend) nimmt ein Agent nicht; er darf sie nur vorbereiten, wenn die Datei das verlangt.

## Domänen

Jeder Sprint gehört zu **genau einer Domäne** und ändert nur deren Dateien (plus Tests und Doku dazu).
Was eine andere Domäne braucht, wird ein Ticket.

| Kürzel | Domäne | Dateien |
|---|---|---|
| **REG** | Regelwerk & Balancing | `docs/game-design.md`, `docs/rules/`, `docs/playtests/`, **Werte** in `data/*.json` |
| **SIM** | Spiel-Logik (Go) | `engine/sim/`, `engine/level/`, **neue Felder** in `data/*.json` |
| **SRV** | Server & Betrieb (Go), Entwickler-MCP | `engine/room/`, `engine/net/`, `engine/store/`, `cmd/`, Entwickler-Werkzeug `tools/k3c-dev/` (eigenes Go-Modul mit Oberfläche), Docker |
| **CLI** | Client: Darstellung, HUD, Grafik, Audio, Verbindung | `src/scenes/`, `public/`, `src/online/client.ts`, `src/core/saveStore.ts` |
| **PLAT** | Plattform: Eingabe, Shell, Seiten | `src/input/`, `src/core/shell.ts`, `src/core/fullscreen.ts`, `src/landing/`, `src/tools/`, `*.html`, Pages-Präsentation `site/` |
| **INF** | Frameworks, Tooling, CI, Repo-Aufbau, Arbeitsweise | `package.json`, `go.mod`, `vite*.ts`, `tsconfig.json`, Lint-Konfiguration, `.github/`, `tests/projectRules.test.ts`, `tests/planning.test.ts`, `docs/arbeitsweise.md`, `docs/vorlagen/` |

Grenzfälle:

- **Planungs-Dateien** (`docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`) darf jede Session für Status und Tickets ändern.
- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Protokoll** (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`, `testdata/protocol/`) betrifft
  Client und Server. Eine Änderung daran bekommt eine eigene Session, die nur das Protokoll und beide Enden anpasst.
- **Alt-Engine löschen (SP09, B-049):** INF löscht `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs`,
  `vite.server.config.ts` und zieht dafür Importe in `src/scenes/`, `src/online/`, `src/core/`, Dev-Server, `Taskfile.yml`, CI,
  `tools/k3c-dev/services.json` und die Landingpage-Hinweise für GitHub Pages (B-032) nach. Neue Funktionen gehören nicht dazu.
- **Feature-Kette:** Ein neues Spielelement läuft als REG → SIM → CLI in direkt aufeinanderfolgenden Sprints.

## Sprint-Lebenslauf

1. **Geplant:** Ordner `sprints/geplant/SPnn-name/` mit `README.md` nach Vorlage, `Spec: Entwurf`. `Reife: Entwurf`
   erlaubt Stichpunkte für die Sessions, die Kriterien stehen trotzdem schon fest.
2. **Bereit machen** (Planung, meist am Ende des vorigen Reviews): Tickets sichten und bewerten, Offene Fragen klären,
   jede Session als Datei nach Vorlage schreiben (Feld `Kriterien`), `Reife: bereit`. Nur der **nächste** Sprint je Domäne wird so
   detailliert. Danach 🧑 um Freigabe der Spec bitten.
3. **Aktivieren** (meist in der Review-Session des vorigen Sprints), nur mit `Spec: freigegeben`:
   `git mv docs/sprints/geplant/SPnn-name docs/sprints/aktiv/`,
   `Status: aktiv`, Fahrplan in `sprints/README.md` anpassen. Höchstens ein aktiver Sprint **je Domäne**;
   einschiebbare Sprints (`Einschiebbar: ja`) zählen nicht mit (B-174).
   Die Aktivierung ist der erste Commit auf `sprint/<präfix>`; ein Sprint mit Branch auf `origin`
   (`git ls-remote --heads origin 'sprint/*'`) gilt als aktiv, auch solange sein PR noch offen ist.
   Das Feld `Start-Commit` setzt die **erste Session** des Sprints: `git rev-parse --short origin/develop` beim Anlegen des Sprint-Branchs.
4. **Abschließen:** Erledigt ist ein Sprint erst, wenn **alle** Sessions `fertig` oder `verworfen` sind. Die letzte
   davon (meist das Review, sonst die letzte Hardware-Session) verschiebt den Ordner nach `sprints/erledigt/` und setzt `Status: erledigt`.

- **Prio:** Jeder Sprint trägt die **höchste Prio seiner Tickets** (ohne Tickets frei gewählt). `plan_create` und
  `plan_set` (mit `Tickets`) leiten sie ab, `tests/planning.test.ts` prüft sie. Ändert sich die Prio eines Tickets, zieht
  die Sprint-Prio mit. Aktiviert und abgearbeitet wird nach Prio, bei Gleichstand in Fahrplan-Reihenfolge.
- **Umgebung:** Ticket und Session tragen `offline` (ohne laufende Dienste prüfbar: Code, Unit-/Mock-Tests, Werkzeuge
  ohne Serverzugriff) oder `live` (braucht laufenden Server, Browser oder Gerät); `?` nur bis zur Einordnung beim
  Einplanen. Nur Sessions mit `Agent: autonom` und `Umgebung: offline` dürfen in einem eigenen Worktree ohne Rückfrage
  laufen; `live` arbeitet im Checkout mit den laufenden Diensten.
- **Klein:** 2–4 Sessions. In Code-Sprints ist die letzte das **Review**; Doku- und Planungs-Sprints (nur `docs/`)
  haben keins, ihre letzte Session schließt den Sprint ab (Schritte 4–5 der Review-Session). Mehr Arbeit → zweiter Sprint.
- **Blockade** (🧑 fehlt): Sprint bleibt aktiv, blockierte Session `Status: blockiert`. Ein einschiebbarer Sprint oder
  der nächste Sprint einer anderen Domäne darf vorgezogen werden.
- **Hardware entkoppelt** (Beschluss 🧑 2026-10-03): Alles, was ein Gerät braucht (Xbox, TV, Pi, Handy, Controller),
  wartet nicht auf die App und die App wartet nicht darauf. Bis zur Validierung gelten die **angenommenen Werte**
  (`plan-weiterentwicklung.md` § 11.6); Code und Doku nennen sie „angenommen (Quelle)“. Eine Hardware-Session
  (`Agent: Mensch`, Test oder Abnahme am Gerät) ist **keine Abhängigkeit** einer App-Session und auch nicht des
  Reviews: Das Review nimmt ab, führt das Kriterium als `angenommen, Validierung offen (Session)` und öffnet den PR,
  der Sprint bleibt aber **aktiv**, bis die Hardware-Session fertig ist (entkoppelt heißt: weiter zum nächsten Sprint,
  nicht erledigt). Offene Hardware-Sessions stehen im Fahrplan unter „Offen am Gerät“ und werden erledigt, wenn das
  Gerät da ist; weicht das Ergebnis von der Annahme ab, entsteht ein Ticket (die Arbeit dahinter läuft weiter). Ein
  aktiver Sprint, in dem nur noch Sessions mit `Agent: Mensch` offen sind, sperrt seine Domäne nicht (`tests/planning.test.ts`).
- **Richtwert Session:** ein Commit mit ≤ ~400 geänderten Code-Zeilen (ohne Bilder, Daten-JSON, Lockfiles).
- **Übergang:** Sprints, die vor dieser Regel (2026-10-03) schon Session-PRs hatten (F4), schließen nach altem Ablauf ab
  (Review-Session mit eigenem PR). Ab dem nächsten aktivierten Sprint gilt ein PR je Sprint.
- **Commit-Titel** mit Domäne: `feat(sim): Taunt`, `fix(srv): Raum aufräumen`, `docs(reg): Wirtschaft v1`.

## Review-Session (Sprint-Abnahme) 🔍

Leicht und billig: Die Automatik prüft die Komplexität, das Review sucht nur **schwere Fehler**. Ein günstiges
Modell (z. B. Sonnet) reicht.

1. `task check` und `task check:go` → grün. Damit gelten die Grenzen aus dem Komplexitäts-Budget als geprüft.
2. `git fetch && git diff origin/develop...origin/sprint/<präfix>` lesen (alles, was der Sprint ändert), **nur den Diff**, nicht jede Datei vollständig.
3. Nur diese Befunde zählen:
   - falsches Verhalten oder Datenverlust (Spielstände, Berichte, Dateien)
   - Sicherheit: Pfade, Shell-Aufrufe, ungeprüfte Eingaben von außen
   - Spiel-Logik nicht deterministisch (`Math.random()`), mit 2 Spielern kaputt
   - Regeln aus `CLAUDE.md` verletzt (Seiten, Vollbild, B-Taste)
   - ein Kriterium der Spec ohne Nachweis oder umformuliert, damit es zum Code passt

   Stil, Doku, Benennung, mögliche Vereinfachungen sind **kein** Befund. Schwere Befunde in der Domäne im Review-Commit
   beheben, außerhalb → Ticket. Das Review läuft auf dem Sprint-Branch, nie im selben Lauf wie eine Umsetzung.
4. **Abnahme** in der Sprint-README, höchstens fünf Zeilen: Datum, Kriterien (Verweis auf die Session-Ergebnisse,
   `verschoben` mit Ticket), behobene Befunde, neue Tickets.
5. Sind alle anderen Sessions fertig: Sprint-Ordner nach `sprints/erledigt/` verschieben, `Status: erledigt`; sonst bleibt
   er in `aktiv/` (Hardware entkoppelt). Fahrplan anpassen, committen, `git merge origin/develop`,
   pushen und **den einen PR des Sprints** öffnen (Vorlage, eine Zeile je Session). Gemergt wird er von 🧑.
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
  Go-Verschachtelung: `tests/nesting_test.go`; Dateilänge beider Sprachen: `tests/projectRules.test.ts`.

## Golden aktualisieren

Die Golden-Daten (`testdata/golden/`) halten fest, was Simulation und Level-Generator rechnen; die Go-Tests vergleichen Feld für Feld.

- **Erlaubt** nur bei einer gewollten Regel- oder Wertänderung (Beschluss, Ticket, Daten in `data/`). Bricht ein Golden-Test ohne solche Änderung, ist das ein Fehler im Code, kein Anlass zum Update.
- **Befehl:** `task golden:update` schreibt `sim-*.json`, `campaign-abstieg.json` und `level-*.json` aus dem Ist-Zustand neu. Unveränderte Einträge behalten ihre Bytes; der Diff zeigt nur, was sich wirklich geändert hat.
- **Prüfen:** `git diff --stat testdata/golden` lesen. Ändert sich mehr als die Regel erklärt, Ursache klären und nicht einchecken.
- **Begründung im Commit-Text:** welche Regel, welcher Wert, welche Kennzahl sich dadurch ändert. Golden-Änderungen in eigenem Commit.
- **Bestätigung:** Kein Freigabe-Zwang durch 🧑 (Beschluss Q09); die Review-Session des Sprints prüft Diff und Begründung.
- **`rng.json` nie per Task ändern:** Sie ist die Referenz der Zufallsfolge (`engine/rng`); der Task fasst sie nicht an.
- Golden-Daten ändert nur der Sprint, der gerade in der Bahn SIM/INF läuft (`plan-weiterentwicklung.md` § 11.5).

## Spielstand-Format ändern

Gespeicherte Stände (`saves/`) überleben jedes Update; dafür sorgt die Regel aus B-137.

- **Jede Formatänderung** des Spielstands (Feld neu, entfernt, umbenannt oder mit neuer Bedeutung) erhöht `IslandSaveVersion` (`engine/sim/island_save.go`); `ParseIslandSave` liest jede ältere Version und überführt sie.
- **Fixture:** Ein kleiner, aus dem Code erzeugter Stand liegt unter `testdata/saves/v<n>/`; alte Fixtures bleiben unverändert liegen. `TestJedeVersionHatFixture` ist rot, solange eines fehlt.
- **Test „alter Stand lädt“:** `engine/sim/save_migration_test.go` lädt jedes Fixture über `ParseIslandSave` und prüft Seed, Spieler und Hubs; eine neue Version ergänzt dort ihren Fall.
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
| Spielstand-Migration | `testdata/saves/v<n>/` je `IslandSaveVersion` (`engine/sim/island_save.go`), `engine/sim/save_migration_test.go` | `TestJedeVersionHatFixture` und Ladetests grün (Abschnitt „Spielstand-Format ändern“) |
| Dev-Reste aus (B-098, B-107, B-080) | Release-Server ohne `K3C_DEV` (`engine/room/dev.go` lehnt Dev-Aktionen ab); Client: Debug-Overlay und `window.game` nur mit `?dev=1` (`src/main.ts`) | Dev-Aktion wird mit „🚫 Dev-Aktion abgelehnt“ geloggt, ohne `?dev=1` kein Overlay |
| Pi-Image | CI-Job `docker` (amd64 und arm64); beim Tag schiebt `release.yml` das Image nach `ghcr.io/tenvan/k3c-family` | beide Plattformen gebaut, Image mit Tag und `latest` vorhanden |
| Version stimmt (B-141) | `VERSION` im `Taskfile.yml` (`git describe`), `/api/health` (Feld `version`), Versionszeile der Landingpage | alle drei nennen den Tag, kein Versionsunterschied zwischen Client und Server |
| Credits vollständig (B-165) | `src/tools/credits.ts` und `credits.test.ts` über `task test` | Test grün, jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Credits-Eintrag |
| Tag-Schema | „Entscheidungen und Versionen“ | Minor bei Wirkung, Patch bei Doku oder Korrektur |
