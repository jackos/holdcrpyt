import { writable, derived, get } from 'svelte/store';
import type { Writable, Readable } from 'svelte/store';

export const page = writable("landing");

export interface User {
	username: string;
	first_name: string;
	last_name: string;
	coins: Coin[];
}

export interface Coin {
	name: string;
	symbol: string;
	price: number;
	amount: number;
}

export const users: Writable<User[]> = writable([]);
