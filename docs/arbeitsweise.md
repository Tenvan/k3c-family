# Arbeitsweise

**Ticket** ([`backlog/`](backlog/)) → **Sprint** (eine Domäne, 2–4 Sessions) → **Session** (ein PR).
Ein Sprint ist erst fertig, wenn seine **Review-Session** alle im Sprint geänderten Dateien abgenommen hat.
Übersicht aller Sprints: [`sprints/README.md`](sprints/README.md). Zielbild: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).

## Ablage

```text
docs/
  arbeitsweise.md             diese Datei (einzige Prozess-Beschreibung)
  vorlagen/                   Pflicht-Vorlagen: ticket.md, sprint.md, session.md
  backlog/README.md           Index aller Tickets (eine Zeile pro Ticket)
  backlog/B-NNN-kurzname.md   ein Ticket pro Datei
  sprints/README.md           Fahrplan: alle Sprints mit Ordner und Status
  sprints/aktiv/SPnn-name/    der laufende Sprint: README.md (Sprint) + SPnn.m-name.md (Sessions)
  sprints/geplant/…           kommende Sprints, gleicher Aufbau
  sprints/erledigt/…          abgeschlossene Sprints
```

**Lesen:** `sprints/aktiv/` immer, `sprints/geplant/` nur beim Planen, `sprints/erledigt/` nur auf ausdrückliche Nachfrage.

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
   Abhängigkeiten nehmen. Gibt es keine: **nichts tun** und das melden.
2. Session-Datei vollständig lesen. Branch wie im Feld `Branch` anlegen. `Status: in Arbeit` setzen.
3. Nur die **Erlaubten Dateien** ändern. Die **Schritte** der Reihe nach ausführen, **Nicht-Ziele** einhalten.
4. Alles unter **Fertig, wenn** abhaken, die Befehle unter **Prüfen** müssen grün sein. Maßstab sind die Kriterien
   der Session in der Sprint-README (SDD), nicht eine eigene Auslegung.
5. **Ergebnis** mit Nachweis je Kriterium ausfüllen, `Status: fertig` setzen, auch in der Session-Tabelle der Sprint-README. Neue Ideen oder
   Probleme als Ticket anlegen (Vorlage!) und in `backlog/README.md` eintragen. PR öffnen (Vorlage), nicht selbst mergen.

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
| **SIM** | Spiel-Logik (Go) | `engine/sim/`, `engine/level/`, **neue Felder** in `data/*.json`; bis zur Löschung `src/world/` (nur Fehler) |
| **SRV** | Server & Betrieb (Go), Entwickler-MCP | `engine/room/`, `engine/net/`, `engine/store/`, `cmd/` (inkl. `cmd/k3c-mcp/`), Docker; bis zur Löschung `server/`, `src/online/room.ts`, `src/online/wsServer.ts` |
| **CLI** | Client: Darstellung, HUD, Grafik, Audio, Verbindung | `src/scenes/`, `public/`, `src/online/client.ts`, `src/core/saveStore.ts` |
| **PLAT** | Plattform: Eingabe, Shell, Seiten | `src/input/`, `src/core/shell.ts`, `src/core/fullscreen.ts`, `src/landing/`, `src/tools/`, `*.html` |
| **INF** | Frameworks, Tooling, CI, Repo-Aufbau, Arbeitsweise | `package.json`, `go.mod`, `vite*.ts`, `tsconfig.json`, Lint-Konfiguration, `.github/`, `tests/projectRules.test.ts`, `tests/planning.test.ts`, `docs/arbeitsweise.md`, `docs/vorlagen/` |

Grenzfälle:

