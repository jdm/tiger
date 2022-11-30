<template>
	<div class="rounded-md bg-plastic-900 border-4 border-plastic-700 shadow-lg shadow-blue-900/25">
		<div class="flex p-6 pb-5 gap-6 bg-plastic-900">
			<div class="h-9 w-9 flex-shrink-0 flex items-center justify-center rounded-full"
				:class="error ? 'bg-red-400' : 'bg-amber-300'">
				<component :is="icon" class="h-6 w-6" :class="error ? 'text-red-900' : 'text-amber-900'" />
			</div>
			<div>
				<h3 class="text-lg leading-6 font-medium text-plastic-100">{{ title }}</h3>
				<div class="mt-3 text-sm text-plastic-300">
					<slot name="body"></slot>
				</div>
			</div>
		</div>
		<div class="bg-plastic-800 px-6 py-3 flex justify-end gap-3">
			<slot name="actions"></slot>
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue";

defineProps<{
	title: string,
	icon: Component,
	error?: boolean,
}>();
</script>