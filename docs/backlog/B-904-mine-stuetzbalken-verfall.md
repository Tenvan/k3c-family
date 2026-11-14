# B-904 · Die Mine zeigt Stützbalken und Verfall nach der Natur-Eigenschaft decay

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Mine hat seit B-891 die Natur-Eigenschaft `decay` (Stützbalken und Verfall 0–3, nur Darstellung).

## Ziel

Das native Spiel zeigt Stützbalken intakt bis verfallen nach dem Wert der Stufe; fehlende Assets als PAP-Ticket.

## Beteiligte und Zielgruppen

Spieler der Mine; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- Biome mit `decay` zeichnen alle 8 Units einen Stützbalken-Rahmen (zwei Pfosten, Querbalken, 4 Units hoch) in Holzfarbe, nicht im Bereich ±10 Units um die Burg.
- Ein Rahmen ist verfallen, wenn `mix(x) % 4 < Wert`: ein Pfosten steht schief, der Querbalken hängt schräg herab, darunter liegt Geröll (kleine Rechtecke).
- Wert 0 = alle heil, Wert 3 = etwa drei Viertel verfallen; die verfallenen bei kleinerem Wert bleiben bei größerem verfallen.

## Nicht-Ziele

Einsturz als Spielregel; Papierschnitt-Asset der Balken.

## Regeln und Einschränkungen

Wie B-902.

- **Bereich:** `native/k3c-game/props/decor_glow.go`, `native/k3c-game/props/decor.go`, `native/k3c-game/props/decor_test.go`
- **Abhängig von:** B-891
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Mine mit Wert 0 → nur gerade Rahmen; mit 3 → die meisten schief mit Geröll.

## Ausnahme- und Fehlerfälle

Biom ohne `decay` → keine Balken.

## Akzeptanzkriterien

- **AC-01** In der Mine plant props Stützbalken; Wert 0 plant keinen verfallenen Rahmen, Biome ohne `decay` keine Balken (Unit-Test).
- **AC-02** Die Zahl verfallener Rahmen im selben Ausschnitt steigt streng mit dem Wert (1 < 2 < 3) (Unit-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
