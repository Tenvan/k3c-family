# RU3 · SIM · rng, Go-gleiche Zahlen und Daten-Schemas in Rust

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-17

## Ausgangslage

`engine/rng` (85 Zeilen) hat ein Golden. Die Sim nutzt `math.Pow` (7×), `math.Exp`, `math.Round`, `math.Mod`; Go rechnet `Pow` in eigenem Code, Rust ruft die C-Laufzeit. `encoding/json` schreibt Zahlen anders als `serde_json`.

## Ziel

Würfeln und Gleitkomma-Funktionen liefern in Rust dieselben Bits wie in Go, Golden lassen sich lesen und vergleichen, alle Spiel-Daten laden. Am Ende sichtbar: `task rust:test` vergleicht gegen `testdata/golden/rng.json` und gegen Vektoren aus Go.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 3 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Hinweise für die Umsetzung

- `math.Exp` läuft in jedem Schritt (`engine/sim/economy.go:61`), `math.Pow` beim Erzeugen von Gegnern und Bossen (`enemies.go:36`, `boss.go:41`). Maßgeblich ist die Rechnung auf amd64; die Quelle steht in `$(go env GOROOT)/src/math` (`pow.go`, `exp.go`, `exp_amd64.go`, `exp_asm.go`). Erst lesen, welche Fassung amd64 nutzt, dann genau diese portieren.
- Rust-Bordmittel `powf`, `exp`, `f64::max`, `f64::min` sind in den Sim-Crates verboten (Leitfaden § 5).
- Die Vektoren schreibt ein Go-Test in `engine/internal/detvec` nur mit `-run TestWriteVectors -write`; ohne `-write` vergleicht er.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-rng/**`, `rust/crates/k3c-det/**`, `rust/crates/k3c-data/**`, `engine/internal/detvec/**`, `testdata/rust/**`
- **Abhängig von:** RU1, RU2
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `k3c-rng` erzeugt jede Zahl aus `testdata/golden/rng.json`; Überlauf rechnet umlaufend wie in Go.
- **AC-02** `k3c-det` liefert für `pow`, `exp`, `fmax`, `fmin` dieselben Bits wie Go auf mindestens 10 000 Vektoren je Funktion aus `engine/internal/detvec` (Datei `testdata/rust/det-vectors.json.gz`).
- **AC-03** `k3c_det::golden` liest `.json` und `.json.gz`, vergleicht zwei JSON-Bäume (jede Zahl als f64 mit denselben Bits; `null` und leere Liste gelten als gleich) und nennt bei Abweichung den Pfad des ersten Unterschieds.
- **AC-04** Alle Spiel-Daten unter `data/` (ohne die Workbench-Dateien `asset-*` und `assets-*` und ohne `data/.omc`) laden in typisierte Strukturen; je Datei ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur (`testdata/rust/data-tree.json.gz`); Laden von Platte (`K3C_DATA_DIR`) und eingebettet.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU3.1 | `RU3.1-rng.md` | Umsetzung | autonom | fertig |
| RU3.2 | `RU3.2-go-gleiche-zahlen.md` | Umsetzung | autonom | offen |
| RU3.3 | `RU3.3-golden-lesen-und-vergleichen.md` | Umsetzung | autonom | offen |
| RU3.4 | `RU3.4-daten-schemas-teil-1.md` | Umsetzung | autonom | offen |
| RU3.5 | `RU3.5-daten-schemas-teil-2.md` | Umsetzung | autonom | offen |
| RU3.6 | `RU3.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
