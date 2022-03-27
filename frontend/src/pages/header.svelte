<script>
	import Logo from "../svgs/Logo.svelte";
	import { link } from "svelte-routing";
</script>

<header>
	<div class="relative bg-white dark:bg-slate-800">
		<div class="flex justify-between items-center max-w-7xl mx-auto px-4 py-6 sm:px-6 md:justify-start md:space-x-10 lg:px-8">
			<div class="flex justify-start lg:w-0 lg:flex-1">
				<a href="/" use:link>
					<span class="sr-only">Workflow</span>
					<Logo />
				</a>
			</div>
			<div class="-mr-2 -my-2 md:hidden">
				<button
					type="button"
					class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-amber-500"
					aria-expanded="false">
					<span class="sr-only">Open menu</span>
					<!-- Heroicon name: outline/menu -->
					<svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
					</svg>
				</button>
			</div>
			<div class="hidden md:flex items-center justify-end md:flex-1 lg:w-0">
				<a href="/development" use:link class="whitespace-nowrap text-base font-medium text-gray-500 hover:text-gray-900"> Sign in </a>
				<a
					href="/development"
					use:link
					class="ml-8 whitespace-nowrap inline-flex items-center justify-center bg-gradient-to-r from-orange-600 to-amber-600 bg-origin-border px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white hover:from-orange-700 hover:to-amber-700">
					Sign up
				</a>
			</div>
		</div>
	</div>
</header>
