<template>
	<div class="h-full w-full flex flex-col overflow-clip bg-plastic-900">
		<AppBar />
		<div class="flex-1 relative">
			<div class="absolute inset-0 min-h-0 flex gap-5 p-5">
				<div class="basis-[27.375rem] min-w-0 flex flex-col gap-5">
					<AnimationsPane class="flex-1" />
					<FramesPane class="flex-1" />
				</div>
				<div class="flex-1 min-w-0 flex flex-col">
					<div class="flex-1 min-h-0 flex gap-5 pb-5">
						<WorkbenchPane class="flex-1" />
						<div class="basis-80 min-w-0 flex flex-col gap-5">
							<KeyframePane class="flex-1" />
							<DetailsPane class="basis-80" />
						</div>
					</div>
					<TimelinePane />
				</div>
			</div>
			<NotificationOverlay class="absolute inset-0 z-[999]" />
			<ExportOverlay class="absolute inset-0 z-[1000]" />
		</div>
	</div>
</template>

<script setup lang="ts">
import AppBar from "@/components/AppBar.vue"
import ExportOverlay from "@/components/ExportOverlay.vue"
import NotificationOverlay from "@/components/NotificationOverlay.vue"
import AnimationsPane from "@/components/animations/AnimationsPane.vue"
import DetailsPane from "@/components/details/DetailsPane.vue"
import FramesPane from "@/components/frames/FramesPane.vue"
import KeyframePane from "@/components/keyframe/KeyframePane.vue"
import TimelinePane from "@/components/timeline/TimelinePane.vue"
import WorkbenchPane from "@/components/workbench/WorkbenchPane.vue"
</script>