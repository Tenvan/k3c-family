# B-471 · renderReference beendet seine Animationsschleife, wenn die Karten verschwinden

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`renderReference` (`src/tools/spriteReference.ts`) startet je Aufruf eine eigene `requestAnimationFrame`-Schleife ohne Abmeldung. Auf `figuren.html` und `aufstellung.html` läuft es einmal je Seite, im Asset-Studio (Szene Figuren, `tools/k3c-studio/frontend/src/grafik/SlotCards.tsx`) bei jedem Wechsel zur Szene und jedem Neuladen mit geänderten Figuren; alte Schleifen laufen auf abgehängten Canvases weiter (zeichnen dank `isVisible` nichts, kosten aber je Frame Rechenzeit). Gefunden im Review von WZ6.3.

## Ziel

`renderReference` liefert eine Abmeldung (oder endet von selbst, wenn `root` nicht mehr im Dokument hängt); das Asset-Studio ruft sie beim Neuzeichnen und Abbauen der Figuren-Szene auf.

## Beteiligte und Zielgruppen

🧑 hält das Asset-Studio lange offen; Dev-Seiten `figuren.html`, `aufstellung.html`.

## Anforderungen

- Rückgabe einer Abmeldefunktion, bestehende Aufrufer bleiben gültig.

## Nicht-Ziele

Figuren-Vorschau im Asset-Studio (B-463).

## Regeln und Einschränkungen

`src/tools/` (Domäne DEV) und `tools/k3c-studio/frontend`; Funktion ≤ 60 Zeilen.

## Beispiele

Zehnmal zwischen Figuren und Gebäude wechseln → genau eine laufende Schleife.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Aufräumarbeit ohne neues Verhalten.

## Akzeptanzkriterien

- **AC-01** Test: nach der Abmeldung (bzw. Entfernen von `root`) ruft die Schleife `requestAnimationFrame` nicht mehr auf.
- **AC-02** `figuren.html` und `aufstellung.html` zeigen unverändert animierte Figuren; die Szene Figuren im Asset-Studio meldet beim Abbau ab.

## Offene Fragen

keine

## Notizen

Aus dem Review von WZ6.3 (2026-10-17); Hinweis im Code `SlotCards.tsx`.
