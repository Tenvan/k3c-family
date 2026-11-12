# B-698 · Balancing-Tests und Simulationen laufen vorläufig mit Cheats und bestehen spätestens im dritten Lauf

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** BAL
- **Erstellt:** 2026-11-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Biome werden noch überarbeitet; das Ressourcen-Balancing kommt zum Schluss (🧑 Ralf im Chat, 2026-11-05). Strenge Balancing-Tests und Simulationen scheitern deshalb an Werten, die sich ohnehin ändern (z. B. KB6.2: Wellenabstand, Adern-Zielrate).

## Ziel

Bis zu den finalen Tests genügt es, dass die Funktion gegeben ist. Die Strenge kommt mit dem Ressourcen-Balancing zurück.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet und nimmt ab; Sessions REG, SIM.

## Anforderungen

Vorläufige Regel für **alle** Tests und Simulationen, die ab jetzt angepasst werden sollen oder müssen:

- Sie laufen mit Ressourcen- und Gold-Cheats.
- Eine **Runde** ist ein kompletter Simulations- oder Balancing-Lauf (Entscheidung 🧑, 2026-11-05). Der Test muss spätestens im **dritten Lauf** bestehen; dafür dürfen Cheats und Werte-Hilfen nachgelegt werden.
- Geprüft wird die Funktion, nicht die Ressourcen-Werte.

Jeder gelockerte Test bekommt einen Verweis auf B-698, damit er am Ende auffindbar ist.

## Nicht-Ziele

Das eigentliche Ressourcen-Balancing (kommt zuletzt, eigenes Ticket im Projekt BAL).

## Regeln und Einschränkungen

Niedrigste Priorität im Projekt BAL. Testläufe nur über `sim_test`. Coop-Werte nur über Multiplikatoren.

## Beispiele

Simulation der Mine mit Gold-Cheat besteht im ersten bis dritten Lauf; Zielraten-Test prüft nur, dass Adern liefern.

## Ausnahme- und Fehlerfälle

nicht relevant: vorläufige Regel.

## Akzeptanzkriterien

- **AC-01** Jeder angepasste Test und jede Simulation nennt B-698 und läuft mit Cheats und besteht spätestens im dritten Lauf.
- **AC-02** Mit den finalen Tests entfernt 🧑 die Regel; die Lockerungen werden zurückgenommen (Suche nach `B-698`).

## Offene Fragen

keine (Runde = kompletter Lauf, entschieden)

## Notizen

Anlass: Ralf im Chat, 2026-11-05, im Zuge von KB6.2.

Verworfen 2026-11-12 (Entscheidung Ralf): Sim und Balancing gesperrt (Beschluss 2026-11-11); Wiederaufnahme mit neuen Tickets.
