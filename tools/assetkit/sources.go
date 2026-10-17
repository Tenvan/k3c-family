package assets

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"html"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync"
	"time"
)

// Searcher fragt die Quellen der Web-Recherche ab (B-298/AC-04). Basis-Adressen sind für Tests austauschbar.
type Searcher struct {
	Client    *http.Client
	Freesound string // API, braucht Token
	Commons   string // MediaWiki-API von Wikimedia Commons
	OGA       string // OpenGameArt, Suchseite als HTML (ohne API)
	Token     string // freesound; nie im Repo
}

// sourceTimeout begrenzt jede Quelle; eine langsame Quelle hält die anderen nicht auf.
const sourceTimeout = 15 * time.Second

const userAgent = "k3c-dev/0.1 (Ressourcen-Manager, B-298)"

// NewSearcher nutzt die echten Quellen und den freesound-Token aus K3C_FREESOUND_TOKEN oder .work/freesound-token.
func NewSearcher(root string) Searcher {
	token := strings.TrimSpace(os.Getenv("K3C_FREESOUND_TOKEN"))
	if token == "" {
		if b, err := os.ReadFile(filepath.Join(root, ".work", "freesound-token")); err == nil {
			token = strings.TrimSpace(string(b))
		}
	}
	return Searcher{
		Client: &http.Client{Timeout: sourceTimeout}, Token: token,
		Freesound: "https://freesound.org", Commons: "https://commons.wikimedia.org", OGA: "https://opengameart.org",
	}
}

type source struct {
	name string
	run  func(ctx context.Context, query, kind string) ([]Hit, error)
}

// Search fragt alle passenden Quellen parallel; Fehler einer Quelle stehen als Hinweis in notes, die anderen liefern weiter.
func (s Searcher) Search(ctx context.Context, query, kind string) (hits []Hit, notes []string) {
	sources := []source{{"Wikimedia Commons", s.commons}, {"OpenGameArt", s.openGameArt}}
	if kind == KindSound {
		sources = append([]source{{"freesound", s.freesound}}, sources...)
	}
	results := make([][]Hit, len(sources))
	errs := make([]error, len(sources))
	var wg sync.WaitGroup
	for i, src := range sources {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c, cancel := context.WithTimeout(ctx, sourceTimeout)
			defer cancel()
			results[i], errs[i] = src.run(c, query, kind)
		}()
	}
	wg.Wait()
	for i, src := range sources {
		if err := errs[i]; err != nil {
			// *url.Error nennt die Adresse; ohne sie gelangt nichts aus der Anfrage in Hinweise und Logs.
			var ue *url.Error
			if errors.As(err, &ue) {
				err = ue.Err
			}
			notes = append(notes, fmt.Sprintf("%s: %v", src.name, err))
		}
		hits = append(hits, results[i]...)
	}
	return hits, notes
}

func (s Searcher) get(ctx context.Context, rawURL string, header ...string) ([]byte, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, rawURL, nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("User-Agent", userAgent)
	for i := 0; i+1 < len(header); i += 2 {
		req.Header.Set(header[i], header[i+1])
	}
	res, err := s.Client.Do(req)
	if err != nil {
		return nil, err
	}
	defer func() { _ = res.Body.Close() }()
	body, err := io.ReadAll(io.LimitReader(res.Body, 4<<20))
	if err != nil {
		return nil, err
	}
	if res.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP %d", res.StatusCode)
	}
	return body, nil
}

// freesound: Such-API mit Token; Lizenz kommt als URL, Hörprobe als MP3-Vorschau. Download braucht OAuth: nur Link.
func (s Searcher) freesound(ctx context.Context, query, _ string) ([]Hit, error) {
	if s.Token == "" {
		return nil, fmt.Errorf("token fehlt (K3C_FREESOUND_TOKEN oder .work/freesound-token)")
	}
	// Token im Header, nie in der Adresse: so steht er in keiner Fehlermeldung und keinem Log.
	q := url.Values{"query": {query}, "fields": {"id,name,url,username,license,previews"}, "page_size": {"15"}}
	body, err := s.get(ctx, s.Freesound+"/apiv2/search/text/?"+q.Encode(), "Authorization", "Token "+s.Token)
	if err != nil {
		return nil, err
	}
	var res struct {
		Results []struct {
			Name, URL, Username, License string
			Previews                     map[string]string
		}
	}
	if err := json.Unmarshal(body, &res); err != nil {
		return nil, fmt.Errorf("antwort unlesbar: %w", err)
	}
	hits := make([]Hit, 0, len(res.Results))
	for _, r := range res.Results {
		hits = append(hits, Hit{Title: r.Name, Link: r.URL, Source: "freesound", Artist: r.Username,
			License: licenseFromURL(r.License), Preview: r.Previews["preview-hq-mp3"]})
	}
	return hits, nil
}

