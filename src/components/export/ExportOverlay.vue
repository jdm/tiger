<template>
	<div class="pointer-events-none">
		<Transition name="fade">
			<div v-if="app.currentDocument?.exportSettingsBeingEdited"
				class="absolute inset-0 pointer-events-auto bg-black/70" />
		</Transition>
		<Transition name="slide">
			<div v-if="app.currentDocument?.exportSettingsBeingEdited" class="absolute inset-0 pointer-events-auto ">
				<div class="w-full h-full flex flex-row justify-end">
					<div class="h-full w-[40rem] p-10 flex flex-col space-y-16 bg-plastic-700">

						<div class="flex flex-col space-y-4">
							<h1 class="text-plastic-200 text-xl">Output Files</h1>
							<div>
								<InputLabel>Texture File</InputLabel>
								<InputPath class="mt-1 rounded-md"
									placeholder="C:\ExampleGame\Assets\Sprites\Hero.png" />
							</div>
							<div>
								<InputLabel>Metadata File</InputLabel>
								<InputPath class="mt-1 rounded-md"
									placeholder="C:\ExampleGame\Assets\Sprites\Hero.json" />
							</div>
						</div>

						<div class="flex flex-col space-y-4">
							<h1 class="text-plastic-200 text-xl">Metadata Format</h1>
							<div>
								<InputLabel>Metadata Template File</InputLabel>
								<InputPath class="mt-1 rounded-md"
									placeholder="C:\ExampleGame\Tooling\SpritesheetFormat.liquid" />
							</div>
							<div>
								<InputLabel>Metadata Root Directory</InputLabel>
								<InputPath :isDirectory="true" class="mt-1 rounded-md" placeholder="C:\ExampleGame" />
							</div>
						</div>

						<div class="flex space-x-4 justify-end">
							<Button label="Export" :positive="true" />
							<Button label="Cancel" @click="cancelExportAs" />
						</div>

						<!-- TODO Link to documentation -->
						<!-- <div class="flex-1 flex flex-col justify-end">
							<div
								class="flex items-center rounded-md text-sm p-5 text-amber-800 bg-amber-400 shadow-lg  shadow-amber-600/20">
								<BookOpenIcon class="mr-6 w-8 h-8" />
								<div>
									Confused about all these options? Check out the <span
										class="underline underline-offset-2">documentation</span> about what they
									mean.
								</div>
							</div>
						</div> -->
					</div>
				</div>
			</div>
		</Transition>
	</div>
</template>


<script setup lang="ts">
import { BookOpenIcon } from "@heroicons/vue/outline"
import { cancelExportAs } from "@/api/document"
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
import InputLabel from "@/components/basic/InputLabel.vue"
import InputPath from "@/components/basic/InputPath.vue"

const app = useAppStore();

</script>

<style scoped>
.fade-enter-active {
	transition: opacity 0.3s ease-out;
}

.fade-leave-active {
	transition: opacity 0.2s ease-in;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}

.slide-enter-active {
	transition-property: transform;
	transition: 0.3s ease-out;
}

.slide-leave-active {
	transition-property: transform;
	transition: 0.2s ease-in;
}

.slide-enter-from,
.slide-leave-to {
	transform: translateX(100%);
}
</style>