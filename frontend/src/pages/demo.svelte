<script lang="ts">
	import { onMount } from "svelte";
	import { prices, Transaction, coins, holders } from "../store";

	const apiUrl = "https://8itei109kk.execute-api.ap-southeast-2.amazonaws.com/Stage/v1";

	function getTotalCoins(transactions: Transaction[], coin: string): number {
		let total = 0;
		for (const transaction of transactions) {
			if (transaction.coin == coin) {
				total += transaction.buy;
			}
		}
		return total;
	}

	const fetchUsers = async () => {
		const res = await fetch(`${apiUrl}/users`);
		const data = await res.json();
		if (res.ok) {
			$holders = data;
		} else {
			throw new Error(`Unable to fetch coins`);
		}
	};

	const fetchPrices = async () => {
		const res = await fetch(`${apiUrl}/coins`);
		const data = await res.json();
		if (res.ok) {
			for (let coin of $coins) {
				coin.price = data[coin.symbol];
			}
			$coins = $coins;
		} else {
			throw new Error(`Unable to fetch coins`);
		}
	};

	onMount(() => {
		async function fetchWrapper() {
			await fetchPrices();
			await fetchUsers();
		}
		fetchWrapper();
		console.log($prices);
	});
</script>

<div class="p-8">
	<div class="flex flex-col">
		<div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
				<div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
					<table class="min-w-full divide-y divide-gray-200">
						<thead class="bg-gray-50">
							<tr>
								<th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"> Name </th>
								<th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"> Currency </th>
								<th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									Trading Price
								</th>
								<th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"> Coins </th>
								<th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									Value (AUD)
								</th>
							</tr>
						</thead>
						<tbody class="bg-white divide-y divide-gray-200">
							{#each $holders as holder}
								{#each $coins as coin, i}
									<tr>
										{#if i == 0}
											<td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{holder.name}</td>
										{:else}
											<td class="px-6 py-4 whitespace-nowrap text-sm  text-gray-900" />
										{/if}
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{coin.name}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${coin.price.toFixed(2)}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500"
											>{getTotalCoins(holder.transactions, coin.name).toFixed(4)}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-green-600"
											>${(getTotalCoins(holder.transactions, coin.name) * coin.price).toFixed()}</td>
									</tr>{/each}
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
