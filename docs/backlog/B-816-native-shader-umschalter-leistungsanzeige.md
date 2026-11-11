# B-816 · Fluss-Shader und Kacheln lassen sich im Spiel umschalten, Bildrate und Rechenzeit sind im Bild sichtbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Fluss-Shader (NG11, B-790) lässt sich nur über die Umgebungsvariable `K3C_RIVER` gegen Kacheln umschalten. Bildrate und Rechenzeit sind nur im Messmodus sichtbar, nicht im Bild.

## Ziel

Ralf schaltet Kachel und Shader im laufenden Spiel um und sieht Bildrate und Rechenzeit im Bild.

## Beteiligte und Zielgruppen

Ralf (Sichtprüfung am PC und Mini-PC), Claude (Umsetzung).

## Anforderungen

- Umschalten per Taste oder Einstellung, Wert bleibt erhalten.
- Optionale Anzeige von Bildrate und Rechenzeit.
- Keine Kosten, wenn die Anzeige aus ist (keine Wanduhr, keine Allokation je Bild).

## Nicht-Ziele

Neue Shader-Effekte, Messläufe (gesperrt).

## Regeln und Einschränkungen

Entscheidung 008. Die Taste darf `View + Menu` und `B` nicht belegen. Die Verträge `view` und `world` ändern sich nur zusätzlich.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/view/**`, `native/k3c-game/world/**`, `native/k3c-game/game/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Taste drücken → Fluss wechselt von Kacheln zu Shader ohne Neustart.

## Ausnahme- und Fehlerfälle

Shader-Fehler → Rückfall auf Kacheln (wie B-790); Treiberfehler der GPU fällt laut B-790 › Notizen nicht darunter.

## Akzeptanzkriterien

- **AC-01** Test: Umschalten ändert den Zustand und zeichnet im jeweils anderen Modus.
- **AC-02** Test: Anzeige aus → keine Zeitnahme im Bildpfad.
- **AC-03** Test: Anzeige an → Bildrate und Rechenzeit stehen im Frame-Text.

## Offene Fragen

keine

## Notizen

Aus B-790 › Notizen (NG11, 2026-11-11).
