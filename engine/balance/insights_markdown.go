package balance

import (
	"fmt"
	"strings"
)

// Auswertung im Markdown-Bericht (B-632): steht oben in balance-<Zeit>.md, vor der Tabelle je Ziel.

var lightIcon = map[string]string{LightGreen: "🟢", LightYellow: "🟡", LightRed: "🔴", LightGrey: "⚪"}

// whereText: wohin ein Befehl gehört (Command.Where).
var whereText = map[string]string{WhereChat: "in den Chat an Claude", WhereShell: "im Terminal des Checkouts",
	WhereWorkbench: "Workbench › Balancing-Seite › Abstimmen"}

// tableLegend erklärt die Tabelle der Ziele in Klartext (steht im Markdown vor der Tabelle).
const tableLegend = "So liest du die Tabelle: **Wert** ist das Messergebnis (bei Anteilen: in wie viel Prozent der Läufe das Ziel erreicht wurde, " +
	"„Läufe“ sind die Seeds). **Korridor** ist der Soll-Bereich dafür. **Abstand** ist der Unterschied zur verletzten Grenze in Prozentpunkten (Pkt.) " +
	"bzw. in der Einheit des Werts; im Korridor steht dort die Reserve bis zur nächsten Grenze. **Trend** vergleicht mit der Baseline " +
	"(▲ näher am Korridor, ▼ weiter weg).\n\n"

// Insights wertet den Bericht mit den eingebetteten Richtwerten (data/balance-hints.json) aus.
func (s Summary) Insights(base *Summary, m Meta) (Insights, error) {
	h, err := DefaultHints()
	if err != nil {
		return Insights{}, err
	}
	return BuildInsights(s, base, h, m), nil
}

// Markdown schreibt die Auswertung: Kurzauswertung, Ampeln, Stärken und Schwächen, Hinweise, nächster Lauf, Kennzahlen.
func (in Insights) Markdown() string {
	var b strings.Builder
	in.ladderMarkdown(&b)
	b.WriteString("## Kurzauswertung\n\n")
	bullets(&b, in.Summary, "")
	fmt.Fprintf(&b, "\n## Ampeln\n\n🟢 %d gut · 🟡 %d knapp · 🔴 %d verletzt · ⚪ %d ungültig\n\n",
		in.Counts[LightGreen], in.Counts[LightYellow], in.Counts[LightRed], in.Counts[LightGrey])
	b.WriteString(tableLegend)
	b.WriteString("| Ampel | Ziel | Grad | Wert | Korridor | Abstand zum Korridor | Trend |\n|---|---|---|---|---|---|---|\n")
	for _, t := range in.Targets {
		fmt.Fprintf(&b, "| %s %s | %s | %s | %s | %s | %s | %s |\n", lightIcon[t.Light], t.Status, t.Name, t.Grade,
			t.valueText(), t.boundsText(), t.Gap, trendText(t.Trend))
	}
	b.WriteString("\n## Stärken\n\n")
	bullets(&b, in.Strengths, "Keine erkannt.")
	b.WriteString("\n## Schwächen\n\n")
	bullets(&b, in.Weaknesses, "Keine erkannt.")
	b.WriteString("\n## Tuning-Hinweise\n\n")
	if len(in.Hints) == 0 {
		b.WriteString("Keine: alle Ziele im Korridor.\n")
	}
	if len(in.LeverGroups) > 0 {
		b.WriteString("\n### Stellschrauben nach Wirkung\n\nDerselbe Wert in data/ taucht bei mehreren Verletzungen auf; hier je Wert einmal, mit den Verletzungen, die er verbessern soll.\n\n")
		for _, g := range in.LeverGroups {
			fmt.Fprintf(&b, "- %s\n", g.Text)
		}
		b.WriteString("\n### Hinweise je Ziel\n\n")
	}
	for _, h := range in.Hints {
		fmt.Fprintf(&b, "- %s **%s** – %s %s\n", lightIcon[h.Light], h.Target, h.Text, h.Why)
		for i, l := range h.Levers {
			fmt.Fprintf(&b, "  - %s `%s`\n", l.Label, l.Path)
			if i < len(h.Plans) {
				fmt.Fprintf(&b, "    %s\n", h.Plans[i].Text)
			}
		}
	}
	in.deltaMarkdown(&b)
	in.nextMarkdown(&b)
	in.figuresMarkdown(&b)
	return b.String()
}

// deltaMarkdown: Vergleich je Werte-Gruppe (delta.go), nur mit Baseline.
func (in Insights) deltaMarkdown(b *strings.Builder) {
	if len(in.Delta) == 0 {
		return
	}
	fmt.Fprintf(b, "\n## Delta je Werte-Gruppe (%s)\n\n", in.Baseline.Note)
	for _, d := range in.Delta {
		state := "frei"
		if d.Frozen {
			state = "eingefroren"
		}
		fmt.Fprintf(b, "- %s **%s** (%s): %s, %d besser, %d schlechter von %d Zielen", lightIcon[d.Light], d.Group, state, d.Trend, d.Better, d.Worse, d.Targets)
		if d.Warning != "" {
			b.WriteString(" – ⚠️ " + d.Warning)
		}
		b.WriteString("\n")
	}
}

func (in Insights) nextMarkdown(b *strings.Builder) {
	b.WriteString("\n## Nächster Lauf\n\n")
	for i, s := range in.Next.Steps {
		fmt.Fprintf(b, "%d. %s\n", i+1, s)
	}
	for _, c := range in.Next.Commands {
		fmt.Fprintf(b, "\n%s (%s):\n\n```\n%s\n```\n", c.Label, whereText[c.Where], c.Text)
	}
}

func (in Insights) figuresMarkdown(b *strings.Builder) {
	f := in.Figures
	b.WriteString("\n## Kennzahlen\n\n")
	if f.Note != "" {
		b.WriteString(f.Note + "\n\n")
	}
	for _, l := range f.Lights {
		fmt.Fprintf(b, "- %s %s: %s (%s)\n", lightIcon[l.Light], l.label(), l.valueText(), l.Text)
	}
	if f.Island != nil {
		f.Island.markdown(b)
	}
	if f.Ship != nil {
		f.Ship.markdown(b)
	}
	if len(f.Nights) == 0 {
		return
	}
	b.WriteString("\n| Grad | Nacht | Läufe | Überleben % | Gold Tag | Gold Nacht | Holz | Stein | Leerlauf % |\n|---|---|---|---|---|---|---|---|---|\n")
	for _, n := range f.Nights {
		fmt.Fprintf(b, "| %s | %d | %d | %s | %s | %s | %s | %s | %s |\n", gradeName(n.Grade), n.Night, n.Runs, num(round1(n.Survival)),
			num(round1(n.DayIncome-n.DayExpense)), num(round1(n.NightIncome-n.NightExpense)),
			num(round1(n.ResourceIn.Wood-n.ResourceOut.Wood)), num(round1(n.ResourceIn.Stone-n.ResourceOut.Stone)), num(round1(n.IdleShare)))
	}
}

func bullets(b *strings.Builder, items []string, empty string) {
	if len(items) == 0 && empty != "" {
		b.WriteString(empty + "\n")
	}
	for _, s := range items {
		b.WriteString("- " + s + "\n")
	}
}

func trendText(t *Trend) string {
	if t == nil {
		return "–"
	}
	mark := "▼"
	if t.Better {
		mark = "▲"
	}
	return mark + " " + signed(t.Delta)
}
