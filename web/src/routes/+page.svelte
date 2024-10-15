<script lang="ts">
	export let data;
	const { users } = data;
</script>

<main>
	<h1>Hello users!</h1>
	<ul>
		{#each users as user}
			<li>{user.name}, {user.email}</li>
		{/each}
	</ul>
</main>
