# PT1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** gen:paper schneller: Cache je Asset, paralleles Rastern, Fortschritt belegt
- **Projekt:** PAP

## Ziel

Ein `gen:paper` nach kleiner Änderung dauert Sekunden, ein voller Lauf deutlich unter einer Minute; die Fortschritts-Tickets sind belegt und geschlossen. Am Ende sichtbar: zweiter `task gen:paper` direkt nach dem ersten meldet 0 gerenderte Assets; voller Lauf mit 8 Kernen höchstens ein Drittel der heutigen Zeit; Konsole zeigt ⏳-, 🔨- und ↳-Zeilen.

## Ergebnis

2026-11-13, PT1.3 (Claude, autonom): AC-01–AC-03 siehe PT1.1, AC-04 siehe PT1.2; `task check`, `task check:go` grün. Behoben: `cli/pool.ts` im Cache-Hash `codeStand` ergänzt (Review k3c-reviewer, niedrig); sonst keine Befunde. Neue Tickets: keine außer dem Abnahme-Ticket B-834 (Gen nach B-742). Version: keine vorgeschlagen (Werkzeug-Sprint, keine Spieländerung).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Fortschritt je Schritt in Konsole und `.work/grafik/gen.log`, Test prüft Format (`B-740/AC … |
| AC-02 | geprüft | Fehlende Grafik wird kopiert statt neu gerendert (`B-748/AC-01` bis `B-748/AC-03`). |
| AC-03 | geprüft | Startzeile je Asset und Zeilen je Ausgabe großer Assets (`B-749/AC-01`, `B-750/AC-01`). |
| AC-04 | geprüft | Zweiter Lauf rendert 0 Assets; voller Lauf mit 8 Kernen ≤ ein Drittel der Zeit, Golden unv … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-740 | erledigt | AC-01 geprüft | gen:paper zeigt Fortschritt je Schritt in Konsole und Log |
| B-748 | erledigt | AC-02 geprüft | gen:paper rendert, wenn Grafiken fehlen, obwohl Specs unverändert sind |
| B-749 | erledigt | AC-03 geprüft | gen:paper meldet den Start jedes Assets, lange Assets wirken sonst wie ein Still … |
| B-750 | erledigt | AC-03 geprüft | gen:paper meldet die Ausgaben innerhalb großer Assets |
| B-751 | erledigt | AC-04 geprüft | gen:paper schneller: Cache je Asset und parallel rastern |
