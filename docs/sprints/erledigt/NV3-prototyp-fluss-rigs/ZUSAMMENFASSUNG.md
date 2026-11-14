# NV3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Prototyp-Ausbau: Fluss, fester Hintergrund, alle Figuren als Rig
- **Projekt:** NAT

## Ziel

Der Prototyp zeigt das gemeinte Bild des Spiels: Fluss in der unteren Bildhälfte, fester Fern-Hintergrund, Figuren als Rig. Am Ende sichtbar: `task native:ebiten` zeigt im Split-Screen den fließenden Fluss, die stehende Schicht `fern` und Rig-Figuren; ein Messbericht liegt unter `reports/`.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): AC-01 bis AC-04 laut Ergebnis NV3.1 bis NV3.4. AC-05: Sichtprüfung am PC durch 🧑 bestanden (NV3.5, 2026-11-11). Behoben: Mipmaps der Rig-Teile aus. Neue Tickets: B-791 (Bein-Bänder). Kein Abnahme-Ticket (nur `native/`, keine Sim-, Balance-, Gen-Läufe betroffen). Version: keine vorgeschlagen (Prototyp).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Fluss in der unteren Hälfte jeder Sicht, drei Bahnen als fließende Kacheln (`B-789/AC-01`) … |
| AC-02 | – | Schicht `fern` steht, `mitte` und `nah` scrollen (`B-789/AC-02`). |
| AC-03 | – | Figuren mit Teile-Baum als Rig, Posen gegen Sollwerte geprüft (`B-789/AC-03`). |
| AC-04 | geprüft | Messbericht Entwicklungs-PC, Rig gegen Sheet (`B-789/AC-04`). |
| AC-05 | geprüft | Sichtprüfung und Entscheidung 🧑 (`B-789/AC-05`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-789 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft, AC-05 geprüft | Der Ebitengine-Prototyp zeigt den Fluss in der unteren Bildhälfte, einen festen … |
