# RU7 · CLI · Grafik laden und Ansicht: Papierschnitt-Index, Sheets, Kamera

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`native/k3c-game/art` (1174 Zeilen) lädt Gruppen-Indizes, Manifeste und Sheets; `view` (1293 Zeilen) hält Kamera, Maße und Zeichenhilfen, auf die alle Fachpakete bauen.

## Ziel

Das Rust-Spiel lädt die Papierschnitt-Grafik unverändert und hat die gemeinsame Ansicht. Am Ende sichtbar: Testszene mit Figuren, Gebäuden und Hintergrund aus `public/grafik`.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 4 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/view/` → `rust/crates/k3c-game/src/view/`, `native/k3c-game/art/` → `rust/crates/k3c-game/src/art/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/view/`: cell (121, T), crown (7, T), devkeys (132, T), doc (68), frame (192, T) | 520 |
| 2 | `native/k3c-game/view/`: input (126, T), keys (33), layer (76, T), profile (94, T), roles (50), screen (44), session (73) | 496 |
| 3 | `native/k3c-game/view/`: settings (105, T), story (29), stress (61), stress_limits (57), voyage (25); `native/k3c-game/art/`: budget (30), font (108) | 415 |
| 4 | `native/k3c-game/art/`: library (270), manifest (93), outline (133, T), plan (85) | 581 |
| 5 | `native/k3c-game/art/`: rigs (83, T), scales (36), seamap (108, T), sheet (65), voyage (74, T), worldimages (89, T) | 455 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2467 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/view/**`
- **Abhängig von:** RU4, RU5
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Alle Gruppen aus `public/grafik/<gruppe>/index.paper.json` und die Zuordnung aus `data/paper.json` laden ohne Änderung an den Dateien.
- **AC-02** Der Grafikspeicher bleibt bei vollem Level unter dem in G1 gemessenen Wert.
- **AC-03** Kamera, Units (`UNIT_PX` = 32) und zwei Bildhälften verhalten sich wie in `native/k3c-game/view`; die Tests des Pakets sind portiert.

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
