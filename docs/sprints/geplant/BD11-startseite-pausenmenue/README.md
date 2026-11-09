# BD11 · CLI · Eine Startseite, Navigation im Pausenmenü, Vollbild halten

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-768
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-09 🧑 Chat (Konzept und Entscheidungen bestätigt)

## Ausgangslage

Die Kachelseiten für Pausenmenü (BD11.1) und Startseite der Lobby (BD11.2) stehen; Landingpage, Shell und URL-Steuerung bestehen noch. Details: B-768.

## Ziel

Das Spiel ist die einzige Seite: `ip:8080` öffnet das Spielmenü; alle Seiten, Werkzeuge und Test-Szenarien laufen aus dem Menü, ohne Landingpage und ohne URL-Syntax; Vollbild bleibt. Am Ende sichtbar: Xbox/Pi: Adresse öffnen, mit dem Pad Spiel, Optionen, Werkzeuge und Stress-Szenen erreichen.

## Beteiligte und Zielgruppen

Spieler am TV (Xbox-Pad), Entwickler am PC; 🧑 gibt die Spec frei und nimmt am Gerät ab.

## Anforderungen

Siehe B-768 › Anforderungen (Das Spiel ist die Seite, Menü-Ebenen, Werkzeuge eingebettet, RunConfig statt URLs, Dev-Modus fest an, Dungeon Master extern, Vollbild halten).

## Nicht-Ziele

Siehe B-768 › Nicht-Ziele.

## Regeln und Einschränkungen

Shell-Regeln aus `CLAUDE.md` werden mit BD11.5 neu gefasst. Eine Session ändert nur CLI-Dateien; Werkzeugseiten selbst bleiben inhaltlich unverändert (nur ihr Rahmen).

## Beispiele

Siehe B-768 › Beispiele.

## Ausnahme- und Fehlerfälle

Siehe B-768 › Ausnahme- und Fehlerfälle (Vollbild abgelehnt, kein Server, Spiel im Gange).

## Akzeptanzkriterien

- **AC-01** `GET /` liefert das Spiel, keine Landingpage (`B-768/AC-01`, Test).
- **AC-02** Menü-Ebenen und Kacheln für alle Seiten und Szenarien, bedienbar per Pad, Tastatur, Tippen (`B-768/AC-02`, Test).
- **AC-03** Werkzeuge als Panel im selben Dokument (`B-768/AC-03`, Test).
- **AC-04** `RunConfig` als einzige Quelle für Start-Parameter (`B-768/AC-04`, Test).
- **AC-05** Vollbild halten (`B-768/AC-05`, Test).
- **AC-06** Stress-Kachel startet ohne Neuladen (`B-768/AC-06`, Test).
- **AC-07** Abnahme am Gerät: Xbox und Pi, nur Pad (`B-768/AC-07`).

## Offene Fragen

keine (siehe B-768 › Offene Fragen).

## Sessions

Stichpunkte (Reife Entwurf):

- BD11.1 CLI Umsetzung: Pausenmenü als Kachelseite (fertig, `2e478b20`).
- BD11.2 CLI Umsetzung: Startseite der Lobby als Kachelseite mit Unterseiten Raumliste, Seiten, Optionen (fertig, `99b390a7`).
- BD11.3 CLI Umsetzung: `RunConfig` (Menü > URL > Standard), Szenen lesen nur sie, Regel-Test gegen `location.search` (AC-04, AC-06).
- BD11.4 CLI Umsetzung: Menü-Ebenen Seiten und Kategorien aus `PAGES` und `DEV_SECTIONS`, Stress- und Debug-Kacheln starten per `RunConfig` (AC-02, AC-06).
- BD11.5 CLI Umsetzung: Panel-Host für Werkzeugseiten im Spiel-Dokument, `installPageChrome` schließt das Panel, Dungeon Master extern (AC-03).
- BD11.6 CLI Umsetzung: Landingpage, Shell und `game.html` entfernen, `/` ist das Spiel, Vollbild halten, Regeln in `CLAUDE.md` und `projectRules` neu fassen (AC-01, AC-05).
- BD11.7 CLI Review: Sprint BD11 abnehmen (alle).
- BD11.8 CLI Workshop Mensch live: Abnahme an Xbox und Pi (AC-07).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
