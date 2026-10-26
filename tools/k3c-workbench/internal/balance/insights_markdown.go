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
	b.WriteString("## Kurzauswertung\n\n")
	bullets(&b, in.Summary, "")
	fmt.Fprintf(&b, "\n## Ampeln\n\n🟢 %d gut · 🟡 %d knapp · 🔴 %d verletzt · ⚪ %d ungültig\n\n",
		in.Counts[LightGreen], in.Counts[LightYellow], in.Counts[LightRed], in.Counts[LightGrey])
	b.WriteString("| Ampel | Ziel | Grad | Wert | Korridor | Abstand | Trend |\n|---|---|---|---|---|---|---|\n")
	for _, t := range in.Targets {
		fmt.Fprintf(&b, "| %s %s | %s | %s | %s | %s | %s | %s |\n", lightIcon[t.Light], t.Status, t.Name, t.Grade,
			t.valueText(), t.boundsText(), distText(t.Distance), trendText(t.Trend))
	}
	b.WriteString("\n## Stärken\n\n")
	bullets(&b, in.Strengths, "Keine erkannt.")
	b.WriteString("\n## Schwächen\n\n")
	bullets(&b, in.Weaknesses, "Keine erkannt.")
	b.WriteString("\n## Tuning-Hinweise\n\n")
	if len(in.Hints) == 0 {
		b.WriteString("Keine: alle Ziele im Korridor.\n")
	}
	for _, h := range in.Hints {
		fmt.Fprintf(&b, "- %s **%s** – %s %s\n", lightIcon[h.Light], h.Target, h.Text, h.Why)
		for _, l := range h.Levers {
			fmt.Fprintf(&b, "  - %s `%s` (%s%s)\n", l.Label, l.Path, l.Direction, stepText(l.Step))
		}
	}
	in.nextMarkdown(&b)
	in.figuresMarkdown(&b)
	return b.String()
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

func distText(d *float64) string {
	if d == nil {
		return "–"
	}
	return signed(*d) + " %"
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
