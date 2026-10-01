import { installPageChrome } from '../core/shell';
import { installSelection } from './selection';
import { allFigures, installPadScroll, renderReference } from './spriteReference';

installPageChrome();
installPadScroll();
renderReference(document.getElementById('cards')!, allFigures(), false, installSelection('k3c-auswahl-figuren', 'Figuren'));
