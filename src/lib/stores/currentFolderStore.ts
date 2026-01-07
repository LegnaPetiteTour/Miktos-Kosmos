import { writable } from 'svelte/store';

export interface CurrentFolder {
	path: string;
	name: string;
	files: any[];
	loading: boolean;
}

function createCurrentFolderStore() {
	const { subscribe, set, update } = writable<CurrentFolder>({
		path: '',
		name: '',
		files: [],
		loading: false
	});
	
	return {
		subscribe,
		setFolder: (path: string, name: string) => {
			update(state => ({
				...state,
				path,
				name,
				loading: true
			}));
		},
		setFiles: (files: any[]) => {
			update(state => ({
				...state,
				files,
				loading: false
			}));
		},
		setLoading: (loading: boolean) => {
			update(state => ({
				...state,
				loading
			}));
		},
		clear: () => {
			set({
				path: '',
				name: '',
				files: [],
				loading: false
			});
		}
	};
}

export const currentFolderStore = createCurrentFolderStore();
