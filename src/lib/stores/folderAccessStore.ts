import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface FolderAccess {
	path: string;
	name: string;
	count: number;
	last_accessed: string;
}

// Get initial value from localStorage
const getInitialValue = (): FolderAccess[] => {
	if (browser) {
		const stored = localStorage.getItem('folderAccessHistory');
		if (stored) {
			try {
				return JSON.parse(stored);
			} catch {
				return [];
			}
		}
	}
	return [];
};

// Create folder access tracking store
function createFolderAccessStore() {
	const { subscribe, set, update } = writable<FolderAccess[]>(getInitialValue());
	
	return {
		subscribe,
		trackAccess: (path: string, name: string) => {
			update(history => {
				const existing = history.find(h => h.path === path);
				
				let newHistory;
				if (existing) {
					// Increment count and update last accessed
					newHistory = history.map(h => 
						h.path === path 
							? { ...h, count: h.count + 1, last_accessed: new Date().toISOString() }
							: h
					);
				} else {
					// Add new folder with count 1
					newHistory = [...history, {
						path,
						name,
						count: 1,
						last_accessed: new Date().toISOString()
					}];
				}
				
				if (browser) {
					localStorage.setItem('folderAccessHistory', JSON.stringify(newHistory));
				}
				
				return newHistory;
			});
		},
		getFavorites: (): FolderAccess[] => {
			let currentHistory: FolderAccess[] = [];
			subscribe(value => {
				currentHistory = value;
			})();
			
			// Return folders accessed 2 or more times, sorted by count
			return currentHistory
				.filter(h => h.count >= 2)
				.sort((a, b) => b.count - a.count);
		},
		clear: () => {
			set([]);
			if (browser) {
				localStorage.removeItem('folderAccessHistory');
			}
		}
	};
}

export const folderAccessStore = createFolderAccessStore();
