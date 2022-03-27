import { writable, derived, get } from 'svelte/store';
import type { Writable, Readable } from 'svelte/store';

export const page = writable("landing");

export interface Holder {
	name: string;
	transactions: Transaction[];
}

export interface Coin {
	name: string;
	symbol: string;
	price: number;
}
export interface Transaction {
	coin: string;
	buy: number;
	price: number;
}
export interface Prices {
	lastUpdateId: number;
	bids: [[number, number]];
	asks: [[number, number]];
}

export interface Price {
	[key: string]: number;
}

export const prices: Writable<Price> = writable({});

export const coins: Writable<Coin[]> = writable([
	{ name: "Ethereum", symbol: "ETHAUD", price: 0 },
	{ name: "Cardano", symbol: "ADAAUD", price: 0 },
]);

export const holders: Writable<Holder[]> = writable([
	{
		name: "Aria",
		transactions: [
			{ coin: "Ethereum", buy: 0.0088044, price: 5694.36 },
			{ coin: "Cardano", buy: 27.586521622, price: 1.71 },
		],
	},
	{
		name: "Archer",
		transactions: [
			{ coin: "Ethereum", buy: 0.0088044, price: 5694.36 },
			{ coin: "Cardano", buy: 27.586521622, price: 1.71 },
		],
	},
	{
		name: "Benji",
		transactions: [
			{ coin: "Ethereum", buy: 0.0088044, price: 5694.36 },
			{ coin: "Cardano", buy: 27.586521622, price: 1.71 },
		],
	},
	{
		name: "Max",
		transactions: [
			{ coin: "Ethereum", buy: 0.0088044, price: 5694.36 },
			{ coin: "Cardano", buy: 27.586521622, price: 1.71 },
			{ coin: "Ethereum", buy: 0.01298147, price: 3815.99 },
		],
	},
	{
		name: "Cooper",
		transactions: [
			{ coin: "Ethereum", buy: 0.0088044, price: 5694.36 },
			{ coin: "Cardano", buy: 27.586521622, price: 1.71 },
			{ coin: "Ethereum", buy: 0.01298147, price: 3815.99 },
		],
	},
]);
