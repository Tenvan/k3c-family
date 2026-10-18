# WZ17 · REG, SIM, SRV, DEV · Chunk-Editor komfortabel: Inselgrößen, getrennter Zoom, Seed-Chunk umwandeln

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, SRV, DEV
- **Reife:** bereit
- **Tickets:** B-485
- **Start-Commit:** a6d676b4
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 2: Größen-Tabelle im Workshop WZ17.1)

## Ausgangslage

`B-485 › Ausgangslage`: Der Chunk-Editor im Welt-Betrachter (WZ8) ist abgenommen bis auf B-486; seine Bedienung ist aber unkomfortabel (kleine Dropdowns, Schnellfilter verbirgt statt abzublenden, keine Icons). Die Länge einer Stufe kommt nur aus `lengthUnits` des Bioms, eine Chunk-Anzahl oder Inselgrößen gibt es nicht. Beide Vorschauen zoomen gemeinsam, und ein Seed-Chunk lässt sich nicht in feste Objekte überführen.

## Ziel

Der Editor bietet durchsuchbare Auswahllisten mit Emoji-Icons, eine Chunk-Anzahl je Stufe über die Inselgrößen (sehr klein 16 · klein 20 · mittel 24 · groß 32 · sehr groß 40 · mega groß 48), die die Level-Länge im Spiel ändert, einen getrennten Zoom der Vorschauen (Alt + Mausrad, gleiche Weltposition) und die Umwandlung eines Seed-Chunks in feste Objekte (`B-485 › Ziel`).

**Am Ende sichtbar:** im Asset-Studio eine Stufe auf „klein“ gestellt (20 Chunks) in Chunk- und Live-Vorschau als Entwurf, getrennter Zoom bei synchronem Bildlauf und ein umgewandelter Seed-Chunk mit unverändertem Bild; in der CI unveränderte Golden-Daten plus ein neuer Golden-Fall mit Chunk-Anzahl; von 🧑 abgenommen. Der Sprint liefert keine echte Planung und keine Golden-Änderung außer dem neuen Fall aus.

## Beteiligte und Zielgruppen

`B-485 › Beteiligte und Zielgruppen`: 🧑 im Workshop WZ17.1, in der Abnahme WZ17.7 und für die Spec-Freigabe; Agenten in WZ17.2–WZ17.5; ein getrennter Reviewer in WZ17.6.

## Anforderungen

`B-485 › Anforderungen` (Auswahllisten und Icons, Chunk-Anzahl je Stufe, Zoom, Seed-Chunk umwandeln).

Nur für den Sprint:
- **Vorbedingung:** B-486 (Stand-Prüfung meldet nichts) ist behoben und von 🧑 mit Punkt 5 der WZ8.7-Liste bestätigt, bevor WZ17.5 das Speichern erweitert. WZ17.1 bis WZ17.4 hängen nicht davon ab.
- Reihenfolge REG → SIM → SRV → DEV. Die SIM-Session beginnt erst, wenn Workshop-Beschluss, Regeltext und Glossar („Inselgröße“) committet sind (AC-01).
- Die Vorschau-Funktion (`room.PreviewOptions`) bekommt die Option „Chunk-Anzahl“ (und die Funktion für den Seed-Inhalt) ohne globalen Zustand, wie WZ8.4; keine zweite Vorschau-Funktion.
- Spielstand-Format, Protokoll und `IslandSaveVersion` bleiben unverändert; die Länge gilt nur für neu erzeugte Stufen.

## Nicht-Ziele

`B-485 › Nicht-Ziele`: Änderung gespeicherter Welten, Ruinen und Gegnerlager (B-468, WZ14), dauerhaftes Speichern des Zooms, die Sondergegend selbst, echte Planungen oder geänderte Werte in `data/islands.json` einchecken, Spec-Freigaben oder Kriterien umformulieren.

## Regeln und Einschränkungen

`B-485 › Regeln und Einschränkungen`. Nur für den Sprint:
- Jede Session hat genau eine Domäne: WZ17.1 REG, WZ17.2 SIM, WZ17.3 SRV, WZ17.4–WZ17.7 DEV. Die Review-Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff aller vier Domänen.
- Golden: bestehende Dateien bleiben byte-gleich; der neue Golden-Fall mit Chunk-Anzahl entsteht in WZ17.2 als eigener Commit innerhalb der Session (nach dem Code-Commit, Begründung im Commit-Text), wie in WZ8.3.
- Der Sprint committet keine geänderte `data/islands.json`; Editor-Tests schreiben nur in Kopien (`t.TempDir()`).
- Keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Fenster- und Browserprüfungen nur mit Freigabe von 🧑.
- Mit 7 Sessions liegt der Sprint über dem Richtwert 3–6, weil Review und Abnahme getrennt sind.

## Beispiele

`B-485 › Beispiele`: Wald auf „klein“ → 20 Chunks, Hub mittig, beide Vorschauen zeigen die Stufe als Entwurf; Platz +11 bei 20 Chunks → Fehler am Platz; Alt + Mausrad über der Live-Vorschau → nur diese zoomt; Seed-Chunk „lichtung“ umwandeln → gleiche Vorschau, Objekte einzeln in der Liste.

## Ausnahme- und Fehlerfälle

