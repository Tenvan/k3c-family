# B-790 · Ein vierter Prototyp zeigt, was Shader in Ebitengine für Wasser und Papier-Effekte bringen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG11
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Prototyp nach B-789 zeichnet den Fluss als durchlaufende Kacheln; das native Spiel übernimmt das in `native/k3c-game/world` (B-795, NG2). Ebitengine kennt eigene Shader (Kage); im Projekt gibt es noch keinen.

## Ziel

Ein Prototyp zeigt an Bild und Messwerten, ob Shader für den Fluss (Wellen, Spiegelung, Verzerrung) und weitere Effekte ins native Spiel gehören.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung, Entscheidung), Claude (Bau, Messung).

## Anforderungen

- Fluss mit Shader statt reiner Kachel-Verschiebung im nativen Spiel (`native/k3c-game/world/water/`), umschaltbar gegen die Kachel-Fassung.
- Messung Bildrate und Rechenzeit, auch mit integrierter Grafik.

## Nicht-Ziele

Shader im Browser-Client; neue Spiel-Logik.

## Regeln und Einschränkungen

Entscheidung 008. Draw-Call-Batching darf nicht brechen (ein Shader je Schicht, nicht je Kachel). Gebaut wird im nativen Spiel statt im Prototyp `native/k3c-ebiten` (Revision 2, Planung 2026-11-11), damit die Arbeit bleibt; Shader-Quelle unter `world/water/`, Umschalter am Fluss in `world/`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/world/**`
- **Abhängig von:** NG2
- **Braucht 🧑:** Freigabe, Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Taste schaltet den Fluss zwischen Kacheln und Shader um; die Anzeige nennt Bildrate und Rechenzeit beider Fassungen.

## Ausnahme- und Fehlerfälle

Shader übersetzt auf dem Zielgerät nicht → Kachel-Fassung läuft weiter, Meldung im Log.

## Akzeptanzkriterien

- **AC-01** Fluss mit Shader im Fenster, umschaltbar, Screenshot beider Fassungen.
- **AC-02** Messbericht beider Fassungen vom Entwicklungs-PC.
- **AC-03** Entscheidung 🧑 in den Notizen.

## Offene Fragen

keine.

Entschieden von 🧑 am 2026-11-11 (Chat): Gebaut wird im nativen Spiel (nicht im Prototyp), und nur der Fluss; weitere Effekte (Papierstruktur, Licht bei Nacht, Fackeln) werden eigene Tickets nach der Entscheidung.

## Notizen

Angelegt 2026-11-11 aus dem Chat mit 🧑 („für den nächsten Prototyp könnte schon die Verwendung von Shadern eingeplant werden“). Umgesetzt in NG11 (2026-11-11).

### Messung Entwicklungs-PC (NG11.2)

Intel Core Ultra 7 165H, integrierte Intel Arc, 1920×1080, VSync aus, zwei Zellen, 120 Bilder Aufwärmen, 3000 gemessen, je Fassung ein Lauf.

| Fassung | FPS Mittel | 1-%-Low | p99 Bildzeit | Fluss-Draw (CPU) je Bild |
|---|---|---|---|---|
| Kacheln | 2141 | 82 | 3,52 ms | 0,043 ms |
| Shader | 2573 | 193 | 3,75 ms | 0,006 ms |

Umschalten nur über `K3C_RIVER=tiles|shader`; eine Taste und die Anzeige im Bild fehlen im Vertrag. Treiber-Fehler beim Übersetzen auf der GPU fängt der Rückfall vermutlich nicht ab.
