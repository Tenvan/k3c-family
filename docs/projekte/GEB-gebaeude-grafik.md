# GEB · Gebäude-Grafik

- **Status:** aktiv
- **Rang:** 3
- **Ziel-Tickets:** B-435

## Ziel

Alle Gebäude und Strukturen haben stimmige Proportionen, einheitlichen Stil und Erweiterbarkeit (Ausbaustufen, Level-Looks) und entstehen im Asset-Manager aus Beschreibung und Prompt. Sichtbar daran, dass jedes Gebäude einer Grundregel-Vorlage folgt und von 🧑 abgenommen ist. Ticket: B-435.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| GEB1 | Grundregeln für Gebäude-Grafik und Werkstatt | erledigt |
| GEB2 | Gebäude ohne Grafik: Taverne, Heilplatz, Schmiede, Rüstkammer | erledigt |
| GEB8 | Gebäude-Vorlagen in den Daten-JSONs | erledigt |
| GEB3 | Mauer und Turm, Stufen 1 bis 5 | erledigt |
| GEB4 | Burg und Hub-Stufen 1 bis 5, Tor | erledigt |
| GEB5 | Lager, Farm mit Plantage, Kaserne, Treppen | erledigt |
| GEB6 | Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager | erledigt |
| GEB7 | Hintergründe je Biom | erledigt |
| GEB9 | Ausbaustufen 1 bis 5 für alle Hub-Gebäude | erledigt |
| GEB10 | Natur-Nachträge: hohe Bäume, Höhlen-Flora, geschlossene Truhe, Wimpel in Spielerfarbe | erledigt |
| GEB11 | Hub wächst kleinteilig vom Lagerfeuer bis zur Burg | aktiv |
| GEB12 | Entscheidungen zu GEB9 bis GEB11 (Server-Stufenfolge, Markierer im Snapshot, Wald-Dichte) | erledigt |
| GEB13 | Gebäude-Stufen in den Daten und Stufen-Look für Plantage, Camp, Heilplatz | erledigt |
| GEB14 | Wald und Höhle dichter, Gerüst bei Wartenden, Knoten-Markierer im Snapshot | erledigt |
| GEB15 | Höhlen-Wurzeln, Felsen und Minen: mehr Varianten, größer | geplant |
| GEB16 | Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik | geplant |

## Nicht-Ziele

Figuren, Gegner und Animationen (GRA); Münzen, Icons, Items und HUD; die Palette-Pipeline für fremde Packs (GRA, B-331). Freischaltung der Gebäude je Hub-Stufe und der Gebäude-Planer (WZG, WZ9/WZ15, B-462); GEB liefert nur die Grafik je Stufe.

## Notizen

Umfang und Ist-Stand je Slot: B-435 › Notizen (Stand 2026-10-15, überholt: GEB2–GEB7 haben die Lücken geschlossen; aktuell `assets_list` und `public/grafik/gebaeude/index.json`). Von neun Sprints sind GEB1–GEB8 erledigt, GEB9 (Ausbaustufen aller Hub-Gebäude) ist als Entwurf geplant. Zuschnitt:

- GEB1 Grundregeln, Werkstatt, drei Bäume (Stilvorlage und Regeln)
- GEB2 Lücken-Gebäude: Taverne, Heilplatz, Schmiede, Rüstkammer
- GEB3 Mauer und Turm, Stufen 1–5 (Paletten-Mechanik je Material)
- GEB4 Burg, Hub-Stufen 1–5 und Tor
- GEB5 Lager, Farm samt Plantage, Kaserne, Treppen
- GEB6 Natur und Objekte (Bäume, Fels, Erz, Busch, Adern, Truhe, Portal, Ausgang, Rekrutierungslager)
- GEB7 Hintergründe je Biom (15 Hintergrund-Schichten)
- GEB8 Gebäude-Vorlagen in den Daten-JSONs
- GEB9 Ausbaustufen 1–5 für alle Hub-Gebäude (offen: Stufen-Looks, Camp, Reihenfolge zu B-458)

Rang 3 (vor GRA und SND) von 🧑 am 2026-10-15 im Chat gesetzt.

**Abgrenzung zu WZG (Stand 2026-10-18):** Die Asset-Vorschau liegt auf der Grafik-Seite des Asset-Studios (WZ6), die Freischaltung je Hub-Stufe pflegt der Gebäude-Planer (WZ9, Regeltabelle aus `data/hub.json` erzeugt, WZ15). B-458 (Hub wächst kleinteiliger) bleibt Ticket des Projekts GEB, nutzt aber den Planer für die neue Stufenfolge; ein Sprint dafür ist noch nicht angelegt.

**Anforderung an jeden GEB-Sprint (🧑, 2026-10-16, Chat):** Alte Grafik, die das Gebäudesystem im Spiel ersetzt, wird im selben Sprint aufgeräumt und gelöscht: PNG unter `public/grafik/`, Einträge in `public/grafik/index.json`, Register-Wahl in `data/assets.json`, Zuordnungs-Doku (`docs/assets/zuordnung-*.md`), Test-Fixtures und Mocks, CREDITS-Zeilen (wird ein Pack leer, fällt er ganz weg). Gelöscht wird nur, was keine andere Stelle mehr nutzt (vorher per Suche belegen). Wer einen Sprint bereit macht, nimmt das als Anforderung und Kriterium auf.
