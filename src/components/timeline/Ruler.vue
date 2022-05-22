<template>
	<div class="h-6 px-1 w-[2800px] min-w-full ruler" />
</template>

<style scoped>
.ruler {
	background:
		/* 1s ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme('colors.plastic.600') repeat-x;
	background-origin: content-box;
	background-size:
		1000px 100%,
		100px 10px,
		10px 4px,
		100% 100%;
}
</style>