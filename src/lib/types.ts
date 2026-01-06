// Core UI Types for Miktos Kosmos Command Center

export type ThemeMode = 'light' | 'dark' | 'system';

export type Tone = 'neutral' | 'info' | 'success' | 'warning' | 'danger';

export type AppStage =
	| 'NO_WORKSPACE'
	| 'WORKSPACE_SCANNED'
	| 'ANALYSIS_READY'
	| 'ANALYZED'
	| 'TRANSFORM_PREVIEW_READY'
	| 'CHANGES_APPLIED';

export type NavId = 
	| 'home'
	| 'workspace'
	| 'analyze'
	| 'transform'
	| 'review'
	| 'learn'
	| 'settings'
	| 'about';

export interface NavItem {
	id: NavId;
	label: string;
	icon?: string; // Emoji or icon name
	route: string;
	badgeCount?: number; // Only for real alerts
}

export interface StatusBadge {
	id: string;
	tone: Tone;
	text: string;
	tooltip?: string;
	icon?: string;
}

export interface ConfirmSpec {
	title: string;
	body?: string;
	confirmText?: string;
	cancelText?: string;
	tone?: Tone;
}

export interface ActivityItem {
	id: string;
	time: string;
	title: string;
	detail?: string;
	tone?: Tone;
	onOpen?: () => void;
}
