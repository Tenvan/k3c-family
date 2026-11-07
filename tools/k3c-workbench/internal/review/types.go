package review

// Typen mit den JSON-Namen des Vertrags (paper-mcp.md › Oberfläche); internal/assetsui übernimmt sie als Alias.

// Fehler ist eine abgelehnte Regel einer Spec (paper-spec.md › Regeln).
type Fehler struct {
	Feld  string `json:"feld"`
	Regel string `json:"regel"`
	Text  string `json:"text"`
}

// FeldAenderung ist ein geändertes Feld einer Spec gegenüber der Basis.
type FeldAenderung struct {
	Pfad string `json:"pfad"`
	Alt  any    `json:"alt"`
	Neu  any    `json:"neu"`
}

// Status eines Review-Eintrags.
const (
	StatusNeu       = "neu"
	StatusGeaendert = "geaendert"
	StatusGeloescht = "geloescht"
	StatusUngueltig = "ungueltig"
)

// Asset ist ein geändertes Asset; Status neu | geaendert | geloescht | ungueltig.
type Asset struct {
	ID          string          `json:"id"`
	Rig         string          `json:"rig"`
	Pfad        string          `json:"pfad"`
	Status      string          `json:"status"`
	Fehler      []Fehler        `json:"fehler,omitempty"`
	Felder      []FeldAenderung `json:"felder"`
	Teile       []string        `json:"teile"`
	Stil        bool            `json:"stil"`
	PixelAnteil float64         `json:"pixelAnteil"`
}

// Ergebnis ist der Vergleich des Arbeitsstands mit einer Basis; BasisStand aktuell | lokal.
type Ergebnis struct {
	Basis       string  `json:"basis"`
	BasisCommit string  `json:"basisCommit"`
	BasisStand  string  `json:"basisStand"`
	Assets      []Asset `json:"assets"`
}
