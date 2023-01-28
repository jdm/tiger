<template>
	<Transition name="banner-intro">
		<div v-if="visible" class="h-12 overflow-hidden">
			<div class="relative h-full py-3 bg-amber-400 text-amber-800">
				<div v-if="state.updateStep == UpdateStep.UpdateAvailable || state.updateStep == UpdateStep.UpdateRequested"
					class="absolute w-full flex justify-center gap-4">
					<div>A new version of Tiger is available!</div>
					<div class="flex gap-1 font-semibold underline cursor-pointer text-amber-900"
						@click="requestInstallUpdate">
						<CloudArrowDownIcon class="w-6" />Update now
					</div>
				</div>
			</div>
		</div>
	</Transition>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { CloudArrowDownIcon } from "@heroicons/vue/24/outline"
import { requestInstallUpdate } from "@/backend/api"
import { UpdateStep } from "@/backend/dto"
import { useStateStore } from "@/stores/state";

const state = useStateStore();

const visible = computed(() => {
	return	state.updateStep == UpdateStep.UpdateAvailable
		||	state.updateStep == UpdateStep.UpdateRequested
		;
});
</script>

<style>
.banner-intro-enter-active {
	transition: max-height 0.15s ease-out;
	max-height: 48px;
}

.banner-intro-leave-active {
	transition: max-height 0.15s ease-in;
	max-height: 48px;
}

.banner-intro-enter-from,
.banner-intro-leave-to {
	max-height: 0px;
}
</style>
