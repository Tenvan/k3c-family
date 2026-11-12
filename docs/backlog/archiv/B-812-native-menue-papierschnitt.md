# B-812 · Das Menü des nativen Spiels erscheint im Papierschnitt-Stil mit Illustrationen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG13
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Das Menü des nativen Spiels ist schlichter Text. Der Browser-Client zeigt Papier-Hintergrund und Karten im Papierschnitt-Stil (`src/scenes/paperBackdrop.ts`, `src/scenes/paperCard.ts`) und darauf Kacheln mit Illustrationen (`pauseHub.ts:62-82`, `hubArt.ts`). Revision 2 (2026-11-12): Das Ticket war Größe L und ist zerlegt; es deckt nur Hintergrund und Karten ab, die Illustrationen stehen in B-818.

## Ziel

Haupt- und Pause-Menü des nativen Spiels liegen auf einem Papier-Hintergrund, ihre Einträge erscheinen als Karten im Papierschnitt-Stil (Farbe, Rand, Schatten).

## Beteiligte und Zielgruppen

Spieler, Ralf (Sichtprüfung bei der PC-Abnahme), Claude (Umsetzung).

## Anforderungen

- Papier-Hintergrund für Haupt- und Pause-Menü.
- Einträge als Karten mit Papierfarbe, Rand und Schatten wie `src/scenes/paperCard.ts`; Werte aus `public/grafik/stil.paper.json` (Materialien, Töne).
- Eine Quelle für die Werte: Konstanten in `menu`, ein Test vergleicht sie mit `public/grafik/stil.paper.json` (entschieden, siehe Offene Fragen).
- Auswahl (Fokus) bleibt deutlich sichtbar; Zurück per Esc und Gamepad B unverändert.
- Keine Allokation je Bild.

## Nicht-Ziele

Illustrationen der Kacheln (B-818), neue Menüseiten (B-807, B-808, B-809), neue Assets.

## Regeln und Einschränkungen

Entscheidungen 005 und 008. Import-Regeln der Pakete (`game/imports_test.go`): `menu` liest keine Dateien zur Laufzeit außerhalb von `art`, die Werte stehen als Konstanten in `menu`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Hauptmenü: Papier-Hintergrund, „Weiter“, „Neues Spiel“, „Optionen“ als Karten mit Schatten; die gewählte Karte ist hervorgehoben.

## Ausnahme- und Fehlerfälle

`stil.paper.json` ändert einen Ton → der Vergleichstest wird rot, die Konstanten werden nachgezogen (kein stiller Drift).

## Akzeptanzkriterien

- **AC-01** Test in `menu`: Karten-Rechtecke von Haupt- und Pause-Menü liegen im Bild und überlappen nicht; die gewählte Karte ist anders gezeichnet als die übrigen.
- **AC-02** Test: Farben für Hintergrund, Karte, Rand und Schatten stimmen mit `public/grafik/stil.paper.json` überein.
- **AC-03** Ende-zu-Ende-Lauf (`task native:game:e2e`): zwei Läufe gleich in allen Stationen; Bilder unter `reports/native-game-e2e/` für die Sichtprüfung bei der PC-Abnahme.

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Zerlegt (Größe L → S): dieses Ticket nur Papier-Hintergrund und Karten ohne Illustrationen, die Illustrationen in B-818 (Entwurf). Werte als Konstanten in `menu` mit Vergleichstest gegen `stil.paper.json`, weil `menu` zur Laufzeit keine Dateien liest. Die Sichtprüfung läuft bei der nächsten PC-Abnahme mit (Ralf-Queue), keine eigene Mensch-Session.

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11). Revision 2 (2026-11-12): zerlegt, Illustrationen → B-818.
