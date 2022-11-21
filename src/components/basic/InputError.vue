<template>
	<Bubble error>
		<div class="flex pr-2 whitespace-pre-wrap">
			<ExclamationIcon class="ml-2 mr-4 w-6 flex-none" />
			<div class="flex-col">
				<div>{{ shortErrorText }}</div>
				<a v-if="longErrorText" class="block mt-0.5 underline cursor-pointer" @click="openDetails">View
					details</a>
			</div>
		</div>
	</Bubble>
</template>

<script setup lang="ts">
import { ExclamationIcon } from "@heroicons/vue/solid"
import { showErrorMessage } from "@/api/app";
import Bubble from "@/components/basic/Bubble.vue"

const props = defineProps<{
	shortErrorText: string,
	longErrorText?: string,
}>();

function openDetails() {
	if (props.longErrorText) {
		showErrorMessage("Error Details", props.shortErrorText, props.longErrorText);
	}
}
</script>