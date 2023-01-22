<template>
	<Transition name="banner-intro">
		<div v-if="visible" class="h-12 overflow-hidden">
			<div class="relative h-full py-3 bg-amber-400 text-amber-800">
				<Transition name="slide-up">
					<div v-if="state.updateStep == UpdateStep.UpdateAvailable"
						class="absolute w-full flex justify-center gap-4">
						<div>A new version of Tiger is available!</div>
						<div class="flex gap-1 font-semibold underline cursor-pointer text-amber-900"
							@click="installUpdate">
							<RocketLaunchIcon class="w-6" />Install now
						</div>
					</div>
					<div v-else-if="state.updateStep == UpdateStep.InstallingUpdate"
						class="absolute w-full flex justify-center gap-2 animate-pulse">
						<BoltIcon class="w-6" />Updating…
					</div>
				</Transition>
			</div>
		</div>
	</Transition>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { RocketLaunchIcon, BoltIcon } from "@heroicons/vue/24/outline"
import { installUpdate } from "@/backend/api"
import { UpdateStep } from "@/backend/dto"
import { useStateStore } from "@/stores/state";

const state = useStateStore();

const visible = computed(() => {
	return state.updateStep == UpdateStep.UpdateAvailable || state.updateStep == UpdateStep.InstallingUpdate;
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

.slide-up-enter-active,
.slide-up-leave-active {
	transition: all 0.25s ease-out;
}

.slide-up-enter-from {
	opacity: 0;
	transform: translateY(30px);
}

.slide-up-leave-to {
	opacity: 0;
	transform: translateY(-30px);
}
</style>
