<template>
	<li>
		<a href="#" :class="selected ? activeClasses : inactiveClasses">
			<slot></slot>
		</a>
	</li>
</template>

<script setup lang="ts">
const props = defineProps({
	selected: Boolean
})
const activeClasses = "inline-block py-3 px-4 text-white bg-blue-600 active";
const inactiveClasses = "inline-block py-3 px-4 hover:bg-gray-800 hover:text-white";
</script>