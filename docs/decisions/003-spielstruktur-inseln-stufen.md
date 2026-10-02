# 003 · Spielstruktur: Spielstand → Inseln → Stufen, Stufen pro Spieler frei begehbar

Stand: 2026-10-02 · Status: **beschlossen (Regelwerk), Umsetzung offen** · Backlog: B-004, B-025 (Tickets in R1.4)

## Kontext

- Bisher: Eine Kampagne hat eine Stufe nach der anderen (Wald → Höhle → Mine). Jede Stufe ist eine eigene Welt, alle Spieler wechseln
  **gemeinsam** (`engine/sim/campaign.go`, `travel.go`), Stufen ohne Spieler stehen still. `game-design.md` und die bisherige Planung
  folgten dem Vorbild Kingdom Two Crowns (Inseln als Hintereinander).
- 🧑 hat im Workshop R1.3 (2026-10-02) klargestellt, dass das **anders geplant** ist: Stufen sind keine getrennten Inseln, sondern pro Spieler frei begehbar.

## Optionen

| Option | Bewertung |
|---|---|
| **A · Gemeinsame Reise, eine Stufe = eine Welt** (Ist) | einfach, billig; ein Spieler kann nie woanders sein als die anderen |
| **B · 1 Spielstand → n Inseln → n Stufen je Insel, Stufen pro Spieler begehbar** | trägt Koop mit freier Wahl des Ortes, mehr Spielinhalt; Umbau von Simulation, Raum, Protokoll und Client |
| **C · Alles in eine Stufe** | einfachste Struktur, verliert die Tiefen als Idee |

## Entscheidung

Option **B**:

1. Ein Raum hat einen Spielstand mit **n Inseln**, jede Insel hat **n Stufen** (ein Level). Die erste Ausbaustufe: 1 Insel mit 5 Stufen (Wald, Höhle, Mine zuerst; Eisenstollen und Kristallhöhle folgen).
2. Jeder Spieler wechselt **einzeln** die Stufe (Tiefen-Eingang, Treppe); jeder Spieler kann in einer anderen Stufe sein.
3. **Alle Stufen einer Insel laufen weiter** (gemeinsame Zeit, eigene Wellen je Stufe), auch ohne Spieler.
4. Der **Endboss** der tiefsten Stufe macht den Weg zur nächsten Insel frei; der **Inselwechsel** ist gemeinsam.
5. Das **Baumaterial gehört der Insel** (alle Stufen teilen einen Vorrat); die Inseln folgen klassisch als 1 bis n, später optional mehrere Inseln je Ebene (B-108).
6. Wellenstärke skaliert mit der Spieleranzahl der Insel; Gegnerskalierung je Insel mit eigener Tabelle.
7. Raum-Optionen (Schwierigkeitsgrad, Ziel, Niederlage-Modus) siehe `docs/rules/stufen.md` und `wirtschaft.md`.

## Folgen

- **SIM** muss mehrere Stufen einer Insel gleichzeitig simulieren (heute `Campaign.worlds` je Tiefe, nur die aktuelle Stufe tickt) und Spieler an Stufen binden.
- **Raum/Protokoll/Client:** Snapshots je Spieler mit seiner Stufe, Kamera je Spieler in seiner Stufe (auch geteilter Bildschirm in verschiedenen Stufen); Protokoll-Änderung braucht eine eigene Session (`docs/arbeitsweise.md` › Protokoll).
- **Last:** mehr aktive Welten je Raum; relevant für den Raspberry Pi 3 (SP11) und das Leistungsziel aus B-042. Der Balancing-Tester (B-099) misst es.
- **Spielstand:** Insel, Stufe je Spieler, Raum-Optionen; `engine/sim/save.go` und `SAVE_VERSION` ändern sich.
- Die Umsetzung kommt als Feature-Kette REG → SIM → CLI nach R1 (Tickets in R1.4); bis dahin gilt das heutige Verhalten.
