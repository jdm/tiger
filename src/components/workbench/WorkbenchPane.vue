<template>
	<Pane class="overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name + (document.hasUnsavedChanges ? "*" : "") }}
			</PaneTab>
		</PaneTabList>
		<div class="relative flex-1 overflow-hidden pointer-events-none" ref="drawingArea">

			<!--
				Z-Ordering
				----------
				0 sprite BG
				10 sprite
				20 sprite outline
				30 origin
				30 hitbox BG & outline
				40 sprite drag area
				50 hitbox drag area
				60 hitbox resize handle
				70 floating toolbar
			-->

			<DragArea :buttons="['right']" active-cursor="cursor-move" @drag-update="updatePanning" @click="onClick"
				class="flex-1 graph-paper h-full pointer-events-auto" :style="graphPaperStyle" />
			<div class="absolute inset-0 transition-transform" :style="zoomTransform">
				<div class="absolute inset-0" :style="panningTransform">
					<Frame v-if="!app.currentDocument?.hideSprite" v-for="k in allAnimationKeyframes"
						:key="k.keyframe.key" :keyframe="k.keyframe" :direction="k.direction" :index="k.index" />
					<Hitbox v-if="!app.currentDocument?.hideHitboxes" v-for="entry in sortedHitboxes"
						:key="entry.hitbox.key" :hitbox="entry.hitbox" :name="entry.name" />
				</div>
			</div>
			<Origin v-if="!app.currentDocument?.hideOrigin" class="absolute inset-0 z-30" :style="panningTransform" />
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
import { clearSelection, disableSpriteDarkening, enableSpriteDarkening, pan, zoomInWorkbench, zoomOutWorkbench } from "@/api/document"
import { useAppStore } from "@/stores/app"
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
const drawingAreaSize = ref([0, 0]);

const resizeObserver = new ResizeObserver(entries => {
	for (let entry of entries) {
		if (entry.target === drawingArea.value) {
			drawingAreaSize.value = [entry.contentRect.width, entry.contentRect.height];
		}
	}
});

watch(drawingArea, (newArea, oldArea) => {
	if (oldArea) {
		resizeObserver.unobserve(oldArea);
	}
	if (newArea) {
		resizeObserver.observe(newArea);
	}
});

onUnmounted(() => {
	resizeObserver.disconnect();
});

const graphPaperStyle = computed(() => {
	return {
		"background-position": `${origin.value[0]}px ${origin.value[1]}px`,
	}
});

const origin = computed((): [number, number] => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	return [
		Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0],
		Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1],
	];
});

const zoomTransform = computed(() => {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	return {
		transform: `scale(${zoom}, ${zoom})`,
		transformOrigin: `${origin.value[0]}px ${origin.value[1]}px`,
	};
});

const panningTransform = computed(() => {
	return {
		transform: `translate(${origin.value[0]}px, ${origin.value[1]}px)`,
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

type HitboxEntry = {
	name: string,
	hitbox: HitboxDTO,
};

const sortedHitboxes = computed((): HitboxEntry[] => {
	if (!app.currentKeyframe) {
		return [];
	}
	let hitboxEntries = Object.entries(app.currentKeyframe.hitboxes).map(([name, hitbox]) => {
		return { name: name, hitbox: hitbox }
	});
	hitboxEntries.sort((a, b) => {
		const areaA = a.hitbox.size[0] * a.hitbox.size[1];
		const areaB = b.hitbox.size[0] * b.hitbox.size[1];
		if (areaA < areaB) {
			return 1;
		}
		if (areaA > areaB) {
			return -1;
		}
		return 0;
	});
	return hitboxEntries;
});

function onClick() {
	clearSelection();
}

function updatePanning(event: DragAreaEvent) {
	pan([event.mouseEvent.movementX, event.mouseEvent.movementY]);
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
