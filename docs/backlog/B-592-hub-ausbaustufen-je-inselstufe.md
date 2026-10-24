# B-592 · Der Hub hat zwei bis drei Ausbaustufen je Inselstufe, weiter mit dem Material der nächsten Inselstufe

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB17
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Hub hat fünf Ausbaustufen (`data/hub.json › levels`); die Wachstumsreihe der Burg ist nur Grafik (B-538, Entscheidung D1 A am 2026-10-23). Ralf (2026-10-24, GEB11.6): Fünf Stufen sind zu wenig.

## Ziel

Jede Inselstufe hat zwei bis drei Ausbaustufen des Hubs; ein weiterer Ausbau braucht das Material der nächsten Inselstufe.

## Beteiligte und Zielgruppen

Spielende; 🧑 legt Zahl, Kosten und Material fest; Agent REG (Regeln), SIM, SRV (Protokoll), CLI.

## Anforderungen

- Stufenfolge des Hubs mit 2–3 Schritten je Inselstufe (Gebäude, Kosten, Bauzeit in `data/hub.json`).
- Die letzte Stufe einer Inselstufe ist erst mit dem Material der nächsten Inselstufe baubar.
- Regeln in `docs/rules/stufen.md`; Protokoll ggf. mit Feld für den Ausbauschritt (siehe B-538, Option C: `hubStep`).

## Nicht-Ziele

Grafik der Reihe (B-590); Balancing (eigener Sprint).

## Regeln und Einschränkungen

Hebt D1 A (B-538) für diese Entscheidung auf; Protokoll- und Spielstand-Änderung nur mit Entscheidung von 🧑; Golden mit Begründung; keine Rücksicht auf alte Spielstände.

## Beispiele

Inselstufe 1 → Schritte Lager, Zelt, Hütte; Inselstufe 2 → Holzburg, Steinburg …

## Ausnahme- und Fehlerfälle

Material der nächsten Inselstufe fehlt → Wartegrund „Material der nächsten Insel“.

## Akzeptanzkriterien

- **AC-01** Regeldatei beschreibt die Stufenfolge mit 2–3 Schritten je Inselstufe und dem Material-Tor (🧑 bestätigt).
- **AC-02** Daten, Simulation und Protokoll setzen sie um (Go-Tests, Golden).
- **AC-03** Der Client zeigt Schritt und Wartegrund.

## Offene Fragen

Wie viele Schritte genau (2 oder 3), Kosten je Schritt, Zahl der Inselstufen, Verhältnis zu den bisherigen 5 Hub-Stufen: Workshop mit 🧑 vor der Spec. Erledigt D1 A zugunsten Option C.

## Notizen

Aus GEB11.6 (Ralf, 2026-10-24). Spec: Entwurf. Zusammenhang: B-538, B-590.
