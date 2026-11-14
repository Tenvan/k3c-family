# B-908 · Die Präsentationsseite zeigt das Spiel als Bausatz: Einzelspieler und Couch-Koop, nativ, frei für nicht-kommerzielle Nutzung, eigene Welten mit dem MCP-Server

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** GP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 („Übernimm den überarbeiteten Entwurf“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Präsentationsseite `site/` zeigte nach WR1 Papierschnitt-Bilder, aber noch das alte Layout. Ralf wollte einen auffälligeren Auftritt (drei impeccable-Entwürfe, gewählt: „Bausatz“) mit Fokus auf Einzelspieler/Couch-Koop, nativem Client in Go und dem Bau eigener Welten mit dem MCP-Server; Lizenz bleibt PolyForm Noncommercial (Ralf, 2026-11-14).

## Ziel

`site/` ist der Bausatz-Entwurf: zwei Bögen (Spiel und eigene Welten), Lizenz-Abschnitt passend zu PolyForm Noncommercial, ohne Script und ohne Fremdadressen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Nirgends „Open Source“; Text: Quellcode offen, frei für nicht-kommerzielle Nutzung (spielen, ändern, weitergeben, eigene Welten bauen); Verkauf und kommerzielle Nutzung nicht erlaubt.
- Links auf `LICENSE`, `LICENSE-ASSETS` und den Lizenztext bei PolyForm.
- Drehscheibe Tag/Nacht ohne Script (CSS), Schriften lokal mit Lizenzhinweis.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Pages lädt nur `site/` hoch, rein statisch (B-183, B-507).

- **Bereich:** `site/**`, `tests/pagesSite.test.ts`
- **Abhängig von:** B-849
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `tests/pagesSite.test.ts` grün: kein Script, keine Fremdeinbindung, jede lokale Referenz existiert.
- **AC-02** Kein „Open Source“ im Text; Links auf LICENSE, LICENSE-ASSETS und PolyForm liefern 200.
- **AC-03** Desktop und Handy (375 px) ohne Querscrollen, alle Bilder laden, Drehscheibe schaltet.
- **AC-04** Sichtprüfung durch 🧑.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