// licenseFromURL macht aus einer Creative-Commons-Adresse den Kurznamen (CC0 1.0, CC BY-NC 3.0 …).
func licenseFromURL(u string) string {
	switch {
	case strings.Contains(u, "publicdomain/zero"):
		return "CC0 1.0"
	case strings.Contains(u, "/licenses/"):
		parts := strings.Split(strings.Trim(u[strings.Index(u, "/licenses/")+len("/licenses/"):], "/"), "/")
		name := "CC " + strings.ToUpper(parts[0])
		if len(parts) > 1 {
			name += " " + parts[1]
		}
		return name
	}
	return u
}

// commons: MediaWiki-API mit Lizenz und Urheber aus den Metadaten; die Originaldatei ist direkt ladbar.
func (s Searcher) commons(ctx context.Context, query, kind string) ([]Hit, error) {
	filetype := "bitmap"
	if kind == KindSound {
		filetype = "audio"
	}
	q := url.Values{"action": {"query"}, "format": {"json"}, "generator": {"search"}, "gsrnamespace": {"6"}, "gsrlimit": {"15"},
		"gsrsearch": {query + " filetype:" + filetype}, "prop": {"imageinfo"}, "iiprop": {"url|extmetadata"}, "iiurlwidth": {"128"},
		"iiextmetadatafilter": {"LicenseShortName|Artist|LicenseUrl"}}
	body, err := s.get(ctx, s.Commons+"/w/api.php?"+q.Encode())
	if err != nil {
		return nil, err
	}
	var res struct {
		Query struct {
			Pages map[string]struct {
				Title     string
				ImageInfo []struct {
					ThumbURL       string `json:"thumburl"`
					URL            string `json:"url"`
					DescriptionURL string `json:"descriptionurl"`
					Meta           map[string]struct {
						Value string `json:"value"`
					} `json:"extmetadata"`
				} `json:"imageinfo"`
			}
		}
	}
	if err := json.Unmarshal(body, &res); err != nil {
		return nil, fmt.Errorf("antwort unlesbar: %w", err)
	}
	var hits []Hit
	for _, p := range res.Query.Pages {
		if len(p.ImageInfo) == 0 {
			continue
		}
		ii := p.ImageInfo[0]
		hits = append(hits, Hit{Title: strings.TrimPrefix(p.Title, "File:"), Link: ii.DescriptionURL, Source: "Wikimedia Commons",
			Artist: plainText(ii.Meta["Artist"].Value), License: ii.Meta["LicenseShortName"].Value, Preview: ii.ThumbURL, Download: ii.URL})
	}
	sort.Slice(hits, func(i, j int) bool { return hits[i].Title < hits[j].Title }) // Seiten kommen als Map
	return hits, nil
}

var tags = regexp.MustCompile(`<[^>]*>`)

func plainText(s string) string {
	return strings.TrimSpace(html.UnescapeString(tags.ReplaceAllString(s, "")))
}

// OpenGameArt-Suchseite: je Zeile ein Treffer mit Titel und Link, Vorschau als Bild oder MP3; Lizenz nur auf der Einzelseite.
var (
	ogaTitle   = regexp.MustCompile(`class="art-preview-title"><a href="(/content/[^"]+)">([^<]+)</a>`)
	ogaAudio   = regexp.MustCompile(`data-mp3-url='([^']+)'`)
	ogaPicture = regexp.MustCompile(`<img src='([^']+)'`)
)

// ogaTypes sind die Kunst-Arten der Suche: 9 = 2D Art, 13 = Sound Effect, 12 = Music.
var ogaTypes = map[string][]string{KindGraphic: {"9"}, KindSound: {"13", "12"}}

func (s Searcher) openGameArt(ctx context.Context, query, kind string) ([]Hit, error) {
	q := url.Values{"keys": {query}, "field_art_type_tid[]": ogaTypes[kind]}
	body, err := s.get(ctx, s.OGA+"/art-search-advanced?"+q.Encode())
	if err != nil {
		return nil, err
	}
	var hits []Hit
	for _, line := range strings.Split(string(body), "\n") {
		m := ogaTitle.FindStringSubmatch(line)
		if m == nil {
			continue
		}
		h := Hit{Title: strings.TrimSpace(html.UnescapeString(m[2])), Link: s.OGA + m[1], Source: "OpenGameArt"}
		if a := ogaAudio.FindStringSubmatch(line); a != nil {
			h.Preview = a[1]
		} else if p := ogaPicture.FindStringSubmatch(line); p != nil {
			h.Preview = p[1]
		}
		hits = append(hits, h)
	}
	return hits, nil
}
