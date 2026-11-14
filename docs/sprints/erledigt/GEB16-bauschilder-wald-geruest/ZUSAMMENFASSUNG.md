# GEB16 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik
- **Projekt:** GEB

## Ziel

Bauplätze zeigen ein Schild nur dort, wo der Server Münzen annimmt, im Bau ein Gerüst in Gebäudeform als Pixel-Grafik, Abbau-Schilder tragen Axt bzw. Spitzhacke, Wälder stehen voller Bäume. Am Ende sichtbar: Spiel am Gerät mit Schildern, Gerüsten und dichtem Wald.

## Ergebnis

2026-10-25, Claude (GEB16.6): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01: GEB16.1 und .2 (`payable` und `minable` im Snapshot, Review ohne schwere Befunde). AC-02: GEB16.5 (Wald [22, 30], Golden neu). AC-03 und AC-04: GEB16.3 und .4 (Gerüste je Breite, Axt und Spitzhacke). Neues Ticket B-620 (KRT): Die Golden-Grenze steigt von 16 auf 20 MB, Ralf bestätigt. 2026-10-31: AC-05 von Ralf (🧑) am Gerät abgenommen, im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Alle Kriterien erfüllt, Sprint erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Snapshot trägt `payable` je Bauplatz, das Schild erscheint nur dort, `buildableRules.t … |
| AC-02 | geprüft | Waldabschnitte sind dichter als zuvor, Golden grün (B-585/AC-01, B-585/AC-02). |
| AC-03 | geprüft | Im Bau steht ein Gerüst-Sprite in Gebäudeform, nie zugleich mit dem Schild (B-586/AC-01, B … |
| AC-04 | geprüft | Axt und Spitzhacke liegen als Grafik im Katalog und erscheinen am Abbau-Schild (B-588/AC-0 … |
| AC-05 | – | Ralf hat Schilder, Wald, Gerüst und Abbau-Symbole am Gerät abgenommen (B-584/AC-02, B-585/ … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-584 | erledigt | AC-01 geprüft, AC-05 – | Bauplätze zeigen kleine Bauschilder statt grauer Rahmen |
| B-585 | erledigt | AC-02 geprüft, AC-05 – | Wald-Chunks stehen voller Bäume |
| B-586 | erledigt | AC-03 geprüft | Im Bau zeigt der Platz ein Gerüst in der Form des neuen Gebäudes |
| B-587 | erledigt | AC-01 geprüft | Der Server meldet je Bauplatz, ob er jetzt Münzen annimmt |
| B-588 | erledigt | AC-04 geprüft | Abbau-Schilder zeigen eine Axt für Bäume und eine Spitzhacke für Felsen |
