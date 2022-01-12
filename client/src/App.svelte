<script lang="ts">
	import Router from 'svelte-spa-router';
	import routes from './routes';
	import NavBar from './components/NavBar.svelte';
</script>

<main>
	<!-- <NavBar/> -->
	<Router {routes}/>
</main>

<style>
	main {
		text-align: center;
		padding: 0em;
		max-width: 240px;
		margin: 0em;
		max-width: none;
	}

	/* @media (min-width: 640px) {
		main {
			max-width: none;
		}
	} */
</style>
