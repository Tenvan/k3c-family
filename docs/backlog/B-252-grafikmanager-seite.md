# B-252 · Eine GrafikManager-Seite zeigt Bestand, Kandidaten und Zuordnung für die feine Auswahl

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Grafik-Auswahl lief in GR1/GR2 über Chat-Fragen und eine statische Fundseite (`docs/funde/gr2-grafik-funde.html`), die Bestandsbilder aus `public/` in der Vorschau nicht lädt. Seit GR2.3 liegen auch nicht gewählte Kandidaten als Gruppe `kandidaten` unter `public/grafik/` (Wunsch 🧑, 2026-10-04). Die Zuordnung steht in `docs/assets/zuordnung*.md`; `grafiken.html` zeigt nur die Packs.

## Ziel

🧑 kann in einem feineren Auswahl-Workshop je Spielobjekt Bestand und Kandidaten nebeneinander im Spielmaßstab sehen und die Wahl festhalten, ohne Chat-Fragen.

## Beteiligte und Zielgruppen

🧑 wählt; Agent bereitet vor und überträgt die Wahl in die Zuordnung.

## Anforderungen

- Je Objekt-Zeile aus `docs/assets/zuordnung*.md`: aktuelle Grafik und alle passenden Kandidaten aus `public/grafik/` (auch `kandidaten`), skaliert wie im Spiel (`UNIT_PX`), mit Lizenz und Urheber.
- Auswahl je Objekt festhalten und als Text exportieren, den ein Agent in die Zuordnung überträgt.
- Seiten-Regeln aus `CLAUDE.md` (Landingpage, `installPageChrome()`), mit Controller und am PC bedienbar.

## Nicht-Ziele

Grafik bearbeiten, Downloads aus dem Netz, Einbau in den Renderer (GR3).

## Regeln und Einschränkungen

Seiten-Regeln aus `CLAUDE.md`; Domäne PLAT (`*.html`, `src/tools/`); nur CC0 oder CC-BY.

## Beispiele

Objekt `farm` → zeigt `house-a` (zugeordnet) neben Kenney Farm Expansion und Crops (Kandidaten) → 🧑 wählt → Export „farm: …“.

## Ausnahme- und Fehlerfälle

Bild lädt nicht → Platzhalter mit Dateipfad statt leerer Kachel.

## Akzeptanzkriterien

- **AC-01** Die Seite zeigt je Objekt-Zeile die zugeordnete Grafik und die Kandidaten der Gruppe `kandidaten` (Beobachtung im Browser-Pane).
- **AC-02** Eine Auswahl je Objekt lässt sich als Text exportieren (Test der Export-Funktion).
- **AC-03** Seite ist in `src/landing/pages.ts` eingetragen, `tests/projectRules.test.ts` grün.

## Offene Fragen

Wie ordnet die Seite Kandidaten einem Objekt zu (Zuordnung in der Fundseite oder eigene Spalte in `zuordnung*.md`)? Entscheidet 🧑 beim Planen.

## Notizen

Kandidatenliste: `docs/funde/gr2-grafik-funde.html` (GR2.1).
