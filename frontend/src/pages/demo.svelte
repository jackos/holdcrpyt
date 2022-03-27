<script lang="ts">
	import { onMount } from "svelte";
	import { users } from "../store";

	const apiUrl = "https://8itei109kk.execute-api.ap-southeast-2.amazonaws.com/Stage/v1";

	const fetchUsers = async () => {
		const res = await fetch(`${apiUrl}/users`);
		const data = await res.json();
		if (res.ok) {
			$users = data;
		} else {
			throw new Error(`Unable to fetch coins`);
		}
	};

	onMount(() => {
		async function fetchWrapper() {
			await fetchUsers();
		}
		fetchWrapper();
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
							{#each $users as user}
								{#each user.coins as coin, i}
									<tr>
										{#if i == 0}
											<td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{user.first_name}</td>
										{:else}
											<td class="px-6 py-4 whitespace-nowrap text-sm  text-gray-900" />
										{/if}
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{coin.name}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{coin.price.toFixed(2)}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{coin.amount.toFixed(4)}</td>
										<td class="px-6 py-4 whitespace-nowrap text-sm text-green-600">{(coin.amount * coin.price).toFixed(2)}</td>
									</tr>{/each}
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
