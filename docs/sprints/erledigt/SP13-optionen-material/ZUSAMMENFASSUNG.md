# SP13 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum-Optionen, Grade und Material-Lager
- **Projekt:** –

## Ziel

Die Insel kennt **Raum-Optionen** (Schwierigkeitsgrad Dev/Leicht/Normal/Hart/Ultra, Ziel, Niederlage-Modus als Werte), der Grad skaliert Wellen und Gegner, die Wellen wachsen mit der Spieleranzahl der Insel, der Gradwechsel wirkt ab der nächsten Welle. Der Insel-Vorrat hat **fünf Materialien** (mit Eisen und Kristall), ein **Maximum** (300 je Hub plus 300 je Lager), ein **Lager-Gebäude**, und Arbe …

## Ergebnis

- 2026-10-02, Review SP13.4 (getrennter Review-Agent): AC-01 bis AC-04 und AC-08 belegt (SP13.1 › Ergebnis), AC-05 und AC-07 (SP13.2), AC-06 (SP13.3 und Review-Fix). - Behobene Befunde: 1 schwerer (wartender Träger blockiert den Bau) und 1 mittlerer Restfall (Bogen holen verliert Material). Bekannt/dokumentiert: Träger-Material geht beim Speichern verloren, Dev-Prüfung beim Laden liegt beim Raum (B-133), getrennte Rundung Standard/Elite. - Neue Tickets: keine. Golden-Daten unverändert (neue Regeln nur für Inseln).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Wellengröße einer Insel wächst mit der Spieleranzahl der Insel (Faktor 1 + 0,5 je Zusa … |
| AC-02 | – | Je Schwierigkeitsgrad gelten die Faktoren aus `data/difficulty.json` für Wellengröße, Gegn … |
| AC-03 | – | Ein Gradwechsel wirkt ab der nächsten Welle, nie rückwirkend auf laufende oder wartende Ge … |
| AC-04 | – | Grad, Ziel und Niederlage-Modus stehen im Spielstand der Insel und werden geladen; Dev ist … |
| AC-05 | – | Die Kapazität je Rohstoff entspricht 300 × Hubs + 300 × gebaute Lager der Insel (B-113/AC- … |
| AC-06 | – | Arbeiter bringen Material zum nächsten Lager oder zur Burg; bei vollem Maximum wartet der … |
| AC-07 | – | Eisen und Kristall sind im Insel-Vorrat; der Spielstand speichert und lädt fünf Materialie … |
| AC-08 | – | `Campaign`, Raum, Protokoll und Golden-Tests laufen unverändert; `task check` und `task ch … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-101 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Raum-Optionen und fünf Schwierigkeitsgrade wirken in der Simulation |
| B-113 | erledigt | AC-05 –, AC-06 –, AC-07 – | Fünf Materialien, Lager-Maximum und Tragen zum Lager sind umgesetzt |
