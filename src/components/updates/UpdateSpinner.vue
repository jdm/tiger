<template>
	<div class="flex items-center justify-center">
		<Transition appear>
			<div class="flex items-center gap-4 bg-amber-400 text-amber-900 py-5 px-10 rounded-md">
				<div class="h-5 w-5 rounded-full border-2 border-amber-900/50 border-t-amber-900 animate-spin" />
				<div class="font-semibold">Downloading Update…</div>
			</div>
		</Transition>
	</div>
</template>

<style scoped>
.v-enter-active {
	transition: all 0.2s ease-out;
}

.v-leave-active {
	transition: all 0.2s ease-out;
}

.v-enter-from,
.v-leave-to {
	transform: scaleX(0);
}
</style>