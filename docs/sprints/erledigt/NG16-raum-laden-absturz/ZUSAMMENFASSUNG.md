# NG16 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum: Absturzbehandlung nach außen, Position nach dem Laden
- **Projekt:** NAT

## Ziel

Geklärt und belegt ist die Position der Monarchen nach dem Laden; der Raum bietet seine Absturzbehandlung und den Grund abgelehnter Skill-Befehle nach außen an, `local` nutzt beides. Am Ende sichtbar: Tests in `engine/room` und `native/k3c-game/local` grün, Antwort mit Fundstellen in B-814 › Notizen.

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, Befunde behoben oder weitergegeben (siehe Review-Session). Version: keine vorgeschlagen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Belegte Antwort zur Position nach dem Laden in B-814 › Notizen (`B-814/AC-01`). |
| AC-02 | – | Gewollt: dokumentiert und Ticket verworfen; Fehler: behoben mit Test im Raum-Paket (`B-814 … |
| AC-03 | – | Panic im Takt über den exportierten Weg schließt den Raum, danach keine Speicherung (`B-80 … |
| AC-04 | – | Abgelehntes `Learn` nennt den Grund (`B-803/AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-814 | verworfen | AC-01 –, AC-02 – | Geklärt ist, warum Monarchen nach dem Laden an der Burg statt an ihrer gespeiche … |
| B-803 | erledigt | AC-03 –, AC-04 – | Ein Raum bietet seine Absturzbehandlung und den Grund abgelehnter Skill-Befehle … |
