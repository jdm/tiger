<template>
	<div>
		<div class="relative w-full h-full">
			<div class="absolute inset-0 bg-black/70" />
			<div class="absolute inset-0">
				<div class="h-full flex items-center justify-center">
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>