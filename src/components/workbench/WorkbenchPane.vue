<template>
	<Pane class="overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{document.name + (document.hasUnsavedChanges ? "*" : "") }}</PaneTab>
		</PaneTabList>
		<div class="relative flex-1 overflow-hidden pointer-events-none" ref="drawingArea">

			<!--
				Z-Ordering
				----------
				0 sprite BG
				10 sprite
				20 sprite outline
				30 origin
				30 hitbox BG & outline (not selected)
				31 hitbox label (not selected)
				40 sprite drag area
				50 hitbox BG & outline (selected)
				51 hitbox label (selected)
				60 hitbox drag area
				61 hitbox resize handle
				70 floating toolbar
			-->

			<DragArea :buttons="['right']" active-cursor="cursor-move" @drag-update="updatePanning" @click="onClick"
				class="flex-1 graph-paper h-full pointer-events-auto transition-all" :style="graphPaperStyle" />
			<div class="absolute inset-0 transition-transform" :style="zoomTransform">
				<div class="absolute inset-0" :style="panningTransform">
					<Frame v-if="!app.currentDocument?.hideSprite" v-for="k in allAnimationKeyframes"
						:key="k.keyframe.key" :keyframe="k.keyframe" :direction="k.direction" :index="k.index" />
					<Hitbox v-if="!app.currentDocument?.hideHitboxes" v-for="hitbox in sortedHitboxes" :key="hitbox.key"
						:hitbox="hitbox" />
				</div>
			</div>
			<Origin v-if="!app.currentDocument?.hideOrigin" class="absolute inset-0 z-30 transition-all"
				:style="originTransform" />
			<div class="absolute right-0 bottom-0 p-6 text-4xl font-bold text-neutral-600">
				{{ app.currentAnimation?.name }}
			</div>

			<Toolbar class="absolute top-6 right-6 z-[70] pointer-events-auto" />
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { onUnmounted, watch } from "vue"
import { computed, Ref, ref } from "@vue/reactivity"
import { Direction, Keyframe, Hitbox as HitboxDTO } from "@/api/dto"
import { closeDocument, focusDocument } from "@/api/app"
import { clearSelection, pan } from "@/api/document"
import { useAppStore } from "@/stores/app"
import { isStable } from "@/utils/animation"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneTabList from "@/components/basic/PaneTabList.vue"
import Frame from "@/components/workbench/Frame.vue"
import Hitbox from "@/components/workbench/Hitbox.vue"
import Origin from "@/components/workbench/Origin.vue"
import Toolbar from "@/components/workbench/Toolbar.vue"

const app = useAppStore();
const drawingArea: Ref<HTMLElement | null> = ref(null);
const drawingAreaHalfSize = ref([0, 0]);
const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);
const workbenchOffset = computed(() => app.currentDocument?.workbenchOffset || [0, 0]);
const isZoomStable = isStable([zoom]);

const resizeObserver = new ResizeObserver(entries => {
	for (let entry of entries) {
		if (entry.target === drawingArea.value) {
			drawingAreaHalfSize.value = [entry.contentRect.width / 2, entry.contentRect.height / 2];
		}
	}
});

watch(drawingArea, (newArea, oldArea) => {
	if (oldArea) {
		resizeObserver.unobserve(oldArea);
		oldArea.removeEventListener("wheel", onWheel);
	}
	if (newArea) {
		resizeObserver.observe(newArea);
		newArea.addEventListener("wheel", onWheel);
	}
});

onUnmounted(() => {
	resizeObserver.disconnect();
});

const graphPaperStyle = computed(() => {
	const x = drawingAreaHalfSize.value[0] + workbenchOffset.value[0] * zoom.value;
	const y = drawingAreaHalfSize.value[1] + workbenchOffset.value[1] * zoom.value;
	return {
		backgroundPosition: `${x}px ${y}px`,
		transitionProperty: isZoomStable.value ? "none" : "background-position",
	}
});

const originTransform = computed(() => {
	const x = drawingAreaHalfSize.value[0] + workbenchOffset.value[0] * zoom.value;
	const y = drawingAreaHalfSize.value[1] + workbenchOffset.value[1] * zoom.value;
	return {
		transform: `translate(${x}px, ${y}px)`,
		transitionProperty: isZoomStable.value ? "none" : "transform",
	};
});

const zoomTransform = computed(() => {
	return {
		transform: `scale(${zoom.value}, ${zoom.value})`,
		transformOrigin: `${drawingAreaHalfSize.value[0]}px ${drawingAreaHalfSize.value[1]}px`,
	};
});

const panningTransform = computed(() => {
	const x = drawingAreaHalfSize.value[0] + workbenchOffset.value[0];
	const y = drawingAreaHalfSize.value[1] + workbenchOffset.value[1];
	return {
		transform: `translate(${x}px, ${y}px)`,
	};
});

const allAnimationKeyframes = computed((): { direction: Direction, index: number, keyframe: Keyframe }[] => {
	let keyframes = [];
	for (const direction in app.currentAnimation?.sequences) {
		for (const [index, keyframe] of (app.currentAnimation?.sequences[direction as Direction].keyframes.entries()) || []) {
			keyframes.push({
				direction: direction as Direction,
				index: index,
				keyframe: keyframe
			});
		}
	}
	return keyframes;
});

const sortedHitboxes = computed((): HitboxDTO[] => {
	if (!app.currentKeyframe) {
		return [];
	}
	let hitboxes = [...app.currentKeyframe.hitboxes];
	hitboxes.sort((a, b) => {
		const areaA = a.size[0] * a.size[1];
		const areaB = b.size[0] * b.size[1];
		if (areaA < areaB) {
			return 1;
		}
		if (areaA > areaB) {
			return -1;
		}
		return 0;
	});
	return hitboxes;
});

function onClick() {
	clearSelection();
}

function updatePanning(event: DragAreaEvent) {
	pan([event.mouseEvent.movementX, event.mouseEvent.movementY]);
}

function onWheel(event: WheelEvent) {
	if (!event.ctrlKey) {
		return;
	}
	// TODO
}
</script>

<style scoped>
.graph-paper {
	background:
		linear-gradient(-90deg, theme("colors.neutral.700") 1px, transparent 1px),
		linear-gradient(0deg, theme("colors.neutral.700") 1px, transparent 1px),
		linear-gradient(-90deg, theme("colors.neutral.800") 1px, transparent 1px),
		linear-gradient(0deg, theme("colors.neutral.800") 1px, transparent 1px),
		theme("colors.neutral.900");
	background-size:
		128px 128px,
		128px 128px,
		16px 16px,
		16px 16px;
}
</style>
