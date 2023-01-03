<template>
	<button type="button" class="h-11 rounded-md cursor-pointer" :class="buttonClasses" @mouseenter="onMouseEnter"
		@mouseleave="onMouseLeave" @mousedown="onMouseDown" @mouseup="onMouseUp" @click="onClick" :disabled="disabled"
		:tabindex="tabIndex">
		<div class="h-full rounded-md" :class="outline">
			<div class="relative w-full h-full rounded-md overflow-clip border-t border-b-2 outline-offset-2 active:outline-0"
				:class="palette">
				<div class="h-full px-3 flex flex-row items-center justify-center">
					<div class="flex items-center justify-center gap-2 text-sm font-medium">
						<component :is="icon" v-if="icon" class="w-5" :class="active ? 'translate-y-px' : ''" />
						<div v-if="label" :class="active ? 'translate-y-px' : ''">{{ label }}</div>
					</div>
				</div>
				<div class="absolute w-full h-full top-0 left-0 blur-md scale-75 mix-blend-screen" :class="glow"
					v-if="hovered" />
			</div>
		</div>
	</button>
</template>

<script setup lang="ts">
import type { Component } from "vue"
import { computed, ref } from "vue"

type ButtonColor = "pink";

const props = defineProps<{
	label?: string,
	disabled?: boolean,
	tabbable?: boolean,
	positive?: boolean,
	danger?: boolean,
	customColor?: ButtonColor,
	icon?: Component,
}>();

const emit = defineEmits<{
	(e: 'click'): void
}>();

const hovered = ref(false);
const active = ref(false);
const tabIndex = computed(() => props.tabbable ? 0 : -1);

function onClick(event: MouseEvent) {
	(event.currentTarget as HTMLButtonElement)?.blur();
	emit('click');
}

const buttonClasses = computed(() => [
	...active.value? ["pt-px"] : [],
	...props.tabbable && !active.value ? ["focus:outline-2", "focus:outline-blue-500", "focus:outline-dotted"] : ["focus:outline-0"],
]);

const outline = computed(() => {
	if (props.disabled){
		return ["border-2", "border-plastic-600"];
	}
	return ["border-2", "border-plastic-900"];
});

const palette = computed(() => {
	if (props.disabled){
		return ["text-plastic-600", "bg-plastic-800", "border-none"];
	} else if (props.customColor == "pink") {
		return ["text-pink-100", "bg-gradient-to-b", "from-pink-800", "to-pink-600", "border-t-pink-600", "border-b-pink-900"];
	} else if (props.danger) {
		return ["text-red-100", "bg-gradient-to-b", "from-red-800", "to-red-600", "border-t-red-600", "border-b-red-900"];
	} else if (props.positive) {
		return ["text-green-100", "bg-gradient-to-b", "from-green-800", "to-green-600", "border-t-green-600", "border-b-green-900"];
	}
	return ["text-plastic-200", "bg-gradient-to-b", "from-plastic-600", "to-plastic-500", "border-t-plastic-500", "border-b-plastic-700"];
});

const glow = computed(() => {
	if (props.disabled){
		return ["bg-plastic-500/50"];
	} else if (props.customColor == "pink") {
		return ["bg-fuchsia-500/50"];
	} else if (props.danger) {
		return ["bg-rose-500/50"];
	} else if (props.positive) {
		return ["bg-teal-500/50"];
	}
	return ["bg-plastic-500/50"];
});

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
	active.value = false;
}

function onMouseDown() {
	active.value = true;
}

function onMouseUp() {
	active.value = false;
}
</script>