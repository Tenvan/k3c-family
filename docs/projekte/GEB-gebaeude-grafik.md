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
| GEB5 | Lager, Farm mit Plantage, Kaserne, Treppen | aktiv |
| GEB6 | Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager | geplant |
| GEB7 | Hintergründe je Biom | geplant |

## Nicht-Ziele

Figuren, Gegner und Animationen (GRA); Münzen, Icons, Items und HUD; die Palette-Pipeline für fremde Packs (GRA, B-331).

## Notizen

Umfang und Ist-Stand je Slot: B-435 › Notizen. Alle sieben Sprints sind als Entwurf angelegt (Tabelle oben). Zuschnitt:

- GEB1 Grundregeln, Werkstatt, drei Bäume (Stilvorlage und Regeln)
- GEB2 Lücken-Gebäude: Taverne, Heilplatz, Schmiede, Rüstkammer
- GEB3 Mauer und Turm, Stufen 1–5 (Paletten-Mechanik je Material)
- GEB4 Burg, Hub-Stufen 1–5 und Tor
- GEB5 Lager, Farm samt Plantage, Kaserne, Treppen
- GEB6 Natur und Objekte (Bäume, Fels, Erz, Busch, Adern, Truhe, Portal, Ausgang, Rekrutierungslager)
- GEB7 Hintergründe je Biom (optional, nur wenn 🧑 will)

Rang 3 (vor GRA und SND) von 🧑 am 2026-10-15 im Chat gesetzt.

**Anforderung an jeden GEB-Sprint (🧑, 2026-10-16, Chat):** Alte Grafik, die das Gebäudesystem im Spiel ersetzt, wird im selben Sprint aufgeräumt und gelöscht: PNG unter `public/grafik/`, Einträge in `public/grafik/index.json`, Register-Wahl in `data/assets.json`, Zuordnungs-Doku (`docs/assets/zuordnung-*.md`), Test-Fixtures und Mocks, CREDITS-Zeilen (wird ein Pack leer, fällt er ganz weg). Gelöscht wird nur, was keine andere Stelle mehr nutzt (vorher per Suche belegen). Wer einen Sprint bereit macht, nimmt das als Anforderung und Kriterium auf.
