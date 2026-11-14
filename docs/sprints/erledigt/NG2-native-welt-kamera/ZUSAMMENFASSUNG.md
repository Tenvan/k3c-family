# NG2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Welt, Kamera, Schichten, Fluss und Licht
- **Projekt:** NAT

## Ziel

Jede Zelle zeigt Hintergrund, Boden, Fluss und Licht und führt die Kamera wie das heutige Spiel. Am Ende sichtbar: Monarch reitet, `fern` steht, `mitte` und `nah` ziehen vorbei, der Fluss fließt im unteren Drittel, Nacht wird dunkel.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions. Behobene Befunde: siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); am echten Gerät ungeprüft (Fenster, Gamepads). Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Kamera und Culling je Zelle (`B-795/AC-01`). |
| AC-02 | – | Schichten und Fluss im unteren Drittel (`B-795/AC-02`). |
| AC-03 | – | Licht und Stufenübergang (`B-795/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-795 | erledigt | AC-01 –, AC-02 –, AC-03 – | Das native Spiel zeigt Welt, Kamera, Schichten, Fluss und Licht je Split-Screen- … |
