# B-133 · Der Raum rechnet mit einer Insel statt mit einer Kampagne

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP14
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf) per /goal „Sprint SP14 vorbereiten und komplett abarbeiten“, Revision 1

## Ausgangslage

Der Raum (`engine/room/`) rechnet mit der `Campaign` (eine aktuelle Stufe, gemeinsamer Stufenwechsel, Spielstand Version 1). Mit SP12 (B-100) gibt es in `engine/sim` die `Island` (mehrere Stufen ticken, Einzelwechsel, Spielstand Version 2), aber noch keinen Verbraucher.

## Ziel

Der Raum benutzt die `Island`: alle Stufen laufen im Takt, Spieler sind an eine Stufe gebunden und wechseln einzeln, Spielstände werden als Insel gespeichert und geladen (alte Stände werden überführt). Nutzen: Die Entscheidung 003 ist im Spiel wirksam; Protokoll und Client können folgen (B-104, B-106).

## Beteiligte und Zielgruppen

Entwickler (Server); Spieler merken es erst mit B-104 und B-106.

## Anforderungen

- `engine/room/` tickt `StepIsland` statt `Step` einer Stufe; Eingaben der Geräte werden nach Spieler-Index der Insel an die Simulation gegeben.
- Beitreten und Slots: Ein neuer Spieler startet in der Stufe der Burg (Stufe 0) oder, bei Wiederverbinden, in seiner gespeicherten Stufe.
- Autosave und Laden über `engine/store` mit dem Insel-Format (Version 2); Version 1 wird überführt, sicher mit Sicherungen (B-028 bleibt).
- Status-Endpunkt und TUI (`engine/room/status.go`, `cmd/k3c-tui`) zeigen die Räume mit Stufen und Spielern je Stufe.
- Raum pausiert nur, wenn **kein** Gerät verbunden ist (bisherige Regel), nicht je Stufe.

## Nicht-Ziele

Neue Protokoll-Felder (B-104: Stufe je Spieler im Snapshot), Client-Kameras (B-106), Raum-Optionen und Grade (B-101).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/net/`, `engine/store/`, `cmd/`). Protokolländerungen in eigener Session (B-104). Datenverlust vermeiden: alte Stände v1 werden nie ohne Sicherung überschrieben. Komplexitäts-Budget und Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch.

## Beispiele

Zwei Geräte in einem Raum, Spieler A im Wald, Spieler B in der Höhle: der Server tickt beide Stufen; der Autosave enthält beide Spieler mit ihrer Stufe.

## Ausnahme- und Fehlerfälle

Alter Stand v1 → wird überführt, die Datei bleibt als Sicherung erhalten. Beschädigter Stand → Fehlermeldung wie bisher, kein Überschreiben.

## Akzeptanzkriterien

- **AC-01** Der Raum tickt alle Stufen der Insel; Test mit zwei Spielern in verschiedenen Stufen (Raum-Test).
- **AC-02** Speichern und Laden einer Insel über den Store; ein Stand v1 wird geladen, die Datei bleibt als Sicherung (Test).
- **AC-03** Status und TUI zeigen Stufen und Spieler je Stufe.
- **AC-04** `task check:go` ist grün, `-race` in der CI ohne Befund.

## Offene Fragen

Wie viel Zustand der anderen Stufen bekommt ein Gerät (B-104).

## Notizen

Folgt aus SP12. Zusammen mit B-104 (Protokoll) planen; danach B-106 (Kamera je Stufe).

SP12 (2026-10-02): `Island` mit `StepIsland`, Einzelwechsel und Spielstand Version 2 stehen in `engine/sim` (`island*.go`); der Raum kann sie direkt nutzen. `ParseIslandSave` überführt Stände der Version 1.

SP13 (2026-10-02): `Island.Options` (Grad, Ziel, Niederlage-Modus) und fünf Materialien stehen im Spielstand der Insel. Beim Anlegen und Laden muss der Raum den Dev-Mode prüfen (`SetOptions(…, devMode)`; `FromIslandSave` prüft ihn nicht). Träger-Aufträge werden nicht gespeichert, getragenes Material geht beim Speichern verloren.
