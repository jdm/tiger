<template>
	<div class="flex flex-col">
		<h2 class="px-3 py-1 bg-zinc-800 text-slate-50 font-semibold">
			<slot name="title"></slot>
		</h2>
		<div class="flex-1 px-4 py-2 overflow-y-auto bg-neutral-900 border-r border-zinc-800 text-gray-400">
			<slot name="content"></slot>
		</div>
	</div>
</template>