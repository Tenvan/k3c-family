# B-442 · Hintergrundmauern verbinden die Mauern einer Seite wie in Kingdom Two Crowns

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB3
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (Chat, 2026-10-16): Es gibt zwei Arten von Mauern.
- **Typ A** (Mauer, Vordergrund): Begrenzung und Schutz des Hubs am Mauer-Platz der Mauerlinie, wie heute (`wall`, Stufen 1–5, GEB3).
- **Typ B** (Hintergrundmauer): zieht sich im Hintergrund von einer Typ-A-Mauer zur nächsten Typ-A-Mauer derselben Seite. Ihre Ausbaustufe ist die der **äußeren** angrenzenden Typ-A-Mauer, sie reicht von dort nach innen bis zur nächsten Typ-A-Mauer. Genau wie in Kingdom Two Crowns.
Heute gibt es nur Typ A; zwischen den Mauerlinien ist der Hintergrund leer.

## Ziel

Zwischen zwei benachbarten gebauten Mauern (Typ A) einer Seite zeichnet der Client eine durchgehende Hintergrundmauer (Typ B) in der Stufe der äußeren der beiden Mauern; Grafik je Stufe 1–5 aus `tools/gebaeude` (gleiche Farbstufen wie die Mauer), kachelbar in der Breite.

Offen für die Planung (🧑): Glossar-Begriffe („Mauer“/„Hintergrundmauer“), ob die innerste Linie zur Burg hin ebenfalls eine Hintergrundmauer bekommt, ob Typ B nur Optik ist (Vermutung: ja, keine Sim-Änderung), Höhe und Tiefe der Hintergrundmauer, Einordnung (GEB3 Revision 2, GEB4 oder eigener Sprint).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Hintergrundmauer liegt in den Stufen 1–5 als Daten-JSON mit `stufen` und PNG vor (gleiche Farbstufen wie die Mauer, in der Breite kachelbar).
- **AC-02** Der Client zeichnet zwischen je zwei benachbarten gebauten Mauern derselben Seite eine durchgehende Hintergrundmauer hinter den Bauplätzen, in der Stufe der äußeren Mauer; Vitest prüft die Abschnitte (Lage, Stufe, Lücken bei ungebauten Mauern).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
