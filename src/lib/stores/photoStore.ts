import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export enum FileType {
	Image = 'Image',
	Video = 'Video',
	Document = 'Document',
	Audio = 'Audio',
	Archive = 'Archive',
	Other = 'Other'
}

export interface FileMetadata {
	file_name: string;
	file_path: string;
	file_size: number;
	file_type: FileType;
	created_at?: string;
	modified_at?: string;
	
	// Optional metadata (depends on file type)
	date_taken?: string;
	width?: number;
	height?: number;
	duration?: number;      // For videos/audio (seconds)
	page_count?: number;    // For documents
	
	hash: string;
	is_screenshot: boolean;
	is_duplicate: boolean;
	
	// Media-specific
	camera_make?: string;
	camera_model?: string;
}

export interface FileTypeStats {
	images: number;
	videos: number;
	documents: number;
	audio: number;
	archives: number;
	other: number;
}

export interface ScanStats {
	total_files: number;
	file_types: FileTypeStats;
	screenshots: number;
	duplicates: number;
	total_size: number;
	date_range_start?: string;
	date_range_end?: string;
}

export interface ScanResult {
	files: FileMetadata[];  // Changed from 'photos'
	stats: ScanStats;
}

// Get initial value from localStorage if in browser
const getInitialValue = (): ScanResult | null => {
	if (browser) {
		// Clear old photoStore data
		const oldData = localStorage.getItem('photoStore');
		if (oldData) {
			console.log('Clearing old photoStore data...');
			localStorage.removeItem('photoStore');
		}
		
		const stored = localStorage.getItem('fileStore');
		if (stored) {
			try {
				const parsed = JSON.parse(stored);
				// Validate it has the new structure
				if (parsed && parsed.files && Array.isArray(parsed.files)) {
					return parsed;
				} else {
					console.log('Invalid fileStore data, clearing...');
					localStorage.removeItem('fileStore');
					return null;
				}
			} catch {
				localStorage.removeItem('fileStore');
				return null;
			}
		}
	}
	return null;
};

// Store for scan results with localStorage persistence
function createFileStore() {
	const { subscribe, set, update } = writable<ScanResult | null>(getInitialValue());
	
	return {
		subscribe,
		setScanResult: (result: ScanResult) => {
			set(result);
			if (browser) {
				localStorage.setItem('fileStore', JSON.stringify(result));
			}
		},
		clear: () => {
			set(null);
			if (browser) {
				localStorage.removeItem('fileStore');
				localStorage.removeItem('photoStore'); // Clear old data too
			}
		},
		updateFile: (filePath: string, updates: Partial<FileMetadata>) => {
			update(state => {
				if (!state) return state;
				const newState = {
					...state,
					files: state.files.map(file => 
						file.file_path === filePath ? { ...file, ...updates } : file
					)
				};
				if (browser) {
					localStorage.setItem('fileStore', JSON.stringify(newState));
				}
				return newState;
			});
		}
	};
}

export const fileStore = createFileStore();

// Backward compatibility alias
export const photoStore = fileStore;

// Store for selected folder path
export const selectedFolderStore = writable<string>('');

// Store for current scanning state
export const scanningStore = writable<boolean>(false);

// Derived stores for common queries
export const fileCount = derived(
	fileStore,
	$fileStore => $fileStore?.files.length || 0
);

export const totalSize = derived(
	fileStore,
	$fileStore => $fileStore?.stats.total_size || 0
);

export const imageFiles = derived(
	fileStore,
	$fileStore => $fileStore?.files.filter(f => f.file_type === FileType.Image) || []
);

export const videoFiles = derived(
	fileStore,
	$fileStore => $fileStore?.files.filter(f => f.file_type === FileType.Video) || []
);

export const documentFiles = derived(
	fileStore,
	$fileStore => $fileStore?.files.filter(f => f.file_type === FileType.Document) || []
);

export const screenshots = derived(
	fileStore,
	$fileStore => $fileStore?.files.filter(f => f.is_screenshot) || []
);

export const nonScreenshots = derived(
	fileStore,
	$fileStore => $fileStore?.files.filter(f => !f.is_screenshot) || []
);

// Backward compatibility - keeping Photo type for now
export type Photo = FileMetadata;
