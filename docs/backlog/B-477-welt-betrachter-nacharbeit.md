# B-477 · Der Welt-Betrachter zeigt Chunk- und Live-Vorschau in einer gemeinsamen, synchron scrollenden Ansicht

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ16
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Chat (Ralf: „B-477 ist freigegeben (Spec so)“)

## Ausgangslage

Befunde 🧑 bei der Abnahme WZ7.7 (2026-10-18, zwei Screenshots des Studios am PC) zu B-460:
1. Maximiert (Chunk-Vorschau füllt die Seite) fehlt die Live-Vorschau; das Layout am unteren Rand (Knopfleiste „Im Spiel öffnen“) ist abgeschnitten, dazu eine zweite Scrollbar rechts.
2. Chunk-Vorschau und Live-Vorschau scrollen unabhängig voneinander (zwei Scrollbars) und zeigen nicht denselben Ausschnitt.
3. Die gebauten Gebäude stehen als lange Checkbox-Liste (`gate 321`, `wall 325` …) unter der Live-Vorschau und sind unübersichtlich.
4. Zwei getrennte Vorschau-Blöcke übereinander statt einer gemeinsamen Ansicht.
5. Es gibt nur +/− mit festen Schritten, keinen stufenlosen oder mehrstufigen Zoom für beide Vorschauen.

Betroffen: `tools/k3c-studio/frontend/src/world/` (`WorldPage.tsx`, Live-Vorschau, `styles/world.css`). Die Optik war beim Review (WZ7.6) ungeprüft.

## Ziel

Der Welt-Betrachter ist eine Ansicht: Chunk-Streifen und Live-Vorschau liegen untereinander im selben Scrollbereich und bewegen sich gemeinsam. Eine Leiste mit Sprungpunkten und ein Zoom gelten für beide. Maximiert zeigt sie beide Vorschauen und nutzt die Fensterhöhe ohne abgeschnittene Bedienelemente.

## Beteiligte und Zielgruppen

🧑 nutzt den Welt-Betrachter am PC und nimmt ab. Agenten setzen um, ein getrennter Reviewer prüft. Kein Spieler betroffen.

## Anforderungen

- **Eine Ansicht:** Chunk-Streifen oben, Live-Vorschau darunter, beide im selben horizontalen Scrollbereich; Scrollen verschiebt beide gleich (gleiche Welt-Position links). Eine Scrollbar, keine zweite am Rand.
- **Sprungleiste:** eine Leiste über oder unter der Ansicht mit Sprungpunkten: ganz links, Mitte, ganz rechts, Hub, Tor (Gate), Portale, Ausgang (Punkte aus dem Level: Hub-Linie, Chunk-Arten `portal`, `exit`, Bauplätze `gate`). Ein Klick scrollt beide Vorschauen dorthin.
- **Zoom:** stufenlos (Regler, Strg+Mausrad) und zusätzlich feste Stufen (z. B. 25 %, 50 %, 100 %, 200 %); gilt für beide Vorschauen gleich und hält die Mitte der Ansicht fest. Die Tasten + und − bleiben.
- **Maximiert und verkleinert:** Maximiert zeigt die Ansicht beide Vorschauen mit der ganzen Fensterhöhe; der Knopf „Verkleinern“ bzw. „Maximieren“ wechselt zwischen kompakt (Streifen klein, Live-Vorschau groß) und maximiert (beide groß). Knopfleiste „Im Spiel öffnen“ und Optionen sind immer vollständig sichtbar (kein Abschneiden unten, keine zweite Scrollbar rechts, auch bei kleiner Fensterhöhe).
- **Gebäude-Filter:** gruppiert nach Art (z. B. Mauern, Türme, Tore, Werkstätten) mit Schalter je Gruppe „alle ein/aus“ und aufklappbarer Liste der einzelnen Plätze (Beschriftung Art und Position); eine Suche ist nicht nötig. Vorschlag, die Gestaltung entscheidet die Umsetzung (🧑 hat keine feste Vorstellung).
- Live-Optionen (Hub-Stufe, Tageszeit, Hintergrund-Schichten) bleiben in einer Leiste über der Live-Vorschau.

## Nicht-Ziele

Keine Änderung an Go (Snapshot, Binding), an `src/scenes/` und `src/online/`; keine neuen Vorschau-Optionen außer den genannten; Chunk-Planung (B-461) und Gebäude-Planer (B-462).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-studio/frontend/`). Die Phaser-Bühne aus WZ7.3 bleibt die einzige und wiederverwendbar (WZ10); Zoom und Scroll-Position kommen von außen in die Bühne. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Prüfungen im Fenster nur mit Freigabe von 🧑 für den Lauf.

## Beispiele

- Ralf klickt „Hub“ in der Sprungleiste → Chunk-Streifen und Live-Vorschau zeigen beide den Hub, mit derselben linken Kante.
- Ralf zieht den Zoom-Regler auf 50 % → beide Vorschauen zeigen doppelt so viel Welt, die Mitte bleibt.
- Maximiert → beide Vorschauen sichtbar, „Im Spiel öffnen“ unten vollständig, nur eine Scrollbar.
- Gebäude-Filter: „Mauern“ aus → alle Mauern verschwinden, die Türme bleiben.

## Ausnahme- und Fehlerfälle

- Level ohne Hub, Tor, Portal oder Ausgang → der Sprungpunkt fehlt oder ist ausgegraut.
- Sehr kleines Fenster → Ansicht scrollt senkrecht im Seitenbereich, Bedienelemente bleiben erreichbar.
- Zoom an den Grenzen → wird begrenzt, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Chunk-Streifen und Live-Vorschau liegen in einer Ansicht und scrollen gemeinsam (gleiche Welt-Position); es gibt nur eine Scrollbar. Nachweis: Frontend-Test der gemeinsamen Scroll-Position, Beobachtung durch 🧑.
- **AC-02** Die Sprungleiste springt zu links, Mitte, rechts, Hub, Tor, Portal und Ausgang; fehlende Punkte sind ausgegraut. Nachweis: Test der reinen Sprungpunkt-Funktion, Beobachtung durch 🧑.
- **AC-03** Zoom stufenlos und in festen Stufen, gleich für beide Vorschauen, Mitte bleibt fest. Nachweis: Test der Zoom-Funktion (Grenzen, Mitte), Beobachtung durch 🧑.
- **AC-04** Maximiert zeigt die Ansicht beide Vorschauen; die Knopfleiste „Im Spiel öffnen“ ist vollständig sichtbar, keine zweite Scrollbar rechts, auch bei kleiner Fensterhöhe. Nachweis: Beobachtung durch 🧑 (Fenstergrößen), Test des Ansichtszustands.
- **AC-05** Der Gebäude-Filter ist nach Art gruppiert mit Gruppenschalter und aufklappbaren Plätzen. Nachweis: Test der Gruppierung aus dem Snapshot, Beobachtung durch 🧑.
- **AC-06** `task check:all` und CI grün; 🧑 nimmt die Ansicht am PC ab.

## Offene Fragen

keine (Gestaltung des Gebäude-Filters nach Vorschlag der Umsetzung; 🧑 hat dazu keine Vorgabe)

## Notizen

Aus der Abnahme WZ7.7 (WZ7/AC-09), 2026-10-18. Bezug B-460; WZ8.5 und WZ9 bauen auf der Seite auf, daher laufen WZ8.5 und WZ9.2 erst nach WZ16 (SIM- und SRV-Sessions von WZ8 sind unabhängig).
