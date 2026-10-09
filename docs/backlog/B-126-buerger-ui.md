# B-126 · Der Client zeigt Berufe, Ausbildung, Händler, Truppen-Limit und Heilung

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W6
- **Projekt:** WRT
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint W6

## Ausgangslage

Der Client zeigt Bürger ohne Berufe, Limit oder Händler (`src/scenes/worldRenderer.ts`).

## Ziel

Spieler sehen Berufe der Bauern, können ausbilden und beim Händler tauschen, sehen das Truppen-Limit je Hub und den Heilplatz. Nutzen: Die Bürger-Regeln sind bedienbar.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

- Berufe und Handwerker sichtbar (Symbol/Name); Ausbilden und Tauschen über das Aktionen-Overlay (B-125). Jedes Angebot hat ein eigenes Zahlziel neben dem Gebäude, keine neue Taste (Beschluss Q34, 2026-10-04); der Händler hat die Zahlziele „Kaufen“ und „Verkaufen“ (Kurs 10 Material = 5 Gold, Beschluss Q36, 2026-10-04).
- HUD: Truppen-Limit je Hub (Kämpfer/Limit), Hinweis bei erreichtem Limit.
- Händler als Figur im Hub; Heilplatz-Reichweite sichtbar; Platzhalter, bis Grafiken kommen (B-010).

## Nicht-Ziele

Simulation (B-121, B-122), Protokoll (B-123), Grafiken (B-010).

## Regeln und Einschränkungen

`CLAUDE.md`; Client zeichnet nur. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Limit 12/20 im HUD; beim Händler zeigt das Overlay „A halten: 100 Holz gegen 50 Gold tauschen“.

## Ausnahme- und Fehlerfälle

Snapshot ohne neue Felder (alter Server) → Anzeige leer, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Reine Funktionen für Limit-Text und Berufsanzeige sind getestet.
- **AC-02** Berufe, Limit, Händler und Heilplatz werden angezeigt.
- **AC-03** 🧑 hat die Anzeige am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R3.3. Abhängig von B-121, B-122, B-123, B-125.
