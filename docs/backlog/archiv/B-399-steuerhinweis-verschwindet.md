# B-399 · Der Steuerhinweis im HUD verschwindet nach einiger Zeit

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 2)

## Ausgangslage

Beobachtung 🧑 in der Gesamtabnahme U6.4, Prüfliste Punkt 6c (2026-10-11, PC, Split-Screen mit 2 Spielern): Die Tastenhinweise im HUD stören ab dem Split-Screen „extrem“ und sollen nach einiger Zeit verschwinden. Heute steht der Steuerhinweis dauerhaft (`HudScene.showHints`: Text `hud.hint.<Gerät>` im Element `controls`, Schrift `controlsHint`, 24 px) und konkurriert in den halben Zellen mit Spielerzeile und Aktionen-Overlay.

## Ziel

Der Steuerhinweis im HUD ist nur zu Beginn einer Sitzung zu sehen und blendet sich nach einiger Zeit aus. Nutzen: Im Split-Screen bleibt Platz für Spielerzeile, Radar und Aktionen-Overlay.

## Beteiligte und Zielgruppen

Spieler am PC und an der Xbox (höchstens 2); 🧑 nimmt ab und entscheidet die Dauer.

## Anforderungen

- Der Steuerhinweis (Element `controls`) wird 20 s nach dem Beitritt des Spielers ausgeblendet.
- Er erscheint wieder bei einem neuen Beitritt oder wenn 🧑 in den Optionen „Hinweise“ (`guideHints`) einschaltet.
- Das HUD-Layout (B-337) lässt den frei gewordenen Platz für die anderen Elemente.
- Gilt für 1 und 2 Spieler, je Zelle.

## Nicht-Ziele

Das Aktionen-Overlay (B-319, erledigt) und die geführten Hinweise (B-148); neue Hinweistexte.

## Regeln und Einschränkungen

CLI (`src/scenes/`), nur zeichnen (ADR 001); Zeitsteuerung als reine, getestete Funktion; Texte über `t()`; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Spieler 2 tritt bei → der Steuerhinweis steht 20 s, danach ist die Zelle frei für Spielerzeile und Radar.

## Ausnahme- und Fehlerfälle

Spieler wechselt das Gerät (Tastatur → Controller) → Hinweis erscheint für das neue Gerät erneut für die feste Zeit. Raumwechsel → keine neue Anzeige, solange derselbe Spieler im Raum bleibt.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Die Sichtbarkeitsfunktion liefert den Steuerhinweis für die feste Zeit seit dem Beitritt und danach nicht mehr; ein Gerätewechsel startet die Zeit neu.
- **AC-02** 🧑 sieht am PC im Split-Screen mit 2 Spielern, dass der Steuerhinweis nach einiger Zeit verschwindet (Beobachtung).

## Offene Fragen

keine. Entschieden 2026-10-14 (🧑, Chat): Dauer 20 s nach Beitritt; über „Hinweise“ (`guideHints`) in den Optionen wieder einschaltbar.

## Notizen

Herkunft: U6.4, Prüfliste Punkt 6c (2026-10-11). Verwandt: B-337 (HUD ohne Überlagerung), B-319.
