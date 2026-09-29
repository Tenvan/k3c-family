# Family Three Crowns (K3C)

[![CI](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml/badge.svg)](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml)

Couch-Koop-Strategie-Side-Scroller im Stil von *Kingdom Two Crowns*, gebaut für den Browser,
damit er auf der **Xbox (Edge)** mit mehreren Controllern läuft.

## Loslegen

```bash
npm install
npm run dev
```

Dann `http://localhost:5173` öffnen, oder im Heimnetz `http://<PC-IP>:5173` (z.B. von der Xbox aus).
Die Startseite ist eine Landingpage, die man komplett mit dem Controller bedient (D-Pad/Stick + **A**,
**Y** bzw. **F** = Vollbild). Sie bleibt dauerhaft geöffnet und zeigt Spiel und Testseiten in sich an.
Dadurch bleibt Vollbild beim Seitenwechsel erhalten.

Jede Seite hat oben mittig einen **Start**-Button. Zurück zur Übersicht geht es auch mit **View + Menu**
(kurz gemeinsam halten) bzw. **Pos1**. **B** wird abgefangen und schließt nichts versehentlich.

Im Spiel:

- **A** (Controller) / **Leertaste**: Beitreten (bis zu 2 Spieler, Split-Screen)
- **A** / **Leertaste halten**: Münzen geben – an Bauplätze, Werkstatt (Bögen), Landstreicher (werden Bauern),
  Bäume/Felsen (Bauer holt das Material). Ohne Ziel fällt die Münze, der andere Spieler kann sie aufheben.
- Linker Stick / **A**,**D**: laufen, **RT** / **Shift**: sprinten
- **F** / rechten Stick drücken: Vollbild
- Nachts kommen Gegner aus den Portalen. Mauern halten sie auf, Bogenschützen schießen automatisch.
- Spielstand: wird bei jedem Tagesanbruch und beim Verlassen automatisch gespeichert (Heimnetz-Server `saves/k3c.json`,
  sonst localStorage) und beim Start von `game.html` geladen. `?new=1` startet ein neues Spiel.
  Mit `?seed` oder `?depth` wird frei gespielt, ohne zu speichern.
- Dev: **N** neuer Seed, **1/2/3** Tiefe wechseln, URL-Parameter `?seed=abc&depth=1`,
  `?fast=1` (Tag/Nacht 8x schneller), `?dev=1` (**G** +10 Gold, **H** +50 Material, **T** nächste Tageszeit)

## Im Heimnetz hosten (Xbox)

```bash
npm run serve
```

Baut das Spiel und startet den Server auf Port **8080**. Die Konsole zeigt die Adressen, z.B.
`http://192.168.2.230:8080/`. Diese Adresse in Edge auf der Xbox öffnen. Falls die Windows-Firewall fragt:
Zugriff im **privaten** Netzwerk erlauben.

### Gamepad-Test auf der Xbox

1. `npm run serve` am PC starten.
2. Auf der Xbox in Edge `http://<PC-IP>:8080/` öffnen und die Kachel **Gamepad-Test** wählen.
3. Beide Controller verbinden, auf jedem alle Tasten einmal drücken, auch **B** und die Sticks.
4. **Vollbild** anklicken, dann **View** für den FPS-Test drücken (dauert ca. 30 s, danach View = zurück).
5. **Y** drücken. Der Bericht landet am PC in `reports/gamepad-*.json`.

### HTTPS (nur falls nötig)

Falls der Test zeigt, dass die Xbox die Gamepad API ohne HTTPS nicht freigibt: `certs/key.pem` und
`certs/cert.pem` ablegen. Dann läuft zusätzlich HTTPS auf Port **8443**. Ein selbstsigniertes Zertifikat
erzeugt auf der Xbox eine Warnung. Ob Edge die Seite danach als sicher behandelt, zeigt der Test
(„Secure Context“). Sonst bräuchte es ein echtes Zertifikat, etwa eine eigene Domain mit Let's Encrypt.

## Neue Seite hinzufügen (z.B. weitere Testseiten)

1. `meine-seite.html` in den Projektordner legen. Der Build nimmt jede `*.html` automatisch auf.
2. Im Script `installPageChrome()` aus `src/core/shell.ts` aufrufen. Das bringt Start-Button, View + Menu und B-Schutz mit.
3. Eintrag in `src/landing/pages.ts` ergänzen. Danach erscheint die Kachel auf der Startseite.
4. Vollbild nur über `toggleFullscreen()` aus `src/core/fullscreen.ts`.

Die vollständige Regel steht in `CLAUDE.md` unter „Regel: Seiten & Navigation“.

## Level anpassen

Die Eckdaten jeder Stufe stehen in `src/data/biomes/*.json` (Länge, Chunk-Häufigkeiten, Ressourcen,
Portale, Gegner). Nach Änderungen `npm test` ausführen. Die Tests prüfen 500 Seeds pro Biom auf Spielbarkeit.

## CI/CD (GitHub Actions)

- **CI** (jeder Push/PR): Typecheck, Tests, Build, Smoke-Test des Heimnetz-Servers. Der Build liegt als Artefakt `k3c-dist` am Lauf.
- **GitHub Pages** (Push auf `main`): Spiel und Testseiten online, ohne Bericht-Server.
  Einmalig aktivieren: *Settings → Pages → Source: GitHub Actions*.
- **Release** (Tag `v*`, z.B. `git tag v0.2.0 && git push --tags`): Zip mit `dist/` + `server/` am Release.
  Entpacken und `node server/server.mjs` starten, `npm install` ist dafür nicht nötig.

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md) · [Session-Plan](docs/sessions.md)
