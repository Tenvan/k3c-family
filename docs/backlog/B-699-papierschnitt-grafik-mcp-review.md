# B-699 · Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review in der Workbench

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: Spec 005 freigegeben)

## Ausgangslage

Gebäude entstehen als DB32-Pixel aus Daten-JSON (`tools/buildings`), alle anderen Grafiken sind gekaufte Pixel-Packs. Das Asset-Studio (`tools/k3c-studio`) ist ein eigenes Programm mit Edit-Seiten und Chat; das Editieren dort ist unübersichtlich. Ein Prototyp (`docs/decisions/005/prototyp-monarch-rig.html`) zeigt Monarch und Pferd als Papierschnitt-Rig mit IK-Gangarten, Rückengelenk und Umhang-Physik.

## Ziel

Alle Grafiken haben die Papierschnitt-Optik und sind durch je eine Spec reproduzierbar; eine KI ändert sie nur über MCP-Tools, 🧑 prüft jede Änderung im Bereich Assets der Workbench mit Markierung im Git-Diff.

## Beteiligte und Zielgruppen

🧑 entscheidet und reviewt; Claude-Agenten erzeugen und ändern Specs über MCP; Spieler sehen das Ergebnis im Spiel.

## Anforderungen

Verbindlich ist `docs/decisions/005-papierschnitt-assets-mcp.md` (Abschnitte 1–7). Kurz:

- Spec je Asset (`*.paper.json`), zentraler Styleguide (`public/grafik/stil.paper.json`), Rig-Typen in TS, Render mit resvg, byte-gleiche PNGs.
- MCP-Tools `assets_guide`, `assets_list`, `assets_get`, `assets_create`, `assets_set` (atomar), `assets_style_get/set`, `assets_render`, `assets_preview` (Bild), `assets_review`, `assets_delete`; Prompt-Vorlagen auch als MCP-Prompts.
- Workbench mit Bereichen Development und Assets, ohne Edit-Controls, mit Prompt-Buttons; Studio-Programm und Chat entfallen.
- Review gegen `HEAD`, letzten Commit oder `origin/develop` mit Pixel-Diff und Teil-Markierung.
- Sprites zuerst für den PC-Browser.

## Nicht-Ziele

Skalierung für Xbox und Mobile, Engine-Schatten-Technik (spätere Tickets). Sound bleibt wie er ist. Kein Bildmodell.

## Regeln und Einschränkungen

Entscheidung 005; Datei-Schnittstelle und Datei-Watch aus 004 bleiben. Contract-first: Verträge aus Sprint PV1 ändern sich nur über ein Ticket.

## Beispiele

Agent ruft `assets_set {id: monarch, patch: {umhang: {material: purpur}}}` → Spec ändert sich, PNGs werden neu gerendert, Review zeigt den Umhang markiert mit Vorher/Nachher.

## Ausnahme- und Fehlerfälle

Ungültiger Patch → Datei unverändert, Antwort nennt Feld und Regel. Ungültige Spec auf der Platte → roter Eintrag in Assets, Rest rendert weiter; Spielserver behält alten Stand.

## Akzeptanzkriterien

Wortgleich mit `docs/decisions/005-papierschnitt-assets-mcp.md` › Kriterien; die Sprints des Projekts PAP verteilen sie.

- **AC-01** `paper validate` lehnt eine Spec mit Hex-Farbe, unbekanntem Feld, fremdem Material oder fehlendem Pflicht-Teil mit Feld und Regel ab.
- **AC-02** Gleiche Spec ergibt auf zwei Läufen und in der CI byte-gleiche PNGs (Golden-Test).
- **AC-03** Der Monarch aus dem Prototyp rendert als Spec Idle, Lauf, Schritt, Trab und Galopp mit IK-Hufen, Rückengelenk und Umhang-Physik im Papierschnitt-Look.
- **AC-04** Eine Materialänderung im Styleguide ändert alle betroffenen Assets, ohne eine Asset-Spec anzufassen.
- **AC-05** Ein Agent legt ein neues Asset nur mit `assets_guide`, `assets_create`, `assets_set` und `assets_preview` an und sieht dabei das gerenderte Bild.
- **AC-06** `assets_set` mit ungültigem Patch lässt die Datei unverändert.
- **AC-07** Die Workbench zeigt die Bereiche Development und Assets; Assets enthält keine Edit-Controls.
- **AC-08** Die Review-Seite zeigt für eine geänderte Spec Vorher/Nachher und Pixel-Diff-Overlay und markiert die geänderten Teile; Basis `HEAD`, letzter Commit und `origin/develop` sind wählbar.
- **AC-09** Prompt-Buttons kopieren einen vollständigen Prompt mit Asset-ID und Teil; dieselben Vorlagen sind als MCP-Prompts verfügbar.
- **AC-10** `tools/k3c-studio`, seine EXEs, Ports und Tasks sowie der Chat sind entfernt; `task buildall` und `task check:all` laufen grün.
- **AC-11** (manuell, 🧑) Review einer echten Änderung am Monarchen in Assets ist ohne Öffnen von Dateien möglich.

## Offene Fragen

keine

## Notizen

Projekt PAP. Zuschnitt in Wellen und Lanes für parallele Agenten (Entscheidung 005 › Umsetzung).
