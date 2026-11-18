# RU20 · CLI · HUD Teil 1: Anzeigen je Spielerhälfte

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 314b3ab22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`hud` ist mit 4629 Zeilen in 31 Dateien das größte Paket des Spiels (28 Importe der Sim).

## Ziel

Die ständigen Anzeigen des HUD laufen je Spielerhälfte. Am Ende sichtbar: Münzen, Krone, Tageszeit, Tasten-Icons.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 7 von 26, 8 Sessions (B-1015; zwei mehr als geplant: `drawglyph.go` und `keyline.go` (zusammen 380 Zeilen, Tasten-Icons) standen in keinem Sprint-Schnitt, und AC-01 verlangt einen Bildvergleich, der eine eigene Session bekommt).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/hud/` → `rust/crates/k3c-game/src/hud/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru20`. `hud/**` teilt dieser Sprint mit RU23 (läuft später, Abhängigkeit RU20): RU20 legt **keine** Datei von RU23 an (`overlay`, `drawmenu`, `skillmenu`, `skilltext`, `story`, `drawstory`, `seekarte*`, `drawsea*`, `voyage`, `drawvoyage`, `drawworld`, `kampf`); was RU20-Dateien daraus brauchen, steht als Platzhalter in `hud/pending.rs`. Reihenfolge nach Abhängigkeiten (Texte und Rechner vor Zustand, Zustand vor Zeichner):

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/hud/`: texts (197), data (131), fonts (107), layout (193, T) | 628 |
| 2 | `native/k3c-game/hud/`: values (319, T), glyphs (233, T) | 552 |
| 3 | `native/k3c-game/hud/`: hints (242, T), banner (130, T), radar (90) | 462 |
| 4 | `native/k3c-game/hud/`: state (260, T), hud (86), icons (121, T); `pending.rs` mit Platzhaltern für RU23 | 467 |
| 5 | `native/k3c-game/hud/`: paint (149), draw (209), purse (34, T), drawstock (85) | 477 |
| 6 | `native/k3c-game/hud/`: drawglyph (190), keyline (190, Test `keyicons_test.go`) | 380 |
| 7 | Standbild `k3c-game still --hud` und Bildvergleich an vier Ständen (Go-Test `hud/rustref_test.go`) | – |
| 8 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2966 Zeilen Go (Tabelle vorher 2531: Zeilenzahlen mit dem Repo-Stand abgeglichen und `drawglyph`, `keyline` ergänzt). Jede Port-Session erfüllt die Kriterien für ihre Dateien, Session 7 liefert den Bildvergleich, das Review prüft alle. Tests anderer Dateien, die RU23-Dateien brauchen, bleiben bis RU23 `#[ignore = "wartet auf RU23"]`.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `native/k3c-game/hud/rustref_test.go` (Session 7). `k3c-platform` und `k3c-sim` bleiben unverändert.

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/still/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/tests/**`, `native/k3c-game/hud/rustref_test.go`, `testdata/rust/bild/**`
- **Abhängig von:** RU16
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Anzeigen stehen je Hälfte an den Plätzen des Go-Spiels (Bildvergleich).
- **AC-02** Tasten-Icons zeigen nur Xbox-Symbole und Tastenkappen.
- **AC-03** Die Tests der portierten Dateien sind grün.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **`drawglyph.go` und `keyline.go`:** fehlten in RU20 und RU23. Default: RU20 (Tasten-Icons, AC-02), Session 6; dafür 8 statt 6 Sessions.
- **Naht `SimView`:** `hud` ruft `EconomyOf`, `SitePayable`, `AtLair` über `props::SimView` aus RU16.1 (Sim-Funktionen erst RU15/RU18); `CanLearn`, `CanRespec`, `AvailablePoints` (Skill-Menü) braucht RU23 und erweitert die Naht dort (RU23 beachten).
- **Platzhalter statt RU23-Dateien:** Typen und Funktionen von RU23, die `state.go` und `hud.go` verwenden, stehen als Platzhalter mit neutralem Wert in `hud/pending.rs` (RU20.4), damit `Listen` und `draw_items` ohne RU23 laufen; RU23 löscht sie.
- **Bildvergleich:** nur `draw_items` (Anzeigen je Hälfte), ohne Welt-Texte, Seekarte, Menü; Go zeichnet dieselbe Teilmenge über die interne Testdatei `hud/rustref_test.go`. Die Teile von RU23 vergleicht RU23.
- **Textregel-Test:** Rust-Test mit Zeichenscan statt Go-Parser (RU20.4).

**Nachzug aus Go-Sprints:** keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU20.1 | `RU20.1-hud-texts-data-fonts-layout.md` | Umsetzung | autonom | fertig |
| RU20.2 | `RU20.2-hud-values-glyphs.md` | Umsetzung | autonom | fertig |
| RU20.3 | `RU20.3-hud-hints-banner-radar.md` | Umsetzung | autonom | fertig |
| RU20.4 | `RU20.4-hud-state-hud-icons.md` | Umsetzung | autonom | fertig |
| RU20.5 | `RU20.5-hud-paint-draw-purse-drawstock.md` | Umsetzung | autonom | fertig |
| RU20.6 | `RU20.6-hud-drawglyph-keyline.md` | Umsetzung | autonom | fertig |
| RU20.7 | `RU20.7-standbild-bildvergleich.md` | Umsetzung | autonom | offen |
| RU20.8 | `RU20.8-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
