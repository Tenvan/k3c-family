# B-609 · Die Balancing-Konfiguration erklärt jedes Eingabefeld

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24) nach dem ersten Blick auf die Seite „Balancing“ der Workbench (WZ28.5, WZ28.6): Unklar, was in der Konfiguration erwartet wird (Seeds, Tiefe, Spieler, Bot, Grad, Tage, Varianten, Abstimmen mit Parametern, Schritten, Budget).

## Ziel

Jedes Eingabefeld der Balancing-Konfiguration sagt, was es bedeutet, welche Werte erlaubt sind und was es bewirkt; Beispiele zeigen typische Läufe.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

- Je Feld ein kurzer Hilfetext (Bedeutung, Wertebereich, Vorgabe, Wirkung auf den Lauf) und ein Hilfe-Knopf, der die passende Beschreibung als Popup zeigt.
- Varianten und Abstimm-Parameter: Format `datei.json:pfad=wert` bzw. `=min..max` mit Beispielen aus den echten Dateien, Hilfe zur Auswahl eines Pfads (Liste der Zahlenwerte der gewählten Datei).
- Bereich „Beispiele“ mit typischen Läufen (z. B. 20 Seeds Wald, Startgold variieren, Mauerkosten abstimmen), per Klick in die Konfiguration übernehmbar.
- Fehlertexte der Prüfung nennen das erwartete Format.

## Nicht-Ziele

Neue Balancing-Regeln; Änderung der Laufsteuerung.

## Regeln und Einschränkungen

Atomic Design über `@webkit`; Texte deutsch; die Beschreibungen der Dateien kommen aus B-608 (ein Text, nicht doppelt).

## Beispiele

Feld „Tiefe“ → „0 = Wald (Oberfläche), 1 = Höhle …; leer = Tiefe des Ziels“; Klick auf „?“ bei Varianten → Popup mit Format und Beispielen.

## Ausnahme- und Fehlerfälle

Beschreibung fehlt → Popup sagt das und nennt die Datei.

## Akzeptanzkriterien

- **AC-01** Jedes Feld hat Hilfetext und Hilfe-Knopf (Vitest über alle Felder).
- **AC-02** Beispiele lassen sich übernehmen (Vitest).
- **AC-03** Ralf versteht die Konfiguration ohne Nachfrage (Abnahme WZ30.7).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
