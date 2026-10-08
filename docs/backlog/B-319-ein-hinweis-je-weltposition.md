# B-319 · Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** S9
- **Projekt:** BED
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme S3.4 am 2026-10-06 (🧑, PC, Tastatur): „Generell überlappen sich die Hinweistexte und werden mehrfach angezeigt. Es sollte immer nur das nächste angezeigt werden, niemals mehrere gleichzeitig. Und die Texte sind zu groß.“ Betroffen: Bauplatz-Preisschild, Burg, am Spieler. Heute zeigt `src/scenes/actionOverlay.ts` eine Hauptzeile (Schrift `playerValue`) und eine zweite Zeile mit allen weiteren Aktionen (`controlsHint`); Preisschilder (`src/scenes/siteView.ts`) zeichnen unabhängig davon.

## Ziel

An jeder Weltposition steht höchstens ein Anzeige-Element; jeder Spieler sieht genau einen Hinweis, den des nächsten Ziels, in Nebeninfo-Größe. Nutzen: lesbar am TV und PC, keine Überdeckung.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei Abnahmen.

## Anforderungen

- Regel „Ein Element je Weltposition“ (`docs/rules/monarch.md` § 4, Glossar › Weltposition).
- Je Spieler nur der Hinweis des nächsten Ziels; keine zweite Zeile.
- Am Bauplatz trägt das Preisschild die Aktion, kein zusätzlicher Hinweis darüber.
- Schrift Nebeninfo nach `docs/rules/bedienung.md` § 2: 24 px bei 1–2 Spielern, 20 px im Viertel.
- 2 Spieler nahe beieinander: je Weltposition trotzdem nur ein Element (Vorschlag: der Hinweis des näheren Spielers, der andere entfällt).

## Nicht-Ziele

Neue Aktionen; Glyphen (S6).

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; Auswahl als reine, getestete Funktion; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Spieler steht an unbezahltem Bauplatz neben der Burg → nur das Preisschild mit „Leertaste halten“, kein Burg-Hinweis. Spieler allein an der Burg am Tag → genau ein Hinweis (z. B. Skill lernen) in 24 px.

## Ausnahme- und Fehlerfälle

Zwei Ziele gleich weit entfernt → das wichtigere nach Gewicht (`weight` in `actionHints.ts`).

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Die Auswahl liefert je Spieler höchstens einen Hinweis, den des nächsten Ziels.
- **AC-02** Ein Test belegt: Steht ein Spieler im Bereich eines Preisschilds, liefert das Overlay keinen zusätzlichen Hinweis.
- **AC-03** Ein Test belegt: Schriftgröße des Hinweises ist die Nebeninfo-Größe aus `bedienung.md` § 2.
- **AC-04** 🧑 sieht am PC an Bauplatz, Burg und neben dem Spieler nie zwei Texte übereinander (Beobachtung).

## Offene Fragen

- Entschieden 2026-10-06 (🧑, Chat): „Ein Element je Weltposition“ gilt je Bildschirmzelle; jeder Spieler sieht in seiner Zelle nur seinen eigenen Hinweis.

## Notizen

Anlass: S3.4, Ergebnis 2026-10-06; Hinweis aus S3.3 („zweite Zeile kann das Preisschild überlappen“).
