# B-943 · Die Grafik ruckelt trotz stabiler 60 FPS, auch auf schnellen Rechnern

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** FB4
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Die Grafik ruckelt sichtbar, auch auf schnellem Rechner, obwohl die FPS stabil bei 60 liegen. Vermutet (ungeprüft): ungleichmäßige Sim-Takte je Frame (Update-Takt gegen Bildfrequenz), GC-Pausen, Laden/Dekodieren im Frame oder blockierende Aufrufe (Speichern, Audio) im Update.

## Ziel

Flüssige Bewegung ohne Mikroruckler bei 60 Hz.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ursache gemessen (Frame-Zeiten, Ticks je Frame, GC, Profil) und benannt.
- **AC-02** Behoben; Messung zeigt gleichmäßige Frame- und Tick-Abstände.
- **AC-03** Sichtprüfung am TV durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
