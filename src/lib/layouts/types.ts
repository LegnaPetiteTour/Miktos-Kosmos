// Layout System Types for Workspace

export type LayoutId = 'browser' | 'transform' | 'review' | 'analyze';

export type PanelId = 'folders' | 'files' | 'preview' | 'metadata' | 'tools' | 'history';

export interface PanelConfig {
	id: PanelId;
	visible: boolean;
	width?: number;  // for vertical splits (%)
	height?: number; // for horizontal splits (%)
	minWidth?: number;
	minHeight?: number;
}

export interface LayoutConfig {
	id: LayoutId;
	name: string;
	description: string;
	icon?: string;
	panels: {
		left?: PanelConfig[];
		center?: PanelConfig[];
		right?: PanelConfig[];
		bottom?: PanelConfig[];
	};
}

// Predefined layouts
export const LAYOUTS: Record<LayoutId, LayoutConfig> = {
	browser: {
		id: 'browser',
		name: 'Browser',
		description: 'Browse and explore your photo library',
		panels: {
			left: [
				{ id: 'folders', visible: true, width: 20, minWidth: 150 }
			],
			center: [
				{ id: 'files', visible: true, width: 50 }
			],
			right: [
				{ id: 'preview', visible: true, width: 30, minWidth: 200 }
			]
		}
	},
	transform: {
		id: 'transform',
		name: 'Transform',
		description: 'Organize files with live preview',
		panels: {
			left: [
				{ id: 'files', visible: true, width: 40 }
			],
			center: [
				{ id: 'tools', visible: true, width: 60 }
			]
		}
	},
	review: {
		id: 'review',
		name: 'Review',
		description: 'View operation history and results',
		panels: {
			left: [
				{ id: 'files', visible: true, width: 40 }
			],
			right: [
				{ id: 'history', visible: true, width: 60 }
			]
		}
	},
	analyze: {
		id: 'analyze',
		name: 'Analyze',
		description: 'Find duplicates and issues',
		panels: {
			center: [
				{ id: 'files', visible: true, width: 60 }
			],
			right: [
				{ id: 'tools', visible: true, width: 40 }
			]
		}
	}
};
