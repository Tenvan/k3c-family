# NV1 · CLI · Bevy-Prototyp: Papier-Sheets, zwei Gamepads, Messung

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-776, B-777
- **Start-Commit:** 98dfed8e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-10 🧑 Chat-Auftrag (Bevy-Prototyp)

## Ausgangslage

Projekt NAT will K3C nativ in Rust + Bevy auf den Mini-PC bringen (B-775). Ob Bevy die Papier-Sheets flüssig zeichnet und zwei Gamepads trennt, ist nicht belegt.

## Ziel

Ein Bevy-Prototyp zeigt ein Level mit echten Papier-Sheets, zwei Gamepads steuern je einen Monarchen, FPS und Grafikspeicher sind gemessen. Am Ende sichtbar: Fenster mit Split-Screen und ein JSON-Bericht unter `reports/`.

## Beteiligte und Zielgruppen

🧑 Ralf (Messung am Mini-PC, Entscheidung), Claude (Bau, Referenzmessung).

## Anforderungen

B-776 › Anforderungen.

## Nicht-Ziele

B-776 › Nicht-Ziele; keine Simulation, kein Netz, kein HUD.

## Regeln und Einschränkungen

B-776 › Regeln und Einschränkungen. Domain CLI; neue Dateien nur unter `native/`, dazu die Einbindung `native: native/Taskfile.yml` in `Taskfile.yml`.

## Beispiele

B-776 › Beispiele.

## Ausnahme- und Fehlerfälle

B-776 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Bau mit gepinntem Bevy (`B-776/AC-01`).
- **AC-02** Level aus Papier-Sheets (`B-776/AC-02`).
- **AC-03** Zwei Gamepads, Split-Screen (`B-776/AC-03`).
- **AC-04** Messbericht (`B-776/AC-04`).
- **AC-05** Referenzwerte Entwicklungs-PC (`B-776/AC-05`).
- **AC-06** Messwerte Mini-PC (`B-776/AC-06`, 🧑).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NV1.1 | `NV1.1-bevy-prototyp.md` | Umsetzung | autonom | fertig |
| NV1.2 | `NV1.2-ebitengine-prototyp.md` | Umsetzung | autonom | fertig |
| NV1.3 | `NV1.3-messung-mini-pc.md` | Review | Mensch | verworfen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-11, geschlossen auf Anweisung 🧑 (Chat). Kriterien laut Ergebnis NV1.1 und NV1.2; Entscheidung 008 (Go + Ebitengine). Messung am Mini-PC (NV1.3) verworfen, später über B-786. Version: keine vorgeschlagen (Prototyp).
