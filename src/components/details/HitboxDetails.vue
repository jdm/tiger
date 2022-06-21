<template>
	<PaneInset class="m-4 p-4">
		<div class="px-4 grid grid-cols-10 gap-y-2">
			<DetailKey class="col-span-4">X</DetailKey>
			<DetailValue :values="xValues" @update="setHitboxPositionX" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Y</DetailKey>
			<DetailValue :values="yValues" @update="setHitboxPositionY" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Width</DetailKey>
			<DetailValue :values="widthValues" @update="setHitboxWidth" class="col-span-4" unit="px" />

			<div class="col-span-2 row-span-2 flex flex-col justify-center pl-2">
				<div class="h-2.5 w-1/2 border-t border-r"
					:class="preserveAR ? 'border-plastic-300' : 'border-plastic-500'" />
				<Icon name="LinkIcon" @click="togglePreserveAspectRatio" class="cursor-pointer self-center my-1 w-5 h-5"
					:class="preserveAR ? 'text-plastic-300' : 'text-plastic-500'" />
				<div class="h-2.5 w-1/2 border-b border-r"
					:class="preserveAR ? 'border-plastic-300' : 'border-plastic-500'" />
			</div>

			<DetailKey class="col-span-4">Height</DetailKey>
			<DetailValue :values="heightValues" @update="setHitboxHeight" class="col-span-4" unit="px" />
		</div>
	</PaneInset>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"
import Icon from "@/components/basic/Icon.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import DetailKey from "@/components/details/DetailKey.vue"
import DetailValue from "@/components/details/DetailValue.vue"
import { setHitboxPositionX, setHitboxPositionY, setHitboxWidth, setHitboxHeight, togglePreserveAspectRatio } from "@/api/document"

const app = useAppStore();

const preserveAR = computed(() => !!app.currentDocument?.preserveAspectRatio);
const xValues = computed(() => app.selectedHitboxes?.map(h => h.topLeft[0]) || []);
const yValues = computed(() => app.selectedHitboxes?.map(h => h.topLeft[1]) || []);
const widthValues = computed(() => app.selectedHitboxes?.map(h => h.size[0]) || []);
const heightValues = computed(() => app.selectedHitboxes?.map(h => h.size[1]) || []);

</script>