<template>
	<Bubble error :visible="visible">
		<div class="flex whitespace-pre-wrap items-center">
			<ExclamationTriangleIcon class="mr-4 w-6 flex-none" />
			<div class="flex-col">
				<div>{{ shortErrorText }}</div>
				<a v-if="longErrorText" class="block mt-0.5 underline cursor-pointer" @click="openDetails">View
					details</a>
			</div>
		</div>
	</Bubble>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid"
import { showErrorMessage } from "@/backend/api";
import Bubble from "@/components/basic/Bubble.vue"

const props = defineProps<{
	visible: boolean,
	shortErrorText: string,
	longErrorText?: string,
}>();

function openDetails() {
	if (props.longErrorText) {
		showErrorMessage("Error Details", props.shortErrorText, props.longErrorText);
	}
}
</script>