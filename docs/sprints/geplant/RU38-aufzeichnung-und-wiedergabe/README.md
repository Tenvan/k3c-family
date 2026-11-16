# RU38 · SIM · Zustands-Aufzeichnung aus Go und Wiedergabe im Rust-Client

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Rust-Client soll seine Leistung zeigen, bevor die Sim portiert ist. Zeichnen braucht nur den Zustand je Takt, nicht die Regeln.

## Ziel

Der Rust-Client spielt einen in Go aufgezeichneten Spielverlauf ab: derselbe Zustand je Takt, dieselbe Last beim Zeichnen wie im echten Spiel. Am Ende sichtbar: `k3c-game replay <datei> --window` zeigt eine Nacht mit Gegnern in zwei Bildhälften.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 5 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Go-Test schreibt je Szene Level, Zustand je Takt und Ereignisse als `.jsonl.gz` (SIM, AC-01)
2. Rust liest die Aufzeichnung in die Zustands-Typen, Wiedergabe mit 30-Hz-Takt und Zwischenbildern (CLI, AC-02)
3. Unterbefehl `replay` mit `--headless`, `--window`, `--measure`; Messbericht im Format von `measure.go` (CLI, AC-03)
4. Review

### Hinweise für die Umsetzung

- Die Aufzeichnung ist ein Sim-Lauf als Test und braucht die Ausnahme von der Sim-Sperre (B-1014).
- Zwischenbilder wie `native/k3c-game/game/blend.go`: Lage vom Takt davor merken, in `draw` um alpha verschieben. alpha kommt aus der Zeitsammlung der Schleife (RU4), nicht aus der Wanduhr; welcher Weg ruckelfrei ist, sagt der Bericht aus RU0.
- Die Wiedergabe ist eine `App` der Plattform-Schicht; sie bleibt nach dem Sim-Port als Werkzeug für Messungen erhalten.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `engine/internal/trace/**`, `testdata/rust/trace/**`, `rust/crates/k3c-game/src/replay/**`
- **Abhängig von:** RU7, RU8
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test in `engine/internal/trace` schreibt mit `-write` für drei Szenen (ruhiger Tag, Nacht mit Gegnern mit zwei Spielern, ausgebaute Basis) Level, Zustand je Takt und Ereignisse nach `.work/trace/`; eine kurze Aufzeichnung (300 Takte) liegt unter `testdata/rust/trace/`.
- **AC-02** Rust lädt jede Zeile in die Zustands-Typen aus RU8; für die kurze Aufzeichnung ist der JSON-Baum jedes Takts gleich dem aus Go. Die Wiedergabe zeigt die Szene in zwei Bildhälften mit Zwischenbildern.
- **AC-03** `k3c-game replay <datei> --headless` endet zweimal mit demselben Prüfwert; `--window --measure 300` schreibt einen Bericht mit den Feldern des Go-Berichts (`native/k3c-game/game/measure.go` › `Report`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
