import { Badge, Button, SegmentedControl, Text } from '@webkit/atoms';
import { GRAFIK_PACKS, type GrafikPack } from '@game/tools/grafikPacks';
import { textOf } from '@game/tools/texts';
import { useEffect, useState, type CSSProperties } from 'react';
import { backend } from '../api';
import { fileUrl } from '../assets/assets';

// Reiter „Packs“ der Seite Grafik (B-459/AC-06, Inhalt von grafiken.html): Pack-Übersicht aus GRAFIK_PACKS und die
// Bilder aus grafik/index.json. Pad-Scroll und Auswahl sind Xbox-Bedienung und bleiben auf grafiken.html.

interface ImageEntry {
  pack: string;
  group: string;
  file: string;
  w: number;
  h: number;
}

const ZOOMS = ['1', '2', '3', '4'];

export function PacksView() {
  const [index, setIndex] = useState<ImageEntry[] | null>(null);
  const [zoom, setZoom] = useState('2');
  useEffect(() => {
    let live = true;
    fetch('grafik/index.json')
      .then((r) => (r.ok ? (r.json() as Promise<ImageEntry[]>) : []))
      .then((list) => live && setIndex(list), () => live && setIndex([]));
    return () => {
      live = false;
    };
  }, []);
  if (!index) return <Text color="gray">Lade Bildliste …</Text>;
  return (
    <div className="pk-page" style={{ '--z': zoom } as CSSProperties}>
      <div className="as-head">
        <Text size="2">Zoom</Text>
        <SegmentedControl.Root value={zoom} onValueChange={setZoom}>
          {ZOOMS.map((z) => <SegmentedControl.Item key={z} value={z}>{z}×</SegmentedControl.Item>)}
        </SegmentedControl.Root>
        {index.length === 0 && <Text color="red">grafik/index.json fehlt</Text>}
      </div>
      {GRAFIK_PACKS.map((p) => <Pack key={p.id} pack={p} images={index.filter((i) => i.pack === p.id)} />)}
    </div>
  );
}

function Pack({ pack, images }: { pack: GrafikPack; images: ImageEntry[] }) {
  const groups = [...new Set(images.map((i) => i.group))];
  return (
    <section className="pk-pack" id={pack.id}>
      <h3 className="sz-title">{pack.name}</h3>
      <p className="sz-dim">
        {pack.artist} · {pack.license} · <Button size="1" variant="ghost" onClick={() => backend.openUrl(pack.source)}>Quelle</Button>
      </p>
      <div className="pk-chips">{pack.covers.map((c) => <Badge key={c} variant="outline" color="gray" radius="full">{c}</Badge>)}</div>
      <p className="pk-note">{pack.note}</p>
      {pack.licenseNote && <p className="as-warn">{pack.licenseNote}</p>}
      {groups.map((g) => (
        <div key={g}>
          <h4>{textOf(`grafik.group.${g}`, g)}</h4>
          <div className="pk-imgs">{images.filter((i) => i.group === g).map((i) => <Bild key={i.file} entry={i} />)}</div>
        </div>
      ))}
    </section>
  );
}

function Bild({ entry }: { entry: ImageEntry }) {
  const [failed, setFailed] = useState(false);
  return (
    <figure className="sz-fig">
      {failed ? <div className="sz-gap">Datei fehlt</div> : (
        <img src={fileUrl(`grafik/${entry.pack}/${entry.file}`)} alt={entry.file} width={entry.w} height={entry.h}
          style={{ width: `calc(${entry.w}px * var(--z))`, height: 'auto' }} onError={() => setFailed(true)} />
      )}
      <figcaption>{entry.file.split('/').pop()} · {entry.w}×{entry.h}</figcaption>
    </figure>
  );
}
