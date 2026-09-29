# Family Three Crowns (K3C)

Couch-Koop-Strategie-Side-Scroller im Stil von *Kingdom Two Crowns*, gebaut für den Browser,
damit er auf der **Xbox (Edge)** mit mehreren Controllern läuft.

## Loslegen

```bash
npm install
npm run dev
```

Dann `http://localhost:5173` öffnen, oder im Heimnetz `http://<PC-IP>:5173` (z.B. von der Xbox aus).

- **A** (Controller) / **Leertaste**: Beitreten (bis zu 2 Spieler, Split-Screen)
- Linker Stick / **A**,**D**: laufen, **RT** / **Shift**: sprinten
- **F** / rechten Stick drücken: Vollbild
- Dev: **N** neuer Seed, **1/2/3** Tiefe wechseln, URL-Parameter `?seed=abc&depth=1`

## Level anpassen

Die Eckdaten jeder Stufe stehen in `src/data/biomes/*.json` (Länge, Chunk-Häufigkeiten, Ressourcen,
Portale, Gegner). Nach Änderungen `npm test` ausführen. Die Tests prüfen 500 Seeds pro Biom auf Spielbarkeit.

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md)
