# B-962 · Am Hafen wählen die Spieler die Zielinsel auf einer Seekarte

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15 (Mock „So umsetzen“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

B-961 führt Etappen und eine Zielwahl am Hafen ein; das Spiel zeigt sie noch nicht.

## Ziel

Ist der Wechselpunkt offen, zeigt das Spiel am Steg eine Seekarte mit den Etappen der Welt; Spieler wählen das Ziel mit Links/Rechts und bestätigen mit A.

## Beteiligte und Zielgruppen

Spieler am PC/TV, Split-Screen. Entscheidet: 🧑.

## Anforderungen

- Seekarte als Overlay in der Hälfte des Spielers am Steg: Etappen untereinander, Inseln als Knoten (abgeschlossen ✓, Ziel wählbar, gesperrt 🔒), aktuelle Auswahl hervorgehoben.
- Links/Rechts wechselt das Ziel, A sendet die Wahl an die Sim (Aktion aus B-961); die Wahl ist für beide Spieler sichtbar.
- Nur eine Insel als Ziel: Overlay zeigt sie ohne Pfeile.
- Die Anzeige rechnet keine Regel, sie liest Ziele und Wahl aus dem Zustand.
- Funktioniert mit 2 Spielern gleichzeitig; B schließt nichts (im Spiel unbelegt).

## Nicht-Ziele

Grafik-Assets der Karte (vorerst gezeichnete Formen), Editor (B-963).

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/src/input/**`
- **Abhängig von:** B-961
- **Braucht 🧑:** TV-Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Sengoku nach Insel 1: Overlay zeigt Etappe 1 (Insel 1 ✓), Etappe 2 mit Insel 2 und 3; Rechts → Insel 3 markiert; A → Wahl gilt, beide sehen Insel 3.

## Ausnahme- und Fehlerfälle

Wechselpunkt schließt (Schiff verloren) → Overlay verschwindet. Spieler ohne Krone → kann nicht wählen.

## Akzeptanzkriterien

- **AC-01** HUD-Test: Overlay erscheint nur bei offenem Wechselpunkt und zeigt Ziele, Abgeschlossene und Gesperrte (`native/k3c-game/hud/*_test.go`).
- **AC-02** Eingabe-Test: Links/Rechts/A erzeugen die Wahl-Aktion, mit 2 Spielern.
- **AC-03** TV-Abnahme durch 🧑.

## Offene Fragen

keine

## Notizen

–
