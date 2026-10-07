# B-343 · Der Bau des Endbosses liegt an der inneren Kante des Ausgangs-Chunks

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K2.1c sollte den Bau (`lair`) laut Planungs-Vorschlag in das äußerste freie Chunk auf der Seite `exitSide` legen, sonst Fehler beim Erzeugen. Das widerspricht bestehenden Regeln: Als eigenes Chunk vor den Portalen nimmt der Bau in kurzen Kristallhöhlen das einzige Chunk mit Abstand zu den Linien, das ein Camp braucht (B-261, `camps_test.go`, 151 Fehler über 500 Seeds), und senkt die Dichte unter die des Eisenstollens (3,15 statt 3,67 je 100 Units, Q28, `TestBreiteUndDichte`). Nach Portalen und Ereignissen ist auf der Ausgangsseite oft kein freies Chunk mehr übrig (Seed 9 und viele Golden-Seeds: Fehler). Umgesetzt ist deshalb: Der Bau ist nur eine Entität an der inneren Kante des Ausgangs-Chunks („am Ende vor dem Eingang“, `docs/rules/bosse.md` § 1), ohne eigenes Chunk und ohne Würfel (`engine/level/level.go` › `lairX`).

## Ziel

🧑 bestätigt die Lage des Baus oder legt eine andere fest, die mit B-261 und Q28 verträglich ist.

## Beteiligte und Zielgruppen

🧑 entscheidet; SIM setzt eine Änderung um.

## Anforderungen

- Der Bau liegt in jedem Seed der Kristallhöhle an einem festen, spielbaren Ort; Portale, Ereignisse, Ressourcen und Dichte bleiben unverändert.

## Nicht-Ziele

Grafik des Baus (K5), weitere Endbosse.

## Regeln und Einschränkungen

Level-Generator ohne zusätzlichen Würfel für Biome ohne Bau; B-261 (Camp-Abstand), Q28 (Dichte).

## Beispiele

Kristallhöhle mit 450 Units, Ausgang rechts bei 437,5 → Bau bei 425; ein Portal im Nachbar-Chunk liegt dann 12,5 Units vom Bau entfernt.

## Ausnahme- und Fehlerfälle

nicht relevant: Die heutige Lage gibt es in jedem Seed.

## Akzeptanzkriterien

- **AC-01** Die Lage des Baus ist von 🧑 bestätigt oder geändert, `TestLairNurInDerKristallhoehle`, `TestCampAbstandZuLinien500Seeds` und `TestBreiteUndDichte` sind grün.

## Offene Fragen

- Bleibt der Bau an der inneren Kante des Ausgangs-Chunks? Soll er einen Mindestabstand zu Portalen halten (heute kann ein Portal 12,5 Units entfernt liegen)? Entscheidet 🧑.

## Notizen

Entstanden in K2.1c (Sprint K2).
