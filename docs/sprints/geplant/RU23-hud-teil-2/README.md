# RU23 · CLI · HUD Teil 2: Overlays, Skills, Geschichte, Seekarte im Spiel

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

Nach RU20 fehlen die Overlays des HUD.

## Ziel

Alle HUD-Teile des Go-Spiels gibt es im Rust-Spiel. Am Ende sichtbar: Skill-Auswahl, Einblendung der Geschichte, Sieg und Niederlage.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 9 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/hud/` → `rust/crates/k3c-game/src/hud/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/hud/`: overlay (112, T), drawmenu (180), skillmenu (139, T) | 431 |
| 2 | `native/k3c-game/hud/`: skilltext (174), story (196, T), drawstory (60) | 430 |
| 3 | `native/k3c-game/hud/`: seekarte (208, T), seekarte_input (91), seekarte_look (54) | 353 |
| 4 | `native/k3c-game/hud/`: drawsea (140), drawsea_paper (272) | 412 |
| 5 | `native/k3c-game/hud/`: voyage (112, T), drawvoyage (88), drawworld (173), kampf (99, T) | 472 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2098 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** RU20, RU39
- **Braucht 🧑:** –
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
