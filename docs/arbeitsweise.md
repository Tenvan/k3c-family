# Arbeitsweise

**Ticket** ([`backlog/`](backlog/)) → **Sprint** (eine Domäne, 2–4 Sessions) → **Session** (ein PR).
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

## Autonomer Ablauf (Mensch oder Cloud-Agent)

Eine Session muss **ohne Rückfragen und ohne Planungs-Werkzeuge** abzuarbeiten sein. Deshalb:

1. `docs/sprints/aktiv/*/README.md` lesen. Die erste Session mit `Status: offen`, `Agent: autonom` und erledigten
   Abhängigkeiten nehmen, deren Branch noch nicht auf `origin` liegt (`git ls-remote --heads origin <Branch>`;
   existiert er, ist die Session vergeben, die nächste nehmen). Gibt es keine: **nichts tun** und das melden.
2. Session-Datei vollständig lesen. Branch wie im Feld `Branch` anlegen und sofort mit `git push -u origin <Branch>`
   schieben (beansprucht die Session). `Status: in Arbeit` setzen.
3. Nur die **Erlaubten Dateien** ändern. Die **Schritte** der Reihe nach ausführen, **Nicht-Ziele** einhalten.
4. Alles unter **Fertig, wenn** abhaken, die Befehle unter **Prüfen** müssen grün sein. Maßstab sind die Kriterien
   der Session in der Sprint-README (SDD), nicht eine eigene Auslegung.
5. **Ergebnis** mit Nachweis je Kriterium ausfüllen, `Status: fertig` setzen, auch in der Session-Tabelle der Sprint-README. Neue Ideen oder
   Probleme als Ticket anlegen (Vorlage!) und in `backlog/README.md` eintragen. Gemeinsame Planungsdateien
   (`backlog/README.md`, `sprints/README.md`) erst am Ende ändern. Vor dem PR `git fetch` und `git rebase origin/main`.
   PR öffnen (Vorlage), nicht selbst mergen.

**Wenn etwas nicht passt** (Schritt unklar, Befehl scheitert unerklärlich, nötige Datei nicht erlaubt, Kriterium
widerspricht dem Code oder einer Regel):
`Status: blockiert`, im **Ergebnis** Grund und bisherigen Stand notieren, ein Ticket vom Typ `Frage` anlegen,
PR mit dem bisherigen Stand öffnen, **aufhören**. Nicht raten, nicht um die Regeln herum arbeiten.

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
   Das Feld `Start-Commit` setzt die **erste Session** des Sprints: `git rev-parse --short origin/main` vor ihrem Branch.
4. **Abschließen:** Die Review-Session verschiebt den Ordner nach `sprints/erledigt/` und setzt `Status: erledigt`.

- **Klein:** 2–4 Sessions. In Code-Sprints ist die letzte das **Review**; Doku- und Planungs-Sprints (nur `docs/`)
  haben keins, ihre letzte Session schließt den Sprint ab (Schritte 4–5 der Review-Session). Mehr Arbeit → zweiter Sprint.
- **Blockade** (🧑 fehlt): Sprint bleibt aktiv, blockierte Session `Status: blockiert`. Ein einschiebbarer Sprint oder
  der nächste Sprint einer anderen Domäne darf vorgezogen werden.
- **Hardware entkoppelt** (Beschluss 🧑 2026-10-03): Alles, was ein Gerät braucht (Xbox, TV, Pi, Handy, Controller),
  wartet nicht auf die App und die App wartet nicht darauf. Bis zur Validierung gelten die **angenommenen Werte**
  (`plan-weiterentwicklung.md` § 11.6); Code und Doku nennen sie „angenommen (Quelle)“. Eine Hardware-Session
  (`Agent: Mensch`, Test oder Abnahme am Gerät) ist **keine Abhängigkeit** einer App-Session und auch nicht des
  Reviews: Das Review schließt den Sprint ab und führt das Kriterium als `angenommen, Validierung offen (Session)`.
  Offene Hardware-Sessions stehen im Fahrplan unter „Offen am Gerät“ und werden erledigt, wenn das Gerät da ist;
  weicht das Ergebnis von der Annahme ab, entsteht ein Ticket (die Arbeit dahinter läuft weiter). Ein Sprint, in dem
  nur noch Hardware-Sessions (und ihre Auswertung) offen sind, sperrt seine Domäne nicht.
- **Richtwert Session:** ein PR mit ≤ ~400 geänderten Code-Zeilen (ohne Bilder, Daten-JSON, Lockfiles).
- **Commit-Titel** mit Domäne: `feat(sim): Taunt`, `fix(srv): Raum aufräumen`, `docs(reg): Wirtschaft v1`.

## Review-Session (Sprint-Abnahme) 🔍

Leicht und billig: Die Automatik prüft die Komplexität, das Review sucht nur **schwere Fehler**. Ein günstiges
Modell (z. B. Sonnet) reicht.

1. `task check` und `task check:go` → grün. Damit gelten die Grenzen aus dem Komplexitäts-Budget als geprüft.
2. `git fetch && git diff <Start-Commit>..origin/main` lesen, **nur den Diff**, nicht jede Datei vollständig.
3. Nur diese Befunde zählen:
   - falsches Verhalten oder Datenverlust (Spielstände, Berichte, Dateien)
   - Sicherheit: Pfade, Shell-Aufrufe, ungeprüfte Eingaben von außen
   - Spiel-Logik nicht deterministisch (`Math.random()`), mit 2 Spielern kaputt
   - Regeln aus `CLAUDE.md` verletzt (Seiten, Vollbild, B-Taste)
   - ein Kriterium der Spec ohne Nachweis oder umformuliert, damit es zum Code passt

   Stil, Doku, Benennung, mögliche Vereinfachungen sind **kein** Befund. Schwere Befunde in der Domäne im Review-PR
   beheben, außerhalb → Ticket.
4. **Abnahme** in der Sprint-README, höchstens fünf Zeilen: Datum, Kriterien (Verweis auf die Session-Ergebnisse,
   `verschoben` mit Ticket), behobene Befunde, neue Tickets.
5. Sprint-Ordner nach `sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan anpassen, PR öffnen.
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

## Entscheidungen und Versionen

- Größere Entscheidungen als `docs/decisions/NNN-titel.md`: **Kontext · Optionen · Entscheidung · Folgen**, höchstens eine Seite.
- **Version nach jedem Sprint:** Jeder abgeschlossene Sprint endet mit einem **Versionsvorschlag**. Die Review-Session (im Doku-Sprint die letzte Session) trägt in der Abnahme `Version: vX.Y.Z vorgeschlagen (Grund)` ein: **Minor** (`v0.<n+1>.0`) bei Sprints mit Wirkung im Spiel, im Server oder im Werkzeug, **Patch** (`v0.<n>.<m+1>`) bei reiner Doku, Planung oder Korrektur ohne neue Funktion. Der Agent legt den Vorschlag 🧑 vor und setzt den Tag **nur bei ausdrücklicher Bestätigung**: `task check:all` grün, `git tag <Version>`, `git push origin <Version>`; der Release-Workflow (`.github/workflows/release.yml`) baut Zip, Server-Dateien und das Docker-Image für den Pi (`docker compose pull && docker compose up -d`). Ohne Bestätigung entsteht kein Tag. Die Abnahme vermerkt danach `Version: vX.Y.Z gesetzt` oder `nicht gesetzt (Grund)`.
- Zusätzlich ein Release-Tag nach jedem Spieleabend, auch wenn kein Sprint abschließt (Release-Checkliste B-170).
