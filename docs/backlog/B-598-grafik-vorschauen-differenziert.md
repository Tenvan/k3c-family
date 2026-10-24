# B-598 · Die Grafik-Seite hat differenzierte Vorschauen, auch für Skill-Effekte

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

Die Grafik-Seite des Studios hat Slots, Szenen (Gebäude, Hub wächst, Hintergründe, …) und Packs. Ralf (2026-10-24): Das Gleiche wie beim Ton gilt für die Grafiken; es fehlen z. B. Vorschauen für die Effekte der Skills.

## Ziel

Jede Art von Grafik hat eine eigene Vorschau im Spielmaßstab, die den Spielkontext nachbildet.

## Beteiligte und Zielgruppen

Ralf (Auswahl der Grafiken); Agent DEV; später der AI-Chat (B-600).

## Anforderungen

- Neue Szene „Skill-Effekte“: alle Skills nach Linie und Art, je Zeile die Animation des Effekts (`skillLooks.ts`, Ring als Rückfall) am Monarchen, in Spielgröße und ×2/×3, mit Farbe, Radius, Dauer.
- Weitere Szenen nach Bedarf: Bauschilder und Baugerüste (B-584, B-586), Abbau-Schilder (B-588), Gegner je Biom, Bosse mit Phasen, Reittiere, HUD-Elemente, Wimpel/Markierer, Truppen und Bürger.
- Vorschau mit Tag/Nacht-Licht und Biom-Hintergrund wählbar.
- Jede Vorschau nennt Slot, Sheet, Atlas-Frame, Lizenz und Lücke.

## Nicht-Ziele

Neue Grafiken erzeugen (Zuordnung, B-600); Spiel-Logik.

## Regeln und Einschränkungen

Frontend im Studio (`tools/k3c-studio/frontend/src/graphics`), Atomic Design über `@webkit`; Zeichnen mit denselben Funktionen wie das Spiel (`@game/scenes/render`, `effects`), keine zweite Darstellung; keine externen Abrufe.

## Beispiele

Szene „Skill-Effekte“ → Dieb › backstab zeigt den roten Burst am Monarchen, Abspielen wiederholt ihn.

## Ausnahme- und Fehlerfälle

Sheet fehlt → Platzhalter mit Hinweis. Skill ohne Look → Ring der Linie, Zeile markiert „Rückfall“.

## Akzeptanzkriterien

- **AC-01** Szene „Skill-Effekte“ zeigt alle Skills mit ihrem Effekt oder Rückfall (Vitest der Aufbereitung, Beobachtung).
- **AC-02** Mindestens die Szenen Schilder/Gerüste, Gegner je Biom, Reittiere und Bosse sind verfügbar.
- **AC-03** Biom und Tageslicht sind wählbar.

## Offene Fragen

Welche weiteren Szenen Pflicht sind: Defaults wie oben; Ralf ergänzt bei der Abnahme.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ29. Zusammenhang: B-594, B-586, B-588.
