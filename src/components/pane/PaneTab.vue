<template>
	<h2 @click="$emit('select')" class="flex-initial mr-px px-3 py-1 font-medium uppercase text-xs"
		:class="selected ? selectedClasses : unselectedClasses">
		<slot></slot>
	</h2>
</template>


<script setup lang="ts">
const selectedClasses = " bg-zinc-800 text-slate-50 border-b-0";
const unselectedClasses = "bg-zinc-900 text-zinc-500 hover:bg-zinc-800";
defineProps<{
	selected?: Boolean
}>()
</script>