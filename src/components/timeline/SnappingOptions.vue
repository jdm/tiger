<template>
	<MenuBackground class="mt-2 -translate-x-1/2 whitespace-nowrap">
		<div class="flex flex-col space-y-4 p-2 text-sm text-zinc-300">
			<div class="flex space-x-3 items-center">
				<Checkbox id="snappingEnabled" v-model="snappingEnabled" />
				<label for="snappingEnabled" class="cursor-pointer">Enable snapping</label>
			</div>
			<MenuSeparator />
			<div class="flex flex-col space-y-2"
				:class="snappingEnabled ? [] : ['pointer-events-none', 'opacity-20', 'saturate-0']">
				<div class="flex space-x-3 items-center">
					<Checkbox id="snapToKeyframes" v-model="snapToKeyframes" />
					<label for="snapToKeyframes" class="cursor-pointer">Snap to other keyframes</label>
				</div>
				<div class="flex space-x-3 items-center">
					<Checkbox id="snapToMultiplesOf" v-model="snapToMultiplesOf" />
					<div class="flex space-x-2 items-center">
						<label for="snapToMultiplesOf" class="cursor-pointer">Snap to multiples of</label>
						<input type="text" v-model="snappingBaseDuration"
							class="border-0 rounded-md p-0 px-1 w-9 h-6 text-xs text-right bg-plastic-700 focus:ring-0" />
						<div>ms</div>
					</div>
				</div>
			</div>
		</div>
	</MenuBackground>
</template>

<script setup lang="ts">
import { computed, WritableComputedRef } from "vue";
import { setKeyframeSnappingBaseDuration, setSnapKeyframeDurations, setSnapKeyframesToMultiplesOfDuration, setSnapKeyframesToOtherKeyframes } from "@/api/document";
import { useAppStore } from "@/stores/app";
import Checkbox from "@/components/basic/Checkbox.vue";
import MenuBackground from "@/components/basic/MenuBackground.vue";
import MenuSeparator from "@/components/basic/MenuSeparator.vue";

const app = useAppStore();

const snappingEnabled: WritableComputedRef<boolean> = computed({
	get: () => !!app.currentDocument?.snapKeyframeDurations,
	set: setSnapKeyframeDurations,
});

const snapToKeyframes: WritableComputedRef<boolean> = computed({
	get: () => !!app.currentDocument?.snapKeyframesToOtherKeyframes,
	set: setSnapKeyframesToOtherKeyframes,
});

const snapToMultiplesOf: WritableComputedRef<boolean> = computed({
	get: () => !!app.currentDocument?.snapKeyframesToMultiplesOfDuration,
	set: setSnapKeyframesToMultiplesOfDuration,
});

const snappingBaseDuration: WritableComputedRef<number> = computed({
	get: () => app.currentDocument?.keyframeSnappingBaseDurationMillis || 0,
	set: (n) => setKeyframeSnappingBaseDuration(Number(n)),
});
</script>
