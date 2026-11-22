# B-1220 · Spiel mit --playtest zeigt REC und schreibt Pad-Marker mit Spielkontext

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ66
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Rust-Spiel (`rust/crates/k3c-game`) hat keine Verbindung zur Workbench außer der Pipe zum Cheat-Fenster (`devwin/`, `game/devwinhost.rs`). Frame-Zeiten misst es nur mit `--measure`/`--measure-frames` (`game/measure*.rs`). Für Spieltests (WZ64) fehlen ein sichtbares Aufnahme-Zeichen und eine Möglichkeit, vom Sofa aus wichtige Momente zu markieren.

## Ziel

Mit `--playtest <ordner>` zeigt das Spiel ein REC-Zeichen und schreibt Marker, die jeder Spieler per Pad setzt, samt Spielkontext nach `<ordner>/markers.jsonl`. Die Auswertung (WZ65) weiß so, wo und wann etwas auffiel.

## Beteiligte und Zielgruppen

Spieler am Sofa (bis 2 mit eigenem Pad), Ralf (🧑) bei der Auswertung.

## Anforderungen

- Neues CLI-Flag `--playtest <ordner>`; ohne Flag verhält sich das Spiel unverändert.
- REC-Zeichen: kleiner roter Punkt mit „REC“, gut lesbar am TV, verdeckt kein HUD-Element, einmal pro Bildschirm (nicht je Split-Ansicht).
- Marker: Pad-Kombination **Back+Y** je Spieler (B bleibt frei), Tastatur-Taste als Ersatz (vorgeschlagen F9, Belegung in der Session prüfen); kurze Rückmeldung „Markiert“ in der Ansicht des Spielers.
- Je Marker eine Zeile JSON in `markers.jsonl`: `ts` (Unix-ms), `kind` (`pad`), `player` (1/2), `world`, `island`, `stage` je Spieler, Tag/Nacht und Tag-Nummer, schlechteste Frame-Zeit der letzten 5 s in ms.
- Datei wird nach jedem Marker geschrieben (Absturz verliert keine Marker).
- Funktioniert mit 2 Spielern gleichzeitig (Split-Screen, eigene Eingabe).
- Logik in einem eigenen Modul `playtest/`, das Spiel zeichnet nur; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Ruckler-Automarker und Spielstand am Marker (B-1221), Fragebogen (B-1222).
- Aufnahme von Ton oder Bild im Spiel (macht die Workbench, B-1219).
- Neues Protokoll zur Workbench: Austausch nur über Dateien.

## Regeln und Einschränkungen

- Controller-Taste B bleibt zurück im Menü und im Spiel unbelegt.
- Keine neue Mechanik, kein Einfluss auf Simulation oder Spielstand.
- Ordner-Vertrag: B-1218 › Anforderungen.

- **Bereich:** `rust/crates/k3c-game/src/playtest/**`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/src/input/**`, `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

- Spieler 2 drückt Back+Y in der Nacht auf Insel 1, Stufe 2 → Ansicht 2 zeigt „Markiert“, `markers.jsonl` erhält eine Zeile mit `player: 2`, `stage: 2`, `night: true`.
- Spiel ohne Flag → kein REC, Back+Y ohne Wirkung.

## Ausnahme- und Fehlerfälle

- Ordner nicht beschreibbar → Spiel läuft weiter, REC-Zeichen zeigt Fehlerzustand (durchgestrichen), Log ❌.
- Marker-Taste gehalten → höchstens ein Marker je Sekunde und Spieler.
- Back+Y während eines Menüs → Marker wird trotzdem gesetzt, Menü reagiert nicht.

## Akzeptanzkriterien

- **AC-01** Unit-Test: Marker-Zeilen haben alle Felder und sind gültiges JSON, je Spieler gedrosselt auf 1/s.
- **AC-02** Unit-Test: ohne `--playtest` entsteht keine Datei und kein REC.
- **AC-03** Am TV mit 2 Pads: REC sichtbar, beide Spieler setzen Marker, Zeilen tragen den richtigen Spieler (🧑 prüft im Spiel).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Ralf prüft im Spiel: Ist Back+Y bequem erreichbar und stört das REC-Zeichen nicht?
