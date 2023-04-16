<script setup>
	// This starter template is using Vue 3 <script setup> SFCs
	// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
	import Details from "./components/Details.vue";
	import Profiles from "./components/Profiles.vue";
	import Logs from "./components/Logs.vue";
</script>

<template>
	<div class="container">
		<div>
			<Profiles />
			<Logs />
		</div>
		<div>
			<Details />
		</div>
	</div>
</template>

<style scoped></style>
