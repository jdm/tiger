<template>
	<div class="pointer-events-auto w-96 rounded-md bg-green-600 text-green-200 shadow-lg shadow-green-900/25">
		<div class="flex p-3 gap-4">
			<DocumentCheckIcon class="self-start shrink-0 w-5 text-green-100" />
			<div class="flex flex-col gap-0.5 text-sm">
				<div class="font-medium text-green-100">Export Complete</div>
				<div>Successfully exported as <span class="font-medium text-yellow-200">sahagin.lua</span> and
					<span class="font-medium text-yellow-200">sahagin.png</span>.
				</div>
				<div class="mt-2 text-md font-medium text-green-100">View files</div>
			</div>
			<XMarkIcon class="self-start shrink-0 -m-1.5 cursor-pointer w-8 p-1.5 rounded-lg hover:bg-green-900" />
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue";
import { DocumentCheckIcon, XMarkIcon } from "@heroicons/vue/20/solid"

export type NotificationData = {
	text: string,
	actions?: { icon: Component, callback: () => void }[],
};

defineProps<{
	text: string,
}>();
</script>
