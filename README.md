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

## Im Heimnetz hosten (Xbox)

```bash
npm run serve
```

Baut das Spiel und startet den Server auf Port **8080**. Die Konsole zeigt die Adressen, z.B.
`http://192.168.2.230:8080/`. Diese Adresse in Edge auf der Xbox öffnen. Falls die Windows-Firewall fragt:
Zugriff im **privaten** Netzwerk erlauben.

### Gamepad-Test auf der Xbox

1. `npm run serve` am PC starten.
2. Auf der Xbox in Edge `http://<PC-IP>:8080/gamepad-test.html` öffnen.
3. Beide Controller verbinden, auf jedem alle Tasten einmal drücken, auch **B** und die Sticks.
4. **Vollbild** anklicken, dann **View** für den FPS-Test drücken (dauert ca. 30 s, danach View = zurück).
5. **Y** drücken. Der Bericht landet am PC in `reports/gamepad-*.json`.

### HTTPS (nur falls nötig)

Falls der Test zeigt, dass die Xbox die Gamepad API ohne HTTPS nicht freigibt: `certs/key.pem` und
`certs/cert.pem` ablegen. Dann läuft zusätzlich HTTPS auf Port **8443**. Ein selbstsigniertes Zertifikat
erzeugt auf der Xbox eine Warnung. Ob Edge die Seite danach als sicher behandelt, zeigt der Test
(„Secure Context“). Sonst bräuchte es ein echtes Zertifikat, etwa eine eigene Domain mit Let's Encrypt.

## Level anpassen

Die Eckdaten jeder Stufe stehen in `src/data/biomes/*.json` (Länge, Chunk-Häufigkeiten, Ressourcen,
Portale, Gegner). Nach Änderungen `npm test` ausführen. Die Tests prüfen 500 Seeds pro Biom auf Spielbarkeit.

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md)
