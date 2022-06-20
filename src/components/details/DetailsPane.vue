<template>
	<Pane>
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Details</PaneTab>
		</div>
		<PaneInset class="m-4 p-4">
			<div class="px-4 grid grid-cols-10 gap-y-2">
				<DetailKey class="col-span-4">X</DetailKey>
				<DetailValue class="col-span-6" unit="px" />

				<DetailKey class="col-span-4">Y</DetailKey>
				<DetailValue class="col-span-6" unit="px" />

				<DetailKey class="col-span-4">Width</DetailKey>
				<DetailValue class="col-span-4" unit="px" />

				<div class="col-span-2 row-span-2 flex flex-col justify-center pl-2">
					<div class="h-2.5 w-1/2 border-t border-r border-plastic-500" />
					<Icon name="LinkIcon" class="cursor-pointer self-center my-1 w-5 h-5 text-plastic-500" />
					<div class="h-2.5 w-1/2 border-b border-r border-plastic-500" />
				</div>

				<DetailKey class="col-span-4">Height</DetailKey>
				<DetailValue class="col-span-4" unit="px" />
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import Icon from "@/components/basic/Icon.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import DetailKey from "@/components/details/DetailKey.vue"
import DetailValue from "@/components/details/DetailValue.vue"
</script>