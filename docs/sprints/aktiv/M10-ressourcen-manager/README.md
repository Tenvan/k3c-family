# M10 · DEV · Ressourcen-Manager für Grafik- und Sound-Slots in k3c-dev

- **Status:** aktiv
- **Projekt:** GRA
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-298, B-299
- **Start-Commit:** 3880dc29
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-14, 🧑 im Chat (Revision 3, mit Browser-Pane-Prüfung für diesen Lauf)

## Ausgangslage

Grafik- und Sound-Zuordnung steht verteilt in `docs/assets/zuordnung*.md`, `public/audio/kandidaten.json` und `data/sprites.json`; es gibt kein Slot-Register, keinen durchsuchbaren Asset-Katalog und keine Vorschau im Spielmaßstab (B-298, B-299).

## Ziel

🧑 ordnet in k3c-dev jedem Asset-Slot Assets nach Präferenz zu, legt Ausschnitte fest, lässt im Netz recherchieren und importiert Funde mit Credit; das Slot-Register `data/assets.json` ist die einzige Quelle. Am Ende sichtbar: k3c-dev zeigt alle Asset-Slots in den Ansichten Grafik und Sound, und die eingebettete Mini-Szene zeigt die aktive Zuordnung im Spielmaßstab.

## Beteiligte und Zielgruppen

🧑 wählt Assets; der Agent baut Register und Oberfläche.

## Anforderungen

B-298 › Anforderungen; B-299 › Anforderungen (aus DBG4, PJ3).

## Nicht-Ziele

Neue Assets selbst auswählen (wählt 🧑); Grafik oder Sound bearbeiten außer Ausschnitt; das Spiel liest das Slot-Register noch nicht (Renderer und Audio bleiben bei `data/sprites.json` und dem Audio-Atlas); `docs/assets/zuordnung*.md` werden nicht generiert, sie bleiben als Historie; Steuerung per Controller, Betrieb auf der Xbox.

## Regeln und Einschränkungen

- **Slot-Register** `data/assets.json` (Entscheidung 🧑 2026-10-14): legt die DEV-Session M10.1 an (Werkzeug-Daten, kein Spielwert, daher weder SIM noch REG); k3c-dev schreibt es, die Dev-Seite importiert es (Vite-HMR). Recherche-Treffer liegen getrennt in `data/assets-recherche.json`.
- **Migration** (Entscheidung 🧑 2026-10-14): einmaliger Import der Objekt-IDs und Zuordnungen aus `docs/assets/zuordnung*.md` (Präferenz 1) und aus `public/audio/kandidaten.json`; die md-Tabellen bleiben als Historie, `src/tools/zuordnung.test.ts` prüft sie weiter. `kandidaten.json` geht im Register auf (die Hörprobe `soundtest.html` liest danach das Register).
- **Domäne DEV für alle Sessions:** k3c-dev (`tools/k3c-dev/`) und Werkzeug-Seiten (`src/tools/`) sind laut Tabelle DEV, daher auch B-299. Grenzfall der Dev-Seite (M10.2): die Seite `assetvorschau.html` (sonst PLAT), in `vite.config.ts` die Dev-Seite aus dem Produktions-Build nehmen und `tests/projectRules.test.ts` um die Dev-Seiten-Regel ergänzen (sonst INF), nur für B-299/AC-04.
- Lizenzregel Q70 (`docs/fragenkatalog.md`), Credits sofort (B-165), freesound-Token nur lokal, Logging mit Emojis, Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Keine neue Abhängigkeit (Ziehen per HTML5-Drag-and-Drop, HTML-Lesen mit der Go-Standardbibliothek).

## Beispiele

Asset-Slot `farm` (Grafik, Kategorie `gebaeude`) → Suche „house“ → 🧑 wählt `grafik/gothicvania-town/props-einzeln/house-a.png` (Präferenz 1) und `house-b.png` (Präferenz 2) → Mini-Szene Gebäude zeigt `house-a`, mit `wahl=2` `house-b`. Asset-Slot `sfx:coin` → Recherche liefert einen freesound-Treffer (CC0, Hörprobe) und einen itch.io-Fund (nur Link, „Lizenz prüfen“).

## Ausnahme- und Fehlerfälle

Datei einer Zuordnung fehlt auf der Platte → Asset-Slot rot markiert, Mini-Szene zeigt Platzhalter mit Pfad. Slot-Register kaputt (JSON ungültig) → k3c-dev schreibt nicht und meldet den Fehler, die Datei bleibt unverändert. Quelle der Recherche nicht erreichbar → Hinweis an der Quelle, die übrigen liefern weiter.

## Akzeptanzkriterien

- **AC-01** Ein ResourcenManager in k3c-dev ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu (B-298/AC-01, B-298/AC-02, B-298/AC-03, B-298/AC-04, B-298/AC-05, B-298/AC-06, B-298/AC-07).
- **AC-02** Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab (B-299/AC-01, B-299/AC-02, B-299/AC-03, B-299/AC-04; aus DBG4 AC-01, PJ3).

## Offene Fragen

Keine. Entschieden (🧑, 2026-10-14): Slot-Register `data/assets.json`, angelegt von DEV; einmaliger Import der bestehenden Zuordnungen, md bleibt Historie; Recherche und Import bleiben in M10 (sechs Sessions); Kategorien der Mini-Szenen wie in B-299 vorgeschlagen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M10.1 | `M10.1-slot-register.md` | Umsetzung | autonom | fertig |
| M10.2 | `M10.2-assetvorschau.md` | Umsetzung | autonom | fertig |
| M10.3 | `M10.3-oberflaeche.md` | Umsetzung | autonom | fertig |
| M10.4 | `M10.4-recherche.md` | Umsetzung | autonom | fertig |
| M10.5 | `M10.5-import.md` | Umsetzung | autonom | offen |
| M10.6 | `M10.6-review.md` | Review | autonom | offen |

Kriterien: AC-01 in M10.1 (Register, Reihenfolge, Ausschnitt, MCP), M10.3 (Oberfläche, Mini-Szene eingebettet), M10.4 (Recherche), M10.5 (Import); AC-02 in M10.2.

## Abnahme

–
