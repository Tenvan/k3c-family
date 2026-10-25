# B-618 · Die Hintergründe der fünf Stufen sind detailreich (vierfache Auflösung) und wechseln je Seed zwischen fünf Varianten

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB18
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-24, Revision 1 (Auftrag Ralf im Chat: autonom im Plan eintragen)

## Ausgangslage

Ralf (2026-10-24): Die Hintergründe der fünf Stufen (Wald, Höhle, Mine, Eisenstollen, Kristall) sind zu grob und zu eintönig. Heute erzeugt `tools/buildings` je Biom drei Schichten (fern, mitte, nah) aus JSON-Vorlagen mit der DB32-Palette in den Dichten 16 und 32 px (`public/grafik/gebaeude/hg-<teil>-<schicht>-32.png`, GEB7); der Client zeigt je Biom genau eine Fassung (`src/scenes/render/worldSprites.ts`, Slots `bg:<biom>:<n>`).

## Ziel

Jede Stufe hat deutlich detailreichere Hintergründe in mindestens vierfacher Auflösung und mindestens fünf Varianten, die je nach Seed abwechselnd erscheinen.

## Beteiligte und Zielgruppen

Spielende (Abwechslung beim Durchlaufen); 🧑 beurteilt das Aussehen; Agenten DEV (Generator, Vorlagen), SIM (Auswahl nach Seed), SRV (Protokoll), CLI (Anzeige).

## Anforderungen

- **Auflösung:** Hintergrund-Schichten mindestens in vierfacher Auflösung der heutigen Fassung (Dichte 128 statt 32 px je Unit), mit deutlich mehr Details (Silhouetten, Strukturen, Lichtakzente je Biom), weiter mit der Palette und den Regeln aus GEB7 (`docs/rules`); Erzeugung im Generator `tools/buildings` oder als Grafik über das Asset-Studio.
- **Varianten:** Je Stufe (Biom) mindestens fünf Varianten, je Variante alle drei Schichten; Ablage und Slots je Variante (z. B. `bg:<biom>:<schicht>:<variante>`).
- **Auswahl nach Seed:** Die Variante je Abschnitt der Welt wählt der Level-Generator in Go deterministisch über `engine/rng` (gleicher Seed → gleiche Folge), benachbarte Abschnitte wechseln ab (keine gleiche Variante direkt nebeneinander); der Client zeichnet nur, was der Server meldet (additives Feld im Level).
- **Laden:** Der Client lädt nur die Varianten der Stufen, die er zeigt; Speicher und Ladezeit auf der Xbox bleiben im Budget (Messung vorher/nachher).
- Die Szene „Umgebung“ im Studio zeigt alle Varianten je Biom.

## Nicht-Ziele

Neue Biome; Vordergrund-Objekte (Bäume, Felsen: GEB15, GEB16); Parallax-Faktoren ändern; Balancing.

## Regeln und Einschränkungen

Determinismus: Auswahl nur in Go mit `engine/rng`, kein Zufall im Client (CLAUDE.md); Protokolländerung additiv, Version bleibt; Golden nur mit Begründung; keine Rücksicht auf alte Spielstände; 2 Spieler (je Zelle dieselbe Welt); Lizenz: selbst erzeugt oder CC0; Dateien ≤ 400 Zeilen. **Aufräum-Regel des Projekts GEB:** Die alten Schichten (`hg-*-16.png`, `hg-*-32.png` ohne Variante) samt Register-Wahl, Index, Doku, Fixtures und Mocks werden im selben Sprint gelöscht, sobald nichts sie mehr nutzt (vorher per Suche belegen).

## Beispiele

Seed `abc`, Wald: Abschnitte zeigen Variante 3, 1, 4, 2, 5, 3 …; gleicher Seed auf PC und Xbox → gleiche Folge. Seed `xyz` → andere Folge.

## Ausnahme- und Fehlerfälle

Variante fehlt (Datei nicht geladen) → Variante 1 als Rückfall, Log-Meldung 📄. Biom mit weniger als fünf Varianten → Generator- bzw. Katalog-Test schlägt fehl.

## Akzeptanzkriterien

- **AC-01** Der Generator erzeugt Hintergrund-Schichten in mindestens vierfacher Auflösung (Dichte 128) und mit Varianten (Go-Test).
- **AC-02** Jede der fünf Stufen hat mindestens fünf Varianten mit je drei Schichten in vierfacher Auflösung, im Katalog und in der Studio-Szene „Umgebung“ sichtbar (Test des Katalogs).
- **AC-03** Der Level-Generator wählt die Variante je Abschnitt deterministisch nach Seed, benachbarte Abschnitte verschieden (Go-Test, Golden mit Begründung).
- **AC-04** Protokoll und Client: Feld im Level beschrieben (`docs/protocol.md`), der Client zeichnet die gemeldete Variante, nur benötigte Varianten werden geladen (Vitest).
- **AC-05** 🧑 sieht am Gerät detailreichere, abwechselnde Hintergründe; Speicher und Ladezeit auf der Xbox im Budget.
- **AC-06** Die alten Hintergrund-Schichten ohne Variante sind gelöscht, nichts verweist mehr auf sie (Suche, Tests grün).

## Offene Fragen

keine blockierende. Defaults (Claude, autonom): Dichte 128 als vierfache Auflösung; Auswahl im Go-Level-Generator statt im Client (Projektregel: kein Zufall im Client); Abschnittslänge = Kachelbreite der Schicht „mitte“. Ralf bestätigt oder ändert bei der Abnahme.

## Notizen

Aus dem Chat mit Ralf (2026-10-24, während WZ29). Zusammenhang: GEB7 (Hintergründe je Biom), B-526 (Hintergrund-Höhe), B-334 (Performance-Modus). B-453 betrifft die Pack-Hintergründe vor GEB7, nicht diese Schichten. Sprint GEB18.
