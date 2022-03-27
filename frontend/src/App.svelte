<!--
  This example requires Tailwind CSS v2.0+ 
  
  This example requires some changes to your config:
  
  ```
  // tailwind.config.js
  module.exports = {
    // ...
    plugins: [
      // ...
      require('@tailwindcss/forms'),
    ],
  }
  ```
-->
<script>
  import { Router, Link, Route } from "svelte-routing";
  import Demo from "./pages/demo.svelte";
  import Landing from "./pages/landing.svelte";
  import Header from "./pages/header.svelte";
  import Development from "./pages/development.svelte";
  import Contact from "./pages/contact.svelte";
  import NotFound from "./pages/notfound.svelte";
  import { page } from "./store";
  export let url = "";
</script>

<div class="bg-white dark:bg-slate-800">
  <Header />
  <main>
    <Router {url}>
      <div>
        <Route path="/"><Landing /></Route>
        <Route path="demo" component={Demo} />
        <Route path="development" component={Development} />
        <Route path="contact" component={Contact} />
        <Route component={NotFound} />
      </div>
    </Router>
  </main>
</div>
