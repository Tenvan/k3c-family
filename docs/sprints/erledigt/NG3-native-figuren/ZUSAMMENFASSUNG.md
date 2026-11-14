# NG3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Figuren als Rig, Bein-Bänder und Gegnerzahl
- **Projekt:** NAT

## Ziel

Alle Figuren laufen im nativen Spiel aus dem Spielzustand, mit Teile-Baum als Rig, `requisite` als Sheet; die Grenze Rig gegen Sheet am Entwicklungs-PC ist gemessen. Am Ende sichtbar: zwei Monarchen auf Reittieren, Bürger, Truppen und eine Gegner-Welle als Rig, Messbericht unter `reports/`.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions. Behobene Befunde: siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); am echten Gerät ungeprüft (Fenster, Gamepads). Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand). B-780 bleibt offen (Teil Mini-PC in NG9.5).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Bein-Bänder falten nie, `knownFolds` leer (`B-791/AC-01`), Spec-Auffälligkeiten geklärt (` … |
| AC-02 | – | Monarch und Reittier aus dem Zustand (`B-796/AC-01`). |
| AC-03 | – | Jeder Figurentyp hat Rig oder Sheet (`B-796/AC-02`). |
| AC-04 | – | Messbericht Rig gegen Sheet mit 2, 50, 200 Gegnern am Entwicklungs-PC (`B-796/AC-03`, `B-7 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-796 | erledigt | AC-02 –, AC-03 –, AC-04 – | Das native Spiel zeichnet Monarchen, Reittiere, Bürger, Truppen, Gegner und Boss … |
| B-791 | erledigt | AC-01 – | Bein-Bänder von Elefant, Wolf und Alphawolf klappen im Rig nicht mehr um |
| B-780 | eingeplant | AC-04 – | Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Mode … |
