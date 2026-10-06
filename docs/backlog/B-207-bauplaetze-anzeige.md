# B-207 · Der Client zeigt freie und gesperrte Bauplätze mit Grund (ab Hub-Stufe n, Linie fehlt)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W8
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Mit festen Bauplätzen (Q43, B-206) gibt es Plätze, die noch gesperrt sind: Linie k braucht Hub-Stufe k und die Linie k−1 derselben Seite (Q48), ein Tor ist nur an der äußersten gebauten Linie bezahlbar (Q47), Gebäude schalten mit der Hub-Stufe frei. Der Client (`src/scenes/worldRenderer.ts`) zeichnet heute nur gebaute Plätze und Zahlziele. Der früher geplante Hinweis „Kein Platz für <Gebäude>“ (Q26) entfällt, weil kein Platz fehlen kann.

## Ziel

Spieler sehen am Platz, ob er frei ist und was ihnen noch fehlt, statt an einem stummen Platz zu stehen.

## Beteiligte und Zielgruppen

Spieler (auch Kinder, 2+ Spieler im Split-Screen); Entwickler CLI.

## Anforderungen

- Freie Plätze sind erkennbar (Zahlziel sichtbar).
- Gesperrte Plätze zeigen den Grund: „ab Hub-Stufe n“ oder „Linie fehlt“ (Linie k−1 derselben Seite nicht gebaut); ein innerer Tor-Platz zeigt, dass nur an der äußersten Linie gebaut wird.
- Text nach Q03 (Mindestgröße, kurz statt klein), deutsch und zentral für B-172.
- Liest Plätze und Stufen aus dem Protokoll (B-208) bzw. den Daten (B-209), keine eigene Regel-Logik.

## Nicht-Ziele

Layout und Freischaltung selbst (B-206, SIM); Wartezeit „bezahlt bis gebaut“ (B-117); Grafiken (B-010).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3; Client zeichnet nur (Entscheidung 001), kein `Math.random()`. Controller-Taste B bleibt frei, kein Bau-Menü (Q34). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Hub-Stufe 1, Spieler steht am Platz der Linie 2 → Hinweis „ab Hub-Stufe 2“. Hub-Stufe 2, rechts fehlt Linie 1 → am Platz der Linie 2 rechts „Linie fehlt“.

## Ausnahme- und Fehlerfälle

Zwei Spieler an verschiedenen gesperrten Plätzen → jede Bildschirmhälfte zeigt ihren Grund. Protokoll ohne Platzdaten (alter Server) → keine Anzeige, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Eine reine Funktion bildet Platz, Hub-Stufe und gebaute Linien auf „frei“, „ab Hub-Stufe n“ oder „Linie fehlt“ ab.
- **AC-02** Gesperrte und freie Plätze werden in der Welt angezeigt; `task check` grün.
- **AC-03** 🧑 hat die Anzeige am Gerät abgenommen.

## Offene Fragen

Sprint (vermutet W6, Anzeige der Wirtschaft) legt die Planung fest.

## Notizen

Ersetzt den verworfenen Hinweis „Kein Platz für <Gebäude>“ aus Q26 (Beschluss Q43, 2026-10-04). Abhängig von B-206 und B-208.
