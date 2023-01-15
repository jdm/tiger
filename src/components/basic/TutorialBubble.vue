<template>
	<div ref="el" v-if="!inactive">
		<FloatingWidget open ignore-pointer-events :position="position">
			<div :class="bubblePosition">
				<div class="p-4 pointer-events-auto" :class="attract">
					<Bubble :visible="stepActive" help delayed :arrow-position="arrowPosition">
						<slot />
					</Bubble>
				</div>
			</div>
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import { computed, onMounted, Ref, ref, watch } from "vue";
import { debounceAnimation } from "@/utils/animation";
import Bubble, { ArrowPosition } from "@/components/basic/Bubble.vue"
import FloatingWidget from "@/components/basic/FloatingWidget.vue"

const props = defineProps<{
	tutorialActive: boolean,
	stepActive: boolean,
	arrowPosition?: ArrowPosition,
}>();

const el: Ref<HTMLElement | null> = ref(null);
const position = ref([0, 0] as [number, number]);
const inactive = debounceAnimation([() => props.tutorialActive], () => !props.tutorialActive);

const resizeObserver = new ResizeObserver(entries => {
	for (let entry of entries) {
		if (entry.target === document.body) {
			updatePosition();
		}
	}
});

watch(el, updatePosition);

onMounted(() => {
	resizeObserver.observe(document.body);
});

function updatePosition() {
	if (!el.value) {
		return;
	}
	const boundingBox = el.value.getBoundingClientRect();
	position.value = [boundingBox.x, boundingBox.y];
}

const bubblePosition = computed(() => {
	const arrow = props.arrowPosition;
	return [
		...arrow == "left" || arrow == "right" ? ["-translate-y-1/2"] : [],
		...arrow == "bottom" || arrow == "top" ? ["-translate-x-1/2"] : [],
		...arrow == "right" ? ["-translate-x-full"] : [],
		...arrow == "bottom" ? ["-translate-y-full"] : [],
	];
});

const attract = computed(() => {
	const arrow = props.arrowPosition;
	return [
		...arrow == "left" ? ["attract-left"] : [],
		...arrow == "right" ? ["attract-right"] : [],
		...arrow == "top" ? ["attract-top"] : [],
		...arrow == "bottom" ? ["attract-bottom"] : [],
	];
});

</script>

<style>
.attract-left {
	animation: 0.6s infinite alternate attract-left;
	animation-timing-function: cubic-bezier(.25, .1, .8, 1);
}

.attract-right {
	animation: 0.6s infinite alternate attract-right;
	animation-timing-function: cubic-bezier(.25, .1, .8, 1);
}

.attract-top {
	animation: 0.5s infinite alternate attract-top;
	animation-timing-function: cubic-bezier(.25, .1, .8, 1);
}

.attract-bottom {
	animation: 0.5s infinite alternate attract-bottom;
	animation-timing-function: cubic-bezier(.25, .1, .8, 1);
}

@keyframes attract-left {
	from {
		transform: translate(0, 0);
	}

	to {
		transform: translate(10px, 0);
	}
}

@keyframes attract-right {
	from {
		transform: translate(0, 0);
	}

	to {
		transform: translate(-10px, 0);
	}
}

@keyframes attract-top {
	from {
		transform: translate(0, 0);
	}

	to {
		transform: translate(0, 4px);
	}
}

@keyframes attract-bottom {
	from {
		transform: translate(0, 0);
	}

	to {
		transform: translate(0, -4px);
	}
}
</style>