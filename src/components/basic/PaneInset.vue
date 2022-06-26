<template>
	<div class="relative rounded-md overflow-hidden bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
		<slot></slot>
		<div class="absolute top-0 left-0 w-full h-full shadow-inner shadow-zinc-900/70 pointer-events-none" />
	</div>
</template>