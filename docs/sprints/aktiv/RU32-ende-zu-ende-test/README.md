# RU32 · CLI · Ende-zu-Ende-Test und Bildvergleich gegen das Go-Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`native/k3c-game/e2e` treibt die echte Schleife ohne Fenster durch feste Stationen und vergleicht Zustand, Welt-Hash und Bilder.

## Ziel

Derselbe Ablauf läuft gegen das Rust-Spiel und vergleicht mit dem Go-Spiel. Am Ende sichtbar: Bericht mit Bildpaaren je Station.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 17 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`, Session 1 (Schnittstelle Referenz und Hash) `opus`.

### Sessions

1. Go-Referenz der Stationen (`testdata/rust/e2e/flow.json`: Eingabe-Folge, 14 Stationen, Takt, kanonischer Welt-Hash) und derselbe Hash in Rust (AC-01)
2. Ablauf ohne Fenster mit der Eingabe-Folge aus der Referenz, Welt-Hash je Station (auch nach dem Laden) gleich Go, zwei Läufe gleich (AC-01)
3. Aufnahme je Station aus beiden EXE (Go über `images_test.go`, Rust über `e2e_images.rs`) (AC-02)
4. Bildvergleich, Bildpaare und Bericht (AC-02)
5. Review

### Vorentscheidungen

- Der Go-Hash (SHA-256 über `json.Marshal`) hängt an den Bytes von `encoding/json`; verglichen wird deshalb ein kanonischer Hash (Schlüssel sortiert, Zahlen als `f64`-Bits, `null` = `[]`, FNV-1a 64), den Go (Testdatei `rustref_test.go`) und Rust gleich rechnen.
- Beide Spiele bekommen dieselbe Eingabe-Folge aus der Referenz; Rust macht keinen eigenen Probelauf.
- Kennzahlen und Schwellen des Bildvergleichs wie RU13.6; eine Station über der Schwelle wird ein Ticket, die Schwelle wird nie angehoben.
- Fehlt dem Rust-Spiel ein headless-Treiber, eine feste Uhr oder ein Schalter des Aufnahme-Modus, entsteht ein Ticket in RU30 und die Session hält an.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code außer der Testdatei `native/k3c-game/e2e/rustref_test.go`. Optimierungen über den Go-Stand hinaus. Zeichen-Fehler beheben (Tickets in den zuständigen Sprints).

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Der Ablauf ist ein Spieltest wie `flow_test.go`, kein Sim- oder Balancing-Lauf.

- **Bereich:** `rust/crates/k3c-game/tests/**`, `docs/messung/rust-bildvergleich.md`, `native/k3c-game/e2e/rustref_test.go`, `testdata/rust/e2e/**`
- **Abhängig von:** RU29, RU30
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Der Ablauf aus `flow_test.go` läuft headless gegen das Rust-Spiel; zwei Läufe ergeben denselben Welt-Hash wie das Go-Spiel an jeder Station.
- **AC-02** Je Station liegt ein Bildpaar Go und Rust vor; Abweichungen über der Schwelle sind als Ticket erfasst.

## Offene Fragen

Keine offenen.

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Schnitt angepasst: Session 1 ist die Go-Referenz mit kanonischem Hash (der Go-Hash ist byteabhängig und in Rust nicht nachbildbar), Session 2 fährt den Ablauf und prüft Hash und zwei Läufe (früher Session 1 und 2).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Bereich erweitert um die Go-Testdatei `native/k3c-game/e2e/rustref_test.go` und `testdata/rust/e2e/**` (Muster RU13.6).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Bildpaare liegen unter `.work/e2e/paare/` (nicht eingecheckt), der Bericht nennt Pfade und Kennzahlen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU32.1 | `RU32.1-go-referenz-und-welt-hash.md` | Umsetzung | autonom | fertig |
| RU32.2 | `RU32.2-ablauf-ohne-fenster.md` | Umsetzung | autonom | fertig |
| RU32.3 | `RU32.3-aufnahme-je-station.md` | Umsetzung | autonom | offen |
| RU32.4 | `RU32.4-bildvergleich-und-bericht.md` | Umsetzung | autonom | offen |
| RU32.5 | `RU32.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
