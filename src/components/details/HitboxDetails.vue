<template>
	<PaneInset class="m-4 p-4">
		<div class="px-4 grid grid-cols-10 gap-y-2">
			<DetailKey class="col-span-4">X</DetailKey>
			<DetailValueNumber :values="xValues" @update="setHitboxPositionX" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Y</DetailKey>
			<DetailValueNumber :values="yValues" @update="setHitboxPositionY" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Width</DetailKey>
			<DetailValueNumber :values="widthValues" @update="setHitboxWidth" class="col-span-4" unit="px" />

			<div class="col-span-2 row-span-2 flex flex-col justify-center pl-2">
				<div class="h-2.5 w-1/2 border-t border-r"
					:class="preserveAR ? 'border-plastic-300' : 'border-plastic-500'" />
				<LinkIcon @click="togglePreserveAspectRatio" class="cursor-pointer self-center my-1 w-5 h-5"
					:class="preserveAR ? 'text-plastic-300' : 'text-plastic-500'" />
				<div class="h-2.5 w-1/2 border-b border-r"
					:class="preserveAR ? 'border-plastic-300' : 'border-plastic-500'" />
			</div>

			<DetailKey class="col-span-4">Height</DetailKey>
			<DetailValueNumber :values="heightValues" @update="setHitboxHeight" class="col-span-4" unit="px" />
		</div>
	</PaneInset>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { setHitboxPositionX, setHitboxPositionY, setHitboxWidth, setHitboxHeight, togglePreserveAspectRatio } from "@/api/document"
import { useStateStore } from "@/stores/state"
import { LinkIcon } from "@heroicons/vue/20/solid"
import PaneInset from "@/components/basic/PaneInset.vue"
import DetailKey from "@/components/details/DetailKey.vue"
import DetailValueNumber from "@/components/details/DetailValueNumber.vue"

const state = useStateStore();

const preserveAR = computed(() => !!state.currentDocument?.preserveAspectRatio);
const xValues = computed(() => state.selectedHitboxes?.map(h => h.topLeft[0]) || []);
const yValues = computed(() => state.selectedHitboxes?.map(h => h.topLeft[1]) || []);
const widthValues = computed(() => state.selectedHitboxes?.map(h => h.size[0]) || []);
const heightValues = computed(() => state.selectedHitboxes?.map(h => h.size[1]) || []);

</script>