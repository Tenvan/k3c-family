// Zugriff des Bereichs Assets auf die RPC-Methoden (Vertrag docs/standards/paper-mcp.md › 4). Gemeinsam für WB1 und
// RV1 (Sprint-README WB1 › D-02): RV1 importiert von hier, Änderungen per Ticket an WB1.
import { backend } from '../api';

export type * from '../api/typesAssets';

export const assetsList = () => backend.assetsList();
export const assetsIndex = () => backend.assetsIndex();
export const assetsCheck = (pfade: string[]) => backend.assetsCheck(pfade);
export const assetsFrameOhne = (id: string, animation: string, frame: number, ohne: string[]) =>
  backend.assetsFrameOhne(id, animation, frame, ohne);
export const assetsGroesse = (id: string) => backend.assetsGroesse(id);
export const assetsGroesseBasis = (id: string, basis: string) => backend.assetsGroesseBasis(id, basis);
export const assetsHintergruende = () => backend.assetsHintergruende();
export const assetsHintergrund = (teil: string) => backend.assetsHintergrund(teil);
export const assetsGet = (id: string) => backend.assetsGet(id);
export const assetsStyle = () => backend.assetsStyle();
export const assetsPrompts = () => backend.assetsPrompts();
export const assetsReview = (basis: string) => backend.assetsReview(basis);
export const assetsFrame = (id: string, animation: string, frame: number, variante = '', basis = '') =>
  backend.assetsFrame(id, animation, frame, variante, basis);
export const assetsPixelDiff = (id: string, animation: string, frame: number, basis: string) =>
  backend.assetsPixelDiff(id, animation, frame, basis);
