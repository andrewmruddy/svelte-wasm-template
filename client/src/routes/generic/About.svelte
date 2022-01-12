<script lang="ts">
// START Properties
// END Properties


// START Component Imports
import NavBar from "../../components/NavBar.svelte";
import ListView from "../../components/ListView.svelte";
// END Component Imports


// START Other Imports
// END Other Imports


// START Local Vars
// END Local Vars


// START Functions
// END Functions


// START Init
// END Init
</script>

<h1>Example ListView</h1>
<section>
    <div class="search">
        <input placeholder="Input"/>
        <div style="width: 10px;"></div>
        <button>Search</button>
    </div>  
</section>
<main>
<hr>
<section>
    <header>
        <h2>Simple, reusable components</h2>
        <p>For a clean looking design that doesn't get in the way</p>
    </header>
</section>
</main>

<style>
    .search{
        display: flex;
        flex-direction: row;
    }
    button{
        padding:0%;
        height:50px;
        margin: 0 0 0 0;
        width:100px
    }
    input{
        margin: 0 0 0 0;
        height:50px;
    }
</style>