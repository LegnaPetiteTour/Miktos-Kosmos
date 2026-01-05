import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export interface Photo {
	file_name: string;
	file_path: string;
	file_size: number;
	width?: number;
	height?: number;
	date_taken?: string;
	is_screenshot: boolean;
	camera_make?: string;
	camera_model?: string;
}

export interface ScanStats {
	total_files: number;
	total_photos: number;
	total_videos: number;
	total_screenshots: number;
	total_duplicates: number;
	total_size: number;
	date_range_start?: string;
	date_range_end?: string;
}

export interface ScanResult {
	photos: Photo[];
	stats: ScanStats;
}

// Get initial value from localStorage if in browser
const getInitialValue = (): ScanResult | null => {
	if (browser) {
		const stored = localStorage.getItem('photoStore');
		if (stored) {
			try {
				return JSON.parse(stored);
			} catch {
				return null;
			}
		}
	}
	return null;
};

// Store for scan results with localStorage persistence
function createPhotoStore() {
	const { subscribe, set, update } = writable<ScanResult | null>(getInitialValue());
	
	return {
		subscribe,
		setScanResult: (result: ScanResult) => {
			set(result);
			if (browser) {
				localStorage.setItem('photoStore', JSON.stringify(result));
			}
		},
		clear: () => {
			set(null);
			if (browser) {
				localStorage.removeItem('photoStore');
			}
		},
		updatePhoto: (filePath: string, updates: Partial<Photo>) => {
			update(state => {
				if (!state) return state;
				const newState = {
					...state,
					photos: state.photos.map(photo => 
						photo.file_path === filePath ? { ...photo, ...updates } : photo
					)
				};
				if (browser) {
					localStorage.setItem('photoStore', JSON.stringify(newState));
				}
				return newState;
			});
		}
	};
}

export const photoStore = createPhotoStore();

// Store for selected folder path
export const selectedFolderStore = writable<string>('');

// Store for current scanning state
export const scanningStore = writable<boolean>(false);

// Derived stores for common queries
export const photoCount = derived(
	photoStore,
	$photoStore => $photoStore?.photos.length || 0
);

export const totalSize = derived(
	photoStore,
	$photoStore => $photoStore?.stats.total_size || 0
);

export const screenshots = derived(
	photoStore,
	$photoStore => $photoStore?.photos.filter(p => p.is_screenshot) || []
);

export const nonScreenshots = derived(
	photoStore,
	$photoStore => $photoStore?.photos.filter(p => !p.is_screenshot) || []
);
