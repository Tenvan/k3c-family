# B-162 · Für die Grafik-Lücken liegen Kandidaten mit Vorschau, Lizenz und Stilbewertung vor

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** GR2
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint GR2; mit Änderungen aus dem Spec-Review (3 Kandidaten nach Q14, Figuren-Lücken als B-193, Reihenfolge nach GR1)

## Ausgangslage

B-010 nennt Lücken ohne Treffer im Bestand: Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen (`docs/funde/b010-grafik-funde.html`). Die Zuordnungstabelle aus B-161 (`docs/assets/zuordnung.md`) macht die vollständige Lückenliste. Weitere Lücken entstehen mit B-112 (Hub-Stufen, Materialstufen) und den Bossen.

## Ziel

Zu jeder Lücke aus B-161 liegen Kandidaten vor (Quelle, Vorschau, Lizenz, Stilpassung), 🧑 hat je Lücke gewählt, die gewählten Assets liegen im Repo mit Credits. Nutzen: Der Einbau (B-010) hat alle Bilder.

## Beteiligte und Zielgruppen

Der Agent recherchiert; 🧑 wählt je Lücke aus den Vorschlägen (Q14 in `docs/fragenkatalog.md`).

## Anforderungen

- Suche bei OpenGameArt, itch.io (nur CC0 oder CC-BY) und Kenney (CC0) je Lücke aus `docs/assets/zuordnung.md`.
- Je Lücke drei Kandidaten (Q14; weniger nur mit Vermerk) mit Link, Vorschau, Lizenz, Urheber, Raster und Stilbewertung zum Grundstil (B-161); Ergebnis als Seite oder Datei unter `docs/funde/` im Stil von `b010-grafik-funde.html`.
- Nach der Auswahl: Dateien gefiltert nach `public/grafik/<id>/` (nur PNG und Lizenzdatei, Regeln aus `public/grafik/CREDITS.md`), Eintrag in `public/grafik/index.json` und `public/grafik/CREDITS.md`, Zeile in `docs/assets/zuordnung.md` auf „zugeordnet“.
- Nur Skalieren und Palette als Nachbearbeitung.

## Nicht-Ziele

Einbau in den Renderer (B-010), Atlas (B-163), Figuren-Lücken unter `public/sprites/` (B-193), selbst gezeichnete Grafiken, Packs mit nicht erlaubter Lizenz (CC-BY-NC, unklar).

## Regeln und Einschränkungen

Lizenz CC0 oder CC-BY, Namensnennung sofort in den CREDITS-Dateien und `lizenzen.html` (Test `src/tools/grafikPacks.test.ts` verlangt Übereinstimmung); keine Musik, kein Demo-Code im Repo.

## Beispiele

Lücke „Werkstatt“ → drei Kandidaten mit Vorschau → 🧑 wählt einen → Pack liegt unter `public/grafik/`, Credits stehen, Tabelle zeigt „zugeordnet“.

## Ausnahme- und Fehlerfälle

Kein passender Kandidat → Lücke bleibt mit Vermerk „kein Treffer, Platzhalter bleibt“ (B-010: Platzhalter-Form statt leerer Stelle). Lizenz auf der Quellseite und im Pack widersprechen sich → vorsichtigere Lizenz gilt, Hinweis in `CREDITS.md` (wie bei Warped Caves).

## Akzeptanzkriterien

- **AC-01** Zu jeder Lücke aus `docs/assets/zuordnung.md` liegt eine Kandidatenliste mit drei Einträgen, weniger nur mit Vermerk, wenn die Suche nicht mehr hergibt (Link, Vorschau, Lizenz, Urheber, Stilbewertung) unter `docs/funde/` vor.
- **AC-02** 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt; die Entscheidung steht in der Kandidatenliste.
- **AC-03** Gewählte Assets liegen unter `public/grafik/` mit Lizenzdatei, Eintrag in `public/grafik/index.json` und `public/grafik/CREDITS.md`; `task test` (Test `src/tools/grafikPacks.test.ts`) ist grün.
- **AC-04** Die Zuordnungstabelle zeigt für jede entschiedene Lücke „zugeordnet“ oder „kein Treffer, Platzhalter“.

## Offene Fragen

- Welcher Kandidat wird je Lücke gewählt? Entscheidet 🧑, `docs/fragenkatalog.md` Q14.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene G, GR2. Wiederholbar: neue Lücken (Hub-Stufen, Bosse) kommen mit ihren Sprints.
