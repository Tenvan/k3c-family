# RU50 · CLI · Lokales Spiel Teil 2: Spielstände, Aktionen, Entwickler-Rolle

- **Status:** erledigt
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
| RU50.2 | `RU50.2-review.md` | Umsetzung | autonom | fertig |

## Abnahme

2026-11-18 (Second Worker, autonom): AC-01 und AC-02 erfüllt (Nachweis in den Ergebnissen RU50.1 und RU50.2): zwei Spieler tauschen Geräte, lernen Skills und wählen Inseln je Slot, Dev-Rolle (Halt, Einzelschritt, Neu-Laden) läuft; Spielstände im Go-Format am Go-Ort, `testdata/saves/v9/familie.json` lässt sich laden; `local` 63 Tests grün, 0 ignoriert, `pending.rs` gelöscht; `task check` 1967 von 1968 grün (ein Paper-Test mit `EPERM` in `%TEMP%`, einzeln grün), `check:go` Tests ok (`golangci-lint` lokal fehlt).
Offen für den Hauptrechner: Welten-Ordner (`K3C_WORLDS_DIR` nur mit Namen `worlds`, Änderung in `k3c-room`), `dev()` verwirft die Ablehnungsursache, `set_sessions` nicht verdrahtet, Speichern im Frame (`InlineWriter`, Messung RU39).
