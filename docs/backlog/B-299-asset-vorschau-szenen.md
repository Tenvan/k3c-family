# B-299 · Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** DBG4
- **Projekt:** GRA
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der ResourcenManager (B-298) braucht eine Vorschau der Zuordnung im echten Spielmaßstab. Nur der Client-Renderer zeichnet mit `UNIT_PX` und der Skalierung aus `data/sprites.json`; ein Nachbau in k3c-dev würde abweichen. Entscheidung 🧑 im Chat am 2026-10-05.

## Ziel

Eine Dev-Seite `assetvorschau.html` zeigt je Kategorie eine Mini-Szene mit den aktiven Assets aus dem Slot-Register, damit 🧑 die Wahl im Zusammenhang beurteilt, nicht einzeln.

## Beteiligte und Zielgruppen

🧑 beurteilt in k3c-dev (iframe, B-298) oder direkt im Browser; der Agent prüft im Browser-Pane.

## Anforderungen

- Aufruf `assetvorschau.html?szene=<kategorie>[&slot=<id>&wahl=<n>]`; ohne Parameter Kategorie-Auswahl.
- **Grafik-Kategorien:** Figuren (Monarch, Truppen, Gegner, Reittiere in einer Reihe, animiert), Gebäude (Bauplatz und Bau-Stufen nebeneinander), Umgebung (Parallax-Ebenen und Boden je Biom, scrollbar), Objekte (Münzen, Truhen, Portale, Items), HUD (Anzeigen und Controller-Glyphen).
- **Sound-Kategorien:** Ereignis-SFX (Knopf je Slot, Dämpfung links/rechts wie im Split-Screen), Musik je Zustand (Tag/Nacht mit Crossfade wie die Hörprobe), Umgebung.
- Liest Slot-Register, Ausschnitte und Präferenz; `wahl=n` zeigt eine andere Präferenz zum Vergleich.
- Lädt neu, wenn sich das Slot-Register ändert (Vite-HMR oder Nachricht vom iframe-Host).
- Seiten-Regeln aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`.

## Nicht-Ziele

Zuordnung ändern (nur B-298), eigene Mini-Szene je Asset, Spiel-Logik.

## Regeln und Einschränkungen

Domäne PLAT (`*.html`, `src/tools/`); Zeichnen über vorhandene Renderer-Bausteine statt Kopie; kein `Math.random()`; Datei ≤ 400 Zeilen.

## Beispiele

`?szene=gebaeude` → Bauplatz, Farm, Turm, Mauer in allen Bau-Stufen auf Boden im Maßstab. `?szene=gebaeude&slot=building.farm&wahl=2` → Farm mit der zweiten Präferenz.

## Ausnahme- und Fehlerfälle

Slot ohne Zuordnung → Platzhalter-Rahmen mit Slot-ID. Bild oder Sound lädt nicht → Platzhalter mit Pfad, Log `❌`. Aktives Asset unter CC-BY-NC → sichtbares Kennzeichen „NC“ (Q70).

## Akzeptanzkriterien

- **AC-01** Jede Grafik-Kategorie zeigt ihre Slots mit aktiver Zuordnung und Ausschnitt im Spielmaßstab (Beobachtung im Browser-Pane).
- **AC-02** Jede Sound-Kategorie spielt ihre Slots auf Knopfdruck ab (Beobachtung).
- **AC-03** Die Auflösung Slot → aktives Asset (erste Präferenz, `wahl=n`, Ausschnitt) ist getestet (Vitest).
- **AC-04** Die Seite fehlt im Produktions-Build (`dist/`) und ist im Dev-Build erreichbar; `tests/projectRules.test.ts` ist grün (Regel um Dev-Seiten ergänzt).

## Offene Fragen

Kategorien final festlegen (Vorschlag in den Anforderungen).

Entschieden (🧑, Chat 2026-10-05): Die Seite gibt es nur im Dev-Build, nicht im Produktions-Build und nicht auf der Landingpage der Xbox; die Regel „in `pages.ts` eintragen“ braucht dafür eine Ausnahme oder einen Dev-Eintrag (`tests/projectRules.test.ts`).

## Notizen

Hängt an B-298 (Slot-Register). Vorhandene Bausteine: `src/tools/grafiken.ts`, `spriteReference.ts`, `soundtestLogic.ts`, `audioProbe.ts`.
