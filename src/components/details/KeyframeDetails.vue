<template>
	<PaneInset class="m-4 p-4">
		<div class="px-4 grid grid-cols-10 gap-y-2">
			<DetailKey class="col-span-4">Duration</DetailKey>
			<DetailValue :values="durationValues" @update="setKeyframeDuration" class="col-span-6" unit="ms" />

			<DetailKey class="col-span-4">X</DetailKey>
			<DetailValue :values="xValues" @update="setKeyframeOffsetX" class="col-span-6" unit="px" />

			<DetailKey class="col-span-4">Y</DetailKey>
			<DetailValue :values="yValues" @update="setKeyframeOffsetY" class="col-span-6" unit="px" />
		</div>
	</PaneInset>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { setKeyframeDuration, setKeyframeOffsetX, setKeyframeOffsetY } from "@/api/document"
import { useAppStore } from "@/stores/app"
import PaneInset from "@/components/basic/PaneInset.vue"
import DetailKey from "@/components/details/DetailKey.vue"
import DetailValue from "@/components/details/DetailValue.vue"

const app = useAppStore();

const durationValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.durationMillis) || []);
const xValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.offset[0]) || []);
const yValues = computed(() => app.selectedKeyframes?.map(keyframe => keyframe.offset[1]) || []);

</script>
