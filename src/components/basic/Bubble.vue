<template>
	<Transition appear :name="intro">
		<div v-if="visible" class="relative max-w-md text-sm font-medium p-2 px-4 rounded-sm shadow-lg shadow-black/30"
			:class="containerClass">
			<div class="z-10">
				<slot />
			</div>
			<div v-if="arrowLeft" class="absolute -left-1.5 top-1/2 -translate-y-1/2 w-3 h-3 rotate-45"
				:class="palette" />
			<div v-if="arrowRight" class="absolute -right-1.5 top-1/2 -translate-y-1/2 w-3 h-3 rotate-45"
				:class="palette" />
			<div v-if="arrowBottom" class="absolute -bottom-1.5 left-1/2 -translate-x-1/2 w-3 h-3 rotate-45"
				:class="palette" />
			<div v-if="arrowTop" class="z-0 absolute -top-1.5 left-1/2 -translate-x-1/2 w-3 h-3 rotate-45"
				:class="palette" />
		</div>
	</Transition>
</template>

<script setup lang="ts">
import { computed } from "vue";

export type ArrowPosition = "left" | "right" | "top" | "bottom";

const props = defineProps<{
	visible: boolean,
	delayed?: boolean,
	error?: boolean,
	help?: boolean,
	arrowPosition?: ArrowPosition,
}>();

const arrowRight = computed(() => !props.arrowPosition || props.arrowPosition == "right");
const arrowLeft = computed(() => props.arrowPosition == "left");
const arrowTop = computed(() => props.arrowPosition == "top");
const arrowBottom = computed(() => props.arrowPosition == "bottom");

const containerClass = computed(() => [
	...palette.value,
	...props.delayed ? ["delayed"] : [],
]);

const palette = computed(() => {
	if (props.error) {
		return ["bg-red-600", "text-red-100"];
	}
	if (props.help) {
		return ["bg-amber-400", "text-amber-800"];
	}
	return ["bg-gray-300", "text-gray-900"];
});

const intro = computed(() => {
	if (props.arrowPosition == "left")  {
		return "intro-left";
	} else if (props.arrowPosition == "top")  {
		return "intro-top";
	} else if (props.arrowPosition == "bottom")  {
		return "intro-bottom";
	}
	return "intro-right";
});
</script>

<style>
.intro-left-enter-active.delayed,
.intro-right-enter-active.delayed,
.intro-top-enter-active.delayed,
.intro-bottom-enter-active.delayed {
	transition-delay: 300ms;
}

.intro-left-enter-active,
.intro-right-enter-active,
.intro-top-enter-active,
.intro-bottom-enter-active {
	transition: all 0.3s ease-out;
}

.intro-left-leave-active,
.intro-right-leave-active,
.intro-top-leave-active,
.intro-bottom-leave-active {
	transition: all 0.2s ease-in;
}

.intro-left-enter-from,
.intro-left-leave-to {
	transform: translateX(10px);
	opacity: 0%;
}

.intro-right-enter-from,
.intro-right-leave-to {
	transform: translateX(-10px);
	opacity: 0%;
}

.intro-top-enter-from,
.intro-top-leave-to {
	transform: translateY(6px);
	opacity: 0%;
}

.intro-bottom-enter-from,
.intro-bottom-leave-to {
	transform: translateY(-6px);
	opacity: 0%;
}
</style>