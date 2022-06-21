<template>
	<Pane>
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Details</PaneTab>
		</div>
		<HitboxDetails v-if="app.selectedHitboxes?.length" />
		<KeyframeDetails v-else-if="app.selectedKeyframes?.length" />
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import HitboxDetails from "@/components/details/HitboxDetails.vue"
import KeyframeDetails from "@/components/details/KeyframeDetails.vue"

const app = useAppStore();

</script>