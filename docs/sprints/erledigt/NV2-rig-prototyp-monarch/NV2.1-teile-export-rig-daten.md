# NV2.1 · Teile des Monarchen und Rig-Daten exportieren

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** nv2/1-teile-export-rig-daten
- **Abhängig von:** –
- **Tickets:** B-778
- **Kriterien:** AC-01

## Ziel

`task native:rig:export` schreibt die Teil-PNGs von Monarch (je Spielerfarbe) und Pferd, je Figur eine Rig-Datei mit Plätzen und die Soll-Werte für den Test in NV2.2 nach `native/k3c-rig/assets/`; die Dateien sind eingecheckt.

## Kontext

- Specs: `public/grafik/figuren/monarch.paper.json` (Rig `figur`, 12 Teile, `umhang`, `varianten`), Pferd unter `public/grafik/reittiere/` (Rig `vierbeiner`, Felder `bein`, Teile `rumpf-vorn`, `rumpf-hinten`, `hals`, `oberbein-*`, `unterbein-*`, `huf-*`). Stil: `public/grafik/stil.paper.json`.
- Renderer zum Wiederverwenden (nur importieren): `tools/paper/render/svg.ts`, `stil.ts`, `png.ts`, `raster.ts`, `baum.ts`; `rig/figur.ts`, `animation.ts`, `gangarten.ts`, `vierbeiner.ts`, `beinpfad.ts` (`beinPunkte`), `reiten.ts` (Sattel, Ebenen, Zügel).
- **Rig-Format (gilt später für alle Figuren):** `teile` (id, eltern, pivot, bild, versatz, ebene), `plaetze` (name, teil, punkt, drehung, ebene; mindestens `waffe`, `ruestung`, `kopfbedeckung`, beim Vierbeiner `sattel`), `baender` (Umhang, Beine: Stützpunkte, Breiten, Textur), `animationen` (Keyframes), `varianten`. Das Schwert des Monarchen wird als Element am Platz `waffe` exportiert, nicht als festes Teil.
- Aufruf wie `bun tools/paper/bin/paper.ts`; das Export-Skript liegt unter `native/k3c-rig/export/`.
- Defaults aus B-778 › Offene Fragen. `gen:paper` und die Sheets bleiben unverändert.

## Erlaubte Dateien

- `native/k3c-rig/**`
- `native/Taskfile.yml`
- `.gitignore` (nur falls nötig, damit `native/k3c-rig/assets/` eingecheckt bleibt)
- Planung B-778, NV2 (über MCP)

## Nicht-Ziele

Änderungen an `tools/paper`, `public/grafik`, `src/`, `engine/`; Go-Code des Prototyps (NV2.2); zweites Waffen- und Rüstungs-Teil zeichnen ist erlaubt, aber nur als Datei unter `native/k3c-rig/`.

## Schritte

1. Rig-Format als kurze Beschreibung (`native/k3c-rig/RIG.md`) und Go-/TS-Typen festlegen.
2. Export Monarch: je Teil ein PNG in Ruhelage mit Papier-Filter, Versatz zum Pivot; Schwert als Element.
3. Export Pferd: starre Teile als PNG, Beine als Band (Textur plus Breiten), Platz `sattel` aus `reiten.ts`.
4. Platzhalter: zweite Waffe und ein Rüstungs-Teil im selben Stil.
5. Soll-Werte: Teil-Matrizen und Stützpunkte je Frame für `idle`, `lauf` (Monarch) und `trab` (Pferd, Reiter am Sattel).
6. Zweimal ausführen, Dateien per Hash vergleichen; Größe der Ausgabe notieren.

## Fertig, wenn

- [x] AC-01: Export läuft für Monarch und Pferd, Rig-Dateien nennen die Plätze, zweiter Lauf erzeugt byte-gleiche Dateien, Ausgabe eingecheckt; Gesamtgröße und RGBA-Speicher der Teile stehen im Ergebnis (Vergleich: 363 MiB der Monarch-Sheets).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 geprüft:** `task native:rig:export` schreibt 52 Dateien nach `native/k3c-rig/assets/` (46 PNG, `monarch/rig.json`, `pferd/rig.json`, `elemente/elemente.json`, 3 Sollwert-Dateien); `task native:rig:export:check` (zweiter Lauf, byte-genauer Vergleich) grün. Plätze: Monarch `waffe`, `ruestung`, `kopfbedeckung`, `hand`; Pferd `sattel`, `maul`. **Größe 579 KB, 1,94 MiB RGBA** (Monarch-Sheets: 363 MiB). Einchecken folgt mit dem Commit der Session.
- **Abweichungen:** Das Band bekam `breite` je Stützpunkt (Befund aus NV2.2: mit der vollen Bildbreite faltete sich das Bein am gebeugten Knie). Der Papier-Filter gilt je Teil für das ganze Bild statt für die Hülle (sonst schneidet er Schatten kleiner Teile ab). Das Pferd trägt die Papierstruktur des Reiters (Seed `monarch`), weil das Reit-Sheet so gerendert ist. Dateinamen des Exports englisch (`docs/arbeitsweise.md` › Namen und Ablage). Zusätzlich Plätze `hand` und `maul` für die Zügel.
- **Format:** `native/k3c-rig/RIG.md`. Platzhalter `axt` und `brustpanzer` nur in `native/k3c-rig/export/elements.ts`.
- Geprüft von Claude (2026-11-11), keine manuelle Prüfung. Neue Tickets: keine.
