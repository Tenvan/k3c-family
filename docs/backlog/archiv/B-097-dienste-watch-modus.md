# B-097 · k3c-dev startet den Go-Server bei Code-Änderungen von selbst neu

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Dienst `Heimnetz` (`task start`) baut den Go-Server einmal beim Start. Wer danach `engine/` oder `data/` ändert, muss ihn von Hand neu starten; sonst läuft der alte Stand weiter. Aufgefallen beim Level-Betrachter: `/api/level` antwortete 404, weil der Server noch ohne den neuen Endpunkt lief. Der Vite-Dienst lädt Änderungen schon selbst nach (Hot-Reload). 🧑 hat im Chat gefragt, ob k3c-dev den Server im Watch-Modus starten kann („ok, setzt den watch mode so um“).

## Ziel

Der Dienst `Heimnetz` startet bei Änderungen an Go-Code und Daten von selbst neu (bauen und starten), ohne Eingriff. Nutzen: Änderungen am Server sind sofort wirksam, kein Raten, ob der laufende Server aktuell ist.

## Beteiligte und Zielgruppen

Entwickler und Agenten, die `k3c-dev` benutzen.

## Anforderungen

- `services.json` bekommt je Dienst einen optionalen Eintrag `watch` (`paths` relativ zur Repo-Wurzel, optional `ext`); `Heimnetz` beobachtet `cmd`, `engine`, `data`, `go.mod`, `go.sum` (Endungen `.go`, `.json`).
- Eine Änderung führt nach kurzer Ruhezeit (0,5 s) zu Stop und Start über die vorhandene Neustart-Logik (Prozessbaum wird beendet, Port frei, neu bauen); mehrere schnelle Änderungen ergeben einen Neustart.
- Scheitert der Neustart (z. B. Build-Fehler), bleibt die Beobachtung aktiv: Der Dienst steht auf `fehlgeschlagen`, die Konsole nennt den Grund, die nächste Änderung versucht es erneut.
- Ein ausdrückliches Stoppen beendet die Beobachtung; gestoppte und übernommene Dienste werden nie neu gestartet.
- Ohne `watch` bleibt alles wie vorher (Vite, Tests).

## Nicht-Ziele

Neue Abhängigkeiten (Dateibeobachtung per Abfrage der Dateizeiten statt Bibliothek), Watch für Vite (Hot-Reload), Behalten laufender Räume über den Neustart (Räume und Spielstand liegen im Speicher, nur der Autosave bleibt).

## Regeln und Einschränkungen

Domäne SRV (`tools/k3c-dev`). `watch`-Pfade bleiben unterhalb der Repo-Wurzel (kein `..`, kein absoluter Pfad); unbekannte Felder in `services.json` bleiben ein Fehler. Befehle laufen über `task`; die EXE des Servers bleibt `bin/k3c-server` (Firewall-Freigabe, B-085).

## Beispiele

`engine/net/level.go` speichern → nach etwa 1,5 s antwortet der neu gebaute Server; `console_tail Heimnetz` zeigt „k3c-dev: Neustart wegen Änderung an engine/net/level.go“.

## Ausnahme- und Fehlerfälle

Build-Fehler → `fehlgeschlagen`, Fehlertext in der Konsole, Fix speichern → läuft wieder. Dienst wurde von Hand gestoppt → kein Neustart bei Änderungen.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Eine Änderung startet den laufenden Dienst neu (alter Prozess beendet, neuer läuft), mehrere gleichzeitige Änderungen ergeben einen Neustart, Dateien anderer Endungen lösen nichts aus, nach `Stop` kein Neustart mehr.
- **AC-02** Ein Test belegt: Nach einem fehlgeschlagenen Neustart startet die nächste Änderung den Dienst wieder.
- **AC-03** `services.json` wird geprüft (Pfade, Endungen); `Heimnetz` hat `watch`, `Vite` nicht.
- **AC-04** `task check:dev` ist grün; README und MCP-Anleitung nennen das Verhalten.

## Offene Fragen

keine

## Notizen

Rückwirkend aus der umgesetzten Arbeit abgeleitet, keine Freigabe der Spec. Der erste Ansatz über Task (`task --watch` mit `watch: true` auf einem `start:watch`) wurde verworfen: Unter Windows bricht Task den Server beim Dateiwechsel zwar ab, startet ihn aber nicht wieder (auch nicht mit `ignore_error`). Daher der Beobachter in k3c-dev, der den Neustart über `Controller.stop`/`start` macht.
