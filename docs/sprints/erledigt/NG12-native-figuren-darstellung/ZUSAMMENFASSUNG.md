# NG12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Anzeigen an Figuren, Bäume und Geschosse
- **Projekt:** NAT

## Ziel

Figuren, Bäume und Geschosse des nativen Spiels zeigen dieselben Anzeigen wie der Browser-Client. Am Ende sichtbar: Tests in `actors`, `props`, `fx` grün, `task native:game:e2e` mit zwei gleichen Läufen, Bilder unter `reports/native-game-e2e/`.

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, Befunde behoben oder weitergegeben (siehe Review-Session). Version: keine vorgeschlagen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Lebensbalken je Figurentyp wie im Browser (`B-806/AC-01`). |
| AC-02 | – | Goldkreis, Monarchen-Punkt, freier Sitz unsichtbar (`B-806/AC-02`). |
| AC-03 | – | Last der Bauern (`B-806/AC-03`). |
| AC-04 | geprüft | Bildvergleich zweier Läufe gleich, Bilder abgelegt (`B-806/AC-04`). |
| AC-05 | – | Gefällter Baum mit Balken und Schrumpffaktor (`B-811/AC-01`). |
| AC-06 | – | Geschossfarbe nach Seite, Gegner rot (`B-811/AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-806 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft | Das native Spiel zeigt Lebensbalken, Beute-Gold, Monarchen-Punkt und Last wie de … |
| B-811 | erledigt | AC-05 –, AC-06 – | Plantagen-Bäume, gegnerische Geschosse und die Anleitung sehen im nativen Spiel … |
