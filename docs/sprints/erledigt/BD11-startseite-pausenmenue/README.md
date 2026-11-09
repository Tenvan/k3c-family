# BD11 · CLI · Eine Startseite, Navigation im Pausenmenü, Vollbild halten

- **Status:** erledigt
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

Stichpunkte (Reife Entwurf), alle fertig:

- BD11.1 CLI Umsetzung: Pausenmenü als Kachelseite (`2e478b20`).
- BD11.2 CLI Umsetzung: Startseite der Lobby als Kachelseite mit Unterseiten Raumliste, Seiten, Optionen (`99b390a7`).
- BD11.3 CLI Umsetzung: `RunConfig` (Menü > URL > Standard), Regel-Test gegen `location.search` (`422b280e`).
- BD11.4 CLI Umsetzung: Menü-Ebenen Seiten und Kategorien, Stress- und Debug-Kacheln per `startRun` (`593b3247`).
- BD11.5 CLI Umsetzung: Panel-Host für Werkzeugseiten im Spiel-Dokument (`354052cb`).
- BD11.6 CLI Umsetzung: Landingpage und Shell entfernt, `/` ist das Spiel, Vollbild halten, Regeln neu gefasst (`7e48e91a`); Nachtrag B als Zurück-Taste und Lobby ohne Nachwirkung gehaltener Tasten (`7951a652`). Dazu Papierschnitt-Optik der Kachelseiten (`c07349e2`).
- BD11.7 CLI Review: Sprint BD11 abgenommen (2026-11-09, 🧑 Chat).
- BD11.8 CLI Workshop Mensch live: Abnahme an Xbox und Pi (2026-11-09, 🧑 Chat, mit Auffälligkeiten, siehe Abnahme).

## Abnahme

2026-11-09 🧑 Chat: BD11 abgenommen, AC-01 bis AC-07 erfüllt (Tests und Gerätetest Xbox/Pi mit v0.23.0). Behobene Befunde: Spiel nicht verlassbar (A vom Verlassen bestätigte „Fortsetzen“, `7951a652`), URL nur mit Port zeigte die Landingpage (jetzt das Spiel).
Auffälligkeiten am Gerät: Xbox läuft nur im Canvas-Fallback (kein WebGL, keine Seite grün), Leistung trotzdem akzeptabel; offen bleibt eine Aussage, ob Pi 3 (Server) und Xbox (Client) generell tragen (B-763, B-764, RX1.5/RX1.6) und ob ein anderer Browser hilft (Xbox kennt nur Edge).
Kein neues Abnahme-Ticket für Sim, Balance und Gen: der Sprint ändert weder Simulation noch Werte noch Generatoren.
