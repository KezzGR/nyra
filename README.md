# Nyra

A lightweight 3D engine in Rust for anime-style games.

Early development. Renders a procedurally generated spinning cube with
perspective projection, basic Lambert lighting and a depth buffer.
Depth is no longer a lie.

## Why

Nyra aims to bring anime-style character rendering tools together in a
lightweight engine: toon shading, VRM, morph targets, and lip-sync.

These are long-term goals, not implemented features.

## Goals

- Built-in toon shading, VRM, morph targets, and lip-sync.
- Text-based scene format, mergeable in git.
- Lightweight editor that runs on weak hardware.
- Simple, stable API.

## Status

- Window creation and resize handling via winit.
- Procedural cube and cuboid generation with configurable dimensions.
- Indexed cube geometry: 24 vertices with face normals, 36 indices.
- Custom Vec3 type in nyra_math, used for procedural mesh generation.
- Position, quaternion rotation, and scale transforms.
- Basic fixed camera with view and perspective projection matrices.
- Model and projection-view matrices passed through a uniform buffer.
- Directional Lambert lighting in WGSL.
- Depth testing and depth writes using a Depth32Float buffer.
- Depth buffer recreated when the window is resized.

Camera controls, textures, model loading, and toon shading are not
implemented.

## Stack

- Rust (edition 2024)
- wgpu — rendering
- winit — window and input
- bytemuck — CPU↔GPU data transfer
- pollster — sync/async bridge
- glam — transform and camera math (vectors, matrices, quaternions)
- nyra_math - custom vector math for procedural mesh generation

Planned: egui (editor).

## License

MIT
