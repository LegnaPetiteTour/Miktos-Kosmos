import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface HistoryEntry {
	id: string;
	timestamp: string;
	folder_path: string;
	total_files: number;
	total_size: number;
	date_range_start?: string;
	date_range_end?: string;
	file_types: {
		images: number;
		videos: number;
		documents: number;
		audio: number;
		archives: number;
		other: number;
	};
	errors: number;
	warnings: number;
	status: 'success' | 'warning' | 'error';
}

// Get initial value from localStorage
const getInitialValue = (): HistoryEntry[] => {
	if (browser) {
		const stored = localStorage.getItem('workspaceHistory');
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

// Create history store
function createHistoryStore() {
	const { subscribe, set, update } = writable<HistoryEntry[]>(getInitialValue());
	
	return {
		subscribe,
		addEntry: (entry: HistoryEntry) => {
			update(history => {
				const newHistory = [entry, ...history]; // Newest first
				if (browser) {
					localStorage.setItem('workspaceHistory', JSON.stringify(newHistory));
				}
				return newHistory;
			});
		},
		clear: () => {
			set([]);
			if (browser) {
				localStorage.removeItem('workspaceHistory');
			}
		},
		removeEntry: (id: string) => {
			update(history => {
				const newHistory = history.filter(h => h.id !== id);
				if (browser) {
					localStorage.setItem('workspaceHistory', JSON.stringify(newHistory));
				}
				return newHistory;
			});
		}
	};
}

export const historyStore = createHistoryStore();
