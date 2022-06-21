<template>
	<Pane>
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Details</PaneTab>
		</div>
		<HitboxDetails v-if="app.selectedHitboxes?.length" />
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import HitboxDetails from "@/components/details/HitboxDetails.vue"

const app = useAppStore();

</script>