`B-485 › Ausnahme- und Fehlerfälle` (Anzahl außerhalb der Tabelle, geplanter Chunk passt nicht mehr, Datei extern geändert, Seed-Chunk ohne Seed). Dazu für den Sprint:
- Der Generator verträgt 16 Chunks in einem Biom nicht (Hub, Ausgang, Portale, Biom-Summen) → WZ17.2 `blockiert`, Untergrenze mit 🧑 festlegen (Spec-Revision), nicht stillschweigend ändern.
- Der Benchmark der Level-Erzeugung überschreitet +10 % gegenüber der Baseline aus WZ8.3 → Ursache beheben oder Grenze mit 🧑 neu festlegen.
- Der Editor sprengt das Budget (≈ 400 Code-Zeilen je Session) → Rest als Ticket, das Kriterium nicht umformulieren.

## Akzeptanzkriterien

- **AC-01** `B-485/Offene Fragen 1–3` Format und Regeln der Chunk-Anzahl und der Inselgrößen als REG-Beschluss von 🧑, Regeltext in `docs/rules/`, Glossar „Inselgröße“.
- **AC-02** `B-485/AC-02` Ohne Chunk-Anzahl sind `level-*.json` und `sim-*.json` byte-gleich; mit den sechs Größen hat das Level genau diese Chunks, `level.Validate` über 100 Seeds je Größe ohne Befund; neuer Golden-Fall als eigener Commit.
- **AC-03** `B-485/AC-03` Die Prüfung der Planung lehnt einen Platz außerhalb der Anzahl ab und nennt ihn.
- **AC-04** `B-485/AC-04` Größen-Vorauswahl setzt die Anzahl; Chunk- und Live-Vorschau zeigen die Länge des Entwurfs aus derselben Quelle (Vorschau-Option ohne globalen Zustand).
- **AC-05** `B-485/AC-01`, `B-485/AC-05` Durchsuchbare Listen mit Emoji-Icons; Alt + Mausrad zoomt nur die Vorschau unter dem Zeiger, gleiche Weltposition, Zoom bleibt beim Wechsel von Insel und Stufe.
- **AC-06** `B-485/AC-06` Umwandlung eines Seed-Chunks in feste Objekte ohne Änderung des Bildes.
- **AC-07** `B-485/AC-07` 🧑 nimmt Bedienung, Größen-Auswahl, getrennten Zoom und Umwandlung am PC ab; keine Planung und keine Golden-Änderung außer dem neuen Fall wird eingecheckt.
- **AC-08** Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` sind grün, Golden-Diff (nur der neue Fall) und Begründung sind geprüft, `git status` zeigt keine Änderung an `data/islands.json`.

## Offene Fragen

Entschieden von 🧑 am 2026-10-18: `B-485 › Offene Fragen` (Entscheidungen, Revision 2). Im Workshop WZ17.1 (2026-10-18) entschieden: Größen 16 · 20 · 24 · 32 · 40 · 48 für alle Biome, Tabelle in `data/island-sizes.json`, Feld `chunkCounts` je Insel neben `plannedChunks`, ohne Eintrag gilt die Biom-Länge wie heute, Seed-Inhalt aus dem Vorschau-Level.

Nichts Blockierendes offen. Nachzuweisen in WZ17.2: Die Untergrenze 16 gilt in allen fünf Biomen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ17.1 | `WZ17.1-workshop-chunk-anzahl.md` | Workshop | Mensch | fertig |
| WZ17.2 | `WZ17.2-generator-chunk-anzahl.md` | Umsetzung | autonom | in Arbeit |
| WZ17.3 | `WZ17.3-vorschau-option-chunk-anzahl.md` | Umsetzung | autonom | offen |
| WZ17.4 | `WZ17.4-listen-icons-zoom.md` | Umsetzung | autonom | offen |
| WZ17.5 | `WZ17.5-editor-groessen-umwandlung.md` | Umsetzung | autonom | offen |
| WZ17.6 | `WZ17.6-review.md` | Review | autonom | offen |
| WZ17.7 | `WZ17.7-abnahme.md` | Review | Mensch | offen |

- WZ17.1 (REG) Workshop mit 🧑: Format der Chunk-Anzahl, Größen-Tabelle, Untergrenze, Wert ohne Einstellung, Quelle des Seed-Inhalts; Regeltext in `docs/rules/`, Glossar „Inselgröße“ (AC-01).
- WZ17.2 (SIM) Generator und Prüfung nehmen die Chunk-Anzahl je Stufe (Daten, kein globaler Zustand), Benchmark, 100-Seed-Test je Größe, neuer Golden-Fall als eigener Commit (AC-02, AC-03).
- WZ17.3 (SRV) Vorschau-Option „Chunk-Anzahl“ und Funktion für den Seed-Inhalt in `room.PreviewOptions`, Go-Tests (AC-04, AC-06).
- WZ17.4 (DEV) Durchsuchbare Listen mit Emoji-Icons und getrennter Zoom (Alt + Mausrad, gleiche Weltposition) im Welt-Betrachter (AC-05).
- WZ17.5 (DEV) Größen-Auswahl und Chunk-Anzahl im Editor, Umwandlung Seed-Chunk → feste Objekte, Schreiben nach den WZ8-Regeln (AC-04, AC-06; nach B-486).
- WZ17.6 (DEV) Review: `task check:all`, Diff REG, SIM, SRV und DEV, nur schwere Befunde, PR des Sprints (AC-08, alle).
- WZ17.7 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-07).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
