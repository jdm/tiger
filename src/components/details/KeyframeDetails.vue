<template>
	<PaneInset class="m-4 p-4">
		<div class="px-4 grid grid-cols-10 gap-y-2">
			<DetailKey class="col-span-4">Frame</DetailKey>
			<DetailValueString :values="frameValues" :read-only="true" class="col-span-6" />

			<DetailKey class="col-span-4">Duration</DetailKey>
			<DetailValueNumber :values="durationValues" @update="setKeyframeDuration" class="col-span-6" unit="ms" />

			<DetailKey class="col-span-4">X</DetailKey>
			<DetailValueNumber :values="xValues" @update="setKeyframeOffsetX" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Y</DetailKey>
			<DetailValueNumber :values="yValues" @update="setKeyframeOffsetY" class="col-span-6" unit="px" />
		</div>
	</PaneInset>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { setKeyframeDuration, setKeyframeOffsetX, setKeyframeOffsetY } from "@/api/document"
import { useAppStore } from "@/stores/app"
import PaneInset from "@/components/basic/PaneInset.vue"
import DetailKey from "@/components/details/DetailKey.vue"
import DetailValueNumber from "@/components/details/DetailValueNumber.vue"
import DetailValueString from "@/components/details/DetailValueString.vue"

const app = useAppStore();

const frameValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.name) || []);
const durationValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.durationMillis) || []);
const xValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.offset[0]) || []);
const yValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.offset[1]) || []);

</script>
