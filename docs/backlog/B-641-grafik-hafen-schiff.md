# B-641 · Hafen und Schiff haben eigene Grafiken je Bauabschnitt

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS10
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Hafen-Chunk und Schiff entstehen in BS5 (B-637) als Mechanik, der Client zeichnet sie in BS6 mit Platzhaltern. Eigene Grafiken fehlen. Gebäude-Grafiken entstehen nach `docs/standards/gebaeude-grafik.md`: Vorlage und Bausteine als JSON in `public/grafik/gebaeude/`, `task gen:gebaeude` (`tools/buildings`) rendert 16/32/64/128 px, Vorschau im Asset-Studio, Einbau über Asset-Slots in `data/assets.json`.

## Ziel

Hafen und Schiff sehen am TV fertig aus: ein Hafen mit Steg an der Wasserkante und das Schiff in jedem Bauabschnitt (Kiel, Rumpf, Segel, fertig), im Stil der übrigen Gebäude.

## Beteiligte und Zielgruppen

Spieler am TV, Claude (Grafik-Daten, Einbau), 🧑 nimmt im Studio und am Gerät ab.

## Anforderungen

- Hafen: Steg/Kai aus Holz und Stein an einer Wasserkante, Rand des Hafen-Chunks; passt zum Wald-Biom.
- Schiff: je Bauabschnitt aus B-637 eine Grafik (Vorschlag: Kiel auf Helling, Rumpf, Rumpf mit Mast und Segel), dazu „fertig, am Steg“.
- Vorlage (`motiv`, `teile`, `materialien`, `stimmung`, `fein`) vor den Bausteinen; nur DB32, Raster und Maße nach Standard § 1–8.
- Asset-Slots in `data/assets.json` (z. B. `harbor`, `ship:1` … `ship:4`); der Client nutzt sie statt der Platzhalter aus BS6.
- Deterministisch: gleiche Daten → byte-gleiche PNGs.

## Nicht-Ziele

Animation (Wellen, Segel im Wind), Bootsfahrt, Sound.

## Regeln und Einschränkungen

Standard `docs/standards/gebaeude-grafik.md`; Ablauf § 9. Grafik-Daten in DEV, Einbau im Client in CLI. Abnahme durch 🧑.

## Beispiele

Studio › Grafik › Szenen › Gebäude → Streifen zeigt Hafen und vier Schiff-Stufen; im Spiel wechselt das Schiff nach jedem bezahlten Abschnitt die Grafik.

## Ausnahme- und Fehlerfälle

Asset fehlt oder Generator bricht ab → Platzhalter aus BS6 bleibt, Fehler im Commit behoben, nicht überspielt.

## Akzeptanzkriterien

- **AC-01** `public/grafik/gebaeude/` enthält Daten-JSON mit Vorlage für Hafen und Schiff je Abschnitt; `task gen:gebaeude` rendert alle Größen, Tests in `tools/buildings` grün.
- **AC-02** Asset-Slots für Hafen und Schiff-Abschnitte in `data/assets.json`; der Client zeichnet sie (Test der Zuordnung).
- **AC-03** Abnahme durch 🧑 im Studio und am Gerät.

## Offene Fragen

- Wie viele Schiff-Grafiken (Abschnitte + fertig)? Folgt B-637, Default 3 + 1.

## Notizen

Wunsch 🧑 2026-10-26: „Die Grafiken für den Hafen und das Boot in einem letzten Sprint auch noch einplanen und erstellen.“
