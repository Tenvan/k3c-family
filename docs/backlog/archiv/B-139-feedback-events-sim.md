# B-139 · Die Simulation meldet Feedback-Ereignisse für Treffer, Münzen, Schläge und Tod

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** F3
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F3

## Ausgangslage

`World.Events` (`engine/sim/types.go`) kennt nur grobe Ereignisse: `armed`, `arrived`, `built`, `castleFallen`, `chest`, `dawn`, `destroyed`, `dusk`, `gathered`, `goldStolen`, `night`, `playerDown`, `recruited`, `skillPoint`, `wave`. Treffer, Gegner-Tod, Münze aufgehoben oder gegeben, Pfeil und Schlag fehlen. Ohne sie gibt es weder Ton (B-167) noch Effekte (B-164), ohne dass der Client rechnet, und der Client rechnet nicht (`CLAUDE.md`).

## Ziel

Die Simulation erzeugt für jedes spürbare Ereignis genau ein Ereignis mit Typ, Ort und Beteiligten. Nutzen: Ton und Juice werden aus Server-Ereignissen gespeist, deterministisch.

## Beteiligte und Zielgruppen

Spieler (hören und sehen Rückmeldung), Entwickler von SO1/GR5; Liste und Budget entscheidet 🧑 (Beschluss Q08).

## Anforderungen

- Neue Ereignistypen mindestens für: Treffer (Gegner trifft, Truppe trifft), Gegner-Tod, Münze aufgehoben, Münze gegeben, Pfeil abgeschossen, Schlag, Bau fertig, Monarch-Tod; die endgültige Liste legt Q08 fest.
- Jedes Ereignis trägt Typ, Ort (Welt-X in Units, Stufe), Beteiligte (Spielerindex bzw. Einheiten-ID), wo sinnvoll Menge.
- Höchstens K Ereignisse je Tick und Stufe (K legt Q08 fest); Überlauf verwirft die niedrigste Priorität, nie ein Todes- oder Bau-Ereignis.
- Die Ereignisse sind deterministisch: gleiche Seeds und Eingaben ergeben dieselbe Ereignisfolge.
- Ereignisse werden wie bisher bei jedem Step geleert; Golden-Daten werden mit dem Ablauf aus B-137 aktualisiert.

## Nicht-Ziele

Übertragung im Protokoll und Bandbreitenmessung (B-140), Ton (B-167), Effekte (B-164), Replays (B-159).

## Regeln und Einschränkungen

Domäne SIM (`engine/sim/`); Schichtgrenzen aus `docs/arbeitsweise.md`; nur `engine/rng`; Funktion ≤ 60 Zeilen, Datei ≤ 400 (neue Ereignisse in eigener Datei `engine/sim/events.go`, falls nötig). Mit 2+ Spielern, auch Insel mit mehreren Stufen (`engine/sim/island*.go`).

## Beispiele

Gegner trifft die Mauer → ein Ereignis `hit` mit Ziel „Mauer“, Ort, Schaden; Münze wird von Spieler 2 aufgehoben → `coinPickup` mit Spielerindex 1.

## Ausnahme- und Fehlerfälle

Mehr Ereignisse als K in einem Tick (großer Kampf) → die Priorität entscheidet, die Zahl der verworfenen Ereignisse steht als Zähler im Ergebnis des Ticks.

## Akzeptanzkriterien

- **AC-01** Je Typ der beschlossenen Liste existiert ein Test, der das Ereignis in einer Welt auslöst und Typ, Ort und Beteiligte prüft (`task go:test`).
- **AC-02** Test: Zwei Läufe mit gleichem Seed und gleichen Eingaben ergeben identische Ereignisfolgen über 600 Ticks.
- **AC-03** Test: Bei Überschreiten von K Ereignissen je Tick bleiben Tod- und Bau-Ereignisse erhalten und ein Zähler weist die verworfenen aus.
- **AC-04** Test: Auf einer Insel mit zwei aktiven Stufen entsteht ein Ereignis nur in der Stufe, in der es passiert, mit deren Stufen-Index.
- **AC-05** Golden-Daten sind aktualisiert (Ablauf aus B-137); `task check:go` grün.

## Offene Fragen

Endgültige Liste der Ereignisse, K je Tick und Priorität: `docs/fragenkatalog.md` Q08, entscheidet 🧑. Blockiert die Freigabe der Sprint-Spec F3.

## Notizen

Aus Plan Phase 0 (F3) und Lücke 1. Protokoll: B-140.
