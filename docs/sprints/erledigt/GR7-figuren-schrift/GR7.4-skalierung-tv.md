# GR7.4 · Figuren am TV beobachten, ganzzahlig skalieren oder verwerfen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr7/4-skalierung-tv
- **Abhängig von:** –
- **Tickets:** B-251
- **Kriterien:** AC-02

## Ziel

🧑 beobachtet am TV, ob die Figuren beim Laufen flimmern. Stört es, laufen die Figuren mit ganzzahliger Skalierung (×2 oder ×3, gleiche Figurgröße erhalten); stört es nicht, ist B-251 mit Begründung verworfen.

## Kontext

- **Agent: Mensch**, weil B-251 verlangt, erst am TV zu beobachten, ob das Flimmern sichtbar ist („nur dann umstellen“); Beobachtung und Entscheid gehören 🧑 (`docs/arbeitsweise.md` › SDD, Abnahme gehört 🧑). Hardware entkoppelt: Die App wartet nicht auf diese Session, der Sprint bleibt bis zu ihrem Ergebnis aktiv (Fahrplan „Offen am Gerät“); bis dahin gilt die Skalierung aus `data/sprites.json` als **angenommen** (Workshop GR1.1: „passt“).
- **Ausgangslage:** 19 von 21 Figuren-Packs laufen nicht ganzzahlig (×1,25 bis ×3,25), Liste in `docs/assets/zuordnung.md` › Figuren-Packs (`scale` je Sheet in `data/sprites.json`). Q13 verlangt ×2 bis ×3. Gleiche Figurgröße ist abgestimmt; Umstellen heißt Frames zuschneiden oder auf ×2/×3 runden und Figurgröße im Spiel neu abstimmen.
- **Umstellung nach der Beobachtung:** Ein Agent darf sie vorbereiten, sobald 🧑 entschieden hat: Werte nur in `data/sprites.json` (`scale`, ggf. `height`, Frame-Zuschnitt unter `public/sprites/`), nicht im Code; Sattelpunkte der Reittiere (`mounts.*.saddle`) und `riderWaist` hängen am Faktor und sind nachzuziehen (GR7.2 prüfen).

## Erlaubte Dateien

- `data/sprites.json` (`scale`, `height`, Reittier-Sattelpunkte), `public/sprites/` (zugeschnittene Frames), `docs/assets/zuordnung.md`
- `docs/sprints/geplant/GR7-figuren-schrift/`, `docs/sprints/aktiv/GR7-figuren-schrift/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Neue Figuren (B-193), Umgebungsgrafik, Ziel-Palette und Pixeldichte-Pipeline (B-331), neue Animationen.

## Schritte

1. 🧑 spielt am TV (Xbox, Edge) mit Figuren in Bewegung und beurteilt das Flimmern beim Laufen (z. B. Monarch, Truppen, Gegner mit krummem Faktor wie `wolf` ×3,25, `medieval-warrior` ×1,25).
2. Entscheid von 🧑 ins Ergebnis schreiben: stört (Umstellen) oder stört nicht (B-251 verwerfen, Begründung).
3. Bei „stört“: je Pack `scale` auf ×2 oder ×3 runden und Frames zuschneiden, Reittier-Sattelpunkte nachziehen, `docs/assets/zuordnung.md` aktualisieren; `task check` grün; 🧑 prüft am TV erneut.
4. Ergebnis mit Nachweis AC-02, `Status: fertig`.

## Fertig, wenn

- [x] AC-02: Entschieden von 🧑 (2026-10-24): Die bisherigen Umsetzungen sind ausreichend, keine Umstellung; B-251 verworfen mit Begründung.

## Prüfen

```bash
task check
```

Beobachtung am TV nur durch 🧑.

## Ergebnis

**Entscheid 🧑 (Chat, 2026-10-24):** Alle Figuren und Grafiken gelten als abgenommen; die bisherigen Umsetzungen sind ausreichend. Grafik wird künftig mit der Integration des AI-Chats anders und besser gelöst. Daher keine Umstellung auf ganzzahlige Skalierung (B-251 verworfen, B-547 B nicht umgesetzt), keine weitere Beobachtung am TV. Nicht einzeln am Gerät geprüft.
