# LT1 · SRV · Lasttest-Werkzeug

- **Status:** erledigt
- **Projekt:** TST
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-175, B-042
- **Start-Commit:** 9e6849e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-175, B-042; mit Änderungen aus dem Spec-Review (B-042 im Feld Tickets, Review-Diff korrigiert, `-max-duration` 60 min bestätigt, Protokoll = aktuelle Version)

## Ausgangslage

Die Lastmessung am Pi (B-042) ist Handarbeit; die erste Messung am 2026-10-03 liegt knapp am Ziel (Nacht 10,2 bis 10,3 ms bei Ziel < 10 ms). SP11 schließt ohne diese Messung ab (AC-04 verschoben, siehe SP11 Revision 2). `/api/status` kennt keine CPU-Last.

## Ziel

Ein Befehl `task load` misst Tick-Dauer und CPU gegen das Ziel und bewertet es. Am Ende sichtbar: Ein Messlauf am Pi mit Tabelle und Bewertung („erreicht“, „knapp“, „verfehlt“).

## Beteiligte und Zielgruppen

🧑 startet den Messlauf am Pi; Agenten setzen das Werkzeug um.

## Anforderungen

B-175 › Anforderungen.

## Nicht-Ziele

Bandbreite (B-140, F4), Lasttest mit echten Geräten, Balancing (B-099).

## Regeln und Einschränkungen

Domäne SRV (`cmd/k3c-load`, `engine/net/status.go`, `Taskfile.yml` für den Task `load`, Tests). Der Task gehört zu INF-Dateien; Freigabe dieser Spec erlaubt die eine Zeile im `Taskfile.yml`.

## Beispiele

`task load -- -url http://pi-gaming:8080 -token … -rooms 2 -players 3 -duration night` → Bericht mit p99 je Raum und Phase.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Exit-Code 2; Ziel verfehlt → Messwerte und Befund als Ticket (keine stille Absenkung des Ziels).

## Akzeptanzkriterien

- **AC-01** Das Werkzeug erzeugt gegen einen In-Prozess-Server einen Bericht mit Tick-Reihe je Raum (B-175/AC-01).
- **AC-02** Gleicher Seed ergibt dieselbe Eingabefolge (B-175/AC-02).
- **AC-03** Bewertung und Exit-Code folgen den drei Stufen (B-175/AC-03).
- **AC-04** `/api/status` liefert `cpu`, wo die Quelle existiert (B-175/AC-04).
- **AC-05** Keine Test-Räume bleiben offen, kein Token in der Ausgabe (B-175/AC-05).
- **AC-06** Der Messlauf am Pi ist bewertet; B-042 ist archiviert (B-175/AC-06).

## Offene Fragen

keine (Obergrenze der Nachtmessung: Flag `-max-duration`, Standard 60 min, 🧑 2026-10-04)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LT1.1 | `LT1.1-bots-client.md` | Umsetzung | autonom | fertig |
| LT1.2 | `LT1.2-status-bericht.md` | Umsetzung | autonom | fertig |
| LT1.3 | `LT1.3-messlauf-pi.md` | Workshop | Mensch | verworfen |
| LT1.4 | `LT1.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-04, Review LT1.4: AC-01, AC-02, AC-05 (LT1.1) sowie AC-03, AC-04 (LT1.2) sind mit Tests nachgewiesen (`TestLaufSammeltTicksUndRaeumtAuf`, `TestGleicherSeedGleicheEingaben`, `TestBewertungUndExitCode`, `TestStatusNenntCPU`); AC-05 mit der Abweichung B-204 (Test-Räume schließen erst nach der Leer-Frist).
- AC-06: angenommen, Validierung offen (LT1.3, 🧑 am Pi; B-042 bleibt eingeplant bis zur Messung, angenommen laut Messung 2026-10-03: Nacht 10,2 bis 10,3 ms, Ziel < 10 ms).
- Ein Befund behoben (SRV): Der Poller konnte nach dem Ende des Laufs noch Proben anhängen, während der Bericht las (Datenwettlauf); `load` wartet jetzt auf ihn. Keine Tickets neu; `task check` und `task check:go` grün; Token, `test-`-Räume, Schichtgrenzen und `go.mod` geprüft.
- Version: v0.9.0 vorgeschlagen (Minor, Wirkung im Werkzeug/Server: `task load` und `cpu` in `/api/status`; nach den offenen Vorschlägen bis v0.8.0 bei gemeinsamem Setzen anpassen).
- 2026-10-07, LT1.3: Messlauf 2 × 3 über eine Nacht am Entwicklungs-PC „erreicht“ (p99 max 8,45 ms, ohne CPU-Werte unter Windows); Pi-Lauf weiter offen, AC-06 bleibt „angenommen, Validierung offen“.
2026-10-08, PJ3.2 (B-359): AC-06 angenommen, Validierung in HW1.5 (Projekt ABN); Sprint geschlossen.
