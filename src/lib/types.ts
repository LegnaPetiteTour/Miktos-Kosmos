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
	| 'organize'
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

// ============================================================================
// ORGANIZATION / TRANSFORM TYPES
// ============================================================================

export type OrganizationStrategy = 
	| 'Date'           // YYYY/MM-Month format
	| 'Year'           // YYYY only
	| 'YearMonth'      // YYYY/MM
	| 'FileType'       // Images/Videos/Documents
	| 'DateAndType';   // YYYY/MM-Month/Images
export type OperationMode = 
	| 'Copy'   // Safe: Keep originals
	| 'Move';  // Destructive: Remove originals

export interface FolderPreview {
	path: string;
	file_count: number;
	total_size: number;
	files: string[]; // File names that will go here
}

export interface OrganizationPlan {
	destination_root: string;
	strategy: OrganizationStrategy;
	mode: OperationMode;
	folders: FolderPreview[];
	total_files: number;
	total_size: number;
	files_without_dates: number; // Files that will go to "Unknown"
}

export type OperationStatus = 'Success' | 'Failed' | 'Skipped';

export interface FileOperation {
	source_path: string;
	destination_path: string;
	status: OperationStatus;
	error_message?: string;
}

export interface OperationResult {
	success: boolean;
	operations: FileOperation[];
	successful_count: number;
	failed_count: number;
	skipped_count: number;
	total_size_processed: number;
	duration_ms: number;
	timestamp: string; // ISO datetime
}

export interface OperationProgress {
	current_file: string;
	processed: number;
	total: number;
	percentage: number;
}
