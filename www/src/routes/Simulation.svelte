<script lang="ts">
	import { onMount } from 'svelte';
	import * as sim from '../../../libs/simulation-wasm/pkg';
	import Stats from './Stats.svelte';
	import Tooltip from './Tooltip.svelte';

	// let { animals, foods } = $props();

	let animals = $state(40);
	let foods = $state(60);

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;

	let ch = 800;
	let cw = 1200;
	let size = 10;
	let foodSize = 3;

	let stats = $state({
		min: 0,
		max: 0,
		avg: 0,
		gen: 0
	});

	let simulation: sim.Simulation | null = null;

	// let simulation = new sim.Simulation(animals, foods);

	function drawTriangle(ctx: CanvasRenderingContext2D, x: number, y: number, angle: number) {
		ctx.beginPath();
		ctx.moveTo(x - Math.sin(angle) * size * 1.5, y + Math.cos(angle) * size * 1.5);
		ctx.lineTo(
			x - Math.sin(angle + (2 / 3) * Math.PI) * size,
			y + Math.cos(angle + (2 / 3) * Math.PI) * size
		);
		ctx.lineTo(
			x - Math.sin(angle + (4 / 3) * Math.PI) * size,
			y + Math.cos(angle + (4 / 3) * Math.PI) * size
		);
		ctx.lineTo(x - Math.sin(angle) * size * 1.5, y + Math.cos(angle) * size * 1.5);
		ctx.fillStyle = '#e6dde6';
		ctx.fill();
	}

	function drawCircle(ctx: CanvasRenderingContext2D, x: number, y: number) {
		ctx.beginPath();
		ctx.arc(x, y, foodSize, 0, 2 * Math.PI);
		ctx.fillStyle = '#3cb088';
		ctx.fill();
	}

	function drawWold() {
		ctx?.clearRect(0, 0, cw, ch);

		if (!simulation) return;

		simulation.world().animals.forEach((a) => {
			const x = a.x * cw;
			const y = a.y * ch;
			const angle = a.rotation;
			if (!ctx) return;

			drawTriangle(ctx, x, y, angle);
		});

		simulation.world().foods.forEach((f) => {
			const x = f.x * cw;
			const y = f.y * ch;

			if (!ctx) return;
			drawCircle(ctx, x, y);
		});
	}

	const loop = (t: number) => {
		if (!simulation) return;
		const data = simulation.step();
		if (data) {
			stats = {
				min: data.stats.min,
				max: data.stats.max,
				avg: data.stats.avg,
				gen: data.info.generation
			};
		}
		drawWold();
		requestAnimationFrame(loop);
	};

	function train() {
		if (!simulation) return;
		const train = simulation.train();
		stats = {
			min: train.stats.min,
			max: train.stats.max,
			avg: train.stats.avg,
			gen: train.info.generation
		};
	}

	function randon() {
		simulation = new sim.Simulation(animals, foods);
		drawWold();
	}

	onMount(() => {
		ctx = canvas.getContext('2d');
		simulation = new sim.Simulation(animals, foods);
		drawWold();
	});

	function start() {
		requestAnimationFrame(loop);
	}

	$effect(() => {
		const a = animals;
		const f = foods;
		let to = setTimeout(() => {
			simulation = new sim.Simulation(a, f);
			drawWold();
		}, 100);

		return () => {
			clearTimeout(to);
		};
	});
</script>

<div class="controls">
	<div class="params">
		<label>
			Animals: {animals}
			<input type="range" min="5" max="100" bind:value={animals} />
		</label>
		<label>
			Foods: {foods}
			<input type="range" min="5" max="200" bind:value={foods} />
		</label>
	</div>

	<div class="buttons">
		<Tooltip content="Generate a new random simulation">
			<button onclick={randon}>Random</button>
		</Tooltip>
		<Tooltip content="Start the simulation from the beginning">
			<button onclick={start}>Start</button>
		</Tooltip>
		<Tooltip content="Train the current generation">
			<button onclick={train}>Train</button>
		</Tooltip>
	</div>
</div>

<canvas bind:this={canvas} width="800" height="600"></canvas>
<Stats {...stats} />

<style>
	canvas {
		width: 100%;
		height: 100%;
		border: rgba(255, 255, 255, 0.5) solid 1px;
		border-radius: 10px;
		background-color: rgba(255, 255, 255, 0.1);
	}
	label {
		display: flex;
		flex-direction: column;
	}

	.params {
		display: flex;
		gap: 1rem;
	}

	.controls {
		display: flex;
		gap: 1rem;
		justify-content: space-between;
	}

	.buttons {
		display: flex;
		gap: 1rem;
	}
</style>
