# Bird IA

Este proyecto es una simulación de vida artificial que utiliza algoritmos genéticos y redes neuronales para entrenar a una población de "pájaros" a sobrevivir en un mundo virtual. El objetivo de cada pájaro es encontrar comida y evitar los límites del mundo, aprendiendo y evolucionando a lo largo de generaciones.

## Como funciona

El comportamiento de cada pájaro está controlado por una red neuronal. Esta red neuronal toma como entrada la distancia y el ángulo a la comida más cercana, y produce como salida la velocidad y la rotación del pájaro.

El algoritmo genético se utiliza para evolucionar las redes neuronales de los pájaros a lo largo del tiempo. Los pájaros que tienen más éxito en la búsqueda de comida tienen más probabilidades de reproducirse y transmitir sus "genes" (los pesos de sus redes neuronales) a la siguiente generación. Esto conduce a una mejora gradual del comportamiento de la población de pájaros a lo largo del tiempo.

## Tecnologías utilizadas

- **Rust:** El núcleo de la simulación, el algoritmo genético y la red neuronal están implementados en Rust.
- **WebAssembly (WASM):** La simulación de Rust se compila en WebAssembly para que pueda ejecutarse en un navegador web.
- **Svelte:** La interfaz de usuario web para visualizar la simulación está construida con Svelte.

## Estructura del proyecto

El proyecto está organizado en los siguientes directorios:

- **`app`:** Una aplicación de escritorio (no utilizada actualmente) para ejecutar la simulación.
- **`libs`:** Las librerías principales del proyecto.
  - **`genetic-algorithm`:** Implementación del algoritmo genético.
  - **`neural-network`:** Implementación de la red neuronal.
  - **`simulation`:** La lógica de la simulación principal.
  - **`simulation-wasm`:** Un wrapper para compilar la simulación a WebAssembly.
- **`www`:** El proyecto web de Svelte que visualiza la simulación.

## Basado en

Este proyecto se inspiró y se basó en la serie de artículos [Learning to Fly](https://pwy.io/posts/learning-to-fly-pt2/) de primates.dev.