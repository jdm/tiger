<template>
	<div ref="el" @click="onClicked" @mousedown="onMouseDown" @mouseup="onMouseUp" @mouseenter="onMouseEnter"
		@mouseleave="onMouseLeave" class="h-11 cursor-pointer" :class="drawActive ? 'pt-px' : ''">
		<div class="h-full rounded-md border-2 border-plastic-900">
			<div class="h-full pl-2 flex gap-1 items-center rounded-md border-2 overflow-clip" :class="dynamicClasses">
				<div class="relative h-6 w-6">
					<component :is="icon" class="absolute w-6" />
					<div class="absolute w-6 h-6 blur-md bg-sky-500 scale-75 mix-blend-screen"
						v-if="drawHover && toggled" />
				</div>
				<ChevronDownIcon v-if="canExpand" @click.stop="expand" @mousedown="onMouseDownChevron"
					@mouseenter="onMouseEnterChevron" @mouseleave="onMouseLeaveChevron" class="w-7 h-7 p-1 rounded-lg"
					:class="chevronClasses" />
			</div>
		</div>
		<FloatingWidget :open="expanded" :position="expansionPosition" @dismissed="close">
			<slot name="expanded" />
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue"
import { computed, Ref, ref } from "vue"
import { ChevronDownIcon } from "@heroicons/vue/20/solid";
import FloatingWidget from "@/components/basic/FloatingWidget.vue";

const props = defineProps<{
	icon: Component,
	toggled: boolean,
	canExpand?: boolean,
}>();

const emit = defineEmits<{
	(e: "toggled", newValue: boolean): void
}>();

const el: Ref<HTMLElement | null> = ref(null);
const hovered = ref(false);
const active = ref(false);
const chevronHovered = ref(false);
const expanded = ref(false);
const expansionPosition = ref([0, 0] as [number, number]);
let mouseDownWhileExpanded = false;

const drawActive = computed(() => active.value && !chevronHovered.value);
const drawHover = computed(() => hovered.value && !chevronHovered.value);

const chevronClasses = computed(() => [
	...!expanded.value ? ["hover:bg-zinc-900", "hover:text-zinc-300"] : [],
	...expanded.value && props.toggled ? ["bg-zinc-200", "text-zinc-800"] : [],
	...expanded.value && !props.toggled ? ["bg-zinc-200", "text-zinc-800"] : [],
]);

const dynamicClasses = computed(() =>  [
	...props.canExpand ? ["pr-1"] : ["pr-2"],

	...props.toggled && !drawHover.value ? ["text-blue-200", "border-blue-600", "border-t-blue-500",] : [],
	...props.toggled && drawHover.value ? ["text-blue-100", "border-blue-500", "border-t-blue-400",] : [],
	...!props.toggled && !drawHover.value ? ["bg-zinc-900", "text-zinc-700", "border-zinc-700",] : [],
	...!props.toggled && drawHover.value ? ["bg-zinc-800", "text-zinc-600", "border-zinc-700",] : [],
	...props.toggled ? ["bg-gradient-to-b", "from-blue-900", "to-blue-700"] : [],
]);

function onClicked() {
	emit("toggled", !props.toggled);
}

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

function onMouseDownChevron() {
	mouseDownWhileExpanded = expanded.value;
}

function onMouseEnterChevron() {
	chevronHovered.value = true;
}

function onMouseLeaveChevron() {
	chevronHovered.value = false;
}

function expand() {
	if (mouseDownWhileExpanded) {
		return;
	}
	if (el.value) {
		const boundingBox = el.value.getBoundingClientRect();
		const x = (boundingBox.left + boundingBox.right) / 2;
		const y = boundingBox.bottom;
		expansionPosition.value = [x, y];
	}
	expanded.value = true;
}

function close() {
	expanded.value = false;
}
</script>
