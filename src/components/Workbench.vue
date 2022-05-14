<template>
	<div class="graph-paper" />
</template>

<style scoped>
.graph-paper {
	background:
		linear-gradient(-90deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(-90deg, theme('colors.neutral.800') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.800') 1px, transparent 1px),
		theme('colors.neutral.900');
	background-size:
		128px 128px,
		128px 128px,
		16px 16px,
		16px 16px;
	background-position: -64px -64px, -64px -64px, -64px -64px;
}
</style>