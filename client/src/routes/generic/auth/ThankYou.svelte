<script>
import { Label } from '@smui/button';
import { push } from 'svelte-spa-router';
import NavBar from '../../../components/NavBar.svelte';
import Fab from '@smui/fab';
import Banner from '../../../components/Visualizations/Banner.svelte';
</script>

<NavBar></NavBar>	
<h1>Thank you!</h1>
<Banner></Banner>
<div class="center top">
	<div class="center">
		<div class="log-in">
			<Fab color="primary" on:click={()=>push("#/Login")} extended ><Label style="width:100px">Log In</Label></Fab>
		</div>
	</div>	
</div>
<hr>
<!-- <NavBar/> -->

<style>
	h1 {
		color: #ff0040;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}
	.log-in{
		padding: 10px;
	}
	.center {
		margin: auto;
		display: flex;
		flex-direction: row;
	}
	.top{
		padding-bottom: 50px;
		padding-top:50px;
	}
	img{
		height: 400px;
	}
	hr{
		width: 98vw;
	}
</style>
