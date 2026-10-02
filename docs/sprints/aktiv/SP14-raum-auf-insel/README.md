# SP14 · SRV · Raum auf Insel

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-133, B-104
- **Start-Commit:** febf4ee
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf) per /goal „Sprint SP14 vorbereiten und komplett abarbeiten“, Revision 1

## Ausgangslage

SP12 und SP13 haben in `engine/sim` die `Island` geliefert (mehrere Stufen ticken, Einzelwechsel, Vorrat je Insel, Raum-Optionen, Grade, Lager, Spielstand Version 2 mit Überführung von Version 1). Der Raum (`engine/room/`) rechnet aber noch mit der `Campaign` (eine aktuelle Stufe, gemeinsamer Wechsel, Spielstand Version 1); das Protokoll v2 (`docs/protocol.md`, `engine/net/protocol.go`, `src/online/clientProtocol.ts`) kennt weder Stufen je Spieler noch Raum-Optionen.

## Ziel

Der Raum rechnet mit einer `Island`: alle Stufen laufen im Takt, Spieler wechseln einzeln, jedes Gerät sieht die Stufe seines ersten Spielers, Spielstände werden als Insel gespeichert und geladen (alte Stände werden überführt). Das Protokoll (Version 3) trägt die Stufe je Platz und die Raum-Optionen (Grad, Ziel, Niederlage-Modus) beim Anlegen; der Server prüft die Optionen und sperrt den Grad „Dev“ außerhalb des Dev-Modes. Am Ende sichtbar: `go test ./engine/...` und `task check` grün, Raum-Tests mit zwei Spielern in verschiedenen Stufen, aktualisierte Protokoll-Beispiele; Status und TUI zeigen Stufen und Spieler je Stufe.

## Beteiligte und Zielgruppen

Entwickler und Agenten (SRV, Team); Client und Spieler merken die Änderung erst mit den CLI-Tickets (B-106 Kamera je Stufe, B-105 Anlegen-Dialog).

## Anforderungen

B-133 › Anforderungen und B-104 › Anforderungen (Stufe je Spieler, Raum-Optionen). Sprint-eigene Abgrenzung: Ein Gerät bekommt weiter **eine** Welt je Nachricht (die Stufe seines Spielers mit dem kleinsten Slot); die Anzeige mehrerer Stufen gleichzeitig und die Kamera je Stufe sind B-106. Der Client wird nur so weit angepasst, dass er das Protokoll v3 liest und Spieler anderer Stufen nicht mit Positionen verwechselt (Zugriff nach `index`).

## Nicht-Ziele

Neue Client-Oberfläche (Dialog, Overlay, Kamera, B-105, B-106, B-125), Wirkung von Ziel und Niederlage-Modus (B-102), Hub-Ausbau (B-112), weitere Protokoll-Funktionen (Skills, Aktionen, B-123).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/net/`, `engine/store/`, `cmd/`); Protokolländerung in **eigener Session** mit beiden Enden (`docs/arbeitsweise.md` › Protokoll: `docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`/`clientProtocol.ts`, `testdata/protocol/`). Komplexitäts-Budget (Datei ≤ 400 Zeilen, Funktion ≤ 60), Schichtgrenzen (`engine/sim` unverändert, soweit möglich; nötige kleine Erweiterungen sind erlaubt). **Datenverlust vermeiden:** alte Stände (Version 1) werden nie ohne Sicherung überschrieben; der Store sichert den vorigen Stand bei jedem Schreiben (B-028). Eingaben von außen (Optionen, Namen) serverseitig prüfen. Alle Tests unter `engine/room` und `engine/net` bleiben grün (angepasst, nicht gelöscht).

## Beispiele

Zwei Geräte in einem Raum: Spieler A im Wald, Spieler B in der Höhle. Der Server tickt beide Stufen; Gerät A bekommt Zustand des Waldes, Gerät B den der Höhle; läuft A allein zum Ausgang, wechselt A die Stufe und bekommt `level` der Höhle, danach den Zustand. Ein Raum wird mit `grade: "hard"` angelegt; `dev` wird ohne Dev-Modus mit `bad_request` abgelehnt.

## Ausnahme- und Fehlerfälle

Alter Stand Version 1 → wird geladen und überführt, die Datei bleibt als Sicherung; beschädigter Stand → Fehler wie bisher, kein Überschreiben. Unbekannter Grad, Ziel oder Niederlage-Modus beim Anlegen → `bad_request`. Gerät ohne besetzten Platz (alle frei) → Zustand der Stufe 0. Ältere Clients mit Version 2 → `version` (Seite neu laden).

## Akzeptanzkriterien

- **AC-01** Der Raum tickt alle Stufen der Insel; Test mit zwei Spielern in verschiedenen Stufen (B-133/AC-01).
- **AC-02** Speichern und Laden einer Insel über den Store; ein Stand Version 1 wird geladen, die Datei bleibt als Sicherung (B-133/AC-02).
- **AC-03** Status und TUI zeigen Stufen und Spieler je Stufe (B-133/AC-03).
- **AC-04** `docs/protocol.md` beschreibt Version 3 (Stufe je Platz, Raum-Optionen), `testdata/protocol/` hat Beispiele, Server und Client parsen sie (B-104/AC-01).
- **AC-05** Der Server prüft Raum-Optionen; „Dev“ ist nur im Dev-Modus wählbar; ungültige Werte → `bad_request` (B-104/AC-02).
- **AC-06** Die Protokollversion ist 3; ein Client mit Version 2 erhält `version` (B-104/AC-03).
- **AC-07** `task check` und `task check:go` sind grün, die Golden-Daten unverändert, `-race` in der CI ohne Befund (B-133/AC-04).

## Offene Fragen

Wie viel Zustand der anderen Stufen bekommt ein Gerät (B-104 › Offene Fragen): hier entschieden — nur die Stufe seines ersten Spielers; mehrere Stufen gleichzeitig folgen mit B-106.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP14.1 | `SP14.1-raum-kern.md` | Umsetzung | autonom | fertig |
| SP14.2 | `SP14.2-protokoll-v3.md` | Umsetzung | autonom | fertig |
| SP14.3 | `SP14.3-optionen-im-raum.md` | Umsetzung | autonom | fertig |
| SP14.4 | `SP14.4-review.md` | Review | autonom | offen |

## Abnahme

–
