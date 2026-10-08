import { installPageChrome } from '../core/shell';
import { GRAFIK_PACKS, type GrafikPack } from './grafikPacks';
import { installSelection, type Selection } from './selection';
import { installPadScroll } from './spriteReference';
import { applyTexts, t, textOf } from './texts';

installPageChrome();
applyTexts();
installPadScroll();

interface ImageEntry {
  pack: string;
  group: string;
  file: string;
  w: number;
  h: number;
}

const ZOOMS = [1, 2, 3, 4];

function el<K extends keyof HTMLElementTagNameMap>(tag: K, className?: string, text?: string): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function renderZoom(root: HTMLElement): void {
  root.append(el('span', undefined, t('grafik.zoom')));
  for (const z of ZOOMS) {
    const button = el('button', undefined, `${z}×`);
    button.type = 'button';
    button.setAttribute('aria-pressed', String(z === 2));
    button.addEventListener('click', () => {
      document.documentElement.style.setProperty('--z', String(z));
      root.querySelectorAll('button').forEach((b) => b.setAttribute('aria-pressed', String(b === button)));
    });
    root.append(button);
  }
}

function renderImage(entry: ImageEntry, selection: Selection): HTMLElement {
  const figure = el('figure');
  figure.style.position = 'relative';
  figure.append(selection.box(`${entry.pack}/${entry.file}`));
  const img = el('img');
  img.src = `grafik/${entry.pack}/${entry.file}`;
  img.alt = entry.file;
  img.width = entry.w;
  img.height = entry.h;
  img.style.width = `calc(${entry.w}px * var(--z))`;
  img.addEventListener('error', () => img.replaceWith(el('div', 'missing', t('grafik.missing'))));
  figure.append(img, el('figcaption', undefined, `${entry.file.split('/').pop()} · ${entry.w}×${entry.h}`));
  return figure;
}

function renderPack(pack: GrafikPack, images: ImageEntry[], selection: Selection): HTMLElement {
  const section = el('section', 'pack');
  section.id = pack.id;
  section.style.position = 'relative';
  section.append(selection.box(pack.id));
  const title = el('h2', undefined, pack.name);
  title.style.paddingLeft = '7rem';
  section.append(title);
  const meta = el('p', 'meta');
  const link = el('a', undefined, t('grafik.source'));
  link.href = pack.source;
  link.target = '_blank';
  link.rel = 'noopener';
  meta.append(`${pack.artist} · ${pack.license} · `, link);
  section.append(meta);
  const chips = el('div', 'chips');
  pack.covers.forEach((c) => chips.append(el('span', undefined, c)));
  section.append(chips, el('p', 'note', pack.note));
  if (pack.licenseNote) section.append(el('p', 'warn', pack.licenseNote));
  const groups = [...new Set(images.map((i) => i.group))];
  for (const group of groups) {
    section.append(el('h3', undefined, textOf(`grafik.group.${group}`, group)));
    const row = el('div', 'imgs');
    images.filter((i) => i.group === group).forEach((i) => row.append(renderImage(i, selection)));
    section.append(row);
  }
  return section;
}

async function main(): Promise<void> {
  renderZoom(document.getElementById('zoom')!);
  const root = document.getElementById('packs')!;
  try {
    const index = (await (await fetch('grafik/index.json')).json()) as ImageEntry[];
    const selection = installSelection('k3c-auswahl-grafiken', t('sel.grafiken'));
    for (const pack of GRAFIK_PACKS) root.append(renderPack(pack, index.filter((i) => i.pack === pack.id), selection));
  } catch {
    root.append(el('p', 'missing', t('grafik.noIndex')));
  }
}

void main();
