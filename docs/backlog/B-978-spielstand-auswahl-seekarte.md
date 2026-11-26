# B-978 · Die Spielstand-Auswahl zeigt die Seekarte mit den Trophäen als Vorschau

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN2
- **Projekt:** WZN
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Spielstand-Auswahl im Menü (`native/k3c-game/menu/saves.go`) zeigt je Stand eine Zeile aus `view.SaveInfo` (Name, Zeitpunkt, Tag, Tiefen, Welt); die Liste liefert `engine/store/saves_list.go` über `native/k3c-game/local/saves.go`. Das Menü kennt Weltbilder (`menu/art.go`, `worldpick.go`). Das Diorama der Seekarte (B-964) und die Chronik im Spielstand (B-966) gibt es dann im Spiel, aber nicht im Menü. Abgespalten aus B-966 (🧑 2026-11-15, Zugang).

## Ziel

Wer in der Spielstand-Auswahl einen Stand markiert, sieht daneben dessen Seekarte mit Trophäen als Vorschau und erkennt, wie weit die Reise ist.

## Beteiligte und Zielgruppen

Spielende am TV vor dem Laden. Claude setzt um, 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

Aus B-966 (🧑 2026-11-15, wörtlich):

- **Zugang: Seekarte und Spielstand-Auswahl.** Im Spiel auf der Seekarte am Hafen (B-964); zusätzlich zeigt die Spielstand-Auswahl im Menü die Karte mit den Trophäen als Vorschau.

Ergänzt:

- **R1 Daten der Liste:** Die Liste der Spielstände liefert je ladbarem Stand zusätzlich die Route der Welt (Inseln mit ID, Name, Etappe), die aktuelle Insel, die abgeschlossenen Inseln und die Chronik (B-966). Die Liste liest dazu nur den Stand und die Welt-Datei, sie baut keine Insel und rechnet keine Simulation.
- **R2 Vorschau:** Neben der Liste zeigt das Menü für den markierten Stand das Diorama (dieselbe Zeichenroutine wie B-964) mit Schiff an der aktuellen Insel, Trophäen auf den abgeschlossenen und, unter der Karte, Abschlusszeile und Zahlen der zuletzt abgeschlossenen Insel. Ohne Zeiger, ohne Wahl.
- **R3 Wechsel:** Mit Auf/Ab in der Liste wechselt die Vorschau sofort; Laden, Löschen und B (zurück) bleiben wie heute.
- **R4 Ohne Daten:** Nicht ladbarer Stand (kaputt, alte Version, unbekannte Welt) → keine Vorschau, die Zeile zeigt wie heute das Problem. Stand ohne abgeschlossene Insel → Karte ohne Trophäen.
- **R5 Rückfall:** Fehlen die Assets, zeigt die Vorschau die gezeichneten Formen (B-964/R5).
- **R6 Bedienung:** Jedes Gerät bedient das Menü wie heute; die Vorschau nimmt keine Eingabe an.

## Nicht-Ziele

Eigener Menüpunkt „Chronik“, Blättern durch alle Abschlusszeilen, Vorschaubild der Burg, Änderungen am Spielstand-Format (B-966), Laden-Logik.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Spielstände liegen in `rust/crates/k3c-room/src/store/`, die Anzeige in `rust/crates/k3c-game` (Entscheidung 013); die Go-Engine bekommt keine neue Mechanik.
- Controller-Taste B geht in jedem Menü zurück (CLAUDE.md).
- Die Session an `store` ist Domäne SRV, die am Menü CLI; Fachprüfung `k3c-hud`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-room/src/store/**`, `rust/crates/k3c-game/src/local/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** B-966, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Zwei Stände: „familie“ (Sengoku, Insel 1 abgeschlossen, auf Insel 2) und „test“ (Standard, Tag 3). Markiert „familie“: rechts die Karte mit Maske und Fahne auf der Reisfeld-Ebene, Schiff an den Bambuswäldern, darunter Abschlusszeile und „14 Tage · 2 Monarchen“. Markiert „test“: Karte mit einer Insel, Schiff daran, keine Trophäe.

## Ausnahme- und Fehlerfälle

- Welt des Stands fehlt oder ist ungültig → Stand nicht ladbar (wie heute), keine Vorschau.
- Stand ohne Chronik (älter als B-966) → Trophäen ohne Zahlen.
- Sehr viele Stände → die Liste liest die Route nur einmal je Aufbau der Seite, nicht je Frame.
- Stand wird gelöscht → Vorschau wechselt auf den nächsten markierten Stand.

## Akzeptanzkriterien

- **AC-01** Go-Test in `engine/store`: Die Liste liefert für einen Stand mit `completed` und `chronicle` Route, aktuelle Insel, abgeschlossene Inseln und Chronik; für einen nicht ladbaren Stand bleiben die Felder leer und `Loadable` ist false.
- **AC-02** Go-Test in `native/k3c-game/menu`: Der markierte Stand bestimmt die Vorschau; Auf/Ab wechselt sie; ein nicht ladbarer Stand zeigt keine Vorschau; Laden, Löschen und B verhalten sich wie vorher (bestehende `saves_test.go` grün).
- **AC-03** Go-Test: Die Vorschau zeigt Trophäe auf abgeschlossenen Inseln, Schiff an der aktuellen und die Zahlen der zuletzt abgeschlossenen Insel; ohne Assets die Formen.
- **AC-04** TV (🧑): Spielstand-Auswahl mit einem Sengoku-Stand nach Insel 1: Karte mit Trophäe ist neben der Liste erkennbar.

## Offene Fragen

- Wo steht die Vorschau? → Default: rechts neben der Liste, für den markierten Stand (vorläufig, Abstimmung am TV).
- Welche Zeile unter der Karte? → Default: Abschlusszeile und Zahlen der zuletzt abgeschlossenen Insel (vorläufig, 🧑 bestätigt).

## Notizen

Abgespalten aus B-966 (2026-11-15, Zerlegung). Zwei Sessions: Liste (SRV), Menü (CLI).
