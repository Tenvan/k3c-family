# RU50 · CLI · Lokales Spiel Teil 2: Spielstände, Aktionen, Entwickler-Rolle

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

RU27 liefert die Sitzung (`local`: Gerüst, Einstellungen, Welten, Route, Takt, Kampagne) und damit den Start für die Smoke-Version. Spielstände, Aktionen der Spieler und die Entwickler-Rolle waren Teil 3 von RU27 (RU27.3) und stehen hier, weil sie den Raum aus RU24 (Speichern, Dev-Aktionen) brauchen und die Smoke-Version nicht aufhalten sollen.

## Ziel

Spielstände (Liste, Laden, Löschen, freier Name), Aktionen der Spieler (Gerät tauschen, Skills, Insel wählen) und die Entwickler-Rolle der Sitzung laufen in Rust wie in Go. Am Ende sichtbar: `cargo test -p k3c-game local` grün, ein Spielstand im Go-Format lässt sich laden.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler). Der Inhalt ist die frühere Session RU27.3 (verworfen, ersetzt durch RU50.1); `docs/sprints/*/RU27-lokales-spiel/RU27.3-saves-actions-dev.md` bleibt als Vorlage lesbar.

## Nicht-Ziele

Änderungen an Go-Code, `k3c-room`, `view`. Migration älterer Spielstände (Beschluss: bis auf Widerruf keine). Neue Abhängigkeiten.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig. Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:
- **Bereich:** `rust/crates/k3c-game/src/local/**`
- **Abhängig von:** RU27, RU24
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Zwei Spieler: Spieler 2 tauscht sein Gerät, Spieler 1 behält seins. Ein Spielstand im Go-Format liegt in `saves/` → er erscheint in der Liste und lässt sich laden.

## Ausnahme- und Fehlerfälle

Nicht beschreibbarer Spielstand-Ordner → Fehlermeldung wie in Go (`TestSaveDirNotWritable`). Defekter Spielstand → erscheint wie in Go in der Liste (`TestBrokenSaves`).

## Akzeptanzkriterien

- **AC-01** Eine Sitzung mit zwei Spielern tauscht Geräte, lernt Skills und wählt Inseln je Spieler (`actions_tests`, `dev_tests` grün).
- **AC-02** Spielstände und Einstellungen liegen im selben Ordner im selben Format wie beim Go-Spiel (`saves_tests` grün, ein Spielstand im Go-Format lässt sich laden).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU50.1 | `RU50.1-saves-actions-dev.md` | Umsetzung | autonom | fertig |
| RU50.2 | `RU50.2-review.md` | Umsetzung | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
