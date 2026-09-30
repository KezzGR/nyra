# Nyra

A lightweight 3D engine in Rust for anime-style games.

Early development. Renders a triangle. That's it for now.

## Why

Anime-style 3D in existing engines is painful. Toon shaders, VRM,
lip-sync — all third-party plugins, half abandoned. Nyra aims to ship
those out of the box.

## Goals

- Toon shading, VRM, morph targets, lip-sync — built in, not plugins.
- Text-based scene format, mergeable in git.
- Lightweight editor that runs on weak hardware.
- Stable API.

## Status

Window opens via winit. wgpu context clears the screen and draws
a triangle from a vertex buffer. No 3D yet, no camera, no depth.

## Stack

- Rust (edition 2024)
- wgpu — rendering
- winit — window and input
- bytemuck — CPU↔GPU data transfer
- pollster — sync/async bridge

Planned: glam (math), egui (editor).

## License

MIT