# Humanizer-Beispiele (Deutsch)

Vollständige Vor/Nach-Beispiele für den Skill `humanizer`. Wird optional bei längeren Inputs als bundled Reference geladen, wenn der Workflow ein Vollbeispiel zur Orientierung braucht.

---

## Beispiel 1 — KI-lastiger Essay-Anfang

**Eingabe (KI-lastig):**

> Großartige Frage! Hier ist ein Essay zu diesem Thema. Ich hoffe, das hilft!
>
> KI-gestützte Programmierung ist ein beständiges Zeugnis für das transformative Potenzial großer Sprachmodelle und markiert einen entscheidenden Wendepunkt in der Evolution der Softwareentwicklung. In der heutigen sich rasant entwickelnden technologischen Landschaft prägen diese bahnbrechenden Werkzeuge—angesiedelt an der Schnittstelle von Forschung und Praxis—die Art und Weise, wie Engineers ideieren, iterieren und ausliefern, und unterstreichen damit ihre entscheidende Rolle in modernen Workflows.

**Entwurf:**

> KI-Coding-Assistenten beschleunigen einige Aufgaben. In einer Studie von 2024 schlossen Entwickler mit Codex einfache Funktionen 55 Prozent schneller ab als die Kontrollgruppe — beim Debugging und bei Architekturentscheidungen gab es keinen Unterschied.
>
> Die Werkzeuge sind gut bei Boilerplate: Konfigurationsdateien, Test-Gerüsten, repetitiven Refactorings. Sie sind schlecht darin zu erkennen, wann sie falsch liegen.

**KI-Audit:**

Was noch nach KI klingt:
- Rhythmus noch zu sauber, klare Kontraste, gleichmäßiges Tempo
- Schlusssatz wirkt etwas zu pointiert-bonmotartig

**Finale Fassung:**

> KI-Coding-Assistenten machen einen schneller bei den langweiligen Sachen. Nicht überall. Definitiv nicht bei Architektur.
>
> Sie sind großartig bei Boilerplate: Konfigs, Test-Gerüst, repetitive Refactorings. Sie sind genauso großartig darin, richtig zu klingen und trotzdem falschzuliegen. Ich habe schon Vorschläge übernommen, die kompilierten, durch den Linter kamen und am Ende komplett am Punkt vorbei waren — weil ich aufgehört hatte hinzuschauen.
>
> Ohne Tests rät man im Grunde.

**Änderungen:**

- Chatbot-Artefakte entfernt („Großartige Frage!", „Ich hoffe, das hilft!")
- Bedeutungsaufblähung entfernt („Zeugnis", „entscheidender Wendepunkt", „sich rasant entwickelnde Landschaft")
- Werblich-blumige Sprache entfernt („bahnbrechend", „angesiedelt an der Schnittstelle")
- Geviertstrich (—) durch Halbgeviertstrich (–) mit Leerzeichen ersetzt
- Kopula-Vermeidung („dient als", „verkörpert") durch direkte Aussagen ersetzt
- Ich-Perspektive und Meinung ergänzt
- Satzrhythmus variiert
- Anglizismus „Engineers ideieren, iterieren und ausliefern" → konkrete Verben

---

## Beispiel 2 — Marketing-Absatz mit Inline-Header-Liste

**Eingabe:**

> In der heutigen schnelllebigen Geschäftswelt ist es entscheidend, dass Unternehmen ihre Prozesse optimieren. Unsere Plattform bietet eine umfassende Lösung:
>
> - **Performance:** Die Performance wurde durch optimierte Algorithmen verbessert.
> - **Sicherheit:** Die Sicherheit wurde durch Verschlüsselung erhöht.
> - **Skalierbarkeit:** Die Skalierbarkeit wurde durch eine neue Architektur gesteigert.
>
> Zudem ermöglicht unsere Plattform es Unternehmen, nicht nur ihre Effizienz zu steigern, sondern auch ihre Wettbewerbsfähigkeit zu erhöhen. Lassen Sie uns gemeinsam in die Welt der Prozessoptimierung eintauchen!

**Finale Fassung:**

> Wir haben drei Dinge an der Plattform geändert: schnellere Algorithmen, Ende-zu-Ende-Verschlüsselung, neue Service-Architektur fürs Skalieren.
>
> Im Test lief der Hauptreport doppelt so schnell wie vorher. Ob das in eurem Setup auch so ankommt, hängt am Datenvolumen — wir richten gerne einen Probelauf mit euren Daten ein.

**Änderungen:**

- Einleitungsfloskel „In der heutigen schnelllebigen Geschäftswelt" gestrichen
- Inline-Header-Liste aufgelöst (Pattern 17)
- „Nicht nur … sondern auch" entfernt
- Chatbot-Artefakt „Lassen Sie uns eintauchen" entfernt
- Bedeutungsaufblähung („entscheidend, dass", „umfassende Lösung") entfernt
- Konkretisierung mit Test-Aussage statt Versprechen
- Direkter Call-to-Action statt generischer Begeisterungsschluss
