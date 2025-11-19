<script lang="ts">
	import { onMount } from 'svelte';
	import * as sim from '../../../libs/simulation-wasm/pkg';

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;

	let x = 50;
	let y = 50;
	let ch = 600;
	let cw = 800;
	let last = 0;
	let size = 15;
	let foodSize = 5;

	const simulation = new sim.Simulation();

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

	const loop = (t: number) => {
		simulation.step();

		ctx?.clearRect(0, 0, cw, ch);

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

		requestAnimationFrame(loop);
	};

	onMount(() => {
		ctx = canvas.getContext('2d');
		requestAnimationFrame(loop);
	});
</script>

<canvas bind:this={canvas} width="800" height="600"></canvas>

<style>
	canvas {
		width: 100%;
		height: 100%;
		border: rgba(255, 255, 255, 0.5) solid 1px;
		border-radius: 10px;
		background-color: rgba(0, 0, 0, 0.5);
	}
</style>
