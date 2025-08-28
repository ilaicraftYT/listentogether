<script>
    let { children } = $props();
    import "../app.css";
</script>

<div
    class="dark:bg-neutral-950 dark:text-neutral-100 bg-neutral-100 text-neutral-950 h-full min-h-screen"
>
    {@render children()}
</div>
