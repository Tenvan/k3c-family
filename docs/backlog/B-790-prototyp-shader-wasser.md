# B-790 · Ein vierter Prototyp zeigt, was Shader in Ebitengine für Wasser und Papier-Effekte bringen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Prototyp nach B-789 zeichnet den Fluss als durchlaufende Kacheln. Ebitengine kennt eigene Shader (Kage); im Projekt gibt es noch keinen.

## Ziel

Ein Prototyp zeigt an Bild und Messwerten, ob Shader für den Fluss (Wellen, Spiegelung, Verzerrung) und weitere Effekte ins native Spiel gehören.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung, Entscheidung), Claude (Bau, Messung).

## Anforderungen

- Fluss mit Shader statt reiner Kachel-Verschiebung, umschaltbar gegen die Kachel-Fassung.
- Messung Bildrate und Rechenzeit, auch mit integrierter Grafik.

## Nicht-Ziele

Shader im Browser-Client; neue Spiel-Logik.

## Regeln und Einschränkungen

Entscheidung 008. Draw-Call-Batching darf nicht brechen (ein Shader je Schicht, nicht je Kachel).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-ebiten/**`
- **Abhängig von:** B-789
- **Braucht 🧑:** Freigabe
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

Welche weiteren Effekte sollen Shader prüfen (Papierstruktur, Licht bei Nacht, Fackeln)? Entscheidet 🧑.

## Notizen

Angelegt 2026-11-11 aus dem Chat mit 🧑 („für den nächsten Prototyp könnte schon die Verwendung von Shadern eingeplant werden“).
