# B-597 · Die Sound-Seite hat differenzierte Hörproben, auch für Skills

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ29
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (mit WZ29 Revision 1)

## Ausgangslage

Die Sound-Seite des Asset-Studios hat Slots, Szenen (Ereignis-SFX, Musik, Umgebung) und eine Hörprobe. Ralf (2026-10-24): Für Skills gibt es keine vernünftige Hörprobe, auch andere Töne fehlen differenziert.

## Ziel

Jede Art von Ton hat eine eigene, gezielte Hörprobe, die den Spielkontext nachbildet.

## Beteiligte und Zielgruppen

Ralf (Auswahl der Töne); Agent DEV; später der AI-Chat (B-600).

## Anforderungen

- Neue Szene „Skills“: alle Skills nach Linie (Tank, Zauberer, Heiler, Dieb) und Art (aktiv, passiv), je Zeile Abspielen mit dem Ton des Skills (Skill-Slot vor Linien-Slot vor `sfx:skill`, wie SO8), Anzeige woher der Ton kommt (Skill, Linie, Rückfall, still).
- Weitere Szenen nach Bedarf: Gegner und Bosse (Auftritt, Tod), Bauen (Fortschritt, fertig, Hub-Ausbau), Reiten, UI (Auswahl, Bestätigen), Warnungen, Kampf-Mischung (Treffer/Pfeil/Tod gleichzeitig).
- Mischprobe: Musik eines Zustands und ein Effekt gleichzeitig, Ducking hörbar; Lautheitsvergleich: gewählte Töne nacheinander mit gleichem Pegel.
- Abstandsprobe: Ton „nah“ und „fern“ (Dämpfung wie im Spiel, `attenuation`).
- Jede Zeile nennt Slot, Datei, Lizenz und Lücke (kein Ton) sichtbar.

## Nicht-Ziele

Neue Töne beschaffen (macht die Zuordnung, B-600); Spiel-Logik.

## Regeln und Einschränkungen

Frontend im Studio (`tools/k3c-studio/frontend/src/sound`), Atomic Design über `@webkit`; nutzt den Audio-Kern aus dem Spiel (`@game/audio`); keine externen Abrufe; Abbildung Skill → Slot nur einmal (aus SO8 teilen, nicht kopieren).

## Beispiele

Szene „Skills“ → Heiler › groupHeal › Abspielen spielt den Heiler-Ton; Passiv ohne Slot zeigt „still“.

## Ausnahme- und Fehlerfälle

Datei fehlt → Zeile zeigt Hinweis statt Fehler. Sound-Atlas (`public/audio/atlas.json`) fehlt → Hinweis auf den Task `soundatlas` aus `tools/Taskfile.yml`.

## Akzeptanzkriterien

- **AC-01** Szene „Skills“ spielt alle Skills mit dem richtigen Ton oder zeigt „still/Rückfall“ (Vitest der Abbildung, Beobachtung).
- **AC-02** Mindestens die Szenen Gegner/Bosse, Bauen, Reiten, UI und Warnungen sind abspielbar.
- **AC-03** Misch-, Lautheits- und Abstandsprobe funktionieren.

## Offene Fragen

Welche weiteren Szenen Pflicht sind: Defaults wie oben; Ralf ergänzt bei der Abnahme.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Hängt an SO8 (Abbildung Skill → Slot) und B-594. Sprint WZ29.
