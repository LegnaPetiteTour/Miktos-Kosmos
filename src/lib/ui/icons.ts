/**
 * Icon System for Miktos Kosmos
 * Clean outline icons using Unicode characters
 */

export const icons = {
	// Navigation
	home: '⌂',           // House
	workspace: '□',      // Folder/workspace
	analyze: '◎',        // Analyze/search target
	transform: '◈',      // Transform diamond
	review: '☑',        // Review checkbox
	learn: '◷',          // Learn/time
	settings: '⚙',       // Settings gear
	about: 'ⓘ',          // About info
	
	// Stats & Metrics
	folder: '□',         // Folder
	files: '⊟',          // Files stack
	storage: '⊞',        // Storage/database
	calendar: '⊟',       // Calendar
	clock: '◷',          // Time/dates
	diamond: '◇',        // Status/badge
	chart: '◫',          // Chart/analytics
	
	// Actions
	add: '⊕',            // Add/new
	remove: '⊖',         // Remove/delete
	close: '⊗',          // Close/cancel
	check: '✓',          // Success/done
	search: '⊙',         // Search
	copy: '⧉',           // Copy
	move: '⇄',           // Move/transfer
	upload: '⬆',         // Upload/import
	download: '⬇',       // Download/export
	
	// States
	success: '✓',        // Success
	warning: '⚠',        // Warning  
	error: '✕',          // Error
	info: 'ⓘ',           // Info
	pending: '◷',        // Pending/loading
	
	// File Types
	image: '▭',          // Image
	video: '▶',          // Video
	document: '⊞',       // Document
	audio: '♪',          // Audio
	archive: '⊟',        // Archive/zip
	
	// UI Elements
	dot: '●',            // Bullet/indicator
	circle: '○',         // Empty circle
	dotOutline: '◌',     // Dot outline
	square: '□',         // Square
	squareFilled: '■',   // Filled square
	
	// Theme
	sun: '◐',            // Light mode (half sun)
	moon: '◑',           // Dark mode (half moon)
} as const;

export type IconName = keyof typeof icons;

// Helper to get icon by name
export function getIcon(name: IconName): string {
	return icons[name];
}
