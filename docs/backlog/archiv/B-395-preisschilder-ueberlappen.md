# B-395 · Nahe Bauplätze zeigen ihre Preisschilder nie übereinander

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S9
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 1

## Ausgangslage

Abnahme S9.5 am 2026-10-11 (🧑, PC, Tastatur, am Tag): Stehen mehrere Bauplätze nah beieinander, überlappen ihre Preisschilder („Taverne · 60 Stein“ und „Lager · 50 Stein“ liegen übereinander, „Turm · 50 Holz“ daneben). S9.1 hat nur den Hinweis des Aktionen-Overlays je Spieler auf einen reduziert; Preisschilder (`src/scenes/siteView.ts`) zeichnen weiter unabhängig voneinander und vom Overlay.

## Ziel

An jeder Weltposition steht höchstens ein Preisschild; nahe Bauplätze überdecken einander nicht. Nutzen: Preise bleiben lesbar, B-319/AC-04 ist erfüllbar.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei der Abnahme am PC.

## Anforderungen

- Von den Preisschildern in Reichweite zeigt jeder Spieler nur das **nächste**; die übrigen entfallen (Beschluss 🧑 2026-10-11, Chat).
- Das Preisschild bleibt vor dem Aktionen-Hinweis (B-319/AC-02).
- Auswahl als reine, getestete Funktion; `src/scenes` rechnet nichts.
- Gilt je Bildschirmzelle, auch mit 2 Spielern: Ein Schild steht, sobald es für einen der Spieler das nächste ist.

## Nicht-Ziele

Neue Preisinhalte; Taste im Preisschild (B-374).

## Regeln und Einschränkungen

Regel „Ein Element je Weltposition“ (`docs/rules/monarch.md` § 4, Glossar › Weltposition); CLI; Schriftgrößen aus `FONTS`; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Taverne und Lager stehen 3 Units auseinander, der Spieler steht davor → die Preisschilder stehen lesbar neben- oder untereinander, nie übereinander.

## Ausnahme- und Fehlerfälle

Zwei Bauplätze gleich weit entfernt → der mit der kleineren Nummer. Steht kein Schild in Reichweite, bleibt es leer. Ein gefallener Monarch (`respawnIn > 0`) bestimmt kein Schild.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Stehen mehrere Preisschilder in Reichweite, liefert die Auswahl je Spieler nur das nächste; bei Gleichstand das mit der kleineren Nummer.
- **AC-02** 🧑 sieht am PC an nahen Bauplätzen nie zwei Preisschilder übereinander (Beobachtung).

## Offene Fragen

Entschieden 2026-10-11 (🧑, Chat): nur das nächste Preisschild zeigen, die übrigen entfallen. Sonst keine.

## Notizen

Anlass: S9.5 (Abnahme B-319/AC-04, 2026-10-11), Screenshot mit Taverne/Lager/Turm. Verwandt: B-319, B-374.
