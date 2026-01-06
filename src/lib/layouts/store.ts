import { writable } from 'svelte/store';
import type { LayoutId, LayoutConfig } from './types';
import { LAYOUTS } from './types';

function createLayoutStore() {
	const { subscribe, set, update } = writable<LayoutConfig>(LAYOUTS.essentials);
	
	return {
		subscribe,
		setLayout: (layoutId: LayoutId) => {
			set(LAYOUTS[layoutId]);
		},
		updatePanelVisibility: (panelId: string, visible: boolean) => {
			update(layout => {
				// Update panel visibility in the current layout
				const updatedLayout = { ...layout };
				Object.keys(updatedLayout.panels).forEach(area => {
					const panels = updatedLayout.panels[area as keyof typeof updatedLayout.panels];
					if (panels) {
						panels.forEach(panel => {
							if (panel.id === panelId) {
								panel.visible = visible;
							}
						});
					}
				});
				return updatedLayout;
			});
		}
	};
}

export const layoutStore = createLayoutStore();
