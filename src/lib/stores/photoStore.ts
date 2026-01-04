import { writable, derived } from 'svelte/store';

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

// Store for scan results
function createPhotoStore() {
	const { subscribe, set, update } = writable<ScanResult | null>(null);
	
	return {
		subscribe,
		setScanResult: (result: ScanResult) => set(result),
		clear: () => set(null),
		updatePhoto: (filePath: string, updates: Partial<Photo>) => {
			update(state => {
				if (!state) return state;
				return {
					...state,
					photos: state.photos.map(photo => 
						photo.file_path === filePath ? { ...photo, ...updates } : photo
					)
				};
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
