# B-943 · Die Grafik ruckelt trotz stabiler 60 FPS, auch auf schnellen Rechnern

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** FB4
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
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

Verworfen (B-1143, 2026-11-19, autonom, Claude): Das Ruckeln des Go-Spiels (Ebitengine, GC) war der Anlass der Rust-Umstellung (RU0, RU39); Bildzeit im Rust-Spiel messen RU42, RU44, RU45, Abnahme im Rust-Spiel RU36.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
