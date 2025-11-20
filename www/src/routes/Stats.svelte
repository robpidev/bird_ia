<script lang="ts">
	let { min, max, avg, gen }: { min: number; max: number; avg: number; gen: number } = $props();
</script>

<div>
	<ul>
		<li>
			<span class="label">Min:</span>
			<span class="value">{min}</span>
		</li>
		<li>
			<span class="label">Max:</span>
			<span class="value">{max}</span>
		</li>
		<li>
			<span class="label">Avg:</span>
			<span class="value">{avg.toFixed(2)}</span>
		</li>
		<li>
			<span class="label">Gen:</span>
			<span class="value">{gen}</span>
		</li>
	</ul>
</div>

<style>
	ul {
		list-style-type: none;
		display: flex;
		gap: 1rem;
		margin: 0;
		padding: 0;
	}
	li {
		display: flex;
		gap: 1rem;
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 1rem;
		border-radius: 6px;
	}
	.label {
		font-weight: bold;
	}
	.value {
		font-weight: normal;
	}
</style>
