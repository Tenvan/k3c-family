# GEB4 · DEV, CLI · Burg und Hub-Stufen 1 bis 5, Tor

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** 16bf107a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), Revision 1 inkl. Tor 2 × 4

## Ausgangslage

Die Burg (Hub-Kern, Zahlziel des Hub-Ausbaus) wird aus drei fremden Packs gezeichnet: Hub-Stufe 1 aus Ausschnitten von `k3c-paletten/ebenen/fort-tileset.png` mit `fort-banner.png` (×2, etwa 10 × 9 Kacheln, `siteView.drawCastle`), Stufe 2 `wooden-fortress-and-animated-doors`, Stufen 3–5 `opp2017-castle-tiles` (nur im Register, im Spiel Platzhalter). **Ungeprüft vermutet / bei der Planung gesehen:** `drawCastle` läuft einmal in `worldRenderer.drawStatic` mit `hubTexture()` ohne Stufe, die Burg wechselt im Spiel also nie mit `World.hubLevel`. Das Tor (`gate`) zeigt `k3c-paletten/props/fort-closed-door.png` ×2. Seit GEB3 gibt es Ausbaustufen in der Gebäude-JSON (Regeldatei § 6), Mauer, Turm und Hintergrundmauer in den Stufen 1–5.

## Ziel

Burg und Tor haben einheitliche Grafik, die Hub-Stufen 1–5 wachsen sichtbar durch Zusätze und Material. Am Ende sichtbar: Stufenreihe in der Vorschau, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet Maße und nimmt Stufenreihe und Spiel ab; Agenten bauen Daten, Werkzeug-Nutzung und Darstellung; Spieler sehen die Burg mit jeder Hub-Stufe wachsen.

## Anforderungen

B-435 › Anforderungen; Regeln aus `docs/standards/gebaeude-grafik.md` (Stufen § 6, Vorlage § 5); Aufräum-Regel aus Projekt GEB › Notizen. Sprint-eigen:
- **Burg** (`burg.json`, Entscheidung 🧑 2026-10-16, Chat): **8 × 9 Kacheln**, Grundform = Hub-Stufe 1 (Holz), `stufen` 2–5 in den Material-Farbstufen der Mauer (Stein, Kupfer, Eisen, Kristall); die Burg **wächst sichtbar** über Zusätze (z. B. Ecktürme, höhere Zinnen, Banner, Fenster, Kristallspitze), nicht nur über die Farbe. Mittig ein Tor/Thron-Eingang als Zahlziel.
- **Tor** (`tor.json`, Entscheidung 🧑 2026-10-16, Chat): Stufen 1–5 mit den Farbstufen der Mauer; im Spiel zeigt jedes Tor die **Stufe der Mauer seiner Linie** (nur Darstellung, keine Sim-Änderung). Maß (Annahme, bestätigt mit der Freigabe): **2 × 4 Kacheln** (Torbogen mit zwei Flügeln, niedriger als die Mauer).
- **Spiel:** Die Burg zeigt die aktuelle Hub-Stufe (`World.hubLevel`) und wechselt beim Ausbau; Slots `castle`, `hub:1`–`hub:5`, `gate` im Register und in `buildingSprites.ts`, Maßstab 1.
- **Aufräumen:** ersetzte Grafik (Fort-Tileset, Fort-Banner, Fort-Türen, Holzfestung, Burg-Kacheln) samt Verweisen löschen, sofern nichts anderes sie nutzt; ein leerer Pack fällt ganz weg (CREDITS, `grafikPacks.ts`, LICENSE).

## Nicht-Ziele

Lager, Farm, Kaserne, Treppen (GEB5); Natur und Objekte (GEB6); Animationen (Banner, Torflügel); HP-Balken und Schadens-Looks der Burg; Sim- oder Protokolländerungen.

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB4.1 Burg, GEB4.2 Tor: `public/grafik/gebaeude/`, `tools/gebaeude/` nur bei Bedarf, `src/tools/assetvorschau*.ts`, Regeldatei) → **CLI** (GEB4.3: `src/scenes/`, `data/assets.json`, Löschen alter Grafik). `engine/` bleibt unberührt.
- Determinismus: gleiche JSON → byte-gleiche PNGs; vorhandene PNGs bleiben unverändert.
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- `task gebaeude` mit `burg.json` → `burg-1-32.png` … `burg-5-32.png` (je 256 × 288 px) plus 16/64/128.
- Hub-Ausbau auf Stufe 3 abgeschlossen → die Burg wechselt im Spiel auf `burg-3-32.png`.
- Linke Linie 2 hat eine Mauer der Stufe 4 → ihr Tor zeigt `tor-4-32.png`.

## Ausnahme- und Fehlerfälle

- `hubLevel` fehlt im Snapshot → Burg in Stufe 1.
- Tor ohne gebaute Mauer seiner Linie (Mauer im Bau oder zerstört) → Tor in Stufe 1.
- Baustelle des Tors → Platzhalter wie heute.
- Grafik nicht geladen → bisherige Platzhalter-Form.

## Akzeptanzkriterien

- **AC-01** Burg (`castle`) und Hub-Stufen `hub:1`–`hub:5` liegen als Daten-JSON und PNG vor und folgen den Regeln aus GEB1.
- **AC-02** Das Tor (`gate`) passt in Maß und Stil zu den Mauer-Stufen aus GEB3.
- **AC-03** 🧑 hat die Stufenreihe abgenommen.
- **AC-04** `burg.json` (8 × 9, Vorlage, Stufen 2–5) und `tor.json` (2 × 4, Vorlage, Stufen 2–5) erzeugen mit `task gebaeude` byte-gleich wiederholbare PNGs; die Burg wächst je Stufe sichtbar über Zusätze; Stufenreihen in der Vorschau.
- **AC-05** Im Spiel zeigt die Burg die aktuelle Hub-Stufe und wechselt beim Ausbau; jedes Tor zeigt die Stufe der gebauten Mauer seiner Linie (sonst 1); reine Auswahl-Funktionen mit Vitest; Register und `buildingSprites.ts` zugeordnet, Maßstab 1.
- **AC-06** Ersetzte Burg- und Tor-Grafik ist mit allen Verweisen gelöscht (Suche ohne Treffer außerhalb historischer Doku), sofern nichts anderes sie nutzt; `task check` und `task check:go` grün.

## Offene Fragen

- Entschieden 2026-10-16 (Chat, Ralf): Burg 8 × 9 Kacheln.
- Entschieden 2026-10-16 (Chat, Ralf): Tor in den Stufen 1–5, zeigt die Stufe der Mauer seiner Linie.
- Annahme, bestätigt mit der Freigabe: Tor 2 × 4 Kacheln.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB4.1 | `GEB4.1-burg-stufen.md` | Umsetzung | autonom | in Arbeit |
| GEB4.2 | `GEB4.2-tor-stufen.md` | Umsetzung | autonom | offen |
| GEB4.3 | `GEB4.3-burg-tor-spiel.md` | Umsetzung | autonom | offen |
| GEB4.4 | `GEB4.4-review-abnahme.md` | Review | autonom | offen |

- GEB4.1 (DEV) Burg 8 × 9 mit Hub-Stufen 1–5 (AC-01, AC-04).
- GEB4.2 (DEV) Tor 2 × 4 in den Stufen 1–5 (AC-02, AC-04).
- GEB4.3 (CLI) Burg wechselt mit der Hub-Stufe, Tor in der Stufe seiner Mauerlinie, alte Grafik löschen (AC-05, AC-06).
- GEB4.4 Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
