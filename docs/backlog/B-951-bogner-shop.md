# B-951 · Der Handwerkerbedarf stellt Hämmer her, ein neuer Bogner stellt Bögen her

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WI1
- **Projekt:** WIR
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Hinweis 🧑 2026-11-15: Der Handwerkerbedarf stellt heute Bögen o. ä. her (Ist-Stand ungeprüft). Mit B-950 braucht es Hämmer.

## Ziel

Der Handwerkerbedarf stellt Hämmer her; ein neuer Bogner-Shop stellt Bögen her (Gebäude in `data/buildings.json`, Grafik über `assets_*`).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`, `public/grafik/**`, `docs/rules/**`, `native/k3c-game/**`
- **Abhängig von:** B-950
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Handwerkerbedarf liefert Hämmer, Bogner liefert Bögen (Tests).
- **AC-02** Bogner baubar, Asset vorhanden, Hub-Plan/Regeltabelle angepasst.

## Offene Fragen

Ab welcher Hub-Stufe ist der Bogner baubar, was kostet er? Default: wie der bisherige Bogen-Laden. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
