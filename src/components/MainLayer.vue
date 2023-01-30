<template>
	<div class="h-full w-full flex flex-col overflow-clip bg-plastic-900">
		<AppBar />
		<UpdateBanner />
		<div class="flex-1 relative">
			<Transition appear v-if="state.startupFinalized">
				<UpdateSpinner v-if="state.updateStep == UpdateStep.InstallingUpdate" class="absolute inset-0" />
				<StartupScreen v-else-if="!state.currentDocument" class="absolute inset-0" />
				<div v-else class="absolute inset-0 min-h-0 flex gap-5 p-5">
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
			</Transition>
			<NotificationOverlay class="absolute inset-0 z-[999]" />
			<ExportOverlay class="absolute inset-0 z-[1000]" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { UpdateStep } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import AppBar from "@/components/AppBar.vue"
import ExportOverlay from "@/components/ExportOverlay.vue"
import StartupScreen from "@/components/StartupScreen.vue"
import NotificationOverlay from "@/components/NotificationOverlay.vue"
import AnimationsPane from "@/components/animations/AnimationsPane.vue"
import DetailsPane from "@/components/details/DetailsPane.vue"
import FramesPane from "@/components/frames/FramesPane.vue"
import KeyframePane from "@/components/keyframe/KeyframePane.vue"
import TimelinePane from "@/components/timeline/TimelinePane.vue"
import UpdateBanner from "@/components/updates/UpdateBanner.vue"
import UpdateSpinner from "@/components/updates/UpdateSpinner.vue"
import WorkbenchPane from "@/components/workbench/WorkbenchPane.vue"

const state = useStateStore();
</script>

<style scoped>
.v-enter-active,
.v-leave-active {
	transition: all 0.15s ease-out;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
	transform: scale(97%, 97%);
}
</style>