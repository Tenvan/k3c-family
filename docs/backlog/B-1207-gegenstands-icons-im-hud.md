# B-1207 · Gegenstands-Icons erscheinen am Regal und im Holen-Hinweis

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** EQ1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf-Entscheidung 2026-11-21 (Dialog), Claude trägt ein
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Icons `icons/icon-guertel|bogen|schwert|ruestung` (FG1.3, B-1201) sind gezeichnet und in `data/paper.json` zugeordnet, aber nirgends im HUD zu sehen. Das Regal zeigt nur die Hammer- und Bogenzeichnung der Plätze.

## Ziel

Das Regal am Gebäude zeigt die Stückzahl als Icons, der Holen-Hinweis zeigt das Icon des Gegenstands.

## Beteiligte und Zielgruppen

Ralf (🧑) nimmt am PC mit Pad ab.

## Anforderungen

- **AC-01** Regal von Werkstatt, Bogner, Waffenschmied und Rüstungsschmied zeigt je Stück ein Icon.
- **AC-02** Der Holen-Hinweis zeigt das Icon des Gegenstands neben dem Text (de/en).
- **AC-03** Frame-Budget: Icons über den vorhandenen Icon-Cache, keine neuen Allokationen je Bild.

## Nicht-Ziele

Neue Grafik, Sim-Änderungen.

## Regeln und Einschränkungen

Papierschnitt-Icons über `refresh_icon_cache` und `draw_icon`; Importregel hud importiert nicht menu; 60 Hz, zwei Ansichten.

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Bogner mit zwei Bogen im Regal: zwei Bogen-Icons am Regal.

## Ausnahme- und Fehlerfälle

Leeres Regal: keine Icons.

## Akzeptanzkriterien

- **AC-01** Test: Regal-Plan enthält je Stück ein Icon.
- **AC-02** Test: Holen-Hinweis nennt das Icon.
- **AC-03** `alloc_je_bild` nicht schlechter als vorher.

## Offene Fragen

keine

## Notizen

Quelle: B-1203, Entscheidung Ralf: Regal und Hinweis.
