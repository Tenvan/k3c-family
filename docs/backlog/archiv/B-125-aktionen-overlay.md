# B-125 · Gültige Aktionen erscheinen überall in der Welt als Overlay am Ort

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S3
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint S3

## Ausgangslage

Preise und Bauplätze zeigen ihre Münz-Slots; sonst gibt es nur Texthinweise am Bildschirmrand (`src/scenes/HudScene.ts`, `worldRenderer.ts`).

## Ziel

Neben dem Spieler und am Ziel erscheint immer die gerade gültige Aktion mit der passenden Taste, passend zum zuletzt benutzten Gerät (Controller, Tastatur, Touch), wie die Preise an den Gebäuden. Nutzen: Steuerung ohne Handbuch (`docs/rules/monarch.md` § 4).

## Beteiligte und Zielgruppen

Spieler am TV, Handy und PC; 🧑 testet am Gerät.

## Anforderungen

- Overlay für alle gültigen Aktionen: Bauen, Zahlen, Ausbauen, Eingang/Treppe, Truhe, Wiederbeleben, Schlag, Skills, Ausbilden, Tauschen.
- Daten kommen aus dem Snapshot (B-123); der Client rechnet nichts.
- Tastensymbol passend zum zuletzt benutzten Gerät; mit 2+ lokalen Spielern je Zelle; oben 70 px frei.

## Nicht-Ziele

Simulation, Protokoll, Skill-Menü (B-124).

## Regeln und Einschränkungen

`CLAUDE.md` (Client zeichnet nur, Home-Button oben, B-Taste frei). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Spieler steht an der Burg mit Gold: „A halten: Hub-Stufe 2 (50 Gold, 100 Stein)“; neben einem Gefallenen: „A halten: Wiederbeleben“.

## Ausnahme- und Fehlerfälle

Mehrere gültige Aktionen am selben Ort → die wichtigste zuerst, die anderen kleiner.

## Akzeptanzkriterien

- **AC-01** Reine Funktion für Aktion → Text und Symbol je Gerät ist getestet.
- **AC-02** Das Overlay zeigt die Aktionen am Ort für Bauen, Zahlen, Wiederbeleben und Schlag.
- **AC-03** 🧑 hat das Overlay am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R3.2 (Zusatz von 🧑). Abhängig von B-123.

2026-10-07: Ohne Abnahme am Gerät abgeschlossen (Entscheidung 🧑); das Abnahme-Kriterium geht in die Gesamtprüfung B-337/AC-05 über.
