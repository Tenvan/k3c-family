# GEB6 · DEV · Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-435, B-438
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bäume, Fels, Erz, Busch, Adern, Truhe, Portal, Ausgang und Rekrutierungslager stammen aus gemischten Packs. Drei Bäume sind als Beispiel in GEB1.

## Ziel

Alle Strukturen und Objekte haben einheitliche Grafik mit Varianten je Biom. Am Ende sichtbar: Vorschau je Biom, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-435 › Anforderungen und `docs/standards/gebaeude-grafik.md` (Raster § 1, Palette § 2, Vorlage § 5, Dichte § 7); B-438 (Bäume im Spiel). Sprint-eigen (Maße sind Annahmen, siehe Offene Fragen):
- **Bäume:** weitere Varianten je Art (Tanne, Pappel, Birke; je 2 zusätzliche Formen), Kronen-Paletten je Biom über Farbtausch statt Neuzeichnen; `node:tree` wählt Art und Form deterministisch aus der Position.
- **Fels** (`felsen`), **Erzbrocken** (`erzbrocken`), **Busch** (`busch`): je 1 × 1 Kachel.
- **Adern** `vein:stone`, `vein:copper`, `vein:iron`, `vein:crystal`: eine JSON `ader` mit vier Stufen als Farbtausch (§ 6), 2 × 2 Kacheln.
- **Truhe** (`truhe`, `pickup:chest`) 1 × 1; **Portal** (`portal`) 2 × 3 und **Ausgang** (`ausgang`) 2 × 3; **Rekrutierungslager** (`rekrutierungslager`, `camp:recruit`) 3 × 2 mit Zelt und Feuerstelle.
- **Biome:** Farbtausch je Biom (Wald, Höhle, Mine) über `stufen`-ähnliche Paletten; kein zweites Zeichnen.
- **Spiel (CLI):** Slots in `data/assets.json` und `src/scenes/worldSprites.ts` auf die neuen PNGs in Maßstab 1; alte Pack-Grafik nach der Aufräum-Regel des Projekts GEB löschen (B-438).

## Nicht-Ziele

Animationen (Portal-Flackern, Lagerfeuer) und Figuren (GR7, GR8); Hintergründe `bg:*` (GEB7); Münzen, Icons, Items, HUD; Änderungen an `engine/`, Protokoll oder Spielständen; neue Biome oder Level-Regeln; zusätzliche Adern-Stufen jenseits der vier Slots.

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB6.1–GEB6.4: `public/grafik/gebaeude/`, `tools/gebaeude/` nur bei Bedarf, `src/tools/assetvorschau*.ts`, Regeldatei) → **CLI** (GEB6.5: `src/scenes/`, `data/assets.json`, Löschen alter Grafik samt Fixtures und Mocks). `engine/` bleibt unberührt.
- Determinismus: gleiche JSON → byte-gleiche PNGs; vorhandene PNGs bleiben unverändert (md5 vor und nach `task gebaeude`). Keine Würfel im Client: Variantenwahl aus der Position.
- Materialien nur aus `gebaeude-grafik.md` § 3; fehlt eine (Erde, Fels, Zelttuch), zuerst dort mit Zustimmung von 🧑 ergänzen.
- GEB5 läuft parallel in einem anderen Checkout (Hub-Gebäude); GEB6 fasst dessen Dateien und `data/assets.json`-Slots (`storage`, `farm`, `barracks`, `stairs*`, `plantation`) nicht an. Merge-Reihenfolge: GEB5 zuerst.
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Bäume (weitere Varianten), Fels, Erzbrocken, Busch, vier Adern, Truhe, Portal, Ausgang und Rekrutierungslager liegen als Daten-JSON und PNG vor.
- **AC-02** Die Slots `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` sind zugeordnet.
- **AC-03** 🧑 hat die Objekte abgenommen.

## Offene Fragen

Keine blockiert den Start der Planung; jede Annahme gilt mit der Freigabe als bestätigt, eine Antwort von 🧑 ersetzt sie vorher.

1. Annahme: Maße Fels, Erzbrocken, Busch, Truhe je 1 × 1; Adern 2 × 2; Portal und Ausgang 2 × 3; Rekrutierungslager 3 × 2 Kacheln.
2. Annahme: Je Art zwei zusätzliche Baum-Formen (Gesamt 3 je Art); Biom-Looks als Farbtausch (Wald, Höhle, Mine).
3. Annahme: Die vier Adern sind **eine** JSON mit vier Stufen (Stein, Kupfer, Eisen, Kristall) nach § 6.
4. Offen für 🧑: Erde, Fels und Zelttuch haben keine Zeile in § 3; Vorschlag: Fels = Zeile Stein (`dim-gray`), Zelt = „Stoff hell“, Erde = „Pfosten, Balken“.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

- GEB6.1 (DEV) Bäume: je 2 weitere Formen für Tanne, Pappel, Birke, Busch 1 × 1; Biom-Farbtausch (AC-01).
- GEB6.2 (DEV) Fels und Erzbrocken, je 1 × 1 (AC-01).
- GEB6.3 (DEV) Adern: eine JSON `ader` mit vier Stufen (Stein, Kupfer, Eisen, Kristall), 2 × 2 (AC-01).
- GEB6.4 (DEV) Truhe 1 × 1, Portal und Ausgang je 2 × 3, Rekrutierungslager 3 × 2 (AC-01).
- GEB6.5 (CLI) Zuordnung der Slots `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` im Spiel in Maßstab 1, alte Grafik löschen, Bäume nach B-438 (AC-02).
- GEB6.6 Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
