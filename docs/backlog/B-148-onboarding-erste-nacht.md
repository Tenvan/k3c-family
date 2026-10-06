# B-148 · Die erste Nacht wird mit kontextuellen Hinweisen geführt, der Grad „Leicht“ kostet keinen Fortschritt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** S6
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 2, mit Sprint S6

## Ausgangslage

Der Kern-Loop (Münzen sammeln, bezahlen, bauen, die Nacht überstehen) ist für Kinder nicht selbsterklärend; Hinweise gibt es nur als Text am Bildschirmrand (`src/scenes/HudScene.ts`). Schwierigkeitsgrade liegen als Daten in `data/difficulty.json`.

## Ziel

Die erste Nacht läuft „geführt“: Hinweise erscheinen über den Objekten genau dann, wenn sie gebraucht werden, und ein Grad „Leicht“ macht Verluste ungefährlich. Nutzen: Kinder verstehen den Spielablauf ohne Handbuch (Plan Lücke 6).

## Beteiligte und Zielgruppen

Kinder und Eltern am TV; 🧑 entscheidet Umfang und Grad „Leicht“ und nimmt am Gerät ab.

## Anforderungen

- Die geführte erste Nacht ist optional (Beschluss Q11). Kontextuelle Hinweise über den Objekten (Münze aufheben, Bauplatz bezahlen, Nacht naht, Bauer zuweisen); jeder Hinweis erscheint höchstens einmal je Gerät und verschwindet nach der Handlung.
- Welcher Hinweis gezeigt wird, entscheidet eine reine Funktion aus Snapshot-Daten und bereits gesehenen Hinweisen; der Client rechnet keine Spielregeln.
- Grad „Leicht“ in `data/difficulty.json` (Werte nur dort): die erste Nacht ohne Verlust von Gold, Krone und Gebäuden; Aktivierung über die Gradwahl.
- „Gesehen“-Merkung je Gerät im `localStorage` (mit try/catch); in den Optionen (B-146) rücksetzbar.
- Funktioniert mit 2+ lokalen Spielern (Hinweise je Zelle), Mindestschrift je Split-Viertel nach B-136.

## Nicht-Ziele

Vollständiges Tutorial über mehrere Nächte, Ton (B-011), Controller-Glyphen im Hinweis (B-149), Gegner-Balancing (B-155).

## Regeln und Einschränkungen

`CLAUDE.md` (Client zeichnet nur, Werte in `data/`, 2+ Spieler); der Grad ist Server-Regel in Go und braucht einen Sim-Test. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Neues Spiel im Grad „Leicht“: über der ersten Münze erscheint „Aufheben“, über dem Bauplatz „Bezahlen“; nach dem Bauen verschwinden sie; die erste Nacht endet auch bei Niederlage ohne Verlust.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt → Hinweise erscheinen bei jedem Start. Spieler ignoriert einen Hinweis → er bleibt, blockiert aber keine Eingabe.

## Akzeptanzkriterien

- **AC-01** Test: Die Hinweis-Funktion liefert aus Snapshot-Daten und gesehenen Hinweisen genau den nächsten Hinweis; ein gesehener erscheint nicht erneut.
- **AC-02** Test (Go): Im Grad „Leicht“ verliert die erste Nacht weder Gold noch Gebäude; die Werte stehen in `data/difficulty.json`.
- **AC-03** Hinweise erscheinen über Münze, Bauplatz und beim Nahen der Nacht (Beobachtung am Gerät) und verschwinden nach der Handlung.
- **AC-04** 🧑 hat mit mindestens einem Kind eine erste Nacht gespielt und die Führung abgenommen.

## Offene Fragen

Umfang der Führung und was der Grad „Leicht“ genau abfedert: 🧑, `docs/fragenkatalog.md Q11`.

## Notizen

Aus Plan Phase 1 (S6) und Lücke 6. Die Grad-Werte brauchen eine kleine Go-Änderung; sprengt sie das Budget, als SIM-Ticket abspalten.
