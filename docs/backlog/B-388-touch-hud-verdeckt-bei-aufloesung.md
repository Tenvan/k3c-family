# B-388 · Touch-Tasten und Start-Button verdecken HUD-Infozeilen bei bestimmten Auflösungen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (Befunde aus U5.4 freigegeben)

## Ausgangslage

Beobachtung 🧑 am 2026-10-10 bei U5.4 (Touch in Chrome, Gerätemodus, Querformat ca. 2000 px breit): Die Touch-Tasten 1–4 und » liegen über dem Spielfeld und verdecken die Zeile „Links/rechts berühren = laufen“ teilweise. Der Start-Button der Shell (oben mittig) verdeckt im selben Lauf einen Teil der oberen HUD-Zeile „Oberwelt (Wald)“. Bei einer anderen Breite (ca. 1567 px) liegen die Tasten unterhalb des Spielfelds und verdecken nichts. Die Lage der Tasten ändert sich also mit der Auflösung; U5 verlangt, dass HUD und Lauf-Flächen frei bleiben (B-191).

## Ziel

HUD-Infozeilen (oben und unten) bleiben in gängigen Auflösungen lesbar, auch mit Touch-Tasten und Shell-Button.

## Beteiligte und Zielgruppen

🧑 testet am PC und Handy; Agent baut in `src/input/touchInput.ts` und `src/scenes/`.

## Anforderungen

- Touch-Tasten verdecken keine Zeile der HUD-Anzeige (Info-Zeile unten, Skill-Zeile, obere HUD-Zeile).
- Der Shell-Start-Button verdeckt keinen Text der oberen HUD-Zeile.
- Gilt für Querformat in mehreren Breiten (Chrome-Gerätemodus, echtes Gerät).

## Nicht-Ziele

Hochformat (ist gesperrt); neue Touch-Tasten.

## Regeln und Einschränkungen

Shell-Button: oben ca. 70 px frei lassen (`CLAUDE.md`, Seiten-Regel 1); `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Querformat ca. 2000 px breit → Infozeile „Links/rechts berühren = laufen“ vollständig lesbar, keine Taste darüber. Querformat ca. 1567 px → unverändert gut.

## Ausnahme- und Fehlerfälle

Sehr schmale Fenster: nicht relevant, solange das Hochformat gesperrt ist.

## Akzeptanzkriterien

- **AC-01** Im Chrome-Gerätemodus (Querformat, zwei Breiten) ist keine HUD-Zeile durch Touch-Tasten oder den Shell-Button verdeckt (Beobachtung 🧑).
- **AC-02** Ein Test oder eine Messung belegt die Position der Touch-Tasten relativ zur Info-Zeile für eine feste Auflösung.

## Offene Fragen

Soll der Shell-Button bei Touch die obere HUD-Zeile meiden (ausblenden, nach unten schieben) oder verschiebt sich nur das HUD? Entscheidet 🧑.

## Notizen

Herkunft: U5.4, Frage 15 mit „nein“ und Screenshots 2 und 3 beantwortet. Verwandt: B-191, U5.4.
