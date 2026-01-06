import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark';

function createThemeStore() {
	// Get initial theme from localStorage or system preference
	const getInitialTheme = (): Theme => {
		if (!browser) return 'light';
		
		const stored = localStorage.getItem('theme') as Theme | null;
		if (stored === 'light' || stored === 'dark') {
			return stored;
		}
		
		// Check system preference
		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		return prefersDark ? 'dark' : 'light';
	};
	
	const { subscribe, set } = writable<Theme>(getInitialTheme());
	
	return {
		subscribe,
		setTheme: (theme: Theme) => {
			if (browser) {
				localStorage.setItem('theme', theme);
				document.documentElement.setAttribute('data-theme', theme);
			}
			set(theme);
		},
		toggle: () => {
			if (!browser) return;
			
			const current = localStorage.getItem('theme') as Theme;
			const next: Theme = current === 'dark' ? 'light' : 'dark';
			
			localStorage.setItem('theme', next);
			document.documentElement.setAttribute('data-theme', next);
			set(next);
		},
		initialize: () => {
			if (!browser) return;
			
			const theme = getInitialTheme();
			document.documentElement.setAttribute('data-theme', theme);
			set(theme);
		}
	};
}

export const themeStore = createThemeStore();
