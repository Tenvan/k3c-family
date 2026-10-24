# GEB17 · CLI, REG, SIM, DEV · Hub-Ausbau: Angebot vor der Burg, Ausbaustufen je Inselstufe, unterscheidbare Wachstumsreihe

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** CLI, REG, SIM, DEV
- **Reife:** Entwurf
- **Tickets:** B-591, B-592, B-590
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme GEB11.6 (Ralf, 2026-10-24, PC): Vor der Burg erscheint kein Ausbau-Angebot (B-591), fünf Hub-Stufen sind zu wenig (B-592), und die Wachstumsreihe zeigt nach dem Lagerfeuer immer dieselbe Burg in anderen Farben (B-590).

## Ziel

Der Hub wächst sichtbar und spielbar: Vor der Burg steht das Ausbau-Angebot, jede Inselstufe hat ihre Ausbauschritte mit Material-Tor zur nächsten Inselstufe, und jeder Schritt hat eine eigene Grafik. Am Ende sichtbar: Spieler steht vor der Burg → „Ausbauen“ mit Kosten; nach dem Ausbau eine sichtbar größere, andere Burg; der letzte Schritt einer Inselstufe wartet auf „Material der nächsten Insel“.

## Beteiligte und Zielgruppen

Spielende; Agenten CLI, REG, SIM, SRV, DEV; 🧑 entscheidet Zahl der Schritte, Kosten und Material im Workshop und nimmt am Gerät ab.

## Anforderungen

B-591 › Anforderungen, B-592 › Anforderungen, B-590 › Anforderungen.

## Nicht-Ziele

Balancing der Kosten über den Workshop hinaus (eigener Sprint); neue Gebäude außerhalb der Wachstumsreihe; Grafik anderer Gebäude (GEB15, GEB16).

## Regeln und Einschränkungen

B-591 zuerst, unabhängig von der neuen Stufenfolge (Entscheidung Ralf, 2026-10-24). Die Stufenfolge (B-592) entscheidet 🧑 im Workshop GEB17.2, erst danach sind Spec und die Sessions ab GEB17.3 freigabefähig. Hebt D1 A (B-538) zugunsten Option C (`hubStep`) auf; Protokoll- und Spielstand-Änderung nur nach der Entscheidung von 🧑, additiv; Golden nur mit Begründung; keine Rücksicht auf alte Spielstände. Client zeigt nur, was der Server meldet; 2 Spieler; Grafik über das Asset-Studio.

## Beispiele

Spieler steht vor der Burg, Ausbau frei → Preisschild „Ausbauen“ mit Gold und Material. Inselstufe 1: Lager → Zelt → Hütte; Hütte braucht Material der Inselstufe 2 → Wartegrund „Material der nächsten Insel“.

## Ausnahme- und Fehlerfälle

Ausbau läuft schon → Wartegrund statt Angebot (B-591). Material der nächsten Inselstufe fehlt → Wartegrund „Material der nächsten Insel“ (B-592).

## Akzeptanzkriterien

- **AC-01** Vor der Burg erscheint das Ausbau-Angebot (Test der Anzeige-Regel und Beobachtung am Gerät) (B-591/AC-01).
- **AC-02** Regeldatei `docs/rules/stufen.md` beschreibt die Stufenfolge mit den Schritten je Inselstufe und dem Material-Tor, 🧑 bestätigt (B-592/AC-01).
- **AC-03** Daten, Simulation und Protokoll setzen sie um (Go-Tests, Golden mit Begründung) (B-592/AC-02).
- **AC-04** Der Client zeigt Schritt und Wartegrund (B-592/AC-03).
- **AC-05** Jeder Schritt der Reihe hat eine eigene, unterscheidbare Grafik in der Studio-Vorschau (B-590/AC-01).
- **AC-06** 🧑 sieht im Spiel Angebot und Reihe als echtes Wachstum (B-590/AC-02, B-591/AC-01 am Gerät).

## Offene Fragen

Zahl der Schritte je Inselstufe (2 oder 3), Kosten und Bauzeit je Schritt, Zahl der Inselstufen, Verhältnis zu den bisherigen 5 Hub-Stufen: 🧑 im Workshop GEB17.2 (blockiert die Freigabe der Sessions ab GEB17.3). Ursache von B-591 (Client oder Server) klärt GEB17.1 per `room_snapshot`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB17.1 | `GEB17.1-ausbau-angebot-burg.md` | Umsetzung | autonom | offen |
| GEB17.2 | `GEB17.2-workshop-stufenfolge.md` | Workshop | Mensch | offen |
| GEB17.3 | `GEB17.3-regeln-stufenfolge.md` | Umsetzung | autonom | offen |
| GEB17.4 | `GEB17.4-sim-protokoll-hubstep.md` | Umsetzung | autonom | offen |
| GEB17.5 | `GEB17.5-client-schritt-wartegrund.md` | Umsetzung | autonom | offen |
| GEB17.6 | `GEB17.6-grafik-wachstumsreihe.md` | Umsetzung | autonom | offen |
| GEB17.7 | `GEB17.7-review.md` | Review | autonom | offen |
| GEB17.8 | `GEB17.8-abnahme-geraet.md` | Workshop | Mensch | offen |

- GEB17.1 (CLI, ggf. SRV) Ausbau-Angebot vor der Burg: Ursache per `room_snapshot` nachstellen (`siteView.ts`, `priceTagRules.ts`, `hubSite` oder `hubUpgrade`), beheben, Test der Anzeige-Regel (AC-01).
- GEB17.2 (Mensch) Workshop Stufenfolge: Schritte je Inselstufe, Kosten, Bauzeit, Material-Tor (AC-02).
- GEB17.3 (REG) Regeldatei `docs/rules/stufen.md` nach dem Workshop (AC-02).
- GEB17.4 (SIM, SRV) `data/hub.json`, Simulation und Protokoll (`hubStep`), Go-Tests, Golden (AC-03).
- GEB17.5 (CLI) Client zeigt Schritt und Wartegrund (AC-04).
- GEB17.6 (DEV) Eigene Grafik je Schritt über das Studio (AC-05).
- GEB17.7 Review: alle Kriterien.
- GEB17.8 (Mensch) Abnahme am Gerät (AC-06).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
