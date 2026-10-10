# Nyra

A lightweight 3D engine in Rust for anime-style games.

Early development. Renders a rotating square made of two triangles.

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

Window opens via winit. wgpu renders a square using four vertices and
six indices. The square rotates around the Z axis via a uniform matrix.

Vertex and index buffers are implemented. No 3D geometry, perspective
projection, camera, or depth buffer yet.

## Stack

- Rust (edition 2024)
- wgpu — rendering
- winit — window and input
- bytemuck — CPU↔GPU data transfer
- pollster — sync/async bridge
- glam — math (matrices, vectors)

Planned: egui (editor).

## License

MIT
