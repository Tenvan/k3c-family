# RU23 · CLI · HUD Teil 2: Overlays, Skills, Geschichte, Seekarte im Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 7edb03edb
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU20 fehlen die Overlays des HUD.

## Ziel

Alle HUD-Teile des Go-Spiels gibt es im Rust-Spiel. Am Ende sichtbar: Skill-Auswahl, Einblendung der Geschichte, Sieg und Niederlage.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 7 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/hud/` → `rust/crates/k3c-game/src/hud/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf `sprint/ru23`. Zeilen gegen das Repo geprüft (2026-11-17).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/hud/`: overlay (112, T), skillmenu (139, T), skilltext (186); dazu `profile_test`, `guide_test` (Speichern) | 437 |
| 2 | `native/k3c-game/hud/`: drawmenu (134), story (196, T), drawstory (60); dazu `menubox_test`, `story_fit_test` | 390 |
| 3 | `native/k3c-game/hud/`: seekarte (203, T), seekarte_input (91), seekarte_look (54); dazu `seekarte_hold_test`, vier Tests aus `seekarte_paper_test` | 348 |
| 4 | `native/k3c-game/hud/`: drawsea (143), drawsea_paper (272); dazu vier Tests aus `seekarte_paper_test` | 415 |
| 5 | `native/k3c-game/hud/`: voyage (112, T), drawvoyage (88), drawworld (173), kampf (99, T); dazu `labels_test`, `guide_test` (Bild), Radar-Tests aus `kampf_test` | 472 |
| 6 | Bildvergleich Seekarte, Fahrt und Geschichte gegen Go-Bilder (Aufnahme-Tests des Go-HUD) | – |
| 7 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2062 Zeilen Go (die erste Fassung nannte 2098: `drawmenu` 134 statt 180, `skilltext` 186 statt 174, `seekarte` 203 statt 208, `drawsea` 143 statt 140). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle. Abweichungen vom Schnitt: `skilltext` wandert in Session 1 und `drawmenu` in Session 2 (`drawmenu` zeichnet die Typen aus `skilltext`); Session 6 ist neu (siehe Offene Fragen).

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

Ausnahme für Session 6: Die Go-Bilder entstehen mit den vorhandenen Aufnahme-Tests des Go-HUD (kein Sim-Lauf, kein Balancing); `task gen:paper` nur dort (B-742).

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/tests/hud_bild.rs`, `testdata/rust/bild/hud/**`, `rust/crates/k3c-game/src/lib.rs` (nur `pub mod hud;`, falls RU20 es nicht tat)
- **Abhängig von:** RU20, RU39
- **Braucht 🧑:** – (Haltepunkt in RU23.6: fehlt der Rasterer `still/` aus RU13.6 oder sind die Go-Bilder größer als etwa 600 KB, hält die Session an und fragt 🧑)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Overlays öffnen und schließen je Spieler, ohne die andere Hälfte anzuhalten.
- **AC-02** Sieg, Niederlage und Seekarte sehen aus wie im Go-Spiel (Bildvergleich).
- **AC-03** Alle Tests aus `native/k3c-game/hud` sind portiert und grün.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client. Spec steht im Ticket:

- GS3.3 Andenken, Abschlusszeile und Zahlen auf der Seekarte (B-966)
- GS4.4 Flasche, Geschichtszeile, Pfeil und Leuchten (B-980)

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- Session 6 (Bildvergleich) ist neu, weil AC-02 einen Bildvergleich verlangt und ihn sonst keine Session liefert; Quelle der Go-Bilder sind die Aufnahme-Tests des Go-HUD (`K3C_SEA_SHOT`, `K3C_VOYAGE_SHOT`, `K3C_STORY_SHOT`), nicht neuer Go-Code.
- „Sieg und Niederlage“ (Ziel): Im Go-HUD gibt es dazu keinen eigenen Bildschirm. Sieg zeigt die Geschichtszeile (`story.go`, Sieg-Zeilen aus der Sim), Niederlage den Hinweis im Hauptmenü (`outcome_note`, RU9.4). AC-02 meint damit Geschichtszeile, Seekarte und Fahrt.
- Sim-Funktionen, die `hud` ruft und die es in `k3c-sim` erst mit RU14 (Inseln, Welt laden), RU18 (`can_learn`, `can_respec`, `learn_skill`, `respec`) und RU19 (`story_*`) gibt: Platzhalter in `hud/pending.rs`, Tests mit `#[ignore = "wartet auf RUnn"]`. RU22 räumt auf; die Sprints RU14, RU18 und RU19 sollten die Platzhalter löschen, sobald sie die Funktion liefern (Bereich der Sim-Sprints müsste dann `hud/pending.rs` einschließen).
- Test-Hilfen aus `overlay_test.go` (`session`, `twoPlayers`, `input`, `acts`) braucht schon `state_test.go` aus RU20: `hud/testutil.rs` legt RU20 an, sonst RU23.1.
- `keyline.go` (190) und `drawglyph.go` (190) stehen weder in RU20 noch in RU23, werden aber von `skilltext`, `seekarte` und dem Skill-Menü gebraucht: sie gehören nach RU20 (Session 4, Tasten-Icons); fehlen sie dort, hält RU23.1 an und trägt ein Ticket in RU20 ein.
- Die zwei Radar-Tests in `kampf_test.go` prüfen `radar.go` (RU20) und werden in RU23.5 portiert, weil ihre Datei dort liegt.
- Allokations-Tests (`testing.AllocsPerRun`, in `seekarte`, `story`, `voyage`) sind in Rust nicht portierbar und entfallen mit Zeile im Ergebnis; das Frame-Budget prüft die Messung (RU39).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU23.1 | `RU23.1-hud-overlay-skillmenu-skilltext.md` | Umsetzung | autonom | fertig |
| RU23.2 | `RU23.2-hud-drawmenu-story-drawstory.md` | Umsetzung | autonom | fertig |
| RU23.3 | `RU23.3-hud-seekarte-logik.md` | Umsetzung | autonom | offen |
| RU23.4 | `RU23.4-hud-drawsea-drawsea-paper.md` | Umsetzung | autonom | offen |
| RU23.5 | `RU23.5-hud-voyage-drawworld-kampf.md` | Umsetzung | autonom | offen |
| RU23.6 | `RU23.6-hud-bildvergleich.md` | Umsetzung | autonom | offen |
| RU23.7 | `RU23.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
