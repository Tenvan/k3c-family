# B-594 · Die Grafik-Seite zeigt je Slot Name, Atlas-Definition und alles für die Zuordnung

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Auf der Grafik-Seite des Asset-Studios (Slots, Szenen, Packs) zeigt die Slot-Liste nur Name, Art und einen Status-Punkt (Ralf, 2026-10-24, Screenshot der Liste „figuren › Gegner“). Für eine Grafik-Zuordnung fehlen die Angaben dahinter.

## Ziel

Zu jedem Slot sieht man alles, was man für die Zuordnung braucht, ohne in Dateien zu suchen.

## Beteiligte und Zielgruppen

Ralf (Zuordnung am Studio); Agent DEV; spätere AI-Chat-Zuordnung profitiert von denselben Angaben.

## Anforderungen

- Je Slot: Name (Id), Anzeigename, Art (Gruppe), Beschreibung aus `data/assets.json`, Status (zugeordnet, Lücke, Platzhalter).
- Definition aus dem Atlas (`public/atlas/atlas.json`): Frame-Namen, Frame-Größe, Anzahl Frames je Animation, Atlas-Seite und Position; Sheet-Daten aus `data/sprites.json` (Sheet, Skalierung, Höhe, Ursprung, Tönung, Animationen mit fps).
- Zuordnung: gewählter Pack, Quelle, Lizenz/Credit, Datei, Zuordnungs-Eintrag aus `docs/assets/zuordnung*.md`.
- Verwendung: wo der Slot im Spiel vorkommt (z. B. Gegner in welchem Biom, Tiefe).
- Detailbereich neben der Liste, Liste filterbar nach Status.

## Nicht-Ziele

Zuordnung ändern über den Chat (kommt später); neue Grafiken erzeugen.

## Regeln und Einschränkungen

Studio-Backend liefert die Daten (`tools/k3c-studio/backend`, `tools/assetkit`), Frontend zeigt nur; keine externen Abrufe (läuft ohne Internet); Atlas wird gelesen, nicht gebaut; Atlas fehlt → Hinweis „task gen:atlas“.

## Beispiele

Slot `crystalGuardian` → Sheet `crystal-golem`, Frame 160×112, idle 6 / run 6 / attack 6 Frames, Skalierung ×1, Quelle irmirx (CC-BY 3.0), Zuordnungs-Zeile, vorkommend in Biom Kristallhöhle.

## Ausnahme- und Fehlerfälle

Slot ohne Sheet (Lücke) → Felder leer mit Vermerk „keine Grafik“. Atlas ohne den Frame → Warnung am Slot.

## Akzeptanzkriterien

- **AC-01** Der Detailbereich eines Slots nennt Name, Beschreibung, Status, Sheet-Daten und Atlas-Definition (Vitest der Aufbereitung, Beobachtung im Studio).
- **AC-02** Er nennt Pack, Quelle, Lizenz und Verwendung des Slots.
- **AC-03** Fehlender Atlas oder fehlendes Sheet zeigt einen klaren Hinweis statt eines Fehlers.

## Offene Fragen

Welche Angaben aus `zuordnung*.md` maschinenlesbar sind (sonst Backend liest die Tabellen); in der Umsetzung klären.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ28.
