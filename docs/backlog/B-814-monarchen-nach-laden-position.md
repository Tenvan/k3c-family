# B-814 · Geklärt ist, warum Monarchen nach dem Laden an der Burg statt an ihrer gespeicherten Stelle stehen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG16
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Im Ende-zu-Ende-Lauf NG9.1 stehen die Monarchen nach dem Laden an der Burg statt an ihrer gespeicherten Stelle. Vorprüfung (Claude, 2026-11-12, Planung): `IslandPlayerSave` in `engine/sim/island_save.go` enthält keine Position des Monarchen (nur Index, Gold, Tiefe, Skills, Slots, SkillBase); ob die Engine den Monarchen beim Laden bewusst an die Burg setzt und ob Regeln das festlegen, ist ungeprüft.

## Ziel

Geklärt und festgehalten ist, ob die Burg-Position nach dem Laden gewollt ist; ein Fehler ist in `engine/` behoben.

## Beteiligte und Zielgruppen

Ralf entscheidet bei Spielregeln, Claude prüft.

## Anforderungen

Prüfen, dann entweder dokumentieren oder beheben:

- Code (`engine/sim` Laden der Insel, `engine/room` Laden) und Regeln (`docs/rules/`, `docs/game-design.md`) prüfen: Setzt die Engine den Monarchen beim Laden bewusst an die Burg?
- **Gewollt** (Position nicht im Spielstand, Burg bewusst gesetzt): Kommentar an der Ladestelle und ein Test im Raum- oder Sim-Paket, der die Burg-Position nach dem Laden festhält; Ergebnis mit Fundstellen in den Notizen; das Ticket wird danach `verworfen` mit Begründung „gewollt, dokumentiert“.
- **Fehler** (Position sollte erhalten bleiben): Behebung in `engine/` mit Test im Raum-Paket (Speichern bei x=40, Laden, Monarch bei x=40); ändert das das Spielstand-Format, gilt `docs/arbeitsweise.md` › Spielstand-Format ändern (Sperre `spielstand`, keine Migration alter Stände).

## Nicht-Ziele

Änderung des Spielstand-Formats, wenn die Prüfung „gewollt“ ergibt; Änderungen am nativen Client.

## Regeln und Einschränkungen

Sim ist deterministisch. Keine Rücksicht auf alte Spielstände (bis auf Widerruf). Golden-Dateien nur nach Regel, falls die Behebung sie ändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `engine/sim/**`, `engine/room/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** spielstand

## Beispiele

Monarch steht bei x=40, Spiel speichern und laden → heute steht er an der Burg.

## Ausnahme- und Fehlerfälle

Spielstand ohne Monarch-Position → Burg als Rückfall (wie heute).

## Akzeptanzkriterien

- **AC-01** Belegte Antwort in den Notizen: gewollt oder Fehler, mit Fundstelle in Code oder Regeln.
- **AC-02** Gewollt: Kommentar an der Ladestelle und Test, der die Burg-Position nach dem Laden festhält; Ticket danach `verworfen` mit Begründung. Fehler: Behebung in `engine/` mit Test im Raum-Paket, Monarch steht nach dem Laden an der gespeicherten Stelle.

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Default „gewollt“, wenn die Position nicht im Spielstand steht und die Engine den Monarchen beim Laden bewusst an die Burg setzt; dann dokumentieren und verwerfen. Nur ein belegter Fehler wird behoben. Sperre `spielstand` vorsorglich gesetzt, weil eine Behebung das Format ändern könnte; in der Welle 4a trägt sie sonst niemand.

## Notizen

Gefunden in NG9.1 (2026-11-11). Revision 2 (2026-11-12): als „prüfen, dann dokumentieren oder beheben“ formuliert, Bereich um `engine/room/**` ergänzt.
