<template>
	<Pane>
		<div class="flex bg-plastic-900">
			<PaneTab :selected="true">Details</PaneTab>
		</div>
		<HitboxDetails v-if="state.selectedHitboxes?.length" />
		<KeyframeDetails v-else-if="state.selectedKeyframes?.length" />
		<BlankDetails v-else />
	</Pane>
</template>

<script setup lang="ts">
import { useStateStore } from "@/stores/state"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import BlankDetails from "@/components/details/BlankDetails.vue"
import HitboxDetails from "@/components/details/HitboxDetails.vue"
import KeyframeDetails from "@/components/details/KeyframeDetails.vue"

const state = useStateStore();

</script>