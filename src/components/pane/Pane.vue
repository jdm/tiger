<template>
	<div class="flex flex-col">
		<div class="flex bg-zinc-900">
			<slot name="header"></slot>
			<div class="flex-1 border-zinc-700"></div>
		</div>
		<div class="flex-1 px-4 py-2 overflow-y-auto bg-zinc-800 text-gray-400">
			<slot name="content"></slot>
		</div>
	</div>
</template>
