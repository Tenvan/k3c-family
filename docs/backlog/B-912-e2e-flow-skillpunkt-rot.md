# B-912 · Der Ende-zu-Ende-Lauf TestFlow scheitert, weil Spieler 2 bis Takt 600 keinen Skill-Punkt findet

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NA1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`native/k3c-game/e2e` › `TestFlow` (`flow_test.go`, läuft nur ohne `-short`, also nicht in der CI) scheitert mit „Spieler 2 findet einen Skill-Punkt: bis Takt 600 nicht erreicht“ (`flow_test.go` Zeile 89). Bestätigt am 2026-11-15 auf `sprint/planung` (Stand `main` e0baf94f1): Die Stationen hauptmenue, lobby, spielstart und muenzen (Takt 299) werden erreicht, danach bricht der Lauf ab. Schon `main` 6802a69d9 scheiterte so (Review EP1). Das Skript `flowScript` lässt Spieler 2 ab Takt 21 nach rechts laufen (`1@21=RS`) und bei Takt 380 anhalten; es erwartet in der Welt Standard einen Skill-Punkt auf diesem Weg.

## Ziel

`TestFlow` ist wieder grün und prüft weiter alle Stationen: Ursache gefunden (vermutet: geänderte Welt- oder Biom-Daten, BI1) und Skript bzw. Erwartung angepasst.

## Beteiligte und Zielgruppen

Agenten (Ende-zu-Ende-Schutz des nativen Spiels); 🧑 gibt frei.

## Anforderungen

- Ursache bestimmen und im Ergebnis nennen: Wo liegt der nächste Skill-Punkt der Welt Standard beim Seed des Laufs, erreicht Spieler 2 ihn mit dem Skript, kommt das Ereignis `skillPoint` für Spieler 2 an.
- Liegt es am Test: `flowScript` oder die Takt-Grenzen anpassen, ohne eine Station zu streichen (Skill-Punkt finden, Skill-Menü, Skill lernen bleiben).
- Liegt es am Spiel (Skill-Punkt wird nicht eingesammelt oder nicht gemeldet): Test nicht verbiegen, Ticket mit Fundort anlegen und dieses Ticket als blockiert melden.
- Der Lauf bleibt deterministisch (feste Eingabe-Folge, 2 Spieler: Tastatur und Pad 1).

## Nicht-Ziele

Spielregeln oder Welt-Daten ändern; `TestFlow` in die CI aufnehmen.

## Regeln und Einschränkungen

Keine Simulation über `sim_test` nötig; der Test rechnet die Engine im Prozess.

- **Bereich:** `native/k3c-game/e2e/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test ./e2e/ -run TestFlow -count=1` im Modul `native/k3c-game` → `ok`, alle Stationen mit ✅.

## Ausnahme- und Fehlerfälle

Ursache ist ein Fehler im Spiel → neues Ticket, `TestFlow` bleibt rot, Session meldet blockiert. Der Skill-Punkt liegt je Seed anders → der Test setzt den Seed fest oder sucht den nächsten Skill-Punkt aus dem Frame, statt eine Strecke zu raten.

## Akzeptanzkriterien

- **AC-01** `go test ./e2e/ -run TestFlow -count=2` in `native/k3c-game` ist grün (zwei Läufe gleich).
- **AC-02** Das Ergebnis der Session nennt die Ursache mit Datei und Zeile; die Stationen Skill-Punkt, Skill-Menü und Skill lernen sind weiter Teil des Laufs (`grep -n "skillPoint\|skillmenue" native/k3c-game/e2e/flow_test.go`).

## Offene Fragen

- Skript anpassen oder Position aus dem Frame lesen? → Default: Position des nächsten Skill-Punkts aus dem Frame lesen und die Laufzeit daraus ableiten, damit die nächste Daten-Änderung den Test nicht wieder bricht (vorläufig, 🧑 bestätigt).

## Notizen

Gefunden in EP1.1; Spieler 2 ist seit B-893 Pad 1 statt der rechten Tastatur-Hälfte, das ändert die Simulation nicht. 2026-11-15 erneut bestätigt (rot).
