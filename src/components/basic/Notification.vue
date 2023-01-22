<template>
	<div class="pointer-events-auto w-full rounded-md shadow-lg" :class="boxClass" @mouseenter="onMouseEnter"
		@mouseleave="onMouseLeave">
		<div class="flex p-3 gap-4">
			<component :is="icon" class="self-start shrink-0 mt-px w-5" :class="highlightClass" />
			<div class="grow flex flex-col gap-0.5 text-sm items-start">
				<div class="font-medium" :class="highlightClass">{{ title }}</div>
				<div v-html="description" />
				<div class="flex gap-4">
					<div v-for="action in actions"
						class="cursor-pointer -m-1.5 -ml-3 mt-0.5 rounded-md  p-1.5 px-3 text-md font-medium underline-offset-2 hover:underline"
						:class="highlightClass" @click="action.callback">
						<a :href="action.url" target="_blank">
							{{ action.text}}
						</a>
					</div>
				</div>
			</div>
			<XMarkIcon class="self-start shrink-0 -m-1.5 cursor-pointer w-8 p-1.5 rounded-lg" :class="closeClass"
				@click="close" />
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue"
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { XMarkIcon } from "@heroicons/vue/20/solid"

export type Flavor = "success" | "error" | "neutral";

export type NotificationData = {
	flavor: Flavor,
	title: string,
	icon: Component,
	description: string,
	actions?: { text: string, callback?: () => void, url?: string }[],
};

const props = defineProps<{
	id: number,
	flavor: Flavor,
	title: string,
	icon: Component,
	description: string,
	actions?: { text: string, callback?: () => void, url?: string }[],
}>();

const emit = defineEmits<{
	(e: "close", id: number): void
}>();

const lifetimeMillis = ref(5000);
let timerHandle: number | undefined = undefined;
let hovered = false;

onMounted(() => {
	const periodMillis = 100;
	timerHandle = window.setInterval(() => {
		if (!hovered || lifetimeMillis.value > 2000) {
			lifetimeMillis.value -= periodMillis;
		}
	}, periodMillis);
});

onUnmounted(() => {
	if (timerHandle) {
		window.clearInterval(timerHandle);
		timerHandle = undefined;
	}
});

watch(lifetimeMillis, () => {
	if (lifetimeMillis.value <= 0) {
		close();
	}
});

const boxClass = computed(() => [
	...props.flavor == "success" ? ["bg-green-600", "text-green-200", "shadow-green-900/50"] : [],
	...props.flavor == "error" ? ["bg-red-600", "text-red-200", "shadow-red-900/50"] : [],
	...props.flavor == "neutral" ? ["bg-zinc-900", "text-zinc-200", "shadow-black/25"] : [],
]);

const highlightClass = computed(() => [
	...props.flavor == "success" ? ["text-green-100"] : [],
	...props.flavor == "error" ? ["text-red-100"] : [],
	...props.flavor == "neutral" ? ["text-zinc-100"] : [],
]);

const closeClass = computed(() => [
	...props.flavor == "success" ? ["hover:bg-green-800"] : [],
	...props.flavor == "error" ? ["hover:bg-red-800"] : [],
	...props.flavor == "neutral" ? ["hover:bg-zinc-800"] : [],
]);

function onMouseEnter() {
	hovered = true;
}

function onMouseLeave() {
	hovered = false;
}

function close() {
	emit("close", props.id);
}
</script>
