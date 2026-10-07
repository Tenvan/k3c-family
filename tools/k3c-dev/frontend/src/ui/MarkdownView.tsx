import { parseMarkdown, type Block, type Inline } from '../mcp/markdown';

/** Markdown als React-Elemente (nie als HTML-String). headingOffset schiebt die Ebenen, wenn die Seite selbst eine h1 hat. */
export function MarkdownView({ source, headingOffset = 0 }: { source: string; headingOffset?: number }) {
  return parseMarkdown(source).map((b, i) => <BlockView key={i} b={b} offset={headingOffset} />);
}

function BlockView({ b, offset }: { b: Block; offset: number }) {
  if (b.kind === 'h') {
    const H = `h${Math.min(6, b.level + offset)}` as 'h2' | 'h3' | 'h4' | 'h5' | 'h6';
    return <H><Inlines list={b.inline} /></H>;
  }
  if (b.kind === 'ul' || b.kind === 'ol') {
    const L = b.kind;
    return <L>{b.items.map((item, i) => <li key={i}><Inlines list={item} /></li>)}</L>;
  }
  if (b.kind === 'pre') return <pre>{b.text}</pre>;
  if (b.kind === 'table') {
    return (
      <div className="md-table">
        <table>
          <thead><tr>{b.head.map((c, i) => <th key={i}><Inlines list={c} /></th>)}</tr></thead>
          <tbody>
            {b.rows.map((r, i) => <tr key={i}>{r.map((c, j) => <td key={j}><Inlines list={c} /></td>)}</tr>)}
          </tbody>
        </table>
      </div>
    );
  }
  return <p><Inlines list={b.inline} /></p>;
}

function Inlines({ list }: { list: Inline[] }) {
  return list.map((x, i) => {
    if (x.kind === 'code') return <code key={i}>{x.text}</code>;
    if (x.kind === 'bold') return <strong key={i}>{x.text}</strong>;
    return <span key={i}>{x.text}</span>;
  });
}
