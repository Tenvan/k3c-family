# RU22 · SIM · Sim-Abgleich: alle Sim- und Kampagnen-Golden bitgleich (Gate G3)

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

Nach RU19 sind alle Sim-Gruppen portiert und einzeln getestet. 13 Sim-Golden und 2 Kampagnen-Golden prüfen den Ablauf als Ganzes.

## Ziel

Die Rust-Sim erzeugt jedes Sim- und Kampagnen-Golden bitgleich. Am Ende sichtbar: Bericht für Gate G3 mit Liste der Golden und gefundenen Abweichungen.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 13 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Abgleich-Werkzeug: erster abweichender Schritt und Feld (AC-01)
2. Golden Wald (7 Dateien) (AC-02)
3. Golden Höhle, Mine, Kristall, Ironhold (6 Dateien) (AC-02)
4. Kampagnen-Golden (2 Dateien) (AC-02)
5. Durchsatz Rust gegen Go, G3-Bericht (AC-03)
6. Review

### Vorentscheidungen

- Der Abgleich läuft als Test je Golden: Eingaben und Zwischenstände aus dem Golden lesen, die Rust-Sim rechnen lassen, JSON-Bäume mit `k3c_det::golden::diff` vergleichen.
- Für die Fehlersuche schreibt ein Go-Test (`engine/sim`, Schalter `-trace`) je Schritt den Zustand als JSON-Zeile nach `.work/trace/<golden>.jsonl`; das Rust-Werkzeug nennt den ersten Schritt und den ersten Pfad, die abweichen. Die Trace-Dateien werden nicht eingecheckt.
- Reihenfolge der Golden: zuerst `sim-forest-ohne-spieler` (kein Spieler, wenig Regeln), dann `sim-forest-tag`, `-nacht`, `-aufbau`, danach die übrigen Wald-Golden, dann die Biome, zuletzt die Kampagnen.
- Am Ende darf kein `unimplemented!` mehr in `k3c-sim` stehen (Test, der die Quelltexte liest).

### Hinweise für die Umsetzung

- Bei einer Abweichung zuerst den ersten abweichenden Schritt suchen (Zustand je Schritt aus Go schreiben lassen), dann das erste abweichende Feld. Typische Ursachen: Reihenfolge eines Würfel-Aufrufs, `fmax`/`fmin` mit NaN, Reihenfolge beim Löschen aus Listen, Sortierung mit Gleichstand.
- Ein Golden wird nie geändert. Zeigt die Suche einen Fehler in Go, entsteht ein Ticket; Rust bildet den Fehler nach.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-sim/tests/**`, `rust/crates/k3c-sim/src/step.rs`, `docs/messung/rust-sim-abgleich.md`
- **Abhängig von:** RU18, RU19
- **Braucht 🧑:** Entscheidung
- **Sperre:** golden

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Ein Test nennt bei einer Abweichung den ersten Schritt und das erste Feld.
- **AC-02** Alle 13 `sim-*.json.gz` und beide `campaign-*.json.gz` stimmen bitgleich (gleicher JSON-Baum, jede Zahl mit denselben Bits), ohne Golden-Neuschrieb.
- **AC-03** Der G3-Bericht nennt Schritte je Sekunde Rust und Go für denselben Lauf und alle behobenen Abweichungen mit Ursache.

## Offene Fragen

Die Golden-Vergleiche sind Sim-Läufe als Tests; sie brauchen die Ausnahme von der Sim-Sperre (🧑, Ziel-Ticket › Offene Fragen).

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
