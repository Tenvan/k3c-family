# SP13 · SIM · Raum-Optionen, Grade und Material-Lager

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-101, B-113
- **Start-Commit:** 1874d9d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf) per /goal „SP13 vorbereiten und im Team komplett abarbeiten“, Revision 1

## Ausgangslage

SP12 hat die `Island` (`engine/sim/island*.go`) geliefert: mehrere Stufen ticken, Einzelwechsel, Vorrat je Insel, Spielstand Version 2. Es fehlen die beschlossenen Regeln für Raum-Optionen und Schwierigkeitsgrade (`docs/rules/wirtschaft.md` § 4, `stufen.md` § 5), den Wellenfaktor je Spieleranzahl und das Material (fünf Rohstoffe, Lager-Maximum, Lager-Gebäude, Tragen zum Lager; `docs/rules/materialien-gebaeude.md` § 1 und § 3.2). Raum, Protokoll und Client benutzen weiter die `Campaign`.

## Ziel

Die Insel kennt **Raum-Optionen** (Schwierigkeitsgrad Dev/Leicht/Normal/Hart/Ultra, Ziel, Niederlage-Modus als Werte), der Grad skaliert Wellen und Gegner, die Wellen wachsen mit der Spieleranzahl der Insel, der Gradwechsel wirkt ab der nächsten Welle. Der Insel-Vorrat hat **fünf Materialien** (mit Eisen und Kristall), ein **Maximum** (300 je Hub plus 300 je Lager), ein **Lager-Gebäude**, und Arbeiter bringen Material zum nächsten Lager oder zur Burg. Am Ende sichtbar: `go test ./engine/sim` mit Tests dazu; `Campaign`, Raum, Protokoll und Golden-Tests bleiben unverändert.

## Beteiligte und Zielgruppen

Entwickler und Agenten (SIM, arbeiten im Team); 🧑 gibt die Spec frei und nimmt nichts am Gerät ab (rein Simulation).

## Anforderungen

B-101 › Anforderungen und B-113 › Anforderungen, soweit sie die Simulation betreffen. Sprint-eigene Abgrenzung: **Alle neuen Regeln gelten nur für Inseln** (`Island`); die `Campaign` und einzelne `World`-Aufrufe behalten ihr Verhalten (kein Wellenfaktor, kein Lager-Maximum, kein Grad). Deshalb bleiben die Golden-Daten unverändert (abweichend von B-101/AC-05 und B-113/AC-04, die „Golden-Daten aktualisiert“ nennen; es ändert sich dort nichts).

## Nicht-Ziele

Protokoll, Raum und Client (B-104, B-123, B-133, B-105, B-107), Wirkung von Ziel und Niederlage-Modus (B-102), Hub-Ausbau und Mauerstufen (B-112), Plantage und Adern (B-114), Skill-Pool (B-118), Bosse (B-130).

## Regeln und Einschränkungen

Domäne SIM: `engine/sim/` und Daten unter `data/` (neue Felder mit vorläufigen Werten aus dem REG-Beschluss). Deterministisch (nur `engine/rng`), Komplexitäts-Budget (Datei ≤ 400 Zeilen, Funktion ≤ 60, Zyklomatik ≤ 15). `engine/sim` importiert nichts aus `room`, `net`, `cmd`. Neue JSON-Felder von bestehenden Strukturen (`Stock`, `QueuedSpawn`) mit `omitempty` oder `json:"-"`, damit Snapshots und Spielstände der `Campaign` unverändert bleiben. Fließkomma: Produkte in Summen mit `float64(…)` runden. Sessions SP13.1 und SP13.2 laufen **parallel in getrennten Worktrees** und berühren sich nur an `data.go`/`island_save.go`; SP13.3 baut auf SP13.2 auf.

## Beispiele

Insel mit 3 Spielern, Grad Hart, Welle 3: Wellengröße × (1 + 0,5 × 2) × 1,25 gerundet; Gegner-HP × 1,3. Wechselt man mitten in der Nacht auf Leicht, gelten die neuen Faktoren erst ab der nächsten Welle. Ein Bauer bringt 10 Holz zum Lager; steht der Vorrat bei 300 von 300, wartet er mit dem Holz, bis Platz ist (oder ein Lager gebaut wird).

## Ausnahme- und Fehlerfälle

Unbekannter Grad, Ziel oder Niederlage-Modus → Fehler. Grad Dev ohne Dev-Mode → abgelehnt. Spielstand ohne Optionen → Standard Normal/Endboss/Stufenverlust. Lager zerstört, Vorrat über der neuen Kapazität → Überschuss bleibt im Vorrat bis verbraucht (kein Verlust). Arbeiter ohne erreichbares Lager und Burg → bleibt stehen.

## Akzeptanzkriterien

- **AC-01** Die Wellengröße einer Insel wächst mit der Spieleranzahl der Insel (Faktor 1 + 0,5 je Zusatzspieler, gerundet); getestet für 1 bis 4 Spieler (B-101/AC-01).
- **AC-02** Je Schwierigkeitsgrad gelten die Faktoren aus `data/difficulty.json` für Wellengröße, Gegner-HP und Gegner-Schaden; Normal ist 1,0 (B-101/AC-02).
- **AC-03** Ein Gradwechsel wirkt ab der nächsten Welle, nie rückwirkend auf laufende oder wartende Gegner (B-101/AC-03).
- **AC-04** Grad, Ziel und Niederlage-Modus stehen im Spielstand der Insel und werden geladen; Dev ist nur im Dev-Mode wählbar (B-101/AC-04).
- **AC-05** Die Kapazität je Rohstoff entspricht 300 × Hubs + 300 × gebaute Lager der Insel (B-113/AC-01).
- **AC-06** Arbeiter bringen Material zum nächsten Lager oder zur Burg; bei vollem Maximum wartet der Arbeiter, nichts geht verloren (B-113/AC-02).
- **AC-07** Eisen und Kristall sind im Insel-Vorrat; der Spielstand speichert und lädt fünf Materialien (B-113/AC-03).
- **AC-08** `Campaign`, Raum, Protokoll und Golden-Tests laufen unverändert; `task check` und `task check:go` sind grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP13.1 | `SP13.1-optionen-grade.md` | Umsetzung | autonom | fertig |
| SP13.2 | `SP13.2-material-kapazitaet.md` | Umsetzung | autonom | fertig |
| SP13.3 | `SP13.3-lager-tragen.md` | Umsetzung | autonom | offen |
| SP13.4 | `SP13.4-review.md` | Review | autonom | offen |

## Abnahme

–
