<script>
	import Cardano from "../svgs/coins/cardano.svelte";
	import Ethereum from "../svgs/coins/ethereum.svelte";
	import Bitcoin from "../svgs/coins/bitcoin.svelte";
	import { page } from "../store";
	import { Link } from "svelte-routing";
</script>

<div class="relative">
	<div class="absolute inset-x-0 bottom-0 h-1/2 bg-gray-100" />
	<div class="max-w-7xl mx-auto sm:px-6 lg:px-8">
		<div class="relative shadow-xl sm:rounded-2xl sm:overflow-hidden">
			<div class="absolute inset-0">
				<img class="h-full w-full object-cover" src="/crypt.jpg" alt="Crypt" />
				<div class="absolute inset-0 bg-gradient-to-r from-orange-800  to-amber-700 mix-blend-multiply" />
			</div>
			<div class="relative px-4 py-16 sm:px-6 sm:py-24 lg:py-32 lg:px-8">
				<h1 class="text-center text-4xl font-extrabold tracking-tight sm:text-5xl lg:text-6xl">
					<span class="block text-white">Hold Crypto</span>
					<span class="block text-amber-200">while earning rewards</span>
				</h1>
				<p class="mt-6 max-w-lg mx-auto text-center text-xl text-amber-100 sm:max-w-3xl">
					Easily invest in the highest yield rewards for your coins, with an option to lock the assets until a given end date. Perfect as a
					gift for someone who doesn't have access to a crytpo account, or to stop yourself from withdrawing and losing money on trades!
				</p>
				<div class="mt-10 max-w-sm mx-auto flex justify-center">
					<div class="mx-auto inline-grid grid-cols-2 gap-5">
						<Link
							class="flex items-center py-2 justify-center border border-transparent text-base font-medium rounded-md shadow-sm text-amber-700 bg-white hover:bg-amber-50 px-8"
							to="/demo">Demo</Link>
						<Link
							class="flex items-center justify-center border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-amber-500 bg-opacity-60 hover:bg-opacity-70 px-8"
							to="/contact">Contact</Link>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- Logo Cloud -->
<div class="bg-gray-100">
	<div class="max-w-7xl mx-auto py-16 px-4 sm:px-6 lg:px-8">
		<p class="text-center text-sm font-semibold uppercase text-gray-500 tracking-wide">Supported Coins</p>
		<div class="mt-6 grid grid-cols-2 gap-8 md:grid-cols-3 lg:grid-cols-3">
			<div class="col-span-1 h-40 flex justify-center">
				<Ethereum />
			</div>
			<div class="col-span-1 h-40 flex justify-center">
				<Cardano />
			</div>
			<div class="col-span-1 h-40 flex justify-center">
				<Bitcoin />
			</div>
		</div>
	</div>
</div>
