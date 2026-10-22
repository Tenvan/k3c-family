package usage

import "sort"

// Count ist ein Eintrag einer Rangliste: ein Argument (mit Laufzeiten) oder eine Fehlermeldung (mit Tool).
type Count struct {
	Tool  string  `json:"tool,omitempty"`
	Value string  `json:"value"`
	Count int     `json:"count"`
	AvgMs float64 `json:"avgMs,omitempty"`
	P95Ms float64 `json:"p95Ms,omitempty"`
	MaxMs float64 `json:"maxMs,omitempty"`
}

// ToolUsage ist die Auswertung eines Tools in einem Bereich.
type ToolUsage struct {
	Name      string  `json:"name"`
	Calls     int     `json:"calls"`
	Errors    int     `json:"errors"`
	AvgMs     float64 `json:"avgMs"`
	P50Ms     float64 `json:"p50Ms"`
	P95Ms     float64 `json:"p95Ms"`
	MaxMs     float64 `json:"maxMs"`
	SumMs     float64 `json:"sumMs"`
	Outliers  int     `json:"outliers"`
	Args      []Count `json:"args"`
	TopErrors []Count `json:"topErrors"`
}

// Scope ist die Auswertung eines Bereichs über alle Tools.
type Scope struct {
	Since          string      `json:"since"`
	Calls          int         `json:"calls"`
	Errors         int         `json:"errors"`
	AvgMs          float64     `json:"avgMs"`
	P50Ms          float64     `json:"p50Ms"`
	P95Ms          float64     `json:"p95Ms"`
	MaxMs          float64     `json:"maxMs"`
	SumMs          float64     `json:"sumMs"`
	Outliers       int         `json:"outliers"`
	Tools          []ToolUsage `json:"tools"`
	TopErrors      []Count     `json:"topErrors"`
	RecentOutliers []SlowCall  `json:"recentOutliers"`
	Slowest        []SlowCall  `json:"slowest"`
}

// Usage ist der Schnappschuss für die Oberfläche: beide Bereiche und die Minuten-Zeitreihe.
type Usage struct {
	Session Scope    `json:"session"`
	AllTime Scope    `json:"allTime"`
	Minutes []Bucket `json:"minutes"`
}

func (s *scope) snapshot() Scope {
	out := Scope{Since: s.Since.Format(timeLayout), Tools: []ToolUsage{}, TopErrors: []Count{},
		RecentOutliers: append([]SlowCall{}, s.Outliers...), Slowest: append([]SlowCall{}, s.Slowest...)}
	var all durAgg
	for name, t := range s.Tools {
		all.merge(&t.durAgg)
		out.Errors += t.Errors
		out.Outliers += t.Outliers
		out.Tools = append(out.Tools, t.usage(name))
		for msg, n := range t.ErrorMsgs {
			out.TopErrors = append(out.TopErrors, Count{Tool: name, Value: msg, Count: n})
		}
	}
	out.Calls, out.SumMs, out.MaxMs = all.Calls, all.SumMs, all.MaxMs
	out.AvgMs, out.P50Ms, out.P95Ms = all.avg(), all.p(0.5), all.p(0.95)
	sort.Slice(out.Tools, func(i, j int) bool {
		a, b := out.Tools[i], out.Tools[j]
		return a.Calls > b.Calls || (a.Calls == b.Calls && a.Name < b.Name)
	})
	out.TopErrors = top(out.TopErrors)
	return out
}

func (t *toolAgg) usage(name string) ToolUsage {
	u := ToolUsage{Name: name, Calls: t.Calls, Errors: t.Errors, AvgMs: t.avg(), P50Ms: t.p(0.5), P95Ms: t.p(0.95),
		MaxMs: t.MaxMs, SumMs: t.SumMs, Outliers: t.Outliers, Args: []Count{}, TopErrors: []Count{}}
	for value, n := range t.Args {
		c := Count{Value: value, Count: n}
		if d := t.ArgDur[value]; d != nil {
			c.AvgMs, c.P95Ms, c.MaxMs = d.avg(), d.p(0.95), d.MaxMs
		}
		u.Args = append(u.Args, c)
	}
	for msg, n := range t.ErrorMsgs {
		u.TopErrors = append(u.TopErrors, Count{Value: msg, Count: n})
	}
	u.Args, u.TopErrors = top(u.Args), top(u.TopErrors)
	return u
}

// top sortiert absteigend nach Anzahl, bei Gleichstand nach Wert, und kürzt auf topN.
func top(list []Count) []Count {
	sort.Slice(list, func(i, j int) bool {
		a, b := list[i], list[j]
		return a.Count > b.Count || (a.Count == b.Count && a.Tool+a.Value < b.Tool+b.Value)
	})
	return list[:min(len(list), topN)]
}
