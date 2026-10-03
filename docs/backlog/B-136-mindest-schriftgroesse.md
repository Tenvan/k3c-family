# B-136 · Die Mindest-Schriftgröße je Split-Viertel ist festgelegt

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** F1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F1

## Ausgangslage

Die HUD-Texte sind 20 bis 56 px groß (`src/scenes/HudScene.ts`, z. B. `fontSize: '20px'` für Info und Steuerungshinweis). Im Viertel-Split (4 Spieler) wird der Bildschirm auf die halbe Breite und Höhe verkleinert (`src/scenes/layout.ts`), ein 20-px-Text wirkt dann am TV wie ca. 10 px. Eine Regel für die Lesbarkeit am TV gibt es nicht.

## Ziel

Eine Regel nennt die Mindest-Schriftgröße (effektiv, in px bei 1920 × 1080 am TV) je Layout 1, 2, 3 und 4 Spieler. Nutzen: Kamera-/Layout-Arbeit (B-106) und HUD-Anzeigen haben ein prüfbares Maß.

## Beteiligte und Zielgruppen

Spieler am TV, auch Kinder und Sitzabstand ca. 2–3 m; 🧑 entscheidet (Beschluss Q03).

## Anforderungen

- Die Regel nennt je Layout (1, 2, 3, 4 Spieler) die Mindest-Schriftgröße in px und die Größe der Ansicht, auf die sie sich bezieht.
- Sie unterscheidet mindestens „Pflicht-Info“ (Gold, Warnungen) und „Nebeninfo“ (Hinweise).
- Sie nennt, wie geprüft wird (Rechnung aus Fontgröße × Skalierung des Layouts, Test oder Sichtprüfung am TV).

## Nicht-Ziele

Layout-Umbau und Kamera je Stufe (B-106), Schrift in HTML-Seiten außerhalb des Spiels.

## Regeln und Einschränkungen

Domäne REG. Auflösung `GAME_WIDTH × GAME_HEIGHT` aus `src/core/constants.ts`; der Client zeichnet nur (`CLAUDE.md`).

## Beispiele

Layout 4 Spieler: Pflicht-Info mindestens X px effektiv → ein 20-px-Text im Viertel mit Skalierung 0,5 liegt bei 10 px und besteht nicht.

## Ausnahme- und Fehlerfälle

Ein Text ist bei der Mindestgröße zu lang für das Viertel → die Regel nennt, ob gekürzt, umgebrochen oder ausgeblendet wird.

## Akzeptanzkriterien

- **AC-01** Abschnitt „Schriftgröße“ in `docs/rules/bedienung.md` enthält je Layout 1, 2, 3 und 4 Spieler eine Mindest-Schriftgröße in px für Pflicht-Info und Nebeninfo (Sichtprüfung der Tabelle).
- **AC-02** Der Abschnitt nennt das Prüfverfahren (Rechnung oder Test oder Sichtprüfung) in einem Satz; `task check` grün.

## Offene Fragen

Zahlen je Layout und Sitzabstand: `docs/fragenkatalog.md` Q03, entscheidet 🧑 (ggf. nach Blick auf den TV).

## Notizen

Aus Plan Lücke 5. Wird in S4 (Layouts mit Mindestschrift) umgesetzt.
