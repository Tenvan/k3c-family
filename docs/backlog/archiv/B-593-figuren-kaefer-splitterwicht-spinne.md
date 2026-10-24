# B-593 · Eisenkäfer, Splitterwicht und Kristallspinne brauchen passende Figuren

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** GRA
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme GR7.3 (Ralf, 2026-10-24, PC im Asset-Studio): Der Eisenkäfer (`lpc-beetle`, Redshrike) sieht nicht wie ein Käfer aus; der Splitterwicht (`eye-rock`, Night Blade) wirkt weiter wie ein Pilz bzw. passt nicht. Die Kristallspinne konnte nicht geladen werden (OGA „Spider Attack, Idle, Shoot, Walk“, CC0, Seiten-URL unbekannt) und bleibt die GR8-Figur.

## Ziel

Eisenkäfer, Splitterwicht und Kristallspinne haben Figuren, die 🧑 als passend abnimmt.

## Beteiligte und Zielgruppen

Spielende; 🧑 wählt; Agent CLI baut ein (Download nur mit Freigabe).

## Anforderungen

- Neue Kandidaten je Gegner nach dem Verfahren aus B-193 (drei Kandidaten, Q70-Lizenzen, Credits). Für den Käfer z. B. `34-view-beetle` oder `beetle1/2/3` von Night Blade, für den Splitterwicht andere Figuren aus den Kandidatenseiten (GR7.3), für die Spinne die gesuchte CC0-Datei.
- Gewählte Figuren mit Credit unter `public/sprites/`, Zuordnung ohne Platzhalter.

## Nicht-Ziele

Skalierung am Gerät (GR7.4), Feuergeist (bleibt „kein Treffer“).

## Regeln und Einschränkungen

Lizenzregel Q70, Credits-Test, Downloads nur mit Freigabe von 🧑; `tools/spritegen/main.go` erzeugt noch `iron-beetle` und `crystal-spider` (Credits-Test wird bei erneutem Lauf rot): beim Einbau bereinigen.

## Beispiele

Käfer mit erkennbarer Käfer-Silhouette von der Seite.

## Ausnahme- und Fehlerfälle

Kein Treffer → GR8-Figur bleibt mit Vermerk.

## Akzeptanzkriterien

- **AC-01** Zu jedem der drei Gegner liegen drei neue Kandidaten vor, 🧑 hat gewählt (Beobachtung).
- **AC-02** Gewählte Figuren liegen mit Credit unter `public/sprites/`, `task check` grün.
- **AC-03** 🧑 nimmt die Figuren im Studio ab.

## Offene Fragen

keine

## Notizen

**Verworfen (🧑, Chat, 2026-10-24):** Die bisherigen Figuren sind ausreichend; Grafik wird künftig mit der Integration des AI-Chats anders und besser gelöst. Der Einbau `f63056f6` bleibt. Offener Rest, falls später gewünscht: Käfer, Splitterwicht, Kristallspinne neu wählen; `tools/spritegen/main.go` erzeugt noch `iron-beetle` und `crystal-spider`.
