# Nyra

A lightweight 3D engine in Rust for anime-style games.

Early development. Nothing renders yet.

## Why

Anime-style 3D in existing engines is painful. Toon shaders, VRM,
lip-sync — all third-party plugins, half abandoned. Nyra aims to ship
those out of the box.

## Goals

- Toon shading, VRM, morph targets, lip-sync — built in, not plugins.
- Text-based scene format, mergeable in git.
- Lightweight editor that runs on weak hardware.
- Stable API.

## Stack

Rust, wgpu, winit, egui, glam.

## License

MIT