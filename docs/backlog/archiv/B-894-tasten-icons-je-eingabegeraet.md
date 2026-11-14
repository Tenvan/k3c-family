# B-894 · Die Oberfläche zeigt Tasten-Hinweise als Icon des Eingabegeräts des jeweiligen Spielers (Tastenkappe oder Xbox-Taste)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** EP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Tasten-Hinweise im Spiel (Menü, Hinweise, F1-Übersicht aus B-888) zeigen heute Text oder nur Xbox-Symbole. Mit Eingabe-Profilen je Spieler (B-893) muss jeder Hinweis zum Gerät des Spielers passen.

## Ziel

Jeder Tasten-Hinweis zeigt das Icon der Taste im Gerät des jeweiligen Spielers: Tastenkappe (z. B. [E], [Leer], [←]) für Tastatur, Xbox-Taste (A/B/X/Y, LB/RB, LT/RT, Steuerkreuz) für Gamepad.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Eine zentrale Funktion Aktion + Profil → Icon; alle Hinweise gehen darüber.
- Icons als Papierschnitt-Assets (Gruppe icons) im gleichen Stil; Tastenkappen mit Beschriftung generiert, Xbox-Tasten in Xbox-Farben (nur Xbox-Symbole für jedes Pad, Plattform-Regel).
- Im Split-Screen zeigt jede Hälfte die Icons ihres Spielers; die F1-Übersicht (UI-Tasten) zeigt Tastenkappen.
- Fehlende Icons als PAP-Ticket.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Nur Xbox-Symbole für jedes Pad; Icons nur über die MCP-Tools `assets_*`.

- **Bereich:** `native/k3c-game/view/**`, `native/k3c-game/input/**`, `public/grafik/icons/**`
- **Abhängig von:** B-893
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Für jede Aktion und jedes Profil gibt es ein Icon (Test).
- **AC-02** Ein Wechsel des Profils ändert die Hinweise sofort.
- **AC-03** Sichtprüfung durch 🧑 (TV und PC).

## Offene Fragen

- Abhängig von B-893 (Profile) und B-888 (F1-Übersicht).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
