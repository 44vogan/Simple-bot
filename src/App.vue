<script setup lang="ts">
	// This starter template is using Vue 3 <script setup> SFCs
	// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
	import Details from "./components/Details.vue";
	import Profiles from "./components/Profiles.vue";
	import Logs from "./components/Logs.vue";
</script>

<template>
	<div class="container">
		<div class="left-ctn">
			<div>
				<Profiles />
				<Logs />
			</div>
		</div>
		<Details />
	</div>
</template>

<style scoped>
	.left-ctn {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
		min-width: 350px;
		max-width: 500px;
	}
	.left-ctn div {
		box-sizing: border-box;
		position: sticky;
		top: 5px;
	}
</style>
