import { writable } from 'svelte/store';
import type { OperationResult } from '$lib/types';

function createOperationsStore() {
	const { subscribe, update } = writable<OperationResult[]>([]);
	
	return {
		subscribe,
		addOperation: (operation: OperationResult) => {
			update(operations => [operation, ...operations]);
		},
		clear: () => {
			update(() => []);
		}
	};
}

export const operationsStore = createOperationsStore();
