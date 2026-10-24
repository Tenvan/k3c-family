# B-590 · Die Wachstumsreihe der Burg zeigt echte Unterschiede statt derselben Burg in Farben

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme GEB11.6 (Ralf, 2026-10-24, PC): Nur das Lagerfeuer sieht anders aus, danach erscheint immer dieselbe Burg in anderen Farben. Zelt, Hütte, Holzburg und die Steinburgen sind kaum unterscheidbar, Form und Größe passen nicht.

## Ziel

Jeder Schritt der Wachstumsreihe (Lagerfeuer, Zelt, Hütte, Holzburg, Steinburgen) hat eine eigene Form und Größe.

## Beteiligte und Zielgruppen

Spielende; 🧑 entscheidet über das Aussehen; Grafik über das Asset-Studio.

## Anforderungen

- Eigene Grafik je Schritt mit sichtbar wachsender Größe und Form, nicht nur Umfärbung.
- Die Reihe richtet sich nach der Stufenfolge aus B-592, sobald sie feststeht.

## Nicht-Ziele

Neue Ausbau-Regeln (B-592).

## Regeln und Einschränkungen

Grafik über das Studio, Regeln aus GEB (`docs/rules`), Gerüst bleibt (GEB11).

## Beispiele

Zelt → Hütte: kleineres, schlichtes Gebäude statt gleicher Burg in anderer Farbe.

## Ausnahme- und Fehlerfälle

nicht relevant, nur Darstellung.

## Akzeptanzkriterien

- **AC-01** Jeder Schritt der Reihe hat eine eigene, unterscheidbare Grafik in der Studio-Vorschau.
- **AC-02** 🧑 sieht im Spiel die Reihe als echtes Wachstum (Beobachtung am Gerät).

## Offene Fragen

Zahl der Schritte hängt an B-592 (2–3 je Inselstufe); zuerst dort entscheiden.

## Notizen

Aus GEB11.6. Spec: Entwurf. Vorgänger: B-458 (GEB11).
