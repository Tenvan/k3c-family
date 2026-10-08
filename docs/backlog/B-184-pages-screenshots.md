# B-184 · Die Präsentationsseite zeigt echte Bilder aus dem Spiel

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PG1
- **Projekt:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Pages-Präsentation (`site/index.html`, B-183) zeigt nur Grafiken aus den Asset-Packs, keine Szene aus dem laufenden Spiel.
Gebäude und Hintergrund sind im Spiel noch Platzhalter (B-010).

## Ziel

Besucher sehen, wie das Spiel wirklich aussieht: Split-Screen, Hub, eine Nacht mit Welle.

## Beteiligte und Zielgruppen

Interessierte mit dem Pages-Link; 🧑 wählt die Szenen aus.

## Anforderungen

- 3–5 Screenshots (oder ein kurzes, stummes Video), im Repo unter `site/`, je Bild höchstens 300 KB.
- Seite bleibt rein statisch (B-183, `tests/pagesSite.test.ts`).

## Nicht-Ziele

Ein Trailer mit Schnitt und Musik.

## Regeln und Einschränkungen

Erst sinnvoll, wenn B-010 (Grafiken für Gebäude und Hintergrund) im Spiel ist.

## Beispiele

Aufruf der Pages-Seite → Abschnitt „So sieht es aus“ mit Bildern aus Wald, Höhle und Split-Screen.

## Ausnahme- und Fehlerfälle

nicht relevant: statische Bilder.

## Akzeptanzkriterien

- **AC-01** Die Seite zeigt mindestens drei Bilder aus dem Spiel; `tests/pagesSite.test.ts` prüft, dass sie existieren.

## Offene Fragen

Screenshots oder Video, welche Szenen (🧑).

## Notizen

–
