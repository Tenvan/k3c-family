# B-797 · Das native Spiel zeichnet Gebäude, Baustellen, Plätze, Natur und Objekte aus dem Spielzustand

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** NG4
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Browser-Client zeichnet Gebäude und Plätze in `src/scenes/render/` (`buildingLook.ts`, `buildingSprites.ts`, `buildingSize.ts`, `scaffoldView.ts`, `siteView.ts`, `siteStatusView.ts`, `buildSignView.ts`, `priceTagRules.ts`, `mineSignView.ts`, `pennant.ts`, `plantation.ts`, `backWallView.ts`, `hubGrowth.ts`, `damageView.ts`) und Natur und Objekte (`natureSprites.ts`, `objectView.ts`). Gebäude kommen aus Papier-Sheets, Natur, Objekte und Gerüste sind bis MG8 Pixel-Grafik.

## Ziel

Paket `native/k3c-game/props` zeichnet Gebäude, Baustellen, Plätze, Natur und Objekte aus dem Spielzustand.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung NG9).

## Anforderungen

- Gebäude mit Stufe, Zustand und Schaden, Rückwand, Wachstum der Hub-Stufe.
- Baustellen, Gerüste, Bau-Schilder, Preis-Anzeigen, Minen-Schilder, Wimpel, Plantagen, Platz-Status.
- Natur (Bäume, Steine, Adern) und Objekte (Münzen, Truhen, Fackeln, was `objectView.ts` kennt).
- Rechnung (Look, Größe, Preis-Text) als reine Funktionen mit Tests nach den Regeln der TS-Dateien.

## Nicht-Ziele

Neue Grafik; Umstellung von Pixel-Grafik auf Papier (MG8); Figuren (NG3).

## Regeln und Einschränkungen

Entscheidung 005. Vertrag `view` aus NG1 unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/props/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Monarch zahlt an einem Platz → Preis-Anzeige zählt herunter, Gerüst erscheint, nach dem Bau steht das Gebäude der neuen Stufe.

## Ausnahme- und Fehlerfälle

Gebäude-Typ ohne Sheet → Meldung beim Laden des Levels. Zerstörtes Gebäude → Ruine wie im Browser.

## Akzeptanzkriterien

- **AC-01** Gebäude: Tests für Look und Größe je Typ und Stufe nach `buildingLook.ts` und `buildingSize.ts` grün; Schaden und Ruine sichtbar.
- **AC-02** Baustellen, Gerüste, Schilder und Preis-Anzeigen: Tests nach `priceTagRules.ts` und `siteView.ts` grün.
- **AC-03** Natur und Objekte: Jeder Natur- und Objekt-Typ aus `data/` hat eine Grafik (Test über alle Typen).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG4).
