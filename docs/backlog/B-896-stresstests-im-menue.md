# B-896 · Das native Spiel hat im Menü einen Bereich Testing, in dem einstellbare Stresstests starten

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ST2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Auftrag im Chat; Defaults)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Phaser-Client hatte Stresstest-Seiten (Werkzeug-Seiten unter Performance). Mit dem Browser-Client (B-848) entfallen sie; das native Spiel hat keinen Ersatz. Simulationen und Balancing über `sim_test` sind gesperrt, ein manuell gestarteter Stresstest im Spiel ist davon nicht betroffen (Ralf startet ihn selbst).

## Ziel

Im Spielmenü gibt es den Bereich „Testing“. Dort stellt Ralf einen Stresstest ein und startet ihn; das Spiel läuft dann mit diesen Werten und zeigt die Leistungswerte.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Menüpunkt „Testing“ im Hauptmenü (nur Entwickler-Build oder per Einstellung sichtbar).
- Einstellbar: Anzahl Spieler (1–4, Rest als Bots), Insel und Stufe/Ebene, Seed, Gegnerzahl je Welle, Anstieg (linear/exponentiell, Faktor je Welle), Truppen/Bauten vorab, Zeitraffer, Dauer bzw. Abbruch nach n Wellen.
- Voreinstellungen (Presets) wie „viele Gegner“, „volle Burg“, „vier Spieler Split-Screen“; eigene Einstellungen merken.
- Während des Laufs: F2-Leistungsanzeige und F10-Overlay (B-888) an; am Ende ein Bericht nach `reports/` (Ticks/s, Taktdauer p50/p95/max, FPS, Objektzahlen, Speicher), lesbar über `report_read`.
- Werte der Presets in `data/`, nicht im Code.

## Nicht-Ziele

Automatische Testläufe über `sim_test` (gesperrt); Balancing-Auswertung.

## Regeln und Einschränkungen

Seiten navigieren nicht weg; alles im Spielmenü. Jede Mechanik mit mehreren Spielern.

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/game/**`, `data/stresstests.json`
- **Abhängig von:** B-888
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Das Menü zeigt „Testing“ mit allen genannten Einstellungen; Gamepad und Tastatur bedienen es.
- **AC-02** Ein gestarteter Test läuft mit den eingestellten Werten (Go-Test: Konfiguration → Raum/Welt-Optionen).
- **AC-03** Am Ende liegt ein Bericht in `reports/` mit den Leistungswerten (Test).
- **AC-04** Sichtprüfung durch 🧑.

## Offene Fragen

- Sichtbarkeit: immer oder nur mit Entwickler-Schalter? Default: per Einstellung „Entwicklermodus“, im Entwickler-Build an.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