- **Planungs-Dateien** (`docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`) darf jede Session für Status und Tickets ändern.
- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Protokoll** (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`, `testdata/protocol/`) betrifft
  Client und Server. Eine Änderung daran bekommt eine eigene Session, die nur das Protokoll und beide Enden anpasst.
- **Portierung:** Ein SIM-Port-Sprint darf `src/world/` lesen und Golden-Daten daraus erzeugen, ändert es aber nicht.
- **Feature-Stopp:** In `src/world/` nur noch Fehlerbehebungen, neue Mechaniken entstehen in Go (Entscheidung 001).
- **Feature-Kette:** Ein neues Spielelement läuft als REG → SIM → CLI in direkt aufeinanderfolgenden Sprints.

## Sprint-Lebenslauf

1. **Geplant:** Ordner `sprints/geplant/SPnn-name/` mit `README.md` nach Vorlage, `Spec: Entwurf`. `Reife: Entwurf`
   erlaubt Stichpunkte für die Sessions, die Kriterien stehen trotzdem schon fest.
2. **Bereit machen** (Planung, meist am Ende des vorigen Reviews): Tickets sichten und bewerten, Offene Fragen klären,
   jede Session als Datei nach Vorlage schreiben (Feld `Kriterien`), `Reife: bereit`. Nur der **nächste** Sprint wird so
   detailliert. Danach 🧑 um Freigabe der Spec bitten.
3. **Aktivieren** (meist in der Review-Session des vorigen Sprints), nur mit `Spec: freigegeben`:
   `git mv docs/sprints/geplant/SPnn-name docs/sprints/aktiv/`,
   `Status: aktiv`, Fahrplan in `sprints/README.md` anpassen. Höchstens **ein** aktiver Sprint
   (ein eingeschobener Sprint mit `Einschiebbar: ja` darf zusätzlich aktiv sein).
   Das Feld `Start-Commit` setzt die **erste Session** des Sprints: `git rev-parse --short origin/main` vor ihrem Branch.
4. **Abschließen:** Die Review-Session verschiebt den Ordner nach `sprints/erledigt/` und setzt `Status: erledigt`.

- **Klein:** 2–4 Sessions, die letzte ist immer das **Review**. Mehr Arbeit → zweiter Sprint.
- **Blockade** (🧑 fehlt): Sprint bleibt aktiv, blockierte Session `Status: blockiert`. Ein einschiebbarer oder der
  nächste unabhängige Sprint darf vorgezogen werden.
- **Richtwert Session:** ein PR mit ≤ ~400 geänderten Code-Zeilen (ohne Bilder, Daten-JSON, Lockfiles).
- **Commit-Titel** mit Domäne: `feat(sim): Taunt`, `fix(srv): Raum aufräumen`, `docs(reg): Wirtschaft v1`.

## Review-Session (Sprint-Abnahme) 🔍

1. `git fetch && git diff --stat <Start-Commit>..origin/main` → **alle** im Sprint erstellten oder geänderten Dateien.
2. Jede Datei **vollständig** lesen (nicht nur den Diff) und gegen die Checkliste prüfen.
3. Befunde **in der Domäne** im Review-PR beheben. Befunde **außerhalb** → Ticket, außer Kleinstes (≤ 5 Zeilen).
4. **Abnahme** in der Sprint-README ausfüllen: Datum, Anzahl geprüfter Dateien, Nachweis je Kriterium, behobene
   Befunde, neue Tickets. Ein Kriterium ohne Nachweis ist `verschoben` (mit Ticket) oder blockiert die Abnahme.
5. Sprint-Ordner nach `sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan anpassen, PR öffnen.

**Checkliste**

- [ ] Jedes Kriterium der Sprint-Spec hat einen Nachweis; keins wurde umformuliert, um zum Code zu passen
- [ ] Datei gehört zur Domäne des Sprints (oder ist ein erlaubter Grenzfall)
- [ ] Komplexitäts-Budget eingehalten, nichts auf Vorrat gebaut, kein toter Code, keine Platzhalter/`skip`
- [ ] Werte stehen in `data/`, nicht im Code
- [ ] Spiel-Logik ist getestet und deterministisch
- [ ] Funktioniert mit mehreren Spielern (lokal und online, falls betroffen)
- [ ] Regeln aus `CLAUDE.md` eingehalten (Seiten, Vollbild, B-Taste, kein `Math.random()`)
- [ ] Doku passt zum Code (`game-design.md`, README, Kommentare, Session-Ergebnisse)

## Komplexitäts-Budget

Niedrige Komplexität ist in **jeder** Session Pflicht, nicht erst im Review.

| Regel | Ziel | Harte Grenze | Gilt für |
|---|---|---|---|
| Zeilen pro Datei | 300 | 400 | Code; Tests nur harte Grenze |
| Zeilen pro Funktion | 40 | 60 | Code, nicht Tests |
| Verschachtelung | 3 | 4 | alles |
| Zyklomatische Komplexität | 10 | 15 | Code |

- **Ratsche für Bestandscode:** Dateien, die beim Einführen schon über dem Ziel liegen, stehen mit ihrem heutigen
  Wert in einer Ausnahmeliste. Der Wert darf nur sinken.
- Keine neue Abhängigkeit ohne Ticket und Zustimmung im Review. Keine Abstraktion für nur einen Fall.
- Schichtgrenzen: `engine/sim` und `engine/level` importieren nichts aus `engine/room`, `engine/net`, `cmd/`;
  `engine/` nichts aus `cmd/`. Im Client rechnet `src/scenes` nichts, es zeichnet Snapshots.
  Bis zur Löschung: `src/world` importiert nichts aus `scenes/`, `online/`, `input/`.
- Werkzeuge: Oxlint (TypeScript), `golangci-lint` mit `funlen`, `gocyclo`, `revive` › `max-control-nesting`, `depguard` (Go);
  Dateilänge beider Sprachen: `tests/projectRules.test.ts`.

## Entscheidungen und Versionen

- Größere Entscheidungen als `docs/decisions/NNN-titel.md`: **Kontext · Optionen · Entscheidung · Folgen**, höchstens eine Seite.
- Nach jeder abgeschlossenen Feature-Kette oder jedem Spieleabend ein Release-Tag `v0.<n>.0`.
