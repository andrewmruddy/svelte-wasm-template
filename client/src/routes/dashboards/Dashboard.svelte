<script lang="ts">
    // SMUI Imports
	import {token} from '../../services/LocalStorage';
    import ListView from '../../components/ListView.svelte';
    import type ProjectCard from '../../components/Cards/GenericCard.svelte';
    import {push, pop, replace} from 'svelte-spa-router';
	import NavBar from '../../components/NavBar.svelte';
    import BannerCard from '../../components/BannerCard.svelte';
    import SearchBar from "../../components/SearchBar.svelte";
    import { Routes } from '../../models/Routes';
    //Vars + Functions
</script>

<NavBar></NavBar>
<SearchBar></SearchBar>
<div class="banner-cards">
    <BannerCard Title={"-EXAMPLE-"} Link={Routes.Home} Count = {12}/>
    <BannerCard Title={"-EXAMPLE-"} Count = {12}/>
    <BannerCard Title={"-EXAMPLE-"} Count = {12}/>
</div>
<hr>
<style>
    @media only screen and (min-width: 601px) {
        .banner-cards{
            display: flex;
            flex-direction: row;
            justify-content: center;
            padding-top:50px;
        }
    }
    @media only screen and (max-width: 600px) {
        .banner-cards{
            display: flex;
            flex-direction: column;
            justify-content: center;
            padding-top:50px;
        }
    }
    hr{
        width: 98vw;
    }
</style